use crate::client::GqlClient;
use crate::events::handle_events;
use crate::parser::QueryParser;
use crate::tui::compute_ui;
use graphql_parser::Pos;
use ratatui::prelude::*;
use ratatui::Terminal;
use reqwest::Response;
use std::future::Future;
use std::io;
use std::pin::Pin;
use std::task::Poll;
use url::ParseError as UrlParseError;

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

pub struct App<'a> {
    pub name: &'a str,
    pub focus: Focus,
    pub query: String,
    pub query_cursor: Pos,
    pub response: String,
    pub should_quit: bool,
}

impl<'a, 'query> App<'a> {
    /// Initializes the app state
    pub fn init(name: &'a str) -> Result<Self, UrlParseError> {
        Ok(Self {
            name,
            focus: Focus::QueryEditor,
            query: String::default(),
            query_cursor: Pos { column: 1, line: 1 },
            response: String::new(),
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

    // pub fn parse_query(&mut self) {
    //     self.query_ast = parse_query(self.query.as_str());
    // }

    pub async fn run<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        gql_client: GqlClient<'_>,
    ) -> io::Result<()> {
        let mut maybe_request: Option<
            Pin<Box<dyn Future<Output = Result<Response, reqwest::Error>>>>,
        > = None;

        let mut query_parser: QueryParser;

        // parse and format the initial query
        if let Ok(formatted_query) = QueryParser::parse_and_serialize(self.query.as_str()) {
            self.set_query(formatted_query);
            query_parser = QueryParser::from_query_str(self.query.as_str());
            query_parser.set_operation();
        } else {
            todo!()
        }

        dbg!(&query_parser);
        loop {
            terminal.draw(|f| compute_ui(f, self, &gql_client))?;
            if let Some(req) = handle_events(self, &gql_client) {
                maybe_request = Some(req);
            }

            if let Some(ref mut req) = maybe_request {
                if let Poll::Ready(result) = futures::poll!(req) {
                    if let Ok(res) = result {
                        self.response = res.text().await.unwrap();
                        maybe_request = None;
                    } else {
                        todo!("handle failed response")
                    }
                }
            };
            if self.should_quit {
                return Ok(());
            }
        }
    }
}
