use axum::Router;

use crate::state::AppState;

pub mod hello_router;
pub mod user_router;

pub fn api_router() -> Router<AppState> {
    Router::<AppState>::new()
        .merge(hello_router::router()) // Router<()> 可并入任意 Router<S>
        .nest("/users", user_router::routes())
}
