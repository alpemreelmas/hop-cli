use crate::utils;
use httpmock::prelude::*;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::env;
use tokio::time::{sleep, Duration};

#[derive(Deserialize, Debug)]
struct DeviceCodeResponse {
    device_code: String,
    verification_uri: String,
}

#[derive(Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
}

/// Step 1: Start the login flow by requesting a device code from the server
pub async fn start_login_flow() -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client
        .post(format!("{}/auth/device-code", env::var("SERVER_URL").expect("API_URL must be set")))
        .send()
        .await?;

    let body: DeviceCodeResponse = res.json().await?;

    println!("🔑 Please open this URL to log in:\n\n{}", body.verification_uri);
    utils::open_browser(&body.verification_uri);

    Ok(body.device_code)
}

/// Step 2: Poll the server every few seconds to wait for the user to log in
pub async fn poll_for_token(device_code: &str) -> Result<String, Box<dyn std::error::Error>> {
    let server_url = env::var("SERVER_URL").expect("SERVER_URL must be set");
    let client = Client::new();

    for _ in 0..60 {
        let res = client
            .post(format!("{}/auth/token", server_url))
            .json(&serde_json::json!({ "device_code": device_code }))
            .send()
            .await;

        if let Ok(response) = res {
            if response.status().is_success() {
                let token: TokenResponse = response.json().await?;
                println!("✅ Login successful!");
                return Ok(token.access_token);
            }
        }

        println!("⌛ Waiting for login...");
        sleep(Duration::from_secs(2)).await;
    }

    Err("❌ Login timed out. Please try again.".into())
}

#[tokio::test]
async fn test_start_login_flow_success() {
    let server = MockServer::start();

    let _mock = server.mock(|when, then| {
        when.method(POST).path("/auth/device-code");
        then.status(200)
            .header("Content-Type", "application/json")
            .json_body(json!({
                "device_code": "mock-device-code",
                "verification_uri": "http://localhost/verify"
            }));
    });

    unsafe { env::set_var("SERVER_URL", &server.base_url()); }

    let device_code = start_login_flow().await.expect("Should get device code");
    assert_eq!(device_code, "mock-device-code");
}

#[tokio::test]
async fn test_poll_for_token_success() {
    let server = MockServer::start();

    let _mock = server.mock(|when, then| {
        when.method(POST).path("/auth/token");
        then.status(200)
            .header("Content-Type", "application/json")
            .json_body(json!({
                "access_token": "mock-token",
                "expires_in": 3600
            }));
    });

    unsafe { env::set_var("SERVER_URL", &server.base_url()); }

    let token = poll_for_token("mock-device-code").await.expect("Should receive token");
    assert_eq!(token, "mock-token");
}

#[tokio::test]
async fn test_poll_for_token_timeout() {
    let server = MockServer::start();

    let _mock = server.mock(|when, then| {
        when.method(POST).path("/auth/token");
        then.status(404);
    });

    unsafe { env::set_var("SERVER_URL", &server.base_url()); }

    let result = poll_for_token("mock-device-code").await;
    assert!(result.is_err());
    assert!(format!("{}", result.unwrap_err()).contains("Login timed out"));
}
