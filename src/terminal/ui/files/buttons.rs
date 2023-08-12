use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Paragraph, Borders},
    Frame,
    style::{Color, Style}
};
use crate::controller::cursor::{Button, Section};
use crate::controller::state::State;

pub fn render<B: Backend>(frame: &mut Frame<B>, area: Rect, state: &State) {
    let constraints = [Constraint::Percentage(100)];
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints.as_ref())
        .split(area);

    let selected_style = Style::default().bg(Color::White).fg(Color::Black);

    let text = Line::from(
        vec![
            Span::styled(
                "[ Select All ]",
                if state.cursor.is_in(Section::Buttons) && *state.cursor.get_button() == Button::SelectAll {
                    selected_style
                } else {
                    Style::default()
                }
            )
        ]
    );

    let buttons = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
        );

    frame.render_widget(buttons, layout[0]);
}
