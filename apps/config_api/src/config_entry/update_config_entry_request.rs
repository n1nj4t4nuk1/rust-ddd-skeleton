use serde::Deserialize;

/// JSON request body for `PUT /config/{key}`.
#[derive(Deserialize)]
pub struct UpdateConfigEntryRequest {
    pub value: String,
}
