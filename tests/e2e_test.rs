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

    // 依次打印：请求方法+地址 / 状态码 / 每个 header / 响应体，返回 body 文本
    // method 从 build 出的 Request 中读取，无需手工传字符串
    async fn print_response(
        client: &reqwest::Client,
        builder: reqwest::RequestBuilder,
    ) -> Result<String> {
        let req = builder.build()?;
        let method = req.method().clone();
        let res = client.execute(req).await?;

        println!("Response for {method} {}", res.url());
        println!("Status: {}", res.status());
        for (name, value) in res.headers() {
            println!("{name}: {}", value.to_str().unwrap_or("<非 UTF-8 值>"));
        }
        let body = res.text().await?;
        println!("Body:\n{body}");
        Ok(body)
    }

    #[tokio::test(flavor = "multi_thread")] // 有后台 server task，用多线程 runtime 更稳
    async fn real_request() -> Result<()> {
        let base = spawn_server().await;
        let client = reqwest::Client::new();

        let text: String = print_response(&client, client.get(format!("{base}/hello"))).await?;
        assert_eq!(text, "<h1>Hello, World!</h1>");

        let res = client.get(format!("{base}/hello")).send().await.unwrap();

        assert_eq!(res.status(), 200);
        let text = res.text().await.unwrap();
        assert_eq!(text, "<h1>Hello, World!</h1>");

        Ok(())
    }
}
