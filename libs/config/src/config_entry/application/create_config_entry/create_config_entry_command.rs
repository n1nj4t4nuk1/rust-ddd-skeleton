//! Command for creating a config entry.

use shared_cqrs::command::domain::command::Command;

use crate::config_entry::domain::value_objects::config_key::ConfigKey;
use crate::config_entry::domain::value_objects::config_value::ConfigValue;

/// Command that requests the creation of a new config entry.
pub struct CreateConfigEntryCommand {
    pub key: ConfigKey,
    pub value: ConfigValue,
}

impl Command for CreateConfigEntryCommand {}
