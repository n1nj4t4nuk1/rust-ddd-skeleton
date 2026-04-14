//! GET /config/{key} handler for the Config API.

use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;
use tracing::{debug, info, warn};

use config::config_entry::application::find_config_entry::find_config_entry_query::FindConfigEntryQuery;
use config::config_entry::application::find_config_entry::find_config_entry_response::FindConfigEntryResponse;
use config::config_entry::domain::value_objects::config_key::ConfigKey;

use crate::AppState;

/// JSON response body for `GET /config/{key}`.
#[derive(Serialize)]
pub struct GetConfigEntryResponse {
    pub key: String,
    pub value: String,
}

/// Handles `GET /config/{key}`.
///
/// # Responses
///
/// - `200 OK` – entry found.
/// - `404 Not Found` – no entry exists for the given key.
/// - `500 Internal Server Error` – unexpected error.
#[get("/config/{key}")]
pub async fn handler(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let key_str = path.into_inner();
    debug!(key = %key_str, "GET /config/{{key}}");

    let query = FindConfigEntryQuery { key: ConfigKey::new(key_str.clone()) };

    match state.query_bus.ask(Box::new(query)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<FindConfigEntryResponse>()
                .expect("Unexpected response type from FindConfigEntryQueryHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        info!(key = %key_str, "Config entry not found");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(key = %key_str, error = %error.message, "Failed to find config entry");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else if let Some(ref entry) = response.config_entry {
                info!(key = %key_str, "Config entry found");
                HttpResponse::Ok().json(GetConfigEntryResponse {
                    key: entry.key.clone(),
                    value: entry.value.clone(),
                })
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            warn!(key = %key_str, error = %e, "Failed to find config entry");
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}
