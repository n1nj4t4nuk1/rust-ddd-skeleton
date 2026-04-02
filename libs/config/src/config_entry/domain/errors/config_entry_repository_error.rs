//! Error types for config entry repository operations.

use thiserror::Error;

/// Errors that can occur during [`ConfigEntryRepository`] operations.
///
/// [`ConfigEntryRepository`]: crate::config_entry::domain::repositories::config_entry_repository::ConfigEntryRepository
#[derive(Debug, Error)]
pub enum ConfigEntryRepositoryError {
    /// The requested entry was not found.
    #[error("config entry not found")]
    NotFound,

    /// An entry with the same key already exists.
    #[error("config entry already exists")]
    AlreadyExists,

    /// An unexpected storage error occurred.
    #[error("unexpected error: {0}")]
    Unexpected(String),
}
