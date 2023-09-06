use std::{
    error::Error,
    io::{self, Stdout},
};
use std::sync::{Arc, Mutex};

use crossterm::{
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

use ui::Ui;

use crate::global::state::State;

pub mod ui;
mod keying;
mod render;

pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

pub fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    Ok(terminal.show_cursor()?)
}

pub fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, state: Arc<Mutex<State>>) -> Result<(), Box<dyn Error>> {
    Ok(loop {
        terminal.draw(|f| Ui::default().main(f, Arc::clone(&state)))?;

        if !keying::listen()? {
            break;
        }
    })
}

