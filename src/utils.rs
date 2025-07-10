use anyhow::{Context, Result};
use colored::*;
use std::fs;
use std::path::Path;
use std::process::Command;

/// Print an error message in red
pub fn print_error(message: &str) {
    eprintln!("{}: {}", "Error".red().bold(), message);
}

/// Print a success message in green
pub fn print_success(message: &str) {
    println!("{}: {}", "Success".green().bold(), message);
}

/// Print an info message in blue
pub fn print_info(message: &str) {
    println!("{}: {}", "Info".blue().bold(), message);
}

/// Print a warning message in yellow
pub fn print_warning(message: &str) {
    println!("{}: {}", "Warning".yellow().bold(), message);
}

/// Ensure a directory exists, creating it if necessary
pub fn ensure_dir_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)
            .with_context(|| format!("Failed to create directory: {}", path.display()))?;
    }
    Ok(())
}

/// Validate IP address format (basic validation)
pub fn is_valid_ip(ip: &str) -> bool {
    // Basic IP validation - could be improved with regex or proper parsing
    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() != 4 {
        return false;
    }
    
    for part in parts {
        if let Ok(num) = part.parse::<u8>() {
            if num > 255 {
                return false;
            }
        } else {
            return false;
        }
    }
    
    true
}

/// Validate server name (alphanumeric, hyphens, underscores)
pub fn is_valid_server_name(name: &str) -> bool {
    !name.is_empty() && 
    name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

pub fn open_browser(url: &str) {
    #[cfg(target_os = "windows")]
    Command::new("cmd").args(&["/C", "start", url]).spawn().unwrap();

    #[cfg(target_os = "macos")]
    Command::new("open").arg(url).spawn().unwrap();

    #[cfg(target_os = "linux")]
    Command::new("xdg-open").arg(url).spawn().unwrap();
}


/// Prompt user for confirmation
pub fn confirm_action(message: &str) -> bool {
    print!("{} [y/N]: ", message);
    use std::io::{self, Write};
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_ip() {
        assert!(is_valid_ip("192.168.1.1"));
        assert!(is_valid_ip("10.0.0.1"));
        assert!(is_valid_ip("255.255.255.255"));
        assert!(!is_valid_ip("256.1.1.1"));
        assert!(!is_valid_ip("192.168.1"));
        assert!(!is_valid_ip("192.168.1.1.1"));
        assert!(!is_valid_ip("not.an.ip.address"));
    }

    #[test]
    fn test_is_valid_server_name() {
        assert!(is_valid_server_name("server1"));
        assert!(is_valid_server_name("test-server"));
        assert!(is_valid_server_name("test_server"));
        assert!(is_valid_server_name("Server123"));
        assert!(!is_valid_server_name(""));
        assert!(!is_valid_server_name("server with spaces"));
        assert!(!is_valid_server_name("server@special"));
    }
} 