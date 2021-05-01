use std::time::Instant;

use anyhow::{anyhow, bail, Result};

use crossbeam_channel::{select, Receiver};
use ide::FileId;
use log::info;
use lsp_server::{Connection, Notification, Request, Response};
use lsp_types::{notification::Notification as _, Diagnostic};

use crate::{
    config::Config,
    dispatch::{NotificationDispatcher, RequestDispatcher},
    document::DocumentData,
    from_proto,
    global_state::{file_id_to_url, GlobalState},
    handlers,
    lsp_utils::apply_document_changes,
    to_proto::{self, url_from_abs_path},
};

#[derive(Debug)]
pub(crate) enum Event {
    Lsp(lsp_server::Message),
    Task(Task),
}

#[derive(Debug)]
pub(crate) enum Task {
    Response(Response),
    Diagnostics(Vec<(FileId, Vec<Diagnostic>)>),
}

pub fn main_loop(config: Config, connection: Connection) -> Result<()> {
    GlobalState::new(connection.sender, config).run(connection.receiver)
}

impl GlobalState {
    fn run(mut self, inbox: Receiver<lsp_server::Message>) -> Result<()> {
        while let Some(event) = self.next_event(&inbox) {
            if let Event::Lsp(lsp_server::Message::Notification(not)) = &event {
                if not.method == lsp_types::notification::Exit::METHOD {
                    log::error!("We exited properly");
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
                    Task::Diagnostics(diagnostics_per_file) => {
                        for (file_id, diagnostics) in diagnostics_per_file {
                            self.diagnostics.set_native_diagnostics(file_id, diagnostics)
                        }
                    }
                }

                task = match self.task_pool.receiver.try_recv() {
                    Ok(task) => task,
                    Err(_) => break,
                };
            },
        }

        let state_changed = self.process_changes();

        if state_changed {
            self.maybe_update_diagnostics();
        }

        if let Some(diagnostic_changes) = self.diagnostics.take_changes() {
            for file_id in diagnostic_changes {
                let url = file_id_to_url(&self.vfs.read(), file_id);
                let diagnostics = self.diagnostics.diagnostics_for(file_id).cloned().collect();
                let version = from_proto::abs_path(&url)
                    .map(|path| self.mem_docs.get(&path).map(|it| it.version))
                    .unwrap_or_default();

                self.send_notification::<lsp_types::notification::PublishDiagnostics>(
                    lsp_types::PublishDiagnosticsParams { uri: url, diagnostics, version },
                );
            }
        }

        Ok(())
    }

    fn on_request(&mut self, request_received: Instant, req: Request) -> Result<()> {
        use lsp_types::request::*;

        self.register_request(&req, request_received);

        if self.shutdown_requested {
            self.respond(Response::new_err(
                req.id,
                lsp_server::ErrorCode::InvalidRequest as i32,
                "Shutdown already requested.".to_owned(),
            ));

            return Ok(());
        }

        self.request_dispatcher(req)
            .on_sync::<Shutdown>(|s, ()| {
                s.shutdown_requested = true;
                Ok(())
            })?
            .on::<Completion>(handlers::handle_completion)
            .finish();

        Ok(())
    }

    fn on_notification(&mut self, not: Notification) -> Result<()> {
        use lsp_types::notification::*;

        self.notification_dispatcher(not)
            .on::<DidOpenTextDocument>(|this, params| {
                if let Ok(path) = from_proto::abs_path(&params.text_document.uri) {
                    if this
                        .mem_docs
                        .insert(path.clone(), DocumentData::new(params.text_document.version))
                        .is_some()
                    {
                        log::error!("duplicate DidOpenTextDocument: {}", path.display())
                    }

                    let changed = this
                        .vfs
                        .write()
                        .set_file_contents(path, Some(params.text_document.text.into_bytes()));

                    // If the VFS contents are unchanged, update diagnostics, since `handle_event`
                    // won't see any changes. This avoids missing diagnostics when opening a file.
                    //
                    // If the file *was* changed, `handle_event` will already recompute and send
                    // diagnostics. We can't do it here, since the *current* file contents might be
                    // unset in salsa, since the VFS change hasn't been applied to the database yet.
                    if !changed {
                        this.maybe_update_diagnostics();
                    }
                }
                Ok(())
            })?
            .on::<DidChangeTextDocument>(|this, params| {
                if let Ok(path) = from_proto::abs_path(&params.text_document.uri) {
                    let doc = match this.mem_docs.get_mut(&path) {
                        Some(doc) => doc,
                        None => {
                            log::error!("expected DidChangeTextDocument: {}", path.display());
                            return Ok(());
                        }
                    };
                    let vfs = &mut this.vfs.write();
                    let file_id = vfs.file_id(&path).unwrap();
                    let mut text = String::from_utf8(vfs.file_contents(file_id).to_vec()).unwrap();
                    apply_document_changes(&mut text, params.content_changes);

                    doc.version = params.text_document.version;

                    vfs.set_file_contents(path.clone(), Some(text.into_bytes()));
                }
                Ok(())
            })?
            .on::<DidCloseTextDocument>(|this, params| {
                let mut version = None;
                if let Ok(path) = from_proto::abs_path(&params.text_document.uri) {
                    match this.mem_docs.remove(&path) {
                        Some(doc) => version = Some(doc.version),
                        None => log::error!("orphan DidCloseTextDocument: {}", path.display()),
                    }
                }

                // Clear the diagnostics for the previously known version of the file.
                // This prevents stale "cargo check" diagnostics if the file is
                // closed, "cargo check" is run and then the file is reopened.
                this.send_notification::<lsp_types::notification::PublishDiagnostics>(
                    lsp_types::PublishDiagnosticsParams {
                        uri: params.text_document.uri,
                        diagnostics: Vec::new(),
                        version,
                    },
                );
                Ok(())
            })?
            .on::<DidSaveTextDocument>(|this, params| Ok(()))?
            .on::<DidChangeConfiguration>(|this, _params| {
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
