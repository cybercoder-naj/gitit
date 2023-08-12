mod buttons;

use ratatui::{
    backend::Backend,
    layout::*,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, Borders, Padding},
    Frame,
};

use crate::controller::state::State;

pub fn render<B: Backend>(
    frame: &mut Frame<B>,
    area: Rect,
    state: &State,
) {
    let constraints = [Constraint::Percentage(90), Constraint::Percentage(10)];
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints.as_ref())
        .split(area);

    let content_block = Block::default()
        .title("Files")
        .padding(Padding::uniform(1))
        .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT);

    let all_filenames = generate_modified_files_paragraph(content_block, state);
    frame.render_widget(all_filenames, layout[0]);

    buttons::render(frame, layout[1], state);
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
