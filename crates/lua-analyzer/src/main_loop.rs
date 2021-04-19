use std::time::Instant;

use anyhow::{anyhow, bail, Result};

use crossbeam_channel::{select, Receiver};
use log::info;
use lsp_server::{Connection, Notification, Request, Response};
use lsp_types::notification::Notification as _;

use crate::{
    config::Config,
    dispatch::{NotificationDispatcher, RequestDispatcher},
    from_proto,
    global_state::GlobalState,
    handlers,
};

#[derive(Debug)]
pub(crate) enum Event {
    Lsp(lsp_server::Message),
    Task(Task),
}

#[derive(Debug)]
pub(crate) enum Task {
    Response(Response),
    Diagnostics,
}

pub fn main_loop(config: Config, connection: Connection) -> Result<()> {
    GlobalState::new(connection.sender, config).run(connection.receiver)
}

impl GlobalState {
    fn run(mut self, inbox: Receiver<lsp_server::Message>) -> Result<()> {
        while let Some(event) = self.next_event(&inbox) {
            if let Event::Lsp(lsp_server::Message::Notification(not)) = &event {
                if not.method == lsp_types::notification::Exit::METHOD {
                    return Ok(());
                }
            }
            self.handle_event(event)?;
        }

        bail!("client exited without proper shutdown sequence")
    }

    fn next_event(&self, inbox: &Receiver<lsp_server::Message>) -> Option<Event> {
        select! {
            recv(inbox) -> msg =>
                msg.ok().map(Event::Lsp),

            recv(self.task_pool.receiver) -> task =>
                Some(Event::Task(task.unwrap())),
        }
    }

    fn handle_event(&mut self, event: Event) -> Result<()> {
        let loop_start = Instant::now();
        match event {
            Event::Lsp(msg) => match msg {
                lsp_server::Message::Request(req) => self.on_request(loop_start, req)?,
                lsp_server::Message::Notification(not) => {
                    self.on_notification(not)?;
                }
                lsp_server::Message::Response(resp) => self.complete_request(resp),
            },
            Event::Task(mut task) => loop {
                match task {
                    Task::Response(response) => self.respond(response),
                    Task::Diagnostics => todo!(),
                }

                task = match self.task_pool.receiver.try_recv() {
                    Ok(task) => task,
                    Err(_) => break,
                };
            },
        }

        Ok(())
    }

    fn on_request(&mut self, request_received: Instant, req: Request) -> Result<()> {
        use lsp_types::request::*;

        self.register_request(&req, request_received);

        self.request_dispatcher(req)
            .on::<Completion>(handlers::handle_completion)
            .finish();

        Ok(())
    }

    fn on_notification(&mut self, not: Notification) -> Result<()> {
        use lsp_types::notification::*;

        self.notification_dispatcher(not)
            .on::<DidOpenTextDocument>(|this, params| {
                if let Ok(path) = from_proto::abs_path(&params.text_document.uri) {
                    let changed = this
                        .vfs
                        .write()
                        .set_file_contents(path, Some(params.text_document.text.into_bytes()));
                }
                Ok(())
            })?
            .on::<DidChangeTextDocument>(|this, params| {
                if let Ok(path) = from_proto::abs_path(&params.text_document.uri) {
                    let vfs = &mut this.vfs.write();
                    let file_id = vfs.file_id(&path).unwrap();
                    let mut text = String::from_utf8(vfs.file_contents(file_id).to_vec()).unwrap();
                    // apply_document_changes(&mut text, params.content_changes);

                    vfs.set_file_contents(path.clone(), Some(text.into_bytes()));
                }
                Ok(())
            })?
            .on::<DidSaveTextDocument>(|this, params| Ok(()))?
            .on::<lsp_types::notification::DidChangeConfiguration>(|this, _params| {
                // As stated in https://github.com/microsoft/language-server-protocol/issues/676,
                // this notification's parameters should be ignored and the actual config queried separately.
                this.send_request::<lsp_types::request::WorkspaceConfiguration>(
                    lsp_types::ConfigurationParams {
                        items: vec![lsp_types::ConfigurationItem {
                            scope_uri: None,
                            section: Some("lua-analyzer".to_string()),
                        }],
                    },
                    |this, resp| {
                        log::debug!("config update response: '{:?}", resp);
                        let Response { error, result, .. } = resp;

                        match (error, result) {
                            (Some(err), _) => {
                                log::error!("failed to fetch the server settings: {:?}", err)
                            }
                            (None, Some(mut configs)) => {
                                if let Some(json) = configs.get_mut(0) {
                                    // Note that json can be null according to the spec if the client can't
                                    // provide a configuration. This is handled in Config::update below.
                                    let mut config = Config::clone(&*this.config);
                                    config.update(json.take());
                                    this.update_configuration(config);
                                }
                            }
                            (None, None) => log::error!(
                                "received empty server settings response from the client"
                            ),
                        }
                    },
                );

                return Ok(());
            })?
            .finish();

        Ok(())
    }
}
