use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::rc::Rc;

use crate::{app::AsStaticStr, AppState};

pub fn compute_ui(frame: &mut Frame, app: &AppState) {
    let viewport_chunks = split_frame(frame);
    let viewport_chunks = render_app_scaffold(viewport_chunks, frame, app);

    let query_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    let query = Paragraph::new(Span::raw(app.query.url.as_str())).block(query_block);
    let query_label = Paragraph::new(Span::raw("Query:"));

    frame.render_widget(query_label, viewport_chunks[1]);
    frame.render_widget(query, viewport_chunks[2]);
}

fn split_frame(frame: &mut Frame) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .split(frame.area())
}

fn render_app_scaffold(viewport: Rc<[Rect]>, frame: &mut Frame, app: &AppState) -> Rc<[Rect]> {
    let header = *viewport.first().unwrap();
    let footer = *viewport.last().unwrap();

    let title_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(header);

    let title_top_block = Block::default().style(Style::default().bg(Color::White));
    let title_top = Paragraph::new(Span::raw("Qrust").red().bold().underlined())
        .block(title_top_block)
        .alignment(Alignment::Center);

    frame.render_widget(title_top, title_chunks[0]);

    let title_center_block = Block::default().style(Style::default().bg(Color::White));
    let title_center_line = Line::from(Vec::from([Span::raw("Workspace: "), Span::raw(&app.name)]));
    let title_center = Paragraph::new(title_center_line.red())
        .block(title_center_block)
        .alignment(Alignment::Center);

    frame.render_widget(title_center, title_chunks[1]);

    let title_bottom_block = Block::default().style(Style::default().bg(Color::White));
    let title_bottom_line = Line::from(Vec::from([
        Span::raw("Endpoint: "),
        Span::raw(app.query.url.host_str().unwrap()),
    ]));
    let title_bottom = Paragraph::new(title_bottom_line.red())
        .block(title_bottom_block)
        .alignment(Alignment::Center);

    frame.render_widget(title_bottom, title_chunks[2]);

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Length(app.mode.as_static_str().len() as u16 + 6),
            Constraint::Fill(1),
        ])
        .split(footer);

    let footer_left = Line::from(vec![
        Span::raw("Mode: "),
        Span::raw(app.mode.as_static_str()).bold(),
    ])
    .red()
    .on_white();

    frame.render_widget(footer_left, footer_chunks[0]);

    Rc::from(&viewport[1..viewport.len() - 1])
}
