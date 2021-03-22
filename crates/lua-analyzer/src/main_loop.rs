use std::time::Instant;

use anyhow::{anyhow, bail, Result};

use crossbeam_channel::{select, Receiver};
use log::info;
use lsp_server::{Connection, Notification, Request, Response};
use lsp_types::notification::Notification as _;

use crate::{
    dispatch::{NotificationDispatcher, RequestDispatcher},
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

pub fn main_loop(connection: Connection) -> Result<()> {
    GlobalState::new(connection.sender).run(connection.receiver)
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
                info!("Did open text document");
                Ok(())
            })?
            .on::<DidChangeTextDocument>(|this, params| {
                info!("Did change text document");
                Ok(())
            })?
            .on::<DidSaveTextDocument>(|this, params| {
                info!("Did save text document");
                Ok(())
            })?
            .finish();

        Ok(())
    }
}
