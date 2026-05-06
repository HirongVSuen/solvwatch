use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, Event,KeyCode},
        terminal::{
            disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
        },
        ExecutableCommand,
    },
    Terminal,
};

use std::{io::stdout, panic, time::Duration};
use crate::infra::ratatui::update;

/// Initialize the terminal
pub fn init_terminal() -> anyhow::Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    Ok(terminal)
}

/// Exit the terminal
pub fn exit_terminal() -> anyhow::Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

pub fn handle_event() -> anyhow::Result<update::MESSAGE> {
    event::poll(Duration::from_millis(250))?;
    let event = event::read()?;

    let current_msg = match event {
        Event::Key(key) if key.kind == event::KeyEventKind::Press => {
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') => update::MESSAGE::QUIT,
                KeyCode::Char('h') | KeyCode::Char('H') => update::MESSAGE::HOME,
                KeyCode::Char('s') | KeyCode::Char('S') => update::MESSAGE::SETTING,
                KeyCode::Char(c) => update::MESSAGE::DISTRIBUTE_MSG(c.to_string()),
                _ => update::MESSAGE::NONE,
            }
        }
        _ => update::MESSAGE::NONE,
    };
    Ok(current_msg)
}