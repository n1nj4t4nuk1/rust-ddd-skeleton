//! Response for the create-config-entry use case.

use crate::config_entry::application::find_config_entry::find_config_entry_response::ConfigEntryErrorEntry;

/// Response envelope returned by [`CreateConfigEntryCommandHandler`].
///
/// On success, `error` is `None`. On failure, `error` contains the structured error.
///
/// [`CreateConfigEntryCommandHandler`]: super::create_config_entry_command_handler::CreateConfigEntryCommandHandler
pub struct CreateConfigEntryResponse {
    pub error: Option<ConfigEntryErrorEntry>,
}
