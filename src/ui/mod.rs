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
use widgets::input;

/// Main UI drawing function - ONLY input area
pub fn draw(f: &mut Frame, app: &App) {
    // Calculate input height
    let input_lines = app.input.lines().count().max(1);
    let input_height = input_lines.min(10) as u16;

    // Layout: Just input area with borders
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),                        // Top separator
            Constraint::Length(input_height),             // Input area
            Constraint::Length(1),                        // Bottom separator
        ])
        .split(f.area());

    // Draw top separator bar
    let top_bar = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::default().fg(Color::DarkGray));
    f.render_widget(top_bar, main_layout[0]);

    // Render input area
    input::render_input(f, &app.input, app.input_mode, main_layout[1]);

    // Draw bottom separator bar
    let bottom_bar = Block::default()
        .borders(Borders::TOP)
        .border_style(Style::default().fg(Color::DarkGray));
    f.render_widget(bottom_bar, main_layout[2]);
}