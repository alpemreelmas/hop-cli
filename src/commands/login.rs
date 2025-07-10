use crate::auth::{flow, token};

pub async fn run_login() {
    let device_code = flow::start_login_flow().await.unwrap();
    let access_token = flow::poll_for_token(&device_code).await.unwrap();

    token::store_token(&access_token);
    println!("🔐 Access token saved successfully.");
}

