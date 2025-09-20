// Application state management
//
// This module handles:
// - Application state transitions
// - State validation

/// Represents the current input mode of the application
/// (Currently only one mode - always ready for input)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    /// Always in editing mode - text input active
    Editing,
}

impl Default for InputMode {
    fn default() -> Self {
        InputMode::Editing
    }
}