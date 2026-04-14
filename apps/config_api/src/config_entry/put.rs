//! PUT /config/{key} handler for the Config API.

use actix_web::{put, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use config::config_entry::application::update_config_entry::update_config_entry_command::UpdateConfigEntryCommand;
use config::config_entry::application::update_config_entry::update_config_entry_response::UpdateConfigEntryResponse;
use config::config_entry::domain::value_objects::config_key::ConfigKey;
use config::config_entry::domain::value_objects::config_value::ConfigValue;

use crate::AppState;

use super::update_config_entry_request::UpdateConfigEntryRequest;

/// Handles `PUT /config/{key}`.
///
/// # Responses
///
/// - `200 OK` – the entry was updated successfully.
/// - `404 Not Found` – no entry exists for the given key.
/// - `500 Internal Server Error` – unexpected error.
#[put("/config/{key}")]
pub async fn handler(
    state: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<UpdateConfigEntryRequest>,
) -> impl Responder {
    let key_str = path.into_inner();
    debug!(key = %key_str, "PUT /config/{{key}}");

    let command = UpdateConfigEntryCommand {
        key: ConfigKey::new(key_str.clone()),
        value: ConfigValue::new(body.value.clone()),
    };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<UpdateConfigEntryResponse>()
                .expect("Unexpected response type from UpdateConfigEntryCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        warn!(key = %key_str, "Config entry not found for update");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    "AlreadyExists" => HttpResponse::Conflict().body(error.message.clone()),
                    _ => {
                        warn!(key = %key_str, error = %error.message, "Failed to update config entry");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(key = %key_str, "Config entry updated");
                HttpResponse::Ok().finish()
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
