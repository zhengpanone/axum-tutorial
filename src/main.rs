// src/main.rs
#![allow(unused)]
use axum::{Router, response::Html, routing::get};
use axum_tutorial::app;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app()).await.unwrap();
}
// 进程内测试，不起端口（oneshot）
// Router 本身就是个 tower::Service，可以直接在内存里塞一个 Request 进去，不用 bind socket，毫秒级、无端口冲突。
#[cfg(test)]
mod tests {
    use crate::app;
    use axum::{
        body::{Body, to_bytes},
        http::{Request, StatusCode},
    };
    use tower::ServiceExt; // for oneshot

    #[tokio::test]
    async fn hello_world() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/hello")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // 1) 拆成 (parts, body)，拿到 body 的所有权
        let (parts, body) = response.into_parts();
        // 2) 把流式 body 全读出来，第二个参数是大小上限
        let bytes = to_bytes(body, usize::MAX).await.unwrap();
        // let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        // 3) 转成 &str 打印
        let html = String::from_utf8(bytes.to_vec()).unwrap();

        println!("status = {:?}", parts.status);
        println!("body = {}", html);

        assert!(html.contains("Hello, World!"));

        assert_eq!(&bytes[..], b"<h1>Hello, World!</h1>");
    }

    #[tokio::test]
    async fn not_found() {
        let response = app()
            .oneshot(Request::builder().uri("/nope").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
