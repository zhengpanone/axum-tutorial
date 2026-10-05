#![allow(unused)]
use axum::Json;
use serde::Deserialize;
use serde_json::{Value, json};
use utoipa::ToSchema;

use crate::{AppError, Result};

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginPayload {
    username: String,
    password: String,
}

pub async fn login(playload: Json<LoginPayload>) -> Result<Json<Value>> {
    if playload.username != "admin" || playload.password != "admin" {
        return Err(AppError::BadRequest("Lgoin Failed".to_owned()));
    }
    let body = Json(json!({
        "result":{
            "success": true
        }
    }));
    Ok(body)
}
