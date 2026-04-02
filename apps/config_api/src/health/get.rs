//! GET /health handler for the Config API.

use actix_web::{get, HttpResponse, Responder};

/// Responds to `GET /health` with HTTP 200 OK.
#[get("/health")]
pub async fn handler() -> impl Responder {
    HttpResponse::Ok().finish()
}
