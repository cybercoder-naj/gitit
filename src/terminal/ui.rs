use std::sync::{Arc, Mutex};

use ratatui::{
    backend::Backend,
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Padding},
};

use buttons::Buttons;
use diffs::Diff;
use files::Files;

use crate::global::state::State;

use super::render::Render;

mod diffs;
mod files;
mod buttons;

pub struct Ui {
    files: Arc<Mutex<Files>>,
    diff: Arc<Mutex<Diff>>,
    buttons: Arc<Mutex<Buttons>>
}

impl Default for Ui {
    fn default() -> Self {
        Self {
            files: Files::new(),
            diff: Diff::new(),
            buttons: Buttons::new()
        }
    }
}

impl Ui {
    pub fn main<B: Backend>(&mut self, frame: &mut Frame<B>, state: Arc<Mutex<State>>) {
        let parent_block = Block::default()
            .title("Gitit")
            .borders(Borders::ALL)
            .padding(Padding::new(3, 3, 1, 1));
        frame.render_widget(parent_block, frame.size());

        let vertical_constraints = [Constraint::Percentage(95), Constraint::Percentage(5)];
        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vertical_constraints)
            .margin(1)
            .split(frame.size());

        let window_constraints = [Constraint::Percentage(50), Constraint::Percentage(50)];
        let window_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(window_constraints)
            .margin(1)
            .split(vertical_layout[0]);

        let state = &mut *state.lock().unwrap();

        self.files.lock().unwrap().render(frame, window_layout[0], state);
        self.diff.lock().unwrap().render(frame, window_layout[1], state);
        self.buttons.lock().unwrap().render(frame, vertical_layout[1], state);
    }
}

