//! Factory function for [`ConfigEntryUpdatedEvent`].

use crate::config_entry::domain::entities::config_entry::ConfigEntry;
use crate::config_entry::domain::errors::config_entry_repository_error::ConfigEntryRepositoryError;
use crate::config_entry::domain::events::config_entry_updated_event::ConfigEntryUpdatedEvent;

/// Creates a [`ConfigEntryUpdatedEvent`] from the updated and previous entries.
pub fn create_config_entry_updated_event(
    updated: &ConfigEntry,
    previous: &ConfigEntry,
) -> Result<ConfigEntryUpdatedEvent, ConfigEntryRepositoryError> {
    Ok(ConfigEntryUpdatedEvent::new(
        updated.key().clone(),
        updated.value().clone(),
        previous.value().clone(),
    ))
}
