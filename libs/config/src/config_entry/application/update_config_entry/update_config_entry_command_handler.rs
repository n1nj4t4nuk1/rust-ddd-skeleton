//! [`CommandHandler`] for the update-config-entry use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::config_entry::application::find_config_entry::find_config_entry_response::ConfigEntryErrorEntry;
use crate::config_entry::domain::errors::config_entry_repository_error::ConfigEntryRepositoryError;

use super::update_config_entry_command::UpdateConfigEntryCommand;
use super::update_config_entry_response::UpdateConfigEntryResponse;
use super::config_entry_updater::ConfigEntryUpdater;

/// [`CommandHandler`] that processes [`UpdateConfigEntryCommand`]s by
/// delegating to [`ConfigEntryUpdater`].
pub struct UpdateConfigEntryCommandHandler {
    updater: ConfigEntryUpdater,
}

impl UpdateConfigEntryCommandHandler {
    pub fn new(updater: ConfigEntryUpdater) -> Self {
        Self { updater }
    }
}

#[async_trait]
impl CommandHandler<UpdateConfigEntryCommand> for UpdateConfigEntryCommandHandler {
    type Response = UpdateConfigEntryResponse;

    async fn handle(&self, command: UpdateConfigEntryCommand) -> Result<Self::Response, CommandBusError> {
        match self.updater.execute(command.key, command.value).await {
            Ok(()) => Ok(UpdateConfigEntryResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    ConfigEntryRepositoryError::NotFound => "NotFound",
                    ConfigEntryRepositoryError::AlreadyExists => "AlreadyExists",
                    ConfigEntryRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(UpdateConfigEntryResponse {
                    error: Some(ConfigEntryErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
