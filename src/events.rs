use std::{future::Future, pin::Pin};

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use reqwest::Response;

use crate::{app::AppState, client::GqlClient};

pub async fn handle_events(
    app: &mut AppState<'_>,
    client: &GqlClient<'_>,
) -> Option<Pin<Box<dyn Future<Output = Result<Response, reqwest::Error>>>>> {
    if let Ok(Event::Key(key)) = event::read() {
        if key.kind == KeyEventKind::Release {
            return None;
        }
        match key.code {
            KeyCode::Char(char) => {
                if key.modifiers == KeyModifiers::CONTROL && (char == 'q' || char == 'c') {
                    app.should_quit = true;
                    return None;
                }
            }
            KeyCode::Backspace => {}
            KeyCode::Enter => {
                if let Ok(request) = client.build_request(app.query.as_str()) {
                    return Some(Box::pin(request.send()));
                }
            }
            _ => {}
        };
    }
    return None;
}
