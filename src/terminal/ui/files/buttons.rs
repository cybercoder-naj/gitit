use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};

pub fn render_file_buttons<B: Backend>(frame: &mut Frame<B>, chunk: Rect) -> Rect {
    let buttons_layout = [Constraint::Percentage(100)];
    let buttons_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(buttons_layout.as_ref())
        .split(chunk);

    let text = vec![Line::from(vec![Span::raw("[ Select All ]")])];

    let buttons = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(Block::default());

    frame.render_widget(buttons, buttons_chunk[0]);

    buttons_chunk[0]
}
