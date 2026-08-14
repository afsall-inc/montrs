//! Health and error endpoints.

use axum::http::StatusCode;
use axum::Json;
use axum::routing::{get, post};
use axum::Router;
use serde_json::json;

pub fn routes() -> Router {
    Router::new()
        .route("/ok", get(|| async { Json(json!({ "ok": true })) }))
        .route(
            "/error",
            post(|| async {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "ERROR" })),
                )
            }),
        )
}