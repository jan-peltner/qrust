use app::AppState;
use client::GqlClient;
use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::prelude::*;
use ratatui::Terminal;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use std::{env, io};

mod app;
mod client;
mod events;
mod ui;

#[derive(Deserialize)]
struct Config {
    name: String,
    endpoint: String,
    headers: HashMap<String, String>,
}

impl Config {
    fn to_header_map(&self) -> Result<HeaderMap, Box<dyn Error>> {
        let mut header_map = HeaderMap::new();
        for (k, v) in self.headers.iter() {
            header_map.try_insert(HeaderName::try_from(k)?, HeaderValue::try_from(v)?)?;
        }
        Ok(header_map)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // prepare terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // parse qrust config
    let cfg = read_to_string(format!(
        "{}/.config/qrust/workspace.json",
        env::var("HOME").unwrap()
    ))?;
    let parsed_cfg = serde_json::from_str::<Config>(&cfg)?;

    // construct GraphQL client
    let gql_client = GqlClient::from_config(&parsed_cfg)?;

    // start app and execute render loop
    let mut app = AppState::init(&parsed_cfg.name)?;
    app.set_query(
        r#"
    query {
        capsule {
            id
        }
    }
    "#
        .to_string(),
    );

    let _ = app.run(&mut terminal, gql_client).await;

    // clean up after app is done
    // this is run even if `run_app` returns an error
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
