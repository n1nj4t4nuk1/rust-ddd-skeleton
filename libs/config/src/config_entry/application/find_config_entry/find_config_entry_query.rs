//! Query for finding a config entry by key.

use shared_cqrs::query::domain::query::Query;

use crate::config_entry::domain::value_objects::config_key::ConfigKey;

/// Query that requests a single config entry by its [`ConfigKey`].
pub struct FindConfigEntryQuery {
    pub key: ConfigKey,
}

impl Query for FindConfigEntryQuery {}
