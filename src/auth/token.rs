use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use dirs::home_dir;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthToken {
    pub access_token: String,
}

fn get_config_path() -> PathBuf {
    home_dir()
        .expect("Could not find home directory")
        .join(".hop")
        .join("config.json")
}

/// Save token to ~/.hop/config.json
pub fn store_token(token: &str) {
    let config_path = get_config_path();
    let config_dir = config_path.parent().unwrap();

    if !config_dir.exists() {
        fs::create_dir_all(config_dir).expect("Failed to create config directory");
    }

    let auth = AuthToken {
        access_token: token.to_string(),
    };

    let json = serde_json::to_string_pretty(&auth).expect("Failed to serialize token");

    let mut file = fs::File::create(config_path).expect("Failed to create config file");
    file.write_all(json.as_bytes()).expect("Failed to write token");
}

/// Load token from ~/.hop/config.json
pub fn load_token() -> io::Result<AuthToken> {
    let config_path = get_config_path();
    let contents = fs::read_to_string(config_path)?;
    let token: AuthToken = serde_json::from_str(&contents)?;
    Ok(token)
}
