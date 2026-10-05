use axum::Router;

use crate::handler::not_found::not_found;
pub mod errors;
pub mod handler;
pub mod router;
pub mod state;
pub mod docs;

pub fn app() -> Router {
    router::api_router()
        .with_state(state::AppState::new())
        .fallback(not_found)
}
