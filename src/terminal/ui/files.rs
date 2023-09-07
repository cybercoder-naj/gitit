use std::sync::{Arc, Mutex};

use ratatui::{
    backend::Backend,
    Frame,
    layout::*,
    widgets::{Block, Borders, Padding},
};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{List, ListItem, ListState, Paragraph};

use crate::domain;
use crate::global::{event_emitter, KeypressListener};
use crate::global::cursor::CursorAction;
use crate::global::models::ModifiedFile;
use crate::global::state::State;
use crate::terminal::render::Render;

pub struct Files {
    m_files: Vec<ModifiedFile>,
    list_state: ListState,
}

impl Render for Files {
    fn render<B: Backend>(&mut self, frame: &mut Frame<B>, area: Rect, state: &mut State) {
        let content_block = Block::default()
            .title("Files")
            .borders(Borders::ALL);
        let inner = content_block.inner(area);
        frame.render_widget(content_block, area);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
            .margin(1)
            .split(inner);

        let select = Paragraph::new(format!("  [{}] {}", "-", "Select All"));
        frame.render_widget(select, layout[0]);

        let files: Vec<_> = self.m_files
            .iter()
            .map(&mut transform)
            .collect();

        let files = List::new(files)
            .highlight_symbol("> ");

        frame.render_stateful_widget(files, layout[1], &mut self.list_state);
    }
}

impl Files {
    pub fn new() -> Arc<Mutex<Self>> {
        let files = domain::retrieve_files_from_git();
        let m_files = files
            .iter()
            .map(|name| {
                let flags = &name[..1];

                ModifiedFile::new(
                    String::from(&name[3..]),
                    !(flags == "??" || flags.starts_with(" ")),
                )
            })
            .collect();

        let this = Self {
            m_files,
            list_state: ListState::with_selected(ListState::default(), Some(0)),
        };

        let this = Arc::new(Mutex::new(this));

        Self::on_keypress(Arc::clone(&this));

        this
    }

    fn next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.m_files.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.m_files.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.list_state.select(None);
    }
}

impl KeypressListener for Files {
    fn on_keypress(this: Arc<Mutex<Self>>) {
        event_emitter().on("cursor_action", move |cursor_action: CursorAction| {
            this.lock().unwrap().next();
        });
    }
}

fn transform(m_file: &ModifiedFile) -> ListItem {
    match m_file.is_staged() {
        true => {
            let style = Style::default().fg(Color::Green);

            ListItem::new(Line::from(
                vec![
                    Span::styled("[x] ", style),
                    Span::styled(m_file.name(), style)
                ]
            ))
        }
        false => {
            let style = Style::default().fg(Color::Red).add_modifier(Modifier::CROSSED_OUT);

            ListItem::new(Line::from(
                vec![
                    Span::styled("[ ] ", style),
                    Span::styled(m_file.name(), style)
                ]
            ))

        }
    }

    // let line = match m_file.is_staged() {
    //     true => "[x] ",
    //     false => "[ ] "
    // };
    // let mut line = String::from(line);
    // line.push_str(&m_file.name());

    // match m_file.is_staged() {
    //     true =>
    //         ListItem::new(line)
    //             .style(Style::default().fg(Color::Green)),
    //     false =>
    //         ListItem::new(line)
    //             .style(Style::default().fg(Color::Red).add_modifier(Modifier::CROSSED_OUT))
    // }

    // ListItem::new(Line::from(vec![Span::styled("Hi", Style::default().fg(Color::Green))]))
}