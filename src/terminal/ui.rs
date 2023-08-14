mod diffs;
mod files;

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Padding},
    Frame,
};
use crate::cache::Cache;

use crate::controller::state::State;

pub fn main<B: Backend>(frame: &mut Frame<B>, state: &mut State, cache: &mut Cache) {
    let parent_block = Block::default()
        .title("Gitit")
        .borders(Borders::ALL)
        .padding(Padding::new(3, 3, 1, 1));
    frame.render_widget(parent_block, frame.size());

    let window_constaints = [Constraint::Percentage(50), Constraint::Percentage(50)];
    let window_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(window_constaints)
        .margin(1)
        .split(frame.size());

    files::render(frame, window_layout[0], state);
    diffs::render(frame, window_layout[1], state, cache);
}
