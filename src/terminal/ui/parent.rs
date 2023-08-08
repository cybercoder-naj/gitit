use ratatui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Borders, Padding},
    Frame,
};
use crate::controller::state::State;

pub fn render_parent_block<B: Backend>(frame: &mut Frame<B>, chunk: Rect, _state: &State) -> Rect {
    let main_window = Block::default()
        .title("Gitit")
        .borders(Borders::ALL)
        .padding(Padding::new(3, 3, 1, 1));
    frame.render_widget(main_window, chunk);

    frame.size()
}
