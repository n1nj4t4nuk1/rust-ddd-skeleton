//! [`CommandHandler`] for the update-config-entry use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use super::update_config_entry_command::UpdateConfigEntryCommand;
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
    async fn handle(&self, command: UpdateConfigEntryCommand) -> Result<(), CommandBusError> {
        self.updater
            .execute(command.key, command.value)
            .await
            .map(|_| ())
            .map_err(|e| CommandBusError::HandlerError(e.to_string()))
    }
}
