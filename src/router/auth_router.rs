use axum::{Router, routing::post};

use crate::{handler::auth_handler, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/login", post(auth_handler::login))
}
