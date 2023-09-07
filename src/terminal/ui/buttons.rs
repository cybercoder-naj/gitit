use std::sync::{Arc, Mutex};
use ratatui::{
    backend::Backend,
    Frame,
    layout::{Alignment, Rect},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};
use ratatui::prelude::Color;
use ratatui::style::Style;

use crate::global::cursor::Section;
use crate::global::state::{ButtonIndex, State};
use crate::terminal::render::Render;

#[derive(Default)]
pub struct Buttons;

impl Render for Buttons {
    fn render<B: Backend>(&mut self, frame: &mut Frame<B>, area: Rect, state: &mut State) {
        let separator = || Span::raw("   ");

        let text = Line::from(
            vec![
                Span::styled("[ Commit ]", get_style(ButtonIndex::Commit, state)),
                separator(),
                Span::styled("[ Commit and Push ]", get_style(ButtonIndex::CommitAndPush, state)),
                separator(),
                Span::styled("[ Quit ]", get_style(ButtonIndex::Quit, state)),
            ]
        );

        let paragraph = Paragraph::new(text)
            .alignment(Alignment::Center)
            .block(Block::default());

        frame.render_widget(paragraph, area);
    }
}

impl Buttons {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Buttons::default()))
    }
}

fn get_style(button_index: ButtonIndex, state: &State) -> Style {
    if state.cursor().is_in(&Section::Buttons) && state.get_button_index() == &button_index {
        Style::default().bg(Color::White).fg(Color::Black)
    } else {
        Style::default()
    }
}