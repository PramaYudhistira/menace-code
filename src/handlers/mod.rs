// Event handling and keyboard input processing
//
// This module handles:
// - Keyboard event capture
// - Input mode switching
// - Text input handling
// - Command dispatching

use crossterm::event::{self, Event, KeyCode, KeyModifiers, MouseEventKind};
use std::io;
use std::time::{Duration, Instant};

use crate::core::App;

/// Events that can be returned from the event handler
pub enum AppEvent {
    /// Continue running the application
    Continue,
    /// Exit the application
    Quit,
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
                } else {
                    app.send_message();
                }
            }
            KeyCode::Char(c) => {
                app.input.push(c);
            }
            KeyCode::Backspace => {
                app.input.pop();
            }
            KeyCode::Up => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    app.scroll_up(3);
                }
            }
            KeyCode::Down => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    app.scroll_down(3);
                }
            }
            KeyCode::PageUp => {
                app.scroll_up(10);
            }
            KeyCode::PageDown => {
                app.scroll_down(10);
            }
            KeyCode::Home => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    app.scroll_offset = app.messages.len().saturating_sub(1);
                }
            }
            KeyCode::End => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    app.scroll_to_bottom();
                }
            }
            _ => {}
        }
        }
        Event::Mouse(mouse) => {
            match mouse.kind {
                MouseEventKind::ScrollUp => {
                    app.scroll_up(3);
                }
                MouseEventKind::ScrollDown => {
                    app.scroll_down(3);
                }
                _ => {}
            }
        }
        _ => {}
    }
    Ok(AppEvent::Continue)
}