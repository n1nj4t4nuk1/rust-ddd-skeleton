//! Domain service for deleting a config entry.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info, warn};

use crate::config_entry::domain::errors::config_entry_repository_error::ConfigEntryRepositoryError;
use crate::config_entry::domain::events::create_config_entry_deleted_event::create_config_entry_deleted_event;
use crate::config_entry::domain::repositories::config_entry_repository::ConfigEntryRepository;
use crate::config_entry::domain::value_objects::config_key::ConfigKey;

/// Domain service that deletes a [`ConfigEntry`] and publishes
/// a [`ConfigEntryDeletedEvent`] via the event bus.
///
/// [`ConfigEntry`]: crate::config_entry::domain::entities::config_entry::ConfigEntry
/// [`ConfigEntryDeletedEvent`]: crate::config_entry::domain::events::config_entry_deleted_event::ConfigEntryDeletedEvent
pub struct ConfigEntryDeleter {
    repository: Arc<dyn ConfigEntryRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl ConfigEntryDeleter {
    pub fn new(repository: Arc<dyn ConfigEntryRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self { repository, event_bus }
    }

    pub async fn execute(
        &self,
        key: ConfigKey,
    ) -> Result<(), ConfigEntryRepositoryError> {
        debug!(key = %key, "Deleting config entry");

        let entry = self
            .repository
            .find_by_key(&key)
            .await?
            .ok_or_else(|| {
                warn!(key = %key, "Config entry not found for deletion");
                ConfigEntryRepositoryError::NotFound
            })?;

        self.repository.delete(&key).await?;

        let event = create_config_entry_deleted_event(&entry)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| ConfigEntryRepositoryError::Unexpected(e.to_string()))?;

        info!(key = %key, "Config entry deleted");
        Ok(())
    }
}
