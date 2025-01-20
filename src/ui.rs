use crate::{
    app::{AsStaticStr, Focus},
    client::GqlClient,
    AppState,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};
use std::rc::Rc;

pub fn compute_ui(frame: &mut Frame, app: &AppState, gql_client: &GqlClient) {
    let viewport_chunks = split_frame(frame);
    let outlet = render_app_scaffold(viewport_chunks, frame, app, gql_client);
    render_app_content(outlet, frame, app)
}

fn split_frame(frame: &mut Frame) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .split(frame.area())
}

fn render_app_scaffold(
    viewport: Rc<[Rect]>,
    frame: &mut Frame,
    app: &AppState,
    gql_client: &GqlClient,
) -> Rc<Rect> {
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

    let title_top_block = Block::default();
    let title_top = Paragraph::new(Span::raw("Qrust").red().bold().underlined())
        .block(title_top_block)
        .alignment(Alignment::Center);

    frame.render_widget(title_top, title_chunks[0]);

    let title_center_block = Block::default();
    let title_center_line = Line::from(Vec::from([Span::raw("Workspace: "), Span::raw(app.name)]));
    let title_center = Paragraph::new(title_center_line.red())
        .block(title_center_block)
        .alignment(Alignment::Center);

    frame.render_widget(title_center, title_chunks[1]);

    let title_bottom_block = Block::default();
    let title_bottom_line = Line::from(Vec::from([
        Span::raw("Endpoint: "),
        Span::raw(gql_client.endpoint),
    ]));
    let title_bottom = Paragraph::new(title_bottom_line.red())
        .block(title_bottom_block)
        .alignment(Alignment::Center);

    frame.render_widget(title_bottom, title_chunks[2]);

    let footer_line = Line::from(vec![
        Span::raw("Focus: "),
        Span::raw(app.focus.as_static_str()).bold(),
    ])
    .red();

    frame.render_widget(footer_line, footer);

    Rc::from(viewport[1])
}

fn render_app_content(outlet: Rc<Rect>, frame: &mut Frame, app: &AppState) {
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Fill(1), Constraint::Fill(1)])
        .split(*outlet);

    let query_editor_block = Block::bordered()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(if app.focus == Focus::QueryEditor {
            Color::White
        } else {
            Color::LightCyan
        }));

    let response_view_block = Block::bordered()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(if app.focus == Focus::ResponseView {
            Color::White
        } else {
            Color::LightCyan
        }));

    frame.render_widget(query_editor_block, main_chunks[0]);
    frame.render_widget(response_view_block, main_chunks[1]);
}
