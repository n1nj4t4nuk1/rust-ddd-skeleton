use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;

use crate::domain::{
    command::Command, command_bus::CommandBus, command_bus_response::CommandBusResponse,
    command_handler::CommandHandler,
};

pub struct InMemoryCommandBus {
    handlers: HashMap<String, Arc<dyn CommandHandler>>,
}

impl InMemoryCommandBus {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }
}

#[async_trait]
impl CommandBus for InMemoryCommandBus {
    fn register(&mut self, handler: Arc<dyn CommandHandler>) {
        self.handlers
            .insert(handler.subscribet_to().to_string(), handler);
    }

    async fn dispatch(&self, command: Box<dyn Command>) -> Box<dyn CommandBusResponse> {
        let handler = self.handlers.get(&command.command_type());
        if handler.is_none() {
            panic!("No handler found for command: {}", command.command_type());
        }
        let handler = handler.unwrap();
        handler.handle(command).await
    }
}
