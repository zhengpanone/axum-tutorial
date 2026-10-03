use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::extract::FromRef;
use uuid::Uuid;

use crate::handler::user_handler::User;

pub type Db = Arc<RwLock<HashMap<Uuid, User>>>;

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            db: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

// 让 handler 里可以直接写 State<Db>，而不必 State<AppState>.db
impl FromRef<AppState> for Db {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}
