// Chat widget for displaying message history
//
// This widget handles rendering of the chat messages

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{List, ListItem},
    Frame,
};

use crate::types::{Message, MessageRole};

/// Renders the chat messages widget
pub fn render_chat(f: &mut Frame, messages: &[Message], area: ratatui::layout::Rect) {
    let items: Vec<ListItem> = messages
        .iter()
        .map(|msg| {
            let (prefix, style) = match msg.role {
                MessageRole::User => ("You: ", Style::default().fg(Color::Cyan)),
                MessageRole::Assistant => ("AI: ", Style::default().fg(Color::Green)),
                MessageRole::System => ("System: ", Style::default().fg(Color::Yellow)),
            };

            let content = Line::from(vec![
                Span::styled(prefix, style),
                Span::raw(&msg.content),
            ]);

            ListItem::new(content)
        })
        .collect();

    let widget = List::new(items);

    f.render_widget(widget, area);
}