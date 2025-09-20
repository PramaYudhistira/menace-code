// Chat widget for displaying message history
//
// This widget handles rendering of the chat messages

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
    Frame,
};

use crate::types::MessageRole;
use crate::core::App;

/// Renders the chat messages widget with text wrapping and auto-scroll
pub fn render_chat_with_scroll(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    // Build all message lines
    let mut lines = Vec::new();

    for msg in &app.messages {
        let (prefix, style) = match msg.role {
            MessageRole::User => ("You: ", Style::default().fg(Color::Cyan)),
            MessageRole::Assistant => ("AI: ", Style::default().fg(Color::Green)),
            MessageRole::System => ("System: ", Style::default().fg(Color::Yellow)),
        };

        // Simple: just add the message with prefix
        lines.push(Line::from(vec![
            Span::styled(prefix, style),
            Span::raw(&msg.content),
        ]));

        // Add spacing between messages
        lines.push(Line::from(""));
    }

    // Create paragraph with all messages - wrapping enabled
    let paragraph = Paragraph::new(lines)
        .wrap(Wrap { trim: false });

    // Simple scrolling: scroll_offset tracks how many lines from the TOP to skip
    // When scroll_offset = 0, we want to be at the BOTTOM (show latest messages)
    // So we need to calculate how many lines to skip from the top

    let total_lines = app.messages.len() * 2; // Each message + spacing
    let visible_lines = area.height as usize;

    let scroll_position = if total_lines > visible_lines {
        // Content overflows - need to scroll
        if app.scroll_offset == 0 {
            // At bottom - skip enough lines to show the last page
            (total_lines - visible_lines) as u16
        } else {
            // User scrolled up - adjust position
            let bottom_position = total_lines - visible_lines;
            (bottom_position.saturating_sub(app.scroll_offset)) as u16
        }
    } else {
        // Everything fits - no scroll needed
        0
    };

    // Apply the scroll and render
    let scrolled = paragraph.scroll((scroll_position, 0));
    f.render_widget(scrolled, area);
}