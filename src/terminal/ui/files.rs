use crate::controller::cursor::Section;
use ratatui::{
    backend::Backend,
    layout::*,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};

use crate::controller::state::State;
use crate::terminal::ui::Render;

pub struct Files;

impl Render for Files {
    fn render<B: Backend>(frame: &mut Frame<B>, area: Rect, state: &mut State) {
        let content_block = Block::default()
            .title("Files")
            .padding(Padding::uniform(1))
            .borders(Borders::ALL);

        let all_filenames = generate_modified_files_paragraph(content_block, state);
        frame.render_widget(all_filenames, area);
    }

}

fn generate_modified_files_paragraph<'a>(block: Block<'a>, state: &'a mut State) -> Paragraph<'a> {
    let m_files = state.get_files();

    let text: Vec<_> = m_files
        .iter()
        .enumerate()
        .map(|(i, m_file)| {
            let mut style = Style::default().fg(match m_file.is_staged() {
                true => Color::Green,
                false => Color::Red,
            });
            let mut prefix: String = String::from(
                if state.cursor().is_in(&Section::Files) && state.cursor().get_file_index() == i
                {
                    "> "
                } else {
                    "  "
                },
            );

            prefix.push_str(match m_file.is_staged() {
                true => "[x] ",
                false => "[ ] ",
            });

            if m_file.name().chars().last().unwrap() == '/' {
                style = style.add_modifier(Modifier::BOLD);
            }
            let mut spans = vec![Span::styled(prefix, style)];

            let filename = m_file.name();
            if !m_file.is_staged() {
                style = style.add_modifier(Modifier::CROSSED_OUT);
            }
            spans.push(Span::styled(filename, style));

            Line::from(spans)
        })
        .collect();

    Paragraph::new(text).block(block).alignment(Alignment::Left)
}
