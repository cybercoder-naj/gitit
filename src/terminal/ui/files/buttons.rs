use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};
use ratatui::prelude::Color;
use ratatui::style::Style;
use crate::controller::state::State;
use crate::utils::BTN_SELECT_ALL;

pub fn render_file_buttons<B: Backend>(frame: &mut Frame<B>, chunk: Rect, state: &State) -> Rect {
    let buttons_layout = [Constraint::Percentage(100)];
    let buttons_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(buttons_layout.as_ref())
        .split(chunk);

    let selected_style = Style::default().bg(Color::White).fg(Color::Black);

    let text = Line::from(
        vec![
            Span::styled(
                "[ Select All ]",
                if state.button_index == BTN_SELECT_ALL {
                    selected_style
                } else {
                    Style::default()
                }
            )
        ]
    );

    let buttons = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(Block::default());

    frame.render_widget(buttons, buttons_chunk[0]);

    buttons_chunk[0]
}
