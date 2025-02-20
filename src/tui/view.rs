use std::rc::Rc;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Text,
    widgets::{Block, Padding, Paragraph},
    Frame,
};

use crate::app::{App, Focus};

pub fn render_app_content(outlet: Rc<Rect>, frame: &mut Frame, app: &App) {
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Fill(1), Constraint::Fill(1)])
        .split(*outlet);

    let query_editor_block = Block::bordered()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(if app.focus == Focus::QueryEditor {
            Color::LightCyan
        } else {
            Color::White
        }))
        .padding(Padding::proportional(1));

    let query_editor = Paragraph::new(Text::raw(&app.query).white()).block(query_editor_block);

    let response_view_block = Block::bordered()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(if app.focus == Focus::ResponseView {
            Color::LightCyan
        } else {
            Color::White
        }))
        .padding(Padding::proportional(1));

    let response_view = Paragraph::new(Text::raw(&app.response).white()).block(response_view_block);

    frame.render_widget(query_editor, main_chunks[0]);
    frame.render_widget(response_view, main_chunks[1]);
}
