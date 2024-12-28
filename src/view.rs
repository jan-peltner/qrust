use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Style,
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::AppState;

pub fn compute_ui(frame: &mut Frame, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .split(frame.area());

    let query_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let query = Paragraph::new(Span::raw(&app.query_url)).block(query_block);
    let query_label = Paragraph::new(Span::raw("Query:"));

    frame.render_widget(query_label, chunks[1]);
    frame.render_widget(query, chunks[2]);
}
