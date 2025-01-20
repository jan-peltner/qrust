use std::rc::Rc;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::Block,
    Frame,
};

use crate::app::{AppState, Focus};

pub fn render_app_content(outlet: Rc<Rect>, frame: &mut Frame, app: &AppState) {
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
        }));

    let response_view_block = Block::bordered()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(if app.focus == Focus::ResponseView {
            Color::LightCyan
        } else {
            Color::White
        }));

    frame.render_widget(query_editor_block, main_chunks[0]);
    frame.render_widget(response_view_block, main_chunks[1]);
}
