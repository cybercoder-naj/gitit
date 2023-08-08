mod parent;
mod files;

use ratatui::{
    backend::Backend,
    Frame,
};

use crate::controller::state::State;

pub fn main<B: Backend>(frame: &mut Frame<B>, state: &mut State) {
    let parent_chunk = parent::render_parent_block(frame);

    let _files_chunk = files::render_files_screen(frame, parent_chunk, state);
}