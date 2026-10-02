// tests/e2e_test.rs
#[cfg(test)]
mod e2e {
    use anyhow::{Ok, Result};
use axum_tutorial::app;
    use tokio::net::TcpListener;

    async fn spawn_server() -> String {
        // 端口写 0：系统随机分配，测试并行跑也不会 "Address already in use"
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, app()).await.unwrap();
        });
        format!("http://{addr}")
    }

    #[tokio::test(flavor = "multi_thread")] // 有后台 server task，用多线程 runtime 更稳
    async fn real_request() -> Result<()> {
        let base = spawn_server().await;
        let client = reqwest::Client::new();

        let res = client.get(format!("{base}/hello")).send().await.unwrap();

        assert_eq!(res.status(), 200);
        let text = res.text().await.unwrap();
        assert_eq!(text, "<h1>Hello, World!</h1>");
        Ok(())
    }
}
