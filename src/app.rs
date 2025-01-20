use crate::client::GqlClient;
use crate::events::handle_events;
use crate::ui::compute_ui;
use ratatui::prelude::*;
use ratatui::Terminal;
use reqwest::Response;
use std::future::Future;
use std::io;
use std::pin::Pin;
use std::task::Poll;
use url::ParseError;

pub trait AsStaticStr {
    fn as_static_str(&self) -> &'static str;
}

/// state machine that handles the ui focus
#[derive(PartialEq)]
pub enum Focus {
    QueryEditor,
    ResponseView,
}

impl AsStaticStr for Focus {
    fn as_static_str(&self) -> &'static str {
        match self {
            Focus::QueryEditor => "QUERY EDITOR",
            Focus::ResponseView => "RESPONSE VIEW",
        }
    }
}

pub struct AppState<'a> {
    pub name: &'a str,
    pub focus: Focus,
    pub query: String,
    pub response: Option<String>,
    pub should_quit: bool,
}

impl<'a> AppState<'a> {
    /// Initializes the app state
    pub fn init(name: &'a str) -> Result<Self, ParseError> {
        Ok(Self {
            name,
            focus: Focus::QueryEditor,
            query: String::default(),
            response: None,
            should_quit: false,
        })
    }

    pub fn handle_focus_transition(&mut self) {
        match self.focus {
            Focus::QueryEditor => self.focus = Focus::ResponseView,
            Focus::ResponseView => self.focus = Focus::QueryEditor,
        }
    }

    pub fn set_query(&mut self, input: String) {
        self.query = input;
    }

    pub async fn run<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        gql_client: GqlClient<'_>,
    ) -> io::Result<()> {
        let mut maybe_request: Option<
            Pin<Box<dyn Future<Output = Result<Response, reqwest::Error>>>>,
        > = None;

        loop {
            terminal.draw(|f| compute_ui(f, self, &gql_client))?;
            if let Some(req) = handle_events(self, &gql_client) {
                maybe_request = Some(req);
            }

            if let Some(ref mut req) = maybe_request {
                if let Poll::Ready(result) = futures::poll!(req) {
                    if result.is_ok() {
                        todo!("handle successful request")
                    } else {
                        todo!("handle failed request")
                    }
                }
            }

            if self.should_quit {
                return Ok(());
            }
        }
    }
}
