mod buttons;

use std::rc::Rc;

use ratatui::{
    backend::Backend,
    layout::*,
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Padding, Paragraph},
    Frame,
};

use crate::controller::state::State;

pub fn render_files_screen<B: Backend>(frame: &mut Frame<B>, parent_chunk: Rect, state: &State) -> Rc<[Rect]> {
    let files_layout = [Constraint::Percentage(10), Constraint::Percentage(90)];
    let files_chunk = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(files_layout.as_ref())
        .split(parent_chunk);

    let files_block = Block::default()
        .padding(Padding::new(3, 3, 1, 1));

    let files = generate_modified_files_paragraph(files_block, state);

    let _buttons_chink = buttons::render_file_buttons(frame, files_chunk[0]);
    frame.render_widget(files, files_chunk[1]);

    files_chunk
}

fn generate_modified_files_paragraph<'a>(block: Block<'a>, state: &'a State) -> Paragraph<'a> {
    let text: Vec<_> = state
        .unstaged_files
        .iter()
        .enumerate()
        .map(|(i, m_file)| {
            let mut style = Style::default().fg(match m_file.staged {
                true => Color::Green,
                false => Color::Red,
            });

            let mut entry: String = String::new();
            if i == state.current_index {
                entry.push_str("> ");
            } else {
                entry.push_str("  ");
            }
            entry.push_str(match m_file.staged {
                true => "[x] ",
                false => "[ ] ",
            });
            entry.push_str(m_file.filename.as_ref());

            if m_file.filename.chars().last().unwrap() == '/' {
                style = style.add_modifier(Modifier::BOLD);
            }

            Line::from(Span::styled(entry, style))
        })
        .collect();

    Paragraph::new(text).block(block).alignment(Alignment::Left)
}