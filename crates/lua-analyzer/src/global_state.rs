use std::time::Instant;

use crossbeam_channel::{unbounded, Receiver, Sender};
use rayon::{ThreadPool, ThreadPoolBuilder};

pub(crate) type ReqHandler = fn(&mut GlobalState, lsp_server::Response);
pub(crate) type ReqQueue = lsp_server::ReqQueue<(String, Instant), ReqHandler>;

pub struct GlobalState {
    req_queue: ReqQueue,
    pub(crate) sender: Sender<lsp_server::Message>,
    pub(crate) thread_pool: ThreadPool,
}

impl GlobalState {
    pub(crate) fn new(sender: Sender<lsp_server::Message>) -> GlobalState {
        let thread_pool = ThreadPoolBuilder::new()
            .build()
            .expect("Failed to initialize threadpool");

        GlobalState {
            req_queue: ReqQueue::default(),
            sender,
            thread_pool,
        }
    }

    pub(crate) fn snapshot(&self) -> GlobalStateSnapshot {
        GlobalStateSnapshot {}
    }

    pub(crate) fn response(&mut self, response: lsp_server::Response) {}

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
