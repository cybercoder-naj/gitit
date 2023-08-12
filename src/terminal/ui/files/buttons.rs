use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Paragraph, Borders},
    Frame,
};

use crate::controller::state::State;

pub fn render<B: Backend>(frame: &mut Frame<B>, area: Rect, state: &State) {
    let constraints = [Constraint::Percentage(100)];
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints.as_ref())
        .split(area);

    let text = vec![Line::from(vec![Span::raw("[ Select All ]")])];

    let buttons = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
        );

    frame.render_widget(buttons, layout[0]);
}
