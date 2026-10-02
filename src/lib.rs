use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};

pub fn app() -> Router {
    let router_hello = Router::new()
        .route("/hello", get(|| async { Html("<h1>Hello, World!</h1>") }))
        .route("/hello2", get(handler_hello));
    router_hello
}

async fn handler_hello() -> impl IntoResponse {
    println!("->> {:<12} - handler_hello 左对齐", "HANDLER"); // 左对齐
    println!("->> {:>12} - handler_hello 右对齐", "HANDLER"); // 右对齐
    println!("->> {:^12} - handler_hello 居中", "HANDLER"); // 居中
    println!("->> {:*<12} - handler_hello 用 * 填充", "HANDLER"); // 用 * 填充
    Html("Hello <strong>World!</strong>")
}
