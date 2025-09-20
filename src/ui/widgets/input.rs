// Input widget for user text entry
//
// This widget handles the input field rendering

use ratatui::{
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};

use crate::core::state::InputMode;

/// Renders the input widget
pub fn render_input(
    f: &mut Frame,
    input: &str,
    _mode: InputMode,
    area: ratatui::layout::Rect,
) {
    // Always use a subtle color for the input text
    let widget = Paragraph::new(input)
        .style(Style::default().fg(Color::White));

    f.render_widget(widget, area);

    // Always show cursor (handle multi-line)
    let lines: Vec<&str> = input.lines().collect();
    let current_line = lines.last().unwrap_or(&"");
    let line_number = lines.len().saturating_sub(1);

    let cursor_x = area.x + current_line.len() as u16;
    let cursor_y = area.y + line_number as u16;

    f.set_cursor_position((cursor_x, cursor_y));
}

