use serde::Deserialize;

/// JSON request body for `POST /config`.
#[derive(Deserialize)]
pub struct CreateConfigEntryRequest {
    pub key: String,
    pub value: String,
}
