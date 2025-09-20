// Input widget for user text entry
//
// This widget handles the input field rendering

use ratatui::{
    style::{Color, Modifier, Style},
    widgets::Paragraph,
    Frame,
};

use crate::core::state::InputMode;

/// Renders the input widget
pub fn render_input(
    f: &mut Frame,
    input: &str,
    mode: InputMode,
    area: ratatui::layout::Rect,
) {
    let style = match mode {
        InputMode::Normal => Style::default(),
        InputMode::Editing => Style::default().fg(Color::Yellow),
        InputMode::Command => Style::default().fg(Color::Magenta),
    };

    let widget = Paragraph::new(input)
        .style(style);

    f.render_widget(widget, area);

    // Set cursor position when editing
    if mode == InputMode::Editing {
        f.set_cursor_position((area.x + input.len() as u16, area.y));
    }
}

/// Renders help text overlay
pub fn render_help(f: &mut Frame, mode: InputMode, area: ratatui::layout::Rect) {
    if mode == InputMode::Normal {
        let help = Paragraph::new("Press 'e' to edit, 'q' to quit, '/' for commands")
            .style(Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD));
        f.render_widget(help, area);
    }
}