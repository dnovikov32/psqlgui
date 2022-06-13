use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::app::{App, AppResult};

/// Handles the key events and update the state oj [`App`]
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on Esc
        KeyCode::Esc => {
            app.running = false;
        },
        // Exit application on Ctrl-D
        KeyCode::Char('d') | KeyCode::Char('D') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.running = false;
            }
        },
        _ => {}
    }

    Ok(())
}