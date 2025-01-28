use crate::{app::AsStaticStr, client::GqlClient, App};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};
use std::rc::Rc;

pub fn render_app_scaffold(
    viewport: Rc<[Rect]>,
    frame: &mut Frame,
    app: &App,
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
