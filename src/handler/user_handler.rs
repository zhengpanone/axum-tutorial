use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{errors::AppError, state::Db};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUser {
    /// 用户名
    pub name: String,
    /// 邮箱
    pub email: String,
}

#[derive(Debug, Default, Deserialize, ToSchema)]
pub struct UpdateUser {
    /// 部分更新：字段缺省时保持原值
    pub name: Option<String>,
    /// 部分更新：字段缺省时保持原值
    pub email: Option<String>,
}

/// 列出所有用户
#[utoipa::path(
    get,
    path = "/users",
    tag = "users",
    responses(
        (status = 200, description = "用户列表", body = [User])
    )
)]
pub async fn list_users(State(db): State<Db>) -> Json<Vec<User>> {
    println!("->> {:<12} - list_users", "HANDLER");
    let users = db.read().unwrap().values().cloned().collect();
    Json(users)
}

/// 创建用户
#[utoipa::path(
    post,
    path = "/users",
    tag = "users",
    request_body = CreateUser,
    responses(
        (status = 201, description = "创建成功", body = User),
        (status = 400, description = "参数校验失败，如 name 为空")
    )
)]
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

/// 查询单个用户
#[utoipa::path(
    get,
    path = "/users/{id}",
    tag = "users",
    params(
        ("id" = Uuid, Path, description = "用户 ID")
    ),
    responses(
        (status = 200, description = "用户详情", body = User),
        (status = 404, description = "用户不存在")
    )
)]
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

/// 更新用户（部分更新）
#[utoipa::path(
    put,
    path = "/users/{id}",
    tag = "users",
    params(
        ("id" = Uuid, Path, description = "用户 ID")
    ),
    request_body = UpdateUser,
    responses(
        (status = 200, description = "更新后的用户", body = User),
        (status = 400, description = "参数校验失败"),
        (status = 404, description = "用户不存在")
    )
)]
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

/// 删除用户
#[utoipa::path(
    delete,
    path = "/users/{id}",
    tag = "users",
    params(
        ("id" = Uuid, Path, description = "用户 ID")
    ),
    responses(
        (status = 204, description = "删除成功"),
        (status = 404, description = "用户不存在")
    )
)]
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
