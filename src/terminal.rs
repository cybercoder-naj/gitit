mod ui;

use std::{
    error::Error,
    io::{self, Stdout},
    time::Duration,
};


use crossterm::{
    event,
    event::{Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

use ratatui::{
    backend::{CrosstermBackend},
    Terminal,
};

use crate::controller::state::State;

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

pub fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, state: &mut State) -> Result<(), Box<dyn Error>> {
    Ok(loop {
        terminal.draw(|f| ui::main(f, state))?;

        // To quit the terminal
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => {
                        if state.current_index > 0 {
                            state.current_index -= 1;
                        }
                    },
                    KeyCode::Down => {
                        if state.current_index < state.unstaged_files.len() {
                            state.current_index += 1;
                        }
                    },
                    KeyCode::Char(' ') => {
                       let m_file = &mut state.unstaged_files[state.current_index];
                        m_file.checked = !m_file.checked;
                    }
                    _ => {}
                };
            }
        }
    })
}

