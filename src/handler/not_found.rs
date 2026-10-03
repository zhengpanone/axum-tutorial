use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

pub async fn not_found() -> impl IntoResponse {
    println!("->> {:<12} - not_found", "HANDLER");
    (
        StatusCode::NOT_FOUND,
        Json(json!({"error":"route not found"})),
    )
}
