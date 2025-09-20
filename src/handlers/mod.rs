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

use crate::core::App;

/// Events that can be returned from the event handler
pub enum AppEvent {
    /// Continue running the application
    Continue,
    /// Exit the application
    Quit,
    /// Input changed - needs redraw
    InputChanged,
    /// Message sent - needs to print
    MessageSent,
}

pub fn handle_events(app: &mut App) -> io::Result<AppEvent> {
    match event::read()? {
        Event::Key(key) => {
        // Handle Ctrl+C double press to quit
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

        // Always in "editing" mode - handle all input
        match key.code {
            KeyCode::Enter => {
                // Alt+Enter or Option+Enter adds newline
                if key.modifiers.contains(KeyModifiers::ALT) {
                    app.input.push('\n');
                    return Ok(AppEvent::InputChanged);
                } else {
                    app.send_message();
                    return Ok(AppEvent::MessageSent);
                }
            }
            KeyCode::Char(c) => {
                app.input.push(c);
                return Ok(AppEvent::InputChanged);
            }
            KeyCode::Backspace => {
                app.input.pop();
                return Ok(AppEvent::InputChanged);
            }
            // Terminal handles all scrolling - we don't manage it
            _ => {}
        }
        }
        // Mouse scrolling handled by terminal
        Event::Mouse(_) => {}
        _ => {}
    }
    Ok(AppEvent::Continue)
}