use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;

use crate::domain::{
    query::Query, query_bus::QueryBus, query_bus_response::QueryBusResponse,
    query_handler::QueryHandler,
};

pub struct InMemoryQueryBus {
    handlers: HashMap<String, Arc<dyn QueryHandler>>,
}

impl InMemoryQueryBus {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }
}

#[async_trait]
impl QueryBus for InMemoryQueryBus {
    fn register(&mut self, handler: Arc<dyn QueryHandler>) {
        self.handlers
            .insert(handler.subscribet_to().to_string(), handler);
    }

    async fn ask(&self, query: Box<dyn Query>) -> Box<dyn QueryBusResponse> {
        let handler = self.handlers.get(&query.get_type());
        if handler.is_none() {
            panic!("No handler found for query: {}", query.get_type());
        }
        let handler = handler.unwrap();
        handler.handle(query).await
    }
}
