use async_trait::async_trait;

use super::{command::Command, command_bus_response::CommandBusResponse};

#[async_trait]
pub trait CommandHandler: Send + Sync + 'static {
    async fn handle(&self, command: Box<dyn Command>) -> Box<dyn CommandBusResponse>;
    fn subscribet_to(&self) -> String;
}
