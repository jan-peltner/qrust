use std::{future::Future, pin::Pin, time::Duration};

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use reqwest::Response;

use crate::{app::AppState, client::GqlClient};

pub fn handle_events(
    app: &mut AppState<'_>,
    client: &GqlClient<'_>,
) -> Option<Pin<Box<dyn Future<Output = Result<Response, reqwest::Error>>>>> {
    if is_event_available() {
        if let Ok(Event::Key(key)) = event::read() {
            if key.kind == KeyEventKind::Release {
                return None;
            }
            match key.code {
                KeyCode::Char(char) => {
                    if key.modifiers == KeyModifiers::CONTROL && (char == 'q' || char == 'c') {
                        app.should_quit = true;
                    }
                }
                KeyCode::Backspace => {}
                KeyCode::Tab => {
                    app.handle_focus_transition();
                }
                KeyCode::Enter => {
                    if let Ok(request) = client.build_request(app.query.as_str()) {
                        dbg!(&request);
                        return Some(Box::pin(request.send()));
                    }
                }
                _ => {}
            };
        }
    }

    return None;
}

fn is_event_available() -> bool {
    if let Ok(is_available) = event::poll(Duration::from_millis(100)) {
        is_available
    } else {
        false
    }
}
