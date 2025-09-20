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
}

impl Default for App {
    fn default() -> Self {
        App {
            input: String::new(),
            messages: Vec::new(),
            input_mode: InputMode::default(),
            last_ctrl_c: None,
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
        }
    }
}