use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{Error, Result};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(payload: Json<LogingPayload>) -> Result<Json<Value>> {
    println!("->> {:12} - api_login", "HANDER");

    // TODO real db/auth logic
    if payload.username != "demo1" || payload.password != "welcome" {
        return Err(Error::LoginFail);
    }

    // TODO set cookies

    let body = Json(json!(
        { "result": { "success": true } }
    ));
    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LogingPayload {
    username: String,
    password: String,
}
