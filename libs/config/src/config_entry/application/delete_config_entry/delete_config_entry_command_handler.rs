//! [`CommandHandler`] for the delete-config-entry use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::config_entry::application::find_config_entry::find_config_entry_response::ConfigEntryErrorEntry;
use crate::config_entry::domain::errors::config_entry_repository_error::ConfigEntryRepositoryError;

use super::delete_config_entry_command::DeleteConfigEntryCommand;
use super::delete_config_entry_response::DeleteConfigEntryResponse;
use super::config_entry_deleter::ConfigEntryDeleter;

/// [`CommandHandler`] that processes [`DeleteConfigEntryCommand`]s by
/// delegating to [`ConfigEntryDeleter`].
pub struct DeleteConfigEntryCommandHandler {
    deleter: ConfigEntryDeleter,
}

impl DeleteConfigEntryCommandHandler {
    pub fn new(deleter: ConfigEntryDeleter) -> Self {
        Self { deleter }
    }
}

#[async_trait]
impl CommandHandler<DeleteConfigEntryCommand> for DeleteConfigEntryCommandHandler {
    type Response = DeleteConfigEntryResponse;

    async fn handle(&self, command: DeleteConfigEntryCommand) -> Result<Self::Response, CommandBusError> {
        match self.deleter.execute(command.key).await {
            Ok(()) => Ok(DeleteConfigEntryResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    ConfigEntryRepositoryError::NotFound => "NotFound",
                    ConfigEntryRepositoryError::AlreadyExists => "AlreadyExists",
                    ConfigEntryRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(DeleteConfigEntryResponse {
                    error: Some(ConfigEntryErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
