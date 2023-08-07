use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    prelude::Direction,
    widgets::{Block, Borders},
    backend::Backend
};

pub fn main<B: Backend>(frame: &mut Frame<B>) {
    let layout = [
        Constraint::Percentage(100)
    ];

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(layout.as_ref())
        .split(frame.size());

    let main_window = Block::default()
        .title("Unstaged Files")
        .borders(Borders::ALL);
    frame.render_widget(main_window, chunks[0]);
}
