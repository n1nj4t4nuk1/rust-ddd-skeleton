//! Domain event raised when a config entry is updated.

use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};

use crate::config_entry::domain::value_objects::config_key::ConfigKey;
use crate::config_entry::domain::value_objects::config_value::ConfigValue;

/// Domain event raised when an existing [`ConfigEntry`] is updated.
///
/// [`ConfigEntry`]: crate::config_entry::domain::entities::config_entry::ConfigEntry
pub struct ConfigEntryUpdatedEvent {
    base: DomainEventBase,
    /// Key of the updated entry.
    pub key: ConfigKey,
    /// New value after the update.
    pub new_value: ConfigValue,
    /// Previous value before the update.
    pub old_value: ConfigValue,
}

impl ConfigEntryUpdatedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "template.config.config_entry.updated";

    /// Creates a new `ConfigEntryUpdatedEvent` with auto-generated metadata.
    pub fn new(key: ConfigKey, new_value: ConfigValue, old_value: ConfigValue) -> Self {
        Self {
            base: DomainEventBase::new(key.value().to_string()),
            key,
            new_value,
            old_value,
        }
    }
}

impl DomainEvent for ConfigEntryUpdatedEvent {
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
