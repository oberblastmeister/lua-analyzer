use std::fmt;

use crossbeam_channel::{select, unbounded, Receiver, Sender};
use log::info;
use stdx::paths::{AbsPath, AbsPathBuf};
use walkdir::WalkDir;

use crate::handle::{self, Config};

#[derive(Debug)]
pub struct FileLoaderHandle {
    sender: Sender<Message>,
    thread: jod_thread::JoinHandle,
}

#[derive(Debug)]
pub enum Message {
    Config(handle::Config),
    Invalidate(AbsPathBuf),
}

impl handle::Handle for FileLoaderHandle {
    fn spawn(sender: handle::Sender) -> Self
    where
        Self: Sized,
    {
        let actor = FileLoaderActor::new(sender);
        let (sender, receiver) = unbounded::<Message>();
        let thread = jod_thread::spawn(move || actor.run(receiver));
        FileLoaderHandle { sender, thread }
    }

    fn set_config(&mut self, config: Config) {
        self.sender.send(Message::Config(config)).unwrap();
    }

    fn invalidate(&mut self, path: AbsPathBuf) {
        self.sender.send(Message::Invalidate(path)).unwrap();
    }

    fn load_sync(&mut self, path: &AbsPath) -> Option<Vec<u8>> {
        read(path)
    }
}

struct FileLoaderActor {
    sender: handle::Sender,
}

impl FileLoaderActor {
    fn new(sender: handle::Sender) -> FileLoaderActor {
        FileLoaderActor { sender }
    }

    fn next_event(&self, receiver: &Receiver<Message>) -> Option<Message> {
        select! {
            recv(receiver) -> it => it.ok()
        }
    }

    fn run(mut self, inbox: Receiver<Message>) {
        while let Some(event) = self.next_event(&inbox) {
            info!("vfs file loader event: {:#?}", event);

            match event {
                Message::Config(config) => {
                    let config_version = config.version;

                    let n_total = config.load.len();
                    self.send(handle::Message::Progress { n_total, n_done: 0, config_version });

                    for (i, entry) in config.load.into_iter().enumerate() {
                        let files = self.load_entry(entry);
                        self.send(handle::Message::Loaded { files });
                        self.send(handle::Message::Progress {
                            n_total,
                            n_done: i + 1,
                            config_version,
                        });
                    }
                }
                Message::Invalidate(path) => {
                    let contents = read(path.as_path());
                    let files = vec![(path, contents)];
                    self.send(handle::Message::Loaded { files });
                }
            }
        }
    }

    fn load_entry(&mut self, entry: handle::Entry) -> Vec<(AbsPathBuf, Option<Vec<u8>>)> {
        let mut res = Vec::new();

        let walkdir = WalkDir::new(entry.root).follow_links(true).into_iter();

        let files = walkdir.filter_map(Result::ok).filter_map(|entry| {
            let is_file = entry.file_type().is_file();
            let abs_path = AbsPathBuf::assert(entry.into_path());
            if !is_file {
                return None;
            }
            let ext = abs_path.extension().unwrap_or_default();
            if ext != "lua" {
                return None;
            }
            Some(abs_path)
        });

        res.extend(files.map(|file| {
            let contents = read(file.as_path());
            (file, contents)
        }));

        res
    }

    fn send(&mut self, msg: handle::Message) {
        (self.sender)(msg)
    }
}

fn read(path: &AbsPath) -> Option<Vec<u8>> {
    std::fs::read(path).ok()
}
