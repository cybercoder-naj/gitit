use std::sync::{Arc, Mutex};

use ratatui::{
    backend::Backend,
    Frame,
    layout::*,
    widgets::{Block, Borders},
};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{HighlightSpacing, List, ListItem, ListState, Paragraph};

use crate::domain;
use crate::global::{event_emitter, KeypressListener};
use crate::global::constants::EVENT_KEYPRESS;
use crate::global::cursor::CursorAction;
use crate::global::models::ModifiedFile;
use crate::global::state::State;
use crate::terminal::render::Render;

enum FileControlState {
    None,
    Some,
    All,
}

pub struct Files {
    m_files: Vec<ModifiedFile>,
    list_state: ListState,
    control_state: FileControlState,
}

impl Render for Files {
    fn render<B: Backend>(&mut self, frame: &mut Frame<B>, area: Rect, _state: &mut State) {
        let content_block = Block::default()
            .title("Files")
            .borders(Borders::ALL);
        let inner = content_block.inner(area);
        frame.render_widget(content_block, area);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(5), Constraint::Percentage(95)].as_ref())
            .margin(1)
            .split(inner);

        render_select_option(self, frame, layout[0]);

        let files: Vec<_> = self.m_files
            .iter()
            .map(&mut transform_file)
            .collect();

        let files = List::new(files)
            .highlight_symbol("> ")
            .highlight_spacing(HighlightSpacing::Always);

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

        let mut this = Self {
            m_files,
            list_state: ListState::with_selected(ListState::default(), Some(0)),
            control_state: FileControlState::None,
        };
        this.configure_control_state();

        let this = Arc::new(Mutex::new(this));

        Self::on_keypress(Arc::clone(&this));

        this
    }

    fn configure_control_state(&mut self) {
        self.control_state = if self.m_files.iter().all(|file| file.is_staged()) {
            FileControlState::All
        } else if self.m_files.iter().any(|file| file.is_staged()) {
            FileControlState::Some
        } else {
            FileControlState::None
        }
    }

    fn next(&mut self) {
        match self.list_state.selected() {
            Some(i) => {
                if i < self.m_files.len() - 1 {
                    self.list_state.select(Some(i + 1));
                }
            }
            None => self.list_state.select(Some(0)),
        };
    }

    fn previous(&mut self) {
        match self.list_state.selected() {
            Some(i) =>
                if i == 0 {
                    self.unselect()
                } else {
                    self.list_state.select(Some(i - 1));
                },
            None => (),
        };
    }

    fn unselect(&mut self) {
        self.list_state.select(None);
    }
}

impl KeypressListener for Files {
    fn on_keypress(this: Arc<Mutex<Self>>) {
        event_emitter().on(EVENT_KEYPRESS, move |cursor_action: CursorAction| {
            let this = &mut *this.lock().unwrap();

            match cursor_action {
                CursorAction::Up => {
                    this.previous();
                }
                CursorAction::Down => {
                    this.next();
                }
                CursorAction::Select => {
                    if let Some(i) = this.list_state.selected() {
                        this.m_files[i].toggle_staged();
                    } else if let FileControlState::All = this.control_state {
                        for m_file in &mut this.m_files {
                            m_file.unset_staged();
                        }
                    } else {
                        for m_file in &mut this.m_files {
                            m_file.set_staged();
                        }
                    }
                    this.configure_control_state();
                }
                _ => ()
            }
        });
    }
}

fn render_select_option<B: Backend>(this: &Files, frame: &mut Frame<B>, area: Rect) {
    let symbol = if let None = this.list_state.selected() {
        "> "
    } else {
        "  "
    };
    let state = match this.control_state {
        FileControlState::None => " ",
        FileControlState::Some => "-",
        FileControlState::All => "x",
    };
    let message = match this.control_state {
        FileControlState::All => "Deselect All",
        _ => "Select All"
    };

    let select = Paragraph::new(format!("{symbol}[{state}] {message}"))
        .style(Style::default().add_modifier(Modifier::BOLD));
    frame.render_widget(select, area);
}

fn transform_file(m_file: &ModifiedFile) -> ListItem {
    match m_file.is_staged() {
        true => {
            let style = Style::default().fg(Color::Green);

            let spans = vec![
                Span::styled("[x] ", style),
                Span::styled(m_file.name(), style),
            ];
            ListItem::new(Line::from(spans))
        }
        false => {
            let style = Style::default().fg(Color::Red);

            let spans = vec![
                Span::styled("[ ] ", style),
                Span::styled(m_file.name(), style.add_modifier(Modifier::CROSSED_OUT)),
            ];
            ListItem::new(Line::from(spans))
        }
    }
}