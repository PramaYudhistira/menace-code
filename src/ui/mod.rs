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
    style::{Color, Style},
    widgets::{Block, Borders},
    Frame,
};

use crate::core::App;
use widgets::{chat, input};

/// Main UI drawing function
pub fn draw(f: &mut Frame, app: &App) {
    // Calculate input height first
    let input_lines = app.input.lines().count().max(1);
    let input_height = input_lines.min(10) as u16; // Cap at 10 lines

    // Main layout: Chat takes all space except bottom input section
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),                           // Chat area (takes remaining space)
            Constraint::Length(1),                        // Top separator line
            Constraint::Length(input_height),             // Input area
            Constraint::Length(1),                        // Bottom separator line
        ])
        .split(f.area());

    // Render chat messages in the top area (no borders)
    chat::render_chat_with_scroll(f, app, main_layout[0]);

    // Draw top separator bar (above input)
    let top_bar = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::default().fg(Color::DarkGray));
    f.render_widget(top_bar, main_layout[1]);

    // Render input area (between the two lines)
    input::render_input(f, &app.input, app.input_mode, main_layout[2]);

    // Draw bottom separator bar (below input)
    let bottom_bar = Block::default()
        .borders(Borders::TOP)
        .border_style(Style::default().fg(Color::DarkGray));
    f.render_widget(bottom_bar, main_layout[3]);
}