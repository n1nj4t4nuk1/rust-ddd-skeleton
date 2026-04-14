//! Response types for the find-config-entry use case.

/// Data entry DTO for a config entry.
pub struct ConfigEntryEntry {
    pub key: String,
    pub value: String,
}

/// Structured error DTO for config entry operations.
pub struct ConfigEntryErrorEntry {
    pub message: String,
    pub concept: String,
}

/// Response envelope returned by [`FindConfigEntryQueryHandler`].
///
/// On success, `config_entry` contains the data and `error` is `None`.
/// On failure, `config_entry` is `None` and `error` contains the structured error.
///
/// [`FindConfigEntryQueryHandler`]: super::find_config_entry_query_handler::FindConfigEntryQueryHandler
pub struct FindConfigEntryResponse {
    pub config_entry: Option<ConfigEntryEntry>,
    pub error: Option<ConfigEntryErrorEntry>,
}
