use axum::Router;
use tower_cookies::CookieManagerLayer;

use crate::{middleware::response_middleware::main_response_mapper, router::static_router};
pub mod docs;
mod errors;
pub mod handler;
pub mod middleware;
pub mod router;
pub mod state;
pub mod constant;

pub use self::errors::{AppError, Result};

pub fn app() -> Router {
    router::api_router()
        .merge(static_router())
        .layer(axum::middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .with_state(state::AppState::new())
}
