// User interface rendering and layout management
//
// This module handles:
// - Terminal UI rendering
// - Layout management
// - Widget creation and styling
// - Cursor positioning

pub mod widgets;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::core::App;
use widgets::{chat, input};

/// Main UI drawing function
pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Min(1),      // Chat area
            Constraint::Length(3),   // Input area
        ])
        .split(f.area());

    // Render chat messages
    chat::render_chat(f, &app.messages, chunks[0]);

    // Render input area
    input::render_input(f, &app.input, app.input_mode, chunks[1]);

    // Render help overlay if in Normal mode
    input::render_help(f, app.input_mode, chunks[1]);
}