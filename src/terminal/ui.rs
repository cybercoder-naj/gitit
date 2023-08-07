use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    prelude::Direction,
    widgets::{Block, Borders},
    backend::Backend
};
use ratatui::layout::Alignment;
use ratatui::prelude::Span;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Padding, Paragraph};
use crate::controller::state::GitState;

pub fn main<B: Backend>(frame: &mut Frame<B>, state: &mut GitState) {
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
        .borders(Borders::ALL)
        .padding(Padding::new(3, 3, 1, 1));

    let paragragph = generate_modified_files_paragraph(main_window, state);

    frame.render_widget(paragragph, chunks[0]);
}

fn generate_modified_files_paragraph<'a>(block: Block<'a>, state: &'a GitState) -> Paragraph<'a> {
    let text: Vec<_> = state
        .get_unstaged_files()
        .iter()
        .map(|s| {
            let mut new_str = String::from("[ ] ");
            new_str.push_str(s.as_ref());

            new_str
        })
        .map(|s| {
            Line::from(Span::styled(s, Style::default().fg(Color::Red)))
        }).collect();

    Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Left)
}