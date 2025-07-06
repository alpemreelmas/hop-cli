use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub name: String,
    pub user: String,
    pub ip: String,
}

impl Server {
    pub fn new(name: String, user: String, ip: String) -> Self {
        Server {
            name,
            user,
            ip,
        }
    }

    /// Returns the identifier for this server (name)
    pub fn identifier(&self) -> &str {
        &self.name
    }

    /// Check if this server matches the given identifier (name)
    pub fn matches(&self, identifier: &str) -> bool {
        self.name == identifier
    }

    /// Generate the SSH command for this server
    pub fn ssh_command(&self) -> String {
        format!("ssh {}@{}", self.user, self.ip)
    }
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} -> {}@{}",
            self.name, self.user, self.ip
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_creation() {
        let server = Server::new("test-server".to_string(), "root".to_string(), "192.168.1.10".to_string());
        assert_eq!(server.name, "test-server");
        assert_eq!(server.user, "root");
        assert_eq!(server.ip, "192.168.1.10");
    }

    #[test]
    fn test_server_matches() {
        let server = Server::new("test-server".to_string(), "root".to_string(), "192.168.1.10".to_string());
        assert!(server.matches("test-server"));
        assert!(!server.matches("other"));
    }

    #[test]
    fn test_ssh_command() {
        let server = Server::new("test-server".to_string(), "root".to_string(), "192.168.1.10".to_string());
        assert_eq!(server.ssh_command(), "ssh root@192.168.1.10");
    }
} 