//! Error types shared by Healthpoint crates.

use thiserror::Error;

/// Result alias for Healthpoint crates.
pub type Result<T> = std::result::Result<T, HealthpointError>;

/// Domain-level error type.
#[derive(Debug, Error)]
pub enum HealthpointError {
    /// Configuration error.
    #[error("configuration error: {0}")]
    Config(String),

    /// Invalid caller-supplied input.
    #[error("invalid input: {0}")]
    InvalidInput(String),

    /// HTTP/API error.
    #[error("Healthpoint API error {status}: {message}")]
    Api {
        /// HTTP status code.
        status: u16,
        /// Human-readable message.
        message: String,
    },

    /// Network/request error.
    #[error("request failed: {0}")]
    Request(String),

    /// Parse/mapping error.
    #[error("parse error: {0}")]
    Parse(String),

    /// Unsupported operation.
    #[error("unsupported operation: {0}")]
    Unsupported(String),
}
