use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct HelloParams {
    name: Option<String>,
}

/// GET /hello1
pub async fn hello1(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello 左对齐", "HANDLER"); // 左对齐
    println!("->> {:>12} - handler_hello 右对齐", "HANDLER"); // 右对齐
    println!("->> {:^12} - handler_hello 居中", "HANDLER"); // 居中
    println!("->> {:*<12} - handler_hello 用 * 填充", "HANDLER"); // 用 * 填充
    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello <strong>{name}!</strong>"))
}

/// GET /hello2
pub async fn hello2(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!("Hello <strong>{name}!</strong>"))
}

#[derive(Serialize)]
pub struct HelloResp {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct HelloQuery {
    /// 缺省时不报错，直接为 None
    #[serde(default)]
    pub name: Option<String>,
}
/// GET /hello-q?name=Tom
pub async fn hello_query(Query(q): Query<HelloQuery>) -> impl IntoResponse {
    let name = q.name.unwrap_or_else(|| "World".to_string());
    (
        StatusCode::OK,
        Json(HelloResp {
            message: format!("Hello, {name}!"),
        }),
    )
}
