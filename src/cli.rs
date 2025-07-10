use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "hop")]
#[command(about = "A fast and minimal CLI tool to manage and connect to frequently used SSH servers")]
#[command(version = "0.1.0")]
#[command(author = "Alp Emre Elmas <elmasalpemre@gmail.com>")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new server to the configuration
    Add {
        /// Name of the server
        #[arg(short, long)]
        name: String,

        /// User for SSH connection
        #[arg(short, long)]
        user: String,

        /// IP address or hostname of the server
        #[arg(short, long)]
        ip: String,
    },

    /// List all configured servers
    List {
        /// Show detailed information
        #[arg(short, long)]
        verbose: bool,
    },

    /// Connect to a server via SSH
    Connect {
        /// Server name to connect to
        identifier: String,

        /// Test connection without actually connecting
        #[arg(short, long)]
        test: bool,
    },

    /// Remove a server from the configuration
    Remove {
        /// Server name to remove
        identifier: String,

        /// Force removal without confirmation
        #[arg(short, long)]
        force: bool,
    },

    /// Edit a server configuration
    Edit {
        /// Server name to edit
        identifier: String,

        /// New name for the server
        #[arg(long)]
        name: Option<String>,

        /// New user for SSH connection
        #[arg(long)]
        user: Option<String>,

        /// New IP address or hostname
        #[arg(long)]
        ip: Option<String>,
    },

    /// Show configuration file information
    Config {
        /// Show the path to the configuration file
        #[arg(short, long)]
        path: bool,

        /// Initialize configuration file
        #[arg(short, long)]
        init: bool,
    },

    Login

}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        // Test basic command parsing
        let cli = Cli::try_parse_from(&["hop", "list"]);
        assert!(cli.is_ok());
        
        match cli.unwrap().command {
            Commands::List { verbose } => assert!(!verbose),
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_add_command_parsing() {
        let cli = Cli::try_parse_from(&[
            "hop", "add", 
            "--name", "test-server", 
            "--user", "ubuntu", 
            "--ip", "192.168.1.1"
        ]);
        
        assert!(cli.is_ok());
        
        match cli.unwrap().command {
            Commands::Add { name, user, ip } => {
                assert_eq!(name, "test-server");
                assert_eq!(user, "ubuntu");
                assert_eq!(ip, "192.168.1.1");
            },
            _ => panic!("Expected Add command"),
        }
    }

    #[test]
    fn test_connect_command_parsing() {
        let cli = Cli::try_parse_from(&["hop", "connect", "test-server"]);
        assert!(cli.is_ok());
        
        match cli.unwrap().command {
            Commands::Connect { identifier, test } => {
                assert_eq!(identifier, "test-server");
                assert!(!test);
            },
            _ => panic!("Expected Connect command"),
        }
    }
} 