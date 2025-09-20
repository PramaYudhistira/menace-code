// Application state and core business logic
//
// This module contains:
// - App struct: main application state
// - Message handling logic
// - Future: RIG agent integration will live here

use super::state::InputMode;
use crate::types::Message;
use std::time::Instant;

/// Core application state
pub struct App {
    /// Current input buffer for user typing
    pub input: String,
    /// Chat history
    pub messages: Vec<Message>,
    /// Current input mode
    pub input_mode: InputMode,
    /// Track last Ctrl+C press for double-press quit
    pub last_ctrl_c: Option<Instant>,
    /// Scroll offset for chat history (0 = bottom/latest)
    pub scroll_offset: usize,
}

impl Default for App {
    fn default() -> Self {
        App {
            input: String::new(),
            messages: Vec::new(),
            input_mode: InputMode::default(),
            last_ctrl_c: None,
            scroll_offset: 0,
        }
    }
}

impl App {
    pub fn send_message(&mut self) {
        let msg: String = self.input.drain(..).collect();
        if !msg.is_empty() {
            self.messages.push(Message::user(msg.clone()));
            // Placeholder response - will be replaced with actual RIG integration
            self.messages.push(Message::assistant(format!("I received: '{}'", msg)));
            // Reset scroll to bottom when new message is sent
            self.scroll_offset = 0;
        }
    }

    pub fn scroll_up(&mut self, amount: usize) {
        self.scroll_offset = self.scroll_offset.saturating_add(amount);
        // Cap at max messages
        let max_offset = self.messages.len().saturating_sub(1);
        self.scroll_offset = self.scroll_offset.min(max_offset);
    }

    pub fn scroll_down(&mut self, amount: usize) {
        self.scroll_offset = self.scroll_offset.saturating_sub(amount);
    }

    pub fn scroll_to_bottom(&mut self) {
        self.scroll_offset = 0;
    }
}