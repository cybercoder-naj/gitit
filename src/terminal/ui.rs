use ratatui::{
    backend::Backend,
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Padding},
};

use crate::controller::state::State;

mod diffs;
mod files;
mod commit_msg;

pub fn main<B: Backend>(frame: &mut Frame<B>, state: &mut State) {
    let parent_block = Block::default()
        .title("Gitit")
        .borders(Borders::ALL)
        .padding(Padding::new(3, 3, 1, 1));
    frame.render_widget(parent_block, frame.size());

    let vertical_constraints = [Constraint::Percentage(80), Constraint::Percentage(10), Constraint::Percentage(10)];
    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vertical_constraints)
        .margin(1)
        .split(frame.size());

    let window_constraints = [Constraint::Percentage(50), Constraint::Percentage(50)];
    let window_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(window_constraints)
        .margin(1)
        .split(vertical_layout[0]);

    files::render(frame, window_layout[0], state);
    diffs::render(frame, window_layout[1], state);
    commit_msg::render(frame, vertical_layout[1], state);
}
