use crate::{client::GqlClient, App};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};
use std::rc::Rc;

mod scaffold;
mod view;

use scaffold::render_app_scaffold;
use view::render_app_content;

pub fn compute_ui(frame: &mut Frame, app: &App, gql_client: &GqlClient) {
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
