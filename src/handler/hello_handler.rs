use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize)]
pub struct HelloParams {
    name: Option<String>,
}

/// Hello, Query 参数
#[utoipa::path(
    get,
    path = "/hello1",
    tag = "hello",
    params(
        ("name" = Option<String>, Query, description = "名字，缺省为 World")
    ),
    responses(
        (status = 200, description = "返回 HTML 问候页面")
    )
)]
pub async fn hello1(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello 左对齐", "HANDLER"); // 左对齐
    println!("->> {:>12} - handler_hello 右对齐", "HANDLER"); // 右对齐
    println!("->> {:^12} - handler_hello 居中", "HANDLER"); // 居中
    println!("->> {:*<12} - handler_hello 用 * 填充", "HANDLER"); // 用 * 填充
    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello <strong>{name}!</strong>"))
}

/// Hello, Path 参数
#[utoipa::path(
    get,
    path = "/hello2/{name}",
    tag = "hello",
    params(
        ("name" = String, Path, description = "名字")
    ),
    responses(
        (status = 200, description = "返回 HTML 问候页面")
    )
)]
pub async fn hello2(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!("Hello <strong>{name}!</strong>"))
}

#[derive(Serialize, ToSchema)]
pub struct HelloResp {
    pub message: String,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct HelloQuery {
    /// 缺省时不报错，直接为 None
    #[serde(default)]
    pub name: Option<String>,
}

/// Hello, JSON 响应
#[utoipa::path(
    get,
    path = "/hello-q",
    tag = "hello",
    params(HelloQuery),
    responses(
        (status = 200, description = "JSON 问候", body = HelloResp)
    )
)]
pub async fn hello_query(Query(q): Query<HelloQuery>) -> impl IntoResponse {
    let name = q.name.unwrap_or_else(|| "World".to_string());
    (
        StatusCode::OK,
        Json(HelloResp {
            message: format!("Hello, {name}!"),
        }),
    )
}
