#[actix_web::test]
async fn health_check_works() {
    spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() {
    let server = email::run().await.expect("failed to bind address");
    // JoinHandle 是一个 Future，`let _ =` 会被 clippy::let_underscore_future
    // 拒绝；这里显式 drop = 把任务从测试中分离，让服务器在后台持续运行。
    std::mem::drop(tokio::spawn(server));
}
