use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{app::AsStaticStr, AppState};

pub fn compute_ui(frame: &mut Frame, app: &AppState) {
    let viewport_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .split(frame.area());

    let title_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(*viewport_chunks.first().unwrap());

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Length(app.mode.as_static_str().len() as u16 + 6),
            Constraint::Fill(1),
        ])
        .split(*viewport_chunks.last().unwrap());

    let title_center_block = Block::default().style(Style::default().bg(Color::White));
    let title_center = Paragraph::new(Span::raw("Postr").red().bold().underlined())
        .block(title_center_block)
        .alignment(Alignment::Center);
    let title_left = Block::default().on_white();
    let title_right = Block::default().on_white();

    let query_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    let query = Paragraph::new(Span::raw(&app.query.url)).block(query_block);
    let query_label = Paragraph::new(Span::raw("Query:"));

    let footer_left = Line::from(vec![
        Span::raw("Mode: "),
        Span::raw(app.mode.as_static_str()).bold(),
    ])
    .red()
    .on_white();

    frame.render_widget(title_left, title_chunks[0]);
    frame.render_widget(title_center, title_chunks[1]);
    frame.render_widget(title_right, title_chunks[2]);
    frame.render_widget(query_label, viewport_chunks[2]);
    frame.render_widget(query, viewport_chunks[3]);
    frame.render_widget(footer_left, footer_chunks[0]);
}
