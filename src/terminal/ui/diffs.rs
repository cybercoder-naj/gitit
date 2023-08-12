use ratatui::{
    backend::Backend,
    Frame,
    layout::Rect, 
    widgets::{Block, Borders, Paragraph, Padding},
    text::{Line, Span}, style::{Style, Modifier, Color}
};

use crate::controller::{state::State, self};

pub fn render<B: Backend>(frame: &mut Frame<B>, area: Rect, state: &State) {
    let block = Block::default()
        .borders(Borders::ALL)
        .padding(Padding::uniform(1))
        .title("Changes");

    let paragraph = generate_git_paragaph(block, state);

    frame.render_widget(paragraph, area);
}

pub fn generate_git_paragaph<'a>(block: Block<'a>, state: &'a State) -> Paragraph<'a> {
    let m_file = state.get_current_file();

    let binding = controller::get_diff_string(m_file);
    let diff = binding.lines();
    let mut text: Vec<Line> = vec![];

    for line in diff.zip(0..) {
        if line.1 < 4 {
            text.push(
                Line::from(
                    Span::styled(
                        String::from(line.0),
                        Style::default().add_modifier(Modifier::BOLD)
                    )
                )
            );   
            continue;
        }

        if line.0.starts_with("@@") {
            let end_ref = line.0.rfind("@@").expect("Git diff returned wrong formating"); 
            text.push(
                Line::from(
                    vec![
                        Span::styled(
                            String::from(&line.0[..(end_ref + 2)]),
                            Style::default().fg(Color::LightBlue)
                        ),
                        Span::raw(
                            String::from(&line.0[(end_ref + 2)..])
                        )
                    ]
                )
            );   
            continue;
        }

        if line.0.starts_with("+") {
            text.push(
                Line::from(
                    Span::styled(
                        String::from(line.0),
                        Style::default().fg(Color::Green)
                    )
                )
            );   
            continue;
        }

        if line.0.starts_with("-") {
            text.push(
                Line::from(
                    Span::styled(
                        String::from(line.0),
                        Style::default().fg(Color::Red)
                    )
                )
            );   
            continue;
        }

        text.push(
            Line::from(
                Span::styled(
                    String::from(line.0),
                    Style::default()
                )
            )
        );   
    }


    Paragraph::new(text)
        .block(block)
}