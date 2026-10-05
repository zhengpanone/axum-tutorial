use axum::Router;

use crate::router::static_router;
pub mod docs;
pub mod errors;
pub mod handler;
pub mod router;
pub mod state;

pub fn app() -> Router {
    router::api_router()
        .merge(static_router())
        .with_state(state::AppState::new())
}
