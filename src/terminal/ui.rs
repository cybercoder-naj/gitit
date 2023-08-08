use ratatui::{
    Frame,
    layout::{Constraint, Layout, Alignment},
    prelude::Direction,
    widgets::{Block,
              Borders,
              Padding,
              Paragraph},
    backend::Backend,
    text::{Line, Span},
    style::{Color, Style}
};

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
        .unstaged_files
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let mut new_str = String::from("[ ] ");
            new_str.push_str(s.filename.as_ref());

            (i, new_str)
        })
        .map(|(i, s)| {
            let mut style = Style::default().fg(Color::Red);
            if i == usize::from(state.current_index) {
                style = style.bg(Color::White);
            }

            Line::from(Span::styled(s, style))
        }).collect();

    Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Left)
}