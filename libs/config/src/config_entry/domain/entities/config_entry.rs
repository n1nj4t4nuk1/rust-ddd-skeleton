//! `ConfigEntry` aggregate root.

use crate::config_entry::domain::value_objects::config_key::ConfigKey;
use crate::config_entry::domain::value_objects::config_value::ConfigValue;

/// Aggregate root representing a key-value config entry.
///
/// The `key` acts as the unique identifier for this entry.
#[derive(Clone)]
pub struct ConfigEntry {
    /// Unique key that identifies this entry.
    key: ConfigKey,
    /// Value stored under the key.
    value: ConfigValue,
}

impl ConfigEntry {
    /// Creates a new `ConfigEntry`.
    pub fn new(key: ConfigKey, value: ConfigValue) -> Self {
        Self { key, value }
    }

    /// Returns the key of this entry.
    pub fn key(&self) -> &ConfigKey {
        &self.key
    }

    /// Returns the value stored under the key.
    pub fn value(&self) -> &ConfigValue {
        &self.value
    }
}
