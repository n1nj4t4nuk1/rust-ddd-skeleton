use std::sync::Arc;

use async_trait::async_trait;

use super::{query::Query, query_bus_response::QueryBusResponse, query_handler::QueryHandler};

#[async_trait]
pub trait QueryBus: Send + Sync + 'static {
    fn register(&mut self, handler: Arc<dyn QueryHandler>);
    async fn ask(&self, query: Box<dyn Query>) -> Box<dyn QueryBusResponse>;
}
