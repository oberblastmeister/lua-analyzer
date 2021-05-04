use std::fmt;

use stdx::paths::{AbsPath, AbsPathBuf};

#[derive(Debug)]
pub struct Entry {
    pub root: AbsPathBuf,
}

impl Entry {
    pub fn new(root: AbsPathBuf) -> Self {
        Self { root }
    }
}

#[derive(Debug)]
pub struct Config {
    /// Version number to associate progress updates to the right config
    /// version.
    pub version: u32,

    pub load: Vec<Entry>,
}

/// Message about an action taken by a [`Handle`].
pub enum Message {
    /// Indicate a gradual progress.
    ///
    /// This is supposed to be the number of loaded files.
    Progress { n_total: usize, n_done: usize, config_version: u32 },
    /// The handle loaded the following files' content.
    Loaded { files: Vec<(AbsPathBuf, Option<Vec<u8>>)> },
}

/// Type that will receive [`Messages`](Message) from a [`Handle`].
pub type Sender = Box<dyn Fn(Message) + Send>;

pub trait Handle: fmt::Debug {
    fn spawn(sender: Sender) -> Self
    where
        Self: Sized;

    fn set_config(&mut self, config: Config);

    fn invalidate(&mut self, path: AbsPathBuf);

    fn load_sync(&mut self, path: &AbsPath) -> Option<Vec<u8>>;
}

impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Message::Loaded { files } => {
                f.debug_struct("Loaded").field("n_files", &files.len()).finish()
            }
            Message::Progress { n_total, n_done, config_version } => f
                .debug_struct("Progress")
                .field("n_total", n_total)
                .field("n_done", n_done)
                .field("config_version", config_version)
                .finish(),
        }
    }
}
