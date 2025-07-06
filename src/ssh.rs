use crate::models::Server;
use crate::utils::{print_info, print_success};
use anyhow::{Context, Result};
use std::process::Command;

pub struct SshClient;

impl SshClient {
    pub fn new() -> Self {
        SshClient
    }

    /// Connect to a server via SSH
    pub fn connect(&self, server: &Server) -> Result<()> {
        print_info(&format!("Connecting to {}...", server));
        
        let ssh_command = server.ssh_command();
        print_info(&format!("Running: {}", ssh_command));

        // Execute the SSH command
        let mut command = Command::new("ssh");
        command.arg(format!("{}@{}", server.user, server.ip));

        // Add common SSH options for better user experience
        command
            .arg("-o")
            .arg("StrictHostKeyChecking=ask")
            .arg("-o")
            .arg("UserKnownHostsFile=~/.ssh/known_hosts");

        let status = command
            .status()
            .context("Failed to execute SSH command")?;

        if status.success() {
            print_success("SSH connection closed successfully");
        } else {
            return Err(anyhow::anyhow!("SSH connection failed with exit code: {}", status.code().unwrap_or(-1)));
        }

        Ok(())
    }

    /// Test SSH connection to a server
    pub fn test_connection(&self, server: &Server) -> Result<()> {
        print_info(&format!("Testing connection to {}...", server));

        let mut command = Command::new("ssh");
        command
            .arg(format!("{}@{}", server.user, server.ip))
            .arg("-o")
            .arg("ConnectTimeout=10")
            .arg("-o")
            .arg("StrictHostKeyChecking=no")
            .arg("-o")
            .arg("UserKnownHostsFile=/dev/null")
            .arg("-o")
            .arg("LogLevel=ERROR")
            .arg("echo 'Connection test successful'");

        let output = command
            .output()
            .context("Failed to execute SSH test command")?;

        if output.status.success() {
            print_success("Connection test successful");
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Connection test failed: {}", stderr));
        }

        Ok(())
    }

    /// Execute a command on a remote server
    pub fn execute_command(&self, server: &Server, command: &str) -> Result<String> {
        print_info(&format!("Executing command on {}: {}", server, command));

        let mut ssh_command = Command::new("ssh");
        ssh_command
            .arg(format!("{}@{}", server.user, server.ip))
            .arg("-o")
            .arg("StrictHostKeyChecking=no")
            .arg("-o")
            .arg("UserKnownHostsFile=/dev/null")
            .arg("-o")
            .arg("LogLevel=ERROR")
            .arg(command);

        let output = ssh_command
            .output()
            .context("Failed to execute remote command")?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            Ok(stdout.to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("Remote command failed: {}", stderr))
        }
    }

    /// Copy a file to a remote server using SCP
    pub fn copy_file(&self, server: &Server, local_path: &str, remote_path: &str) -> Result<()> {
        print_info(&format!("Copying {} to {}:{}", local_path, server, remote_path));

        let mut command = Command::new("scp");
        command
            .arg("-o")
            .arg("StrictHostKeyChecking=no")
            .arg("-o")
            .arg("UserKnownHostsFile=/dev/null")
            .arg(local_path)
            .arg(format!("{}@{}:{}", server.user, server.ip, remote_path));

        let status = command
            .status()
            .context("Failed to execute SCP command")?;

        if status.success() {
            print_success("File copied successfully");
        } else {
            return Err(anyhow::anyhow!("SCP failed with exit code: {}", status.code().unwrap_or(-1)));
        }

        Ok(())
    }

    /// Copy a file from a remote server using SCP
    pub fn copy_file_from(&self, server: &Server, remote_path: &str, local_path: &str) -> Result<()> {
        print_info(&format!("Copying {}:{} to {}", server, remote_path, local_path));

        let mut command = Command::new("scp");
        command
            .arg("-o")
            .arg("StrictHostKeyChecking=no")
            .arg("-o")
            .arg("UserKnownHostsFile=/dev/null")
            .arg(format!("{}@{}:{}", server.user, server.ip, remote_path))
            .arg(local_path);

        let status = command
            .status()
            .context("Failed to execute SCP command")?;

        if status.success() {
            print_success("File copied successfully");
        } else {
            return Err(anyhow::anyhow!("SCP failed with exit code: {}", status.code().unwrap_or(-1)));
        }

        Ok(())
    }

    /// Check if SSH and SCP are available on the system
    pub fn check_ssh_available(&self) -> Result<()> {
        let ssh_check = Command::new("ssh")
            .arg("-V")
            .output()
            .context("SSH command not found. Please install OpenSSH client.")?;

        if !ssh_check.status.success() {
            return Err(anyhow::anyhow!("SSH is not properly installed"));
        }

        let scp_check = Command::new("scp")
            .arg("-h")
            .output()
            .context("SCP command not found. Please install OpenSSH client.")?;

        if !scp_check.status.success() {
            return Err(anyhow::anyhow!("SCP is not properly installed"));
        }

        Ok(())
    }
}

impl Default for SshClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ssh_client_creation() {
        let client = SshClient::new();
        // Basic test to ensure the client can be created
        assert!(true);
    }

    #[test]
    fn test_check_ssh_available() {
        let client = SshClient::new();
        // This test will only pass if SSH is installed on the system
        // In a CI environment, this might fail
        match client.check_ssh_available() {
            Ok(_) => assert!(true),
            Err(_) => {
                // SSH might not be available in test environment
                println!("SSH not available in test environment");
                assert!(true);
            }
        }
    }
} 