// Event handling and keyboard input processing
//
// This module handles:
// - Keyboard event capture
// - Input mode switching
// - Text input handling
// - Command dispatching

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::io;
use std::time::{Duration, Instant};

use crate::core::{App, InputMode};

/// Events that can be returned from the event handler
pub enum AppEvent {
    /// Continue running the application
    Continue,
    /// Exit the application
    Quit,
}

pub fn handle_events(app: &mut App) -> io::Result<AppEvent> {
    if let Event::Key(key) = event::read()? {
        // Handle Ctrl+C double press to quit (works in any mode)
        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
            if let Some(last_press) = app.last_ctrl_c {
                // If less than 500ms since last Ctrl+C, quit
                if last_press.elapsed() < Duration::from_millis(500) {
                    return Ok(AppEvent::Quit);
                }
            }
            app.last_ctrl_c = Some(Instant::now());
            return Ok(AppEvent::Continue);
        }

        match app.input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('e') => {
                    app.input_mode = InputMode::Editing;
                }
                KeyCode::Char('/') => {
                    app.input_mode = InputMode::Command;
                    app.input.clear();
                    app.input.push('/');
                }
                KeyCode::Char('q') => {
                    return Ok(AppEvent::Quit);
                }
                _ => {}
            },
            InputMode::Editing => match key.code {
                KeyCode::Enter => {
                    app.send_message();
                }
                KeyCode::Char(c) => {
                    app.input.push(c);
                }
                KeyCode::Backspace => {
                    app.input.pop();
                }
                KeyCode::Esc => {
                    app.input_mode = InputMode::Normal;
                }
                _ => {}
            },
            InputMode::Command => match key.code {
                KeyCode::Esc => {
                    app.input_mode = InputMode::Normal;
                    app.input.clear();
                }
                KeyCode::Enter => {
                    // Process command
                    app.input.clear();
                    app.input_mode = InputMode::Normal;
                }
                KeyCode::Char(c) => {
                    app.input.push(c);
                }
                KeyCode::Backspace => {
                    app.input.pop();
                }
                _ => {}
            },
        }
    }
    Ok(AppEvent::Continue)
}