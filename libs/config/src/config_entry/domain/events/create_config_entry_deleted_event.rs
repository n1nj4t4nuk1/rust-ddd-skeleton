//! Factory function for [`ConfigEntryDeletedEvent`].

use crate::config_entry::domain::entities::config_entry::ConfigEntry;
use crate::config_entry::domain::errors::config_entry_repository_error::ConfigEntryRepositoryError;
use crate::config_entry::domain::events::config_entry_deleted_event::ConfigEntryDeletedEvent;

/// Creates a [`ConfigEntryDeletedEvent`] from the deleted entry.
pub fn create_config_entry_deleted_event(
    entry: &ConfigEntry,
) -> Result<ConfigEntryDeletedEvent, ConfigEntryRepositoryError> {
    Ok(ConfigEntryDeletedEvent::new(entry.key().clone()))
}
