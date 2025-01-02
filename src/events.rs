use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};

use crate::app::AppState;

/// Handles key events events and returns true if the app should exit
pub fn handle_events(app: &mut AppState) -> bool {
    if let Ok(Event::Key(key)) = event::read() {
        if key.kind == KeyEventKind::Release {
            return false;
        }

        match key.code {
            KeyCode::Char(char) => {
                if key.modifiers == KeyModifiers::CONTROL && (char == 'q' || char == 'c') {
                    return true;
                }
            }

            KeyCode::Backspace => {
                return false;
            }

            KeyCode::Enter => {
                // TODO: Implement logic to handle request
                return false;
            }

            _ => {
                return false;
            }
        };
    }
    return false;
}
