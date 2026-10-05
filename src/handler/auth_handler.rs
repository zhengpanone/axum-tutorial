#![allow(unused)]
use axum::Json;
use serde::Deserialize;
use serde_json::{Value, json};
use tower_cookies::{Cookie, Cookies};
use utoipa::ToSchema;

use crate::{AppError, Result, constant::AUTH_TOKEN};

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginPayload {
    username: String,
    password: String,
}

pub async fn login(cookies: Cookies, playload: Json<LoginPayload>) -> Result<Json<Value>> {
    if playload.username != "admin" || playload.password != "admin" {
        return Err(AppError::BadRequest("Lgoin Failed".to_owned()));
    }

    cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));
    let body = Json(json!({
        "result":{
            "success": true
        }
    }));
    Ok(body)
}
