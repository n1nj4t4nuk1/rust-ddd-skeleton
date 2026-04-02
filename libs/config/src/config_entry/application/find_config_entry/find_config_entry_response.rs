//! Response for the find-config-entry use case.

/// Returned by [`ConfigEntryFinder`] when an entry is found.
///
/// [`ConfigEntryFinder`]: super::config_entry_finder::ConfigEntryFinder
pub struct FindConfigEntryResponse {
    pub key: String,
    pub value: String,
}
