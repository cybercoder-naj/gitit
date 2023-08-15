use ratatui::backend::Backend;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Direction;
use ratatui::widgets::{Block, Borders};
use crate::controller::state::State;

pub fn render<B: Backend>(frame: &mut Frame<B>, area: Rect, state: &State) {
    let constraints = [Constraint::Percentage(50), Constraint::Percentage(50)];
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .horizontal_margin(1)
        .split(area);

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Commit Message");
    frame.render_widget(block, layout[0]);

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Commit Description");
    frame.render_widget(block, layout[1]);
}
