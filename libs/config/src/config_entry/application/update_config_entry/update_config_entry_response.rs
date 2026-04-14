//! Response for the update-config-entry use case.

use crate::config_entry::application::find_config_entry::find_config_entry_response::ConfigEntryErrorEntry;

/// Response envelope returned by [`UpdateConfigEntryCommandHandler`].
///
/// On success, `error` is `None`. On failure, `error` contains the structured error.
///
/// [`UpdateConfigEntryCommandHandler`]: super::update_config_entry_command_handler::UpdateConfigEntryCommandHandler
pub struct UpdateConfigEntryResponse {
    pub error: Option<ConfigEntryErrorEntry>,
}
