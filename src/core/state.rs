// Application state management
//
// This module handles:
// - Input modes
// - Application state transitions
// - State validation

/// Represents the current input mode of the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    /// Normal mode - navigation and commands
    Normal,
    /// Editing mode - text input active
    Editing,
    /// Command mode - for slash commands (future feature)
    Command,
}

impl Default for InputMode {
    fn default() -> Self {
        InputMode::Editing
    }
}