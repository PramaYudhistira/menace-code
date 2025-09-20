// Shared types and data structures
//
// This module contains:
// - Message types for chat history
// - Error types
// - Common enums and structs used across the application

#![allow(dead_code)]

use std::time::SystemTime;

/// Represents a chat message
#[derive(Debug, Clone)]
pub struct Message {
    /// Message content
    pub content: String,
    /// Who sent the message
    pub role: MessageRole,
    /// When the message was sent
    pub timestamp: SystemTime,
}

/// Role of the message sender
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageRole {
    /// User message
    User,
    /// AI/Assistant message
    Assistant,
    /// System message (errors, notifications)
    System,
}

impl Message {
    /// Creates a new user message
    pub fn user(content: String) -> Self {
        Message {
            content,
            role: MessageRole::User,
            timestamp: SystemTime::now(),
        }
    }

    /// Creates a new assistant message
    pub fn assistant(content: String) -> Self {
        Message {
            content,
            role: MessageRole::Assistant,
            timestamp: SystemTime::now(),
        }
    }

    /// Creates a new system message
    pub fn system(content: String) -> Self {
        Message {
            content,
            role: MessageRole::System,
            timestamp: SystemTime::now(),
        }
    }
}