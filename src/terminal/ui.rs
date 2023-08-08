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

use crate::controller::state::State;

pub fn main<B: Backend>(frame: &mut Frame<B>, state: &mut State) {
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

    let paragraph = generate_modified_files_paragraph(main_window, state);

    frame.render_widget(paragraph, chunks[0]);
}

fn generate_modified_files_paragraph<'a>(block: Block<'a>, state: &'a State) -> Paragraph<'a> {
    let text: Vec<_> = state
        .unstaged_files
        .iter()
        .enumerate()
        .map(|(i, m_file)| {
            let mut entry = String::from(
                match m_file.checked {
                    true => "[x] ",
                    false => "[ ] "
                }
            );
            entry.push_str(m_file.filename.as_ref());

            let mut style = Style::default().fg(
                match m_file.checked {
                    true => Color::Green,
                    false => Color::Red
                }
            );
            if i == state.current_index {
                style = style.bg(Color::White);
            }

            Line::from(Span::styled(entry, style))
        })
        .collect();

    Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Left)
}