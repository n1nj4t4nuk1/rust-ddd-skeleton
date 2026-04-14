//! Domain service for creating config entries.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info};

use crate::config_entry::domain::entities::config_entry::ConfigEntry;
use crate::config_entry::domain::errors::config_entry_repository_error::ConfigEntryRepositoryError;
use crate::config_entry::domain::events::create_config_entry_created_event::create_config_entry_created_event;
use crate::config_entry::domain::repositories::config_entry_repository::ConfigEntryRepository;
use crate::config_entry::domain::value_objects::config_key::ConfigKey;
use crate::config_entry::domain::value_objects::config_value::ConfigValue;

/// Domain service that persists a new [`ConfigEntry`] and publishes
/// a [`ConfigEntryCreatedEvent`] via the event bus.
///
/// [`ConfigEntryCreatedEvent`]: crate::config_entry::domain::events::config_entry_created_event::ConfigEntryCreatedEvent
pub struct ConfigEntryCreator {
    repository: Arc<dyn ConfigEntryRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl ConfigEntryCreator {
    pub fn new(repository: Arc<dyn ConfigEntryRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self { repository, event_bus }
    }

    pub async fn execute(
        &self,
        key: ConfigKey,
        value: ConfigValue,
    ) -> Result<(), ConfigEntryRepositoryError> {
        let entry = ConfigEntry::new(key, value);
        debug!(key = %entry.key(), "Creating config entry");

        self.repository.save(&entry).await?;

        let event = create_config_entry_created_event(&entry)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| ConfigEntryRepositoryError::Unexpected(e.to_string()))?;

        info!(key = %entry.key(), "Config entry created");
        Ok(())
    }
}
