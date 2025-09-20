// Menace - A terminal-based chat application with LLM integration
//
// Library root that exposes the public API and organizes internal modules

pub mod agents;
pub mod config;
pub mod core;
pub mod handlers;
pub mod types;
pub mod ui;

// Re-export commonly used items
pub use core::app::App;
pub use core::state::InputMode;
pub use handlers::{handle_events, AppEvent};