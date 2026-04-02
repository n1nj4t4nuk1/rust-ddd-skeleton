//! DELETE /config/{key} handler for the Config API.

use actix_web::{delete, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use config::config_entry::application::delete_config_entry::delete_config_entry_command::DeleteConfigEntryCommand;
use config::config_entry::domain::value_objects::config_key::ConfigKey;

use crate::AppState;

/// Handles `DELETE /config/{key}`.
///
/// # Responses
///
/// - `204 No Content` – the entry was deleted successfully.
/// - `404 Not Found` – no entry exists for the given key.
/// - `500 Internal Server Error` – unexpected error.
#[delete("/config/{key}")]
pub async fn handler(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let key_str = path.into_inner();
    debug!(key = %key_str, "DELETE /config/{{key}}");

    let command = DeleteConfigEntryCommand { key: ConfigKey::new(key_str.clone()) };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(_) => {
            info!(key = %key_str, "Config entry deleted");
            HttpResponse::NoContent().finish()
        }
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("not found") || msg.contains("NotFound") {
                warn!(key = %key_str, "Config entry not found for deletion");
                HttpResponse::NotFound().finish()
            } else {
                warn!(key = %key_str, error = %msg, "Failed to delete config entry");
                HttpResponse::InternalServerError().body(msg)
            }
        }
    }
}
