use std::{path::PathBuf, sync::Arc, time::Instant};

use anyhow::Result;
use crossbeam_channel::{unbounded, Receiver, Sender};
use ide::{Analysis, AnalysisHost, Cancelable, Change, FileId, LineIndex};
use lsp_server::{Notification, Request};
use lsp_types::Url;
use parking_lot::RwLock;
use rayon::{ThreadPool, ThreadPoolBuilder};
use rustc_hash::FxHashMap;
use stdx::paths::AbsPathBuf;

use crate::{
    config::Config,
    diagnostics::DiagnosticCollection,
    dispatch::{NotificationDispatcher, RequestDispatcher},
    document::DocumentData,
    handlers,
    lsp_utils::is_canceled,
    main_loop::{Event, Task},
    thread_pool::TaskPool,
    to_proto::url_from_abs_path,
};

pub(crate) type ReqHandler = fn(&mut GlobalState, lsp_server::Response);
pub(crate) type ReqQueue = lsp_server::ReqQueue<(String, Instant), ReqHandler>;

// Enforces drop order
pub(crate) struct Handle<H, C> {
    pub(crate) handle: H,
    pub(crate) receiver: C,
}

pub struct GlobalState {
    req_queue: ReqQueue,
    pub(crate) diagnostics: DiagnosticCollection,
    pub(crate) shutdown_requested: bool,
    pub(crate) sender: Sender<lsp_server::Message>,
    pub(crate) task_pool: Handle<TaskPool<Task>, Receiver<Task>>,
    pub(crate) mem_docs: FxHashMap<AbsPathBuf, DocumentData>,
    pub(crate) vfs: Arc<RwLock<vfs::Vfs>>,
    pub(crate) config: Arc<Config>,
    pub(crate) analysis_host: AnalysisHost,
}

impl GlobalState {
    pub(crate) fn new(sender: Sender<lsp_server::Message>, config: Config) -> GlobalState {
        let task_pool = {
            let (sender, receiver) = unbounded();
            let handle = TaskPool::new(sender);
            Handle { handle, receiver }
        };

        GlobalState {
            req_queue: ReqQueue::default(),
            sender,
            vfs: Arc::new(RwLock::new(vfs::Vfs::default())),
            task_pool,
            mem_docs: FxHashMap::default(),
            config: Arc::new(config),
            analysis_host: AnalysisHost::new(),
            diagnostics: Default::default(),
            shutdown_requested: false,
        }
    }

    pub(crate) fn process_changes(&mut self) -> bool {
        let change = {
            let mut change = Change::new();
            let vfs = &mut *self.vfs.write();
            let changed_files = vfs.take_changes();
            if changed_files.is_empty() {
                return false;
            }

            for file in changed_files {
                let text = if file.exists() {
                    let bytes = vfs.file_contents(file.file_id).to_vec();
                    match String::from_utf8(bytes).ok() {
                        Some(text) => Some(Arc::new(text)),
                        None => None,
                    }
                } else {
                    None
                };
                change.change_file(file.file_id, text);
            }
            change
        };

        self.analysis_host.apply_change(change);

        true
    }

    pub(crate) fn snapshot(&self) -> GlobalStateSnapshot {
        GlobalStateSnapshot { analysis: self.analysis_host.analysis(), vfs: self.vfs.clone() }
    }

    pub(crate) fn send_request<R: lsp_types::request::Request>(
        &mut self,
        params: R::Params,
        handler: ReqHandler,
    ) {
        let request = self.req_queue.outgoing.register(R::METHOD.to_string(), params, handler);
        self.send(request.into());
    }

    pub(crate) fn complete_request(&mut self, response: lsp_server::Response) {
        let handler = self.req_queue.outgoing.complete(response.id.clone());
        handler(self, response)
    }

    pub(crate) fn send_notification<N: lsp_types::notification::Notification>(
        &mut self,
        params: N::Params,
    ) {
        let not = lsp_server::Notification::new(N::METHOD.to_string(), params);
        self.send(not.into());
    }

    pub(crate) fn cancel(&mut self, request_id: lsp_server::RequestId) {
        if let Some(response) = self.req_queue.incoming.cancel(request_id) {
            self.send(response.into());
        }
    }

    fn send(&mut self, message: lsp_server::Message) {
        self.sender.send(message).unwrap()
    }

    pub(crate) fn register_request(
        &mut self,
        request: &lsp_server::Request,
        request_received: Instant,
    ) {
        self.req_queue
            .incoming
            .register(request.id.clone(), (request.method.clone(), request_received));
    }

    pub(crate) fn respond(&mut self, response: lsp_server::Response) {
        if let Some((method, start)) = self.req_queue.incoming.complete(response.id.clone()) {
            let duration = start.elapsed();
            log::info!("handled req#{} in {:?}", response.id, duration);
            self.send(response.into());
        }
    }

    pub(crate) fn notification_dispatcher(&mut self, not: Notification) -> NotificationDispatcher {
        NotificationDispatcher { not: Some(not), global_state: self }
    }

    pub(crate) fn request_dispatcher(&mut self, req: Request) -> RequestDispatcher {
        RequestDispatcher { req: Some(req), global_state: self }
    }

    pub(crate) fn maybe_update_diagnostics(&mut self) {
        let subscriptions = self
            .mem_docs
            .keys()
            .map(|path| self.vfs.read().file_id(&path).unwrap())
            .collect::<Vec<_>>();

        log::trace!("updating notifications for {:?}", subscriptions);
        let snapshot = self.snapshot();
        self.task_pool.handle.spawn(move || {
            let diagnostics = subscriptions
                .into_iter()
                .filter_map(|file_id| {
                    handlers::publish_diagnostics(&snapshot, file_id)
                        .map_err(|err| {
                            if !is_canceled(&*err) {
                                log::error!("failed to compute diagnostics: {:?}", err);
                            }
                            ()
                        })
                        .ok()
                        .map(|diags| (file_id, diags))
                })
                .collect::<Vec<_>>();
            Task::Diagnostics(diagnostics)
        })
    }
}

pub(crate) struct GlobalStateSnapshot {
    pub(crate) analysis: Analysis,
    vfs: Arc<RwLock<vfs::Vfs>>,
}

impl GlobalStateSnapshot {
    pub(crate) fn file_line_index(&self, file_id: FileId) -> Cancelable<Arc<LineIndex>> {
        Ok(self.analysis.file_line_index(file_id)?)
    }
}

pub(crate) fn file_id_to_url(vfs: &vfs::Vfs, id: FileId) -> Url {
    let path = vfs.file_path(id);
    url_from_abs_path(&path)
}
