//! Command for updating a config entry.

use shared_cqrs::command::domain::command::Command;

use crate::config_entry::domain::value_objects::config_key::ConfigKey;
use crate::config_entry::domain::value_objects::config_value::ConfigValue;

/// Command that requests an update of an existing config entry.
pub struct UpdateConfigEntryCommand {
    pub key: ConfigKey,
    pub value: ConfigValue,
}

impl Command for UpdateConfigEntryCommand {}
