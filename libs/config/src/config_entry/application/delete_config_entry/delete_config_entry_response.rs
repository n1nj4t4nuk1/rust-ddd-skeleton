//! Response for the delete-config-entry use case.

use crate::config_entry::application::find_config_entry::find_config_entry_response::ConfigEntryErrorEntry;

/// Response envelope returned by [`DeleteConfigEntryCommandHandler`].
///
/// On success, `error` is `None`. On failure, `error` contains the structured error.
///
/// [`DeleteConfigEntryCommandHandler`]: super::delete_config_entry_command_handler::DeleteConfigEntryCommandHandler
pub struct DeleteConfigEntryResponse {
    pub error: Option<ConfigEntryErrorEntry>,
}
