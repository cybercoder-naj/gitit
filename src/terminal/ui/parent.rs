use ratatui::{layout::Rect, backend::Backend, widgets::{Block, Borders, Padding}, Frame};

pub fn render_parent_block<B: Backend>(frame: &mut Frame<B>) -> Rect {
    let main_window = Block::default()
        .title("Gitit")
        .borders(Borders::ALL)
        .padding(Padding::new(3, 3, 1, 1));
    frame.render_widget(main_window, frame.size());

    frame.size()
}