//! Command for deleting a config entry.

use shared_cqrs::command::domain::command::Command;

use crate::config_entry::domain::value_objects::config_key::ConfigKey;

/// Command that requests the deletion of a config entry by key.
pub struct DeleteConfigEntryCommand {
    pub key: ConfigKey,
}

impl Command for DeleteConfigEntryCommand {}
