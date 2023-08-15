use ratatui::{
    backend::Backend,
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Padding},
};
use ratatui::layout::Rect;

use crate::controller::state::State;

mod diffs;
mod files;
mod buttons;

pub trait Render {
    fn render<B: Backend>(frame: &mut Frame<B>, area: Rect, state: &mut State);
}

pub fn main<B: Backend>(frame: &mut Frame<B>, state: &mut State) {
    let parent_block = Block::default()
        .title("Gitit")
        .borders(Borders::ALL)
        .padding(Padding::new(3, 3, 1, 1));
    frame.render_widget(parent_block, frame.size());

    let vertical_constraints = [Constraint::Percentage(95), Constraint::Percentage(5)];
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

    files::Files::render(frame, window_layout[0], state);
    diffs::Diff::render(frame, window_layout[1], state);
    buttons::Buttons::render(frame, vertical_layout[1], state);
}
