//! [`CommandHandler`] for the create-config-entry use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::config_entry::application::find_config_entry::find_config_entry_response::ConfigEntryErrorEntry;
use crate::config_entry::domain::errors::config_entry_repository_error::ConfigEntryRepositoryError;

use super::create_config_entry_command::CreateConfigEntryCommand;
use super::create_config_entry_response::CreateConfigEntryResponse;
use super::config_entry_creator::ConfigEntryCreator;

/// [`CommandHandler`] that processes [`CreateConfigEntryCommand`]s by
/// delegating to [`ConfigEntryCreator`].
pub struct CreateConfigEntryCommandHandler {
    creator: ConfigEntryCreator,
}

impl CreateConfigEntryCommandHandler {
    pub fn new(creator: ConfigEntryCreator) -> Self {
        Self { creator }
    }
}

#[async_trait]
impl CommandHandler<CreateConfigEntryCommand> for CreateConfigEntryCommandHandler {
    type Response = CreateConfigEntryResponse;

    async fn handle(&self, command: CreateConfigEntryCommand) -> Result<Self::Response, CommandBusError> {
        match self.creator.execute(command.key, command.value).await {
            Ok(()) => Ok(CreateConfigEntryResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    ConfigEntryRepositoryError::NotFound => "NotFound",
                    ConfigEntryRepositoryError::AlreadyExists => "AlreadyExists",
                    ConfigEntryRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(CreateConfigEntryResponse {
                    error: Some(ConfigEntryErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
