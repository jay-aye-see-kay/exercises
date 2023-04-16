#![allow(unused)]

use serde_json::json;

#[tokio::test]
async fn quick_dev() -> anyhow::Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello?name=Jenny").await?.print().await?;
    hc.do_get("/hello2/Mike").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({"username":"demo1", "password": "welcome"}),
    );
    req_login.await?.print().await?;
    Ok(())
}
