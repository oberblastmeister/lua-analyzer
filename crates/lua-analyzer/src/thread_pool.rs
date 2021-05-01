use crossbeam_channel::Sender;
use rayon::{ThreadPool, ThreadPoolBuilder};

pub(crate) struct TaskPool<T> {
    sender: Sender<T>,
    inner: ThreadPool,
}

impl<T> TaskPool<T> {
    pub(crate) fn new(sender: Sender<T>) -> TaskPool<T> {
        let thread_pool =
            ThreadPoolBuilder::new().build().expect("Failed to initialize threadpool");
        TaskPool { sender, inner: thread_pool }
    }

    pub(crate) fn spawn<F>(&mut self, task: F)
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        self.inner.install({
            let sender = self.sender.clone();
            move || sender.send(task()).unwrap()
        })
    }

    pub(crate) fn spawn_with_sender<F>(&mut self, task: F)
    where
        F: FnOnce(Sender<T>) + Send + 'static,
        T: Send + 'static,
    {
        self.inner.install({
            let sender = self.sender.clone();
            move || task(sender)
        })
    }
}
