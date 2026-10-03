use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{errors::AppError, state::Db};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

/// GET /users
pub async fn list_users(State(db): State<Db>) -> Json<Vec<User>> {
    println!("->> {:<12} - list_users", "HANDLER");
    let users = db.read().unwrap().values().cloned().collect();
    Json(users)
}

/// POST /users
pub async fn create_user(
    State(db): State<Db>,
    Json(payload): Json<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    println!("->> {:<12} - create_user", "HANDLER");

    if payload.name.trim().is_empty() {
        return Err(AppError::BadRequest("name 不能为空".into()));
    }

    let user = User {
        id: Uuid::new_v4(),
        name: payload.name,
        email: payload.email,
    };
    db.write().unwrap().insert(user.id, user.clone());

    Ok((StatusCode::CREATED, Json(user)))
}
/// GET /users/{id}
pub async fn get_user(
    State(db): State<Db>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, AppError> {
    println!("->> {:<12} - get_user {id}", "HANDLER");
    let user = db
        .read()
        .unwrap()
        .get(&id)
        .cloned()
        .ok_or_else(|| AppError::NotFound(format!("user {id} 不存在")))?;
    Ok(Json(user))
}

#[derive(Debug, Default, Deserialize)]
pub struct UpdateUser {
    /// 部分更新：字段缺省时保持原值
    pub name: Option<String>,
    pub email: Option<String>,
}

/// PUT /users/{id}
pub async fn update_user(
    State(db): State<Db>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUser>,
) -> Result<impl IntoResponse, AppError> {
    println!("->> {:<12} - update_user {id}", "HANDLER");

    let mut db = db.write().unwrap();
    let user = db
        .get_mut(&id)
        .ok_or_else(|| AppError::NotFound(format!("user {id} 不存在")))?;

    if let Some(name) = payload.name {
        if name.trim().is_empty() {
            return Err(AppError::BadRequest("name 不能为空".into()));
        }
        user.name = name;
    }
    if let Some(email) = payload.email {
        if email.trim().is_empty() {
            return Err(AppError::BadRequest("email 不能为空".into()));
        }
        user.email = email;
    }

    Ok(Json(user.clone()))
}

/// DELETE /users/{id}
pub async fn delete_user(
    State(db): State<Db>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    println!("->> {:<12} - delete_user {id}", "HANDLER");
    db.write()
        .unwrap()
        .remove(&id)
        .map(|_| StatusCode::NO_CONTENT)
        .ok_or_else(|| AppError::NotFound(format!("user {id} 不存在")))
}
