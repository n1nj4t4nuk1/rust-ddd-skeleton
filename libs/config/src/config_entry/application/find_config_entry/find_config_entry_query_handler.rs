//! [`QueryHandler`] for the find-config-entry use case.

use async_trait::async_trait;
use shared_cqrs::query::domain::query_bus_error::QueryBusError;
use shared_cqrs::query::domain::query_handler::QueryHandler;

use crate::config_entry::application::find_config_entry::find_config_entry_response::{
    ConfigEntryEntry, ConfigEntryErrorEntry,
};
use crate::config_entry::domain::errors::config_entry_repository_error::ConfigEntryRepositoryError;

use super::find_config_entry_query::FindConfigEntryQuery;
use super::find_config_entry_response::FindConfigEntryResponse;
use super::config_entry_finder::ConfigEntryFinder;

/// [`QueryHandler`] that processes [`FindConfigEntryQuery`]s by delegating
/// to [`ConfigEntryFinder`].
///
/// The finder returns a domain entity. This handler maps it to a response DTO.
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
    type Response = FindConfigEntryResponse;

    async fn handle(
        &self,
        query: FindConfigEntryQuery,
    ) -> Result<Self::Response, QueryBusError> {
        match self.finder.execute(query.key).await {
            Ok(entry) => Ok(FindConfigEntryResponse {
                config_entry: Some(ConfigEntryEntry {
                    key: entry.key().value().to_string(),
                    value: entry.value().value().to_string(),
                }),
                error: None,
            }),
            Err(e) => {
                let concept = match &e {
                    ConfigEntryRepositoryError::NotFound => "NotFound",
                    ConfigEntryRepositoryError::AlreadyExists => "AlreadyExists",
                    ConfigEntryRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(FindConfigEntryResponse {
                    config_entry: None,
                    error: Some(ConfigEntryErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
