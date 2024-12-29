use app::AppState;
use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::prelude::*;
use ratatui::Terminal;
use std::error::Error;
use std::io;
use ui::compute_ui;

mod app;
mod events;
mod ui;

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut AppState) -> io::Result<()> {
    loop {
        terminal.draw(|f| compute_ui(f, app))?;
        if events::handle_events(app) {
            return Ok(());
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // prepare terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // start app and execute render loop
    let mut app = AppState::init(None);
    let _ = run_app(&mut terminal, &mut app);

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
