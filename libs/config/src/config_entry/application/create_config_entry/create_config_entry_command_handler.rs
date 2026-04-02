//! [`CommandHandler`] for the create-config-entry use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use super::create_config_entry_command::CreateConfigEntryCommand;
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
    async fn handle(&self, command: CreateConfigEntryCommand) -> Result<(), CommandBusError> {
        self.creator
            .execute(command.key, command.value)
            .await
            .map(|_| ())
            .map_err(|e| CommandBusError::HandlerError(e.to_string()))
    }
}
