use std::{path::PathBuf, sync::Arc, time::Instant};

use anyhow::Result;
use crossbeam_channel::{unbounded, Receiver, Sender};
use ide::{Analysis, AnalysisHost};
use lsp_server::{Notification, Request};
use parking_lot::RwLock;
use rayon::{ThreadPool, ThreadPoolBuilder};
use rustc_hash::FxHashMap;

use crate::{
    dispatch::{NotificationDispatcher, RequestDispatcher},
    main_loop::{Event, Task},
    thread_pool::TaskPool,
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
    pub(crate) sender: Sender<lsp_server::Message>,
    pub(crate) task_pool: Handle<TaskPool<Task>, Receiver<Task>>,
    pub(crate) vfs: Arc<RwLock<vfs::Vfs>>,
    pub(crate) analysis_host: AnalysisHost,
}

impl GlobalState {
    pub(crate) fn new(sender: Sender<lsp_server::Message>) -> GlobalState {
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
            analysis_host: AnalysisHost::new(),
        }
    }

    pub(crate) fn snapshot(&self) -> GlobalStateSnapshot {
        GlobalStateSnapshot {
            analysis: self.analysis_host.analysis(),
            vfs: self.vfs.clone(),
        }
    }

    pub(crate) fn send_request<R: lsp_types::request::Request>(
        &mut self,
        params: R::Params,
        handler: ReqHandler,
    ) {
        let request = self
            .req_queue
            .outgoing
            .register(R::METHOD.to_string(), params, handler);
        self.send(request.into());
    }

    pub(crate) fn complete_request(&mut self, response: lsp_server::Response) {
        let handler = self.req_queue.outgoing.complete(response.id.clone());
        handler(self, response)
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
        self.req_queue.incoming.register(
            request.id.clone(),
            (request.method.clone(), request_received),
        );
    }

    pub(crate) fn respond(&mut self, response: lsp_server::Response) {
        if let Some((method, start)) = self.req_queue.incoming.complete(response.id.clone()) {
            let duration = start.elapsed();
            log::info!("handled req#{} in {:?}", response.id, duration);
            self.send(response.into());
        }
    }

    pub(crate) fn notification_dispatcher(&mut self, not: Notification) -> NotificationDispatcher {
        NotificationDispatcher {
            not: Some(not),
            global_state: self,
        }
    }

    pub(crate) fn request_dispatcher(&mut self, req: Request) -> RequestDispatcher {
        RequestDispatcher {
            req: Some(req),
            global_state: self,
        }
    }
}

pub(crate) struct GlobalStateSnapshot {
    analysis: Analysis,
    vfs: Arc<RwLock<vfs::Vfs>>,
}
