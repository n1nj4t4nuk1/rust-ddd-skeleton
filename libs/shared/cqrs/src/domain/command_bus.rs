use std::sync::Arc;

use async_trait::async_trait;

use super::{
    command::Command, command_bus_response::CommandBusResponse, command_handler::CommandHandler,
};

#[async_trait]
pub trait CommandBus: Send + Sync + 'static {
    fn register(&mut self, handler: Arc<dyn CommandHandler>);
    async fn dispatch(&self, command: Box<dyn Command>) -> Box<dyn CommandBusResponse>;
}
