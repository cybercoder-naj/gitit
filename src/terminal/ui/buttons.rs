use ratatui::backend::Backend;
use ratatui::Frame;
use ratatui::layout::{Alignment, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};
use crate::global::state::State;
use crate::terminal::ui::Render;

pub struct Buttons;

impl Render for Buttons {
    fn render<B: Backend>(frame: &mut Frame<B>, area: Rect, state: &mut State) {
        let text = Line::from(
            vec![
                Span::raw("[ Commit ]"),
                Span::raw("   "),
                Span::raw("[ Commit and Push ]"),
                Span::raw("   "),
                Span::raw("[ Quit ]")
            ]
        );

        let paragraph = Paragraph::new(text)
            .alignment(Alignment::Center)
            .block(Block::default());

        frame.render_widget(paragraph, area);
    }
}

