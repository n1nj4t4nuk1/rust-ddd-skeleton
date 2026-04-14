//! Domain service for finding a single config entry.

use std::sync::Arc;

use tracing::debug;

use crate::config_entry::domain::entities::config_entry::ConfigEntry;
use crate::config_entry::domain::errors::config_entry_repository_error::ConfigEntryRepositoryError;
use crate::config_entry::domain::repositories::config_entry_repository::ConfigEntryRepository;
use crate::config_entry::domain::value_objects::config_key::ConfigKey;

/// Domain service that looks up a single [`ConfigEntry`] by key.
///
/// Returns the domain entity directly. The handler is responsible for
/// mapping it to a response DTO.
pub struct ConfigEntryFinder {
    repository: Arc<dyn ConfigEntryRepository>,
}

impl ConfigEntryFinder {
    pub fn new(repository: Arc<dyn ConfigEntryRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        key: ConfigKey,
    ) -> Result<ConfigEntry, ConfigEntryRepositoryError> {
        debug!(key = %key, "Finding config entry");
        let entry = self.repository.find_by_key(&key).await?;

        entry.ok_or(ConfigEntryRepositoryError::NotFound)
    }
}
