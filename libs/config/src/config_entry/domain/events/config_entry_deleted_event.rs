//! Domain event raised when a config entry is deleted.

use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};

use crate::config_entry::domain::value_objects::config_key::ConfigKey;

/// Domain event raised when a [`ConfigEntry`] is deleted.
///
/// [`ConfigEntry`]: crate::config_entry::domain::entities::config_entry::ConfigEntry
pub struct ConfigEntryDeletedEvent {
    base: DomainEventBase,
    /// Key of the deleted entry.
    pub key: ConfigKey,
}

impl ConfigEntryDeletedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "template.config.config_entry.deleted";

    /// Creates a new `ConfigEntryDeletedEvent` with auto-generated metadata.
    pub fn new(key: ConfigKey) -> Self {
        Self {
            base: DomainEventBase::new(key.value().to_string()),
            key,
        }
    }
}

impl DomainEvent for ConfigEntryDeletedEvent {
    fn event_name(&self) -> &'static str {
        Self::EVENT_NAME
    }

    fn aggregate_id(&self) -> &str {
        &self.base.aggregate_id
    }

    fn event_id(&self) -> &str {
        &self.base.event_id
    }

    fn occurred_on(&self) -> SystemTime {
        self.base.occurred_on
    }
}
