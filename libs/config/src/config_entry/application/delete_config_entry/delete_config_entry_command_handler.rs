//! [`CommandHandler`] for the delete-config-entry use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use super::delete_config_entry_command::DeleteConfigEntryCommand;
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
    async fn handle(&self, command: DeleteConfigEntryCommand) -> Result<(), CommandBusError> {
        self.deleter
            .execute(command.key)
            .await
            .map_err(|e| CommandBusError::HandlerError(e.to_string()))
    }
}
