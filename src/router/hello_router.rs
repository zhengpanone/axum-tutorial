use axum::{Router, response::Html, routing::get};

use crate::{handler::hello_handler, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/hello", get(|| async { Html("<h1>Hello, World!</h1>") }))
        .route("/hello1", get(hello_handler::hello1))
        .route("/hello2/{name}", get(hello_handler::hello2))
        .route("/hello-q", get(hello_handler::hello_query))
}
