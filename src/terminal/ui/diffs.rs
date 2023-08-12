use ratatui::{
    backend::Backend,
    Frame,
    layout::Rect, 
    widgets::{Block, Borders}
};

use crate::controller::state::State;

pub fn render<B: Backend>(frame: &mut Frame<B>, area: Rect, _state: &State) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Changes");

    frame.render_widget(block, area);
}