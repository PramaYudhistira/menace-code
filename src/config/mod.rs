// Configuration management
//
// This module will handle:
// - API keys for LLM providers
// - User preferences
// - UI configuration
// - Model selection

#![allow(dead_code)]

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// Selected LLM model
    pub model: String,
    /// API endpoint (if using custom endpoints)
    pub api_endpoint: Option<String>,
    /// Maximum tokens for responses
    pub max_tokens: usize,
    /// Temperature for LLM responses
    pub temperature: f32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            model: String::from("gpt-4"),
            api_endpoint: None,
            max_tokens: 2048,
            temperature: 0.7,
        }
    }
}