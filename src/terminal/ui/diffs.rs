use ratatui::{
    backend::Backend,
    Frame,
    layout::Rect, 
    widgets::{Block, Borders, Paragraph, Padding}
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

    Paragraph::new(controller::get_diff_string(m_file))
        .block(block)
}