//! Repository trait for the config entry aggregate.

use async_trait::async_trait;

use crate::config_entry::domain::entities::config_entry::ConfigEntry;
use crate::config_entry::domain::errors::config_entry_repository_error::ConfigEntryRepositoryError;
use crate::config_entry::domain::value_objects::config_key::ConfigKey;

/// Async persistence contract for [`ConfigEntry`] aggregates.
///
/// Concrete implementations are found in the `infrastructure` layer.
#[async_trait]
pub trait ConfigEntryRepository: Send + Sync {
    /// Persists a new config entry.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigEntryRepositoryError::AlreadyExists`] if an entry with
    /// the same key already exists, or [`ConfigEntryRepositoryError::Unexpected`]
    /// on storage failure.
    async fn save(&self, entry: &ConfigEntry) -> Result<(), ConfigEntryRepositoryError>;

    /// Retrieves a config entry by its [`ConfigKey`].
    ///
    /// Returns `Ok(None)` if no entry is found.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigEntryRepositoryError::Unexpected`] on storage failure.
    async fn find_by_key(
        &self,
        key: &ConfigKey,
    ) -> Result<Option<ConfigEntry>, ConfigEntryRepositoryError>;

    /// Updates an existing config entry.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigEntryRepositoryError::NotFound`] if the entry does not
    /// exist, or [`ConfigEntryRepositoryError::Unexpected`] on storage failure.
    async fn update(&self, entry: &ConfigEntry) -> Result<(), ConfigEntryRepositoryError>;

    /// Deletes a config entry by its [`ConfigKey`].
    ///
    /// # Errors
    ///
    /// Returns [`ConfigEntryRepositoryError::NotFound`] if the entry does not
    /// exist, or [`ConfigEntryRepositoryError::Unexpected`] on storage failure.
    async fn delete(&self, key: &ConfigKey) -> Result<(), ConfigEntryRepositoryError>;
}
