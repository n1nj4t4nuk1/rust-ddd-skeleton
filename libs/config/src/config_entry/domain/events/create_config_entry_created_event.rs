//! Factory function for [`ConfigEntryCreatedEvent`].

use crate::config_entry::domain::entities::config_entry::ConfigEntry;
use crate::config_entry::domain::errors::config_entry_repository_error::ConfigEntryRepositoryError;
use crate::config_entry::domain::events::config_entry_created_event::ConfigEntryCreatedEvent;

/// Creates a [`ConfigEntryCreatedEvent`] from the given entry.
pub fn create_config_entry_created_event(
    entry: &ConfigEntry,
) -> Result<ConfigEntryCreatedEvent, ConfigEntryRepositoryError> {
    Ok(ConfigEntryCreatedEvent::new(
        entry.key().clone(),
        entry.value().clone(),
    ))
}
