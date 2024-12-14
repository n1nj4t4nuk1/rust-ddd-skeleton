use async_trait::async_trait;

use super::{query::Query, query_bus_response::QueryBusResponse};

#[async_trait]
pub trait QueryHandler: Send + Sync + 'static {
    async fn handle(&self, query: Box<dyn Query>) -> Box<dyn QueryBusResponse>;
    fn subscribet_to(&self) -> String;
}
