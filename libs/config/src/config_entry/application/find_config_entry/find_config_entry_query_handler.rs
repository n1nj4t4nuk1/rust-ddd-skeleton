//! [`QueryHandler`] for the find-config-entry use case.

use async_trait::async_trait;
use shared_cqrs::query::domain::query_bus_error::QueryBusError;
use shared_cqrs::query::domain::query_handler::QueryHandler;

use super::find_config_entry_query::FindConfigEntryQuery;
use super::find_config_entry_response::FindConfigEntryResponse;
use super::config_entry_finder::ConfigEntryFinder;

/// [`QueryHandler`] that processes [`FindConfigEntryQuery`]s by delegating
/// to [`ConfigEntryFinder`].
pub struct FindConfigEntryQueryHandler {
    finder: ConfigEntryFinder,
}

impl FindConfigEntryQueryHandler {
    pub fn new(finder: ConfigEntryFinder) -> Self {
        Self { finder }
    }
}

#[async_trait]
impl QueryHandler<FindConfigEntryQuery> for FindConfigEntryQueryHandler {
    type Response = Option<FindConfigEntryResponse>;

    async fn handle(
        &self,
        query: FindConfigEntryQuery,
    ) -> Result<Self::Response, QueryBusError> {
        self.finder
            .execute(query.key)
            .await
            .map_err(|e| QueryBusError::HandlerError(e.to_string()))
    }
}
