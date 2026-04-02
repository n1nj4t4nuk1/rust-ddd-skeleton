//! POST /config handler for the Config API.

use actix_web::{post, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use config::config_entry::application::create_config_entry::create_config_entry_command::CreateConfigEntryCommand;
use config::config_entry::domain::value_objects::config_key::ConfigKey;
use config::config_entry::domain::value_objects::config_value::ConfigValue;

use crate::AppState;

use super::create_config_entry_request::CreateConfigEntryRequest;

/// Handles `POST /config`.
///
/// # Responses
///
/// - `201 Created` – the entry was persisted successfully.
/// - `409 Conflict` – an entry with the given key already exists.
/// - `500 Internal Server Error` – unexpected error.
#[post("/config")]
pub async fn handler(
    state: web::Data<AppState>,
    body: web::Json<CreateConfigEntryRequest>,
) -> impl Responder {
    debug!(key = %body.key, "POST /config");

    let command = CreateConfigEntryCommand {
        key: ConfigKey::new(body.key.clone()),
        value: ConfigValue::new(body.value.clone()),
    };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(_) => {
            info!(key = %body.key, "Config entry created");
            HttpResponse::Created().finish()
        }
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("already exists") || msg.contains("AlreadyExists") {
                warn!(key = %body.key, "Config entry already exists");
                HttpResponse::Conflict().body(msg)
            } else {
                warn!(key = %body.key, error = %msg, "Failed to create config entry");
                HttpResponse::InternalServerError().body(msg)
            }
        }
    }
}
