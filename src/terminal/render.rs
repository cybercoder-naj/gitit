use ratatui::backend::Backend;
use ratatui::Frame;
use ratatui::layout::Rect;
use crate::global::state::State;

pub trait Render {
    fn render<B: Backend>(&mut self, frame: &mut Frame<B>, area: Rect, state: &mut State);
}