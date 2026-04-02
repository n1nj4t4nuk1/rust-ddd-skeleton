//! Domain service for finding a single config entry.

use std::sync::Arc;

use tracing::debug;

use crate::config_entry::domain::errors::config_entry_repository_error::ConfigEntryRepositoryError;
use crate::config_entry::domain::repositories::config_entry_repository::ConfigEntryRepository;
use crate::config_entry::domain::value_objects::config_key::ConfigKey;

use super::find_config_entry_response::FindConfigEntryResponse;

/// Domain service that looks up a single [`ConfigEntry`] by key.
///
/// [`ConfigEntry`]: crate::config_entry::domain::entities::config_entry::ConfigEntry
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
    ) -> Result<Option<FindConfigEntryResponse>, ConfigEntryRepositoryError> {
        debug!(key = %key, "Finding config entry");
        let entry = self.repository.find_by_key(&key).await?;

        Ok(entry.map(|e| FindConfigEntryResponse {
            key: e.key().value().to_string(),
            value: e.value().value().to_string(),
        }))
    }
}
