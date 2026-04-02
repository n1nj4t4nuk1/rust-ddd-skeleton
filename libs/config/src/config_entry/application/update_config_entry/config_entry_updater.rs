//! Domain service for updating an existing config entry.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info, warn};

use crate::config_entry::domain::entities::config_entry::ConfigEntry;
use crate::config_entry::domain::errors::config_entry_repository_error::ConfigEntryRepositoryError;
use crate::config_entry::domain::events::create_config_entry_updated_event::create_config_entry_updated_event;
use crate::config_entry::domain::repositories::config_entry_repository::ConfigEntryRepository;
use crate::config_entry::domain::value_objects::config_key::ConfigKey;
use crate::config_entry::domain::value_objects::config_value::ConfigValue;

use super::update_config_entry_response::UpdateConfigEntryResponse;

/// Domain service that updates an existing [`ConfigEntry`] and publishes
/// a [`ConfigEntryUpdatedEvent`] via the event bus.
///
/// [`ConfigEntryUpdatedEvent`]: crate::config_entry::domain::events::config_entry_updated_event::ConfigEntryUpdatedEvent
pub struct ConfigEntryUpdater {
    repository: Arc<dyn ConfigEntryRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl ConfigEntryUpdater {
    pub fn new(repository: Arc<dyn ConfigEntryRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self { repository, event_bus }
    }

    pub async fn execute(
        &self,
        key: ConfigKey,
        value: ConfigValue,
    ) -> Result<UpdateConfigEntryResponse, ConfigEntryRepositoryError> {
        debug!(key = %key, "Updating config entry");

        let previous = self
            .repository
            .find_by_key(&key)
            .await?
            .ok_or_else(|| {
                warn!(key = %key, "Config entry not found for update");
                ConfigEntryRepositoryError::NotFound
            })?;

        let updated = ConfigEntry::new(key, value);
        self.repository.update(&updated).await?;

        let event = create_config_entry_updated_event(&updated, &previous)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| ConfigEntryRepositoryError::Unexpected(e.to_string()))?;

        info!(key = %updated.key(), "Config entry updated");
        Ok(UpdateConfigEntryResponse)
    }
}
