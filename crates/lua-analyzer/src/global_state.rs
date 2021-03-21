use std::{path::PathBuf, time::Instant};

use anyhow::Result;
use crossbeam_channel::{unbounded, Receiver, Sender};
use rayon::{ThreadPool, ThreadPoolBuilder};
use rustc_hash::FxHashMap;

use crate::{main_loop::{Event, Task}, thread_pool::TaskPool};

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
            task_pool,
        }
    }

    pub(crate) fn snapshot(&self) -> GlobalStateSnapshot {
        GlobalStateSnapshot {}
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

    pub(crate) fn respond(&mut self, response: lsp_server::Response) {
        if let Some((method, start)) = self.req_queue.incoming.complete(response.id.clone()) {
            let duration = start.elapsed();
            log::info!("handled req#{} in {:?}", response.id, duration);
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
}

pub(crate) struct GlobalStateSnapshot {}
