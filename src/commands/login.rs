use crate::auth::{flow, token};
use anyhow::{Result, anyhow};

pub async fn run_login() -> Result<()> {
    let device_code = match flow::start_login_flow().await {
        Ok(code) => code,
        Err(err) => {
            eprintln!("Login flow failed: {}", err);
            return Err(anyhow!("Login flow failed: {}", err));
        }
    };
    
    let access_token = match flow::poll_for_token(&device_code).await {
        Ok(token) => token,
        Err(err) => {
            eprintln!("Failed to get access token: {}", err);
            return Err(anyhow!("Failed to get access token: {}", err));
        }
    };

    token::store_token(&access_token);
    println!("🔐 Access token saved successfully.");
    Ok(())
}

