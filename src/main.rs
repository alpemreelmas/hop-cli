mod cli;
mod config;
mod models;
mod ssh;
mod utils;

use cli::{Cli, Commands};
use config::{load_config, save_config, get_config_file_path, init_config};
use models::Server;
use ssh::SshClient;
use utils::{
    print_error, print_success, print_info, print_warning, 
    is_valid_ip, is_valid_server_name, confirm_action
};

use anyhow::Result;
use colored::*;
use std::fs;
use std::process;

fn main() {
    if let Err(e) = run() {
        print_error(&format!("{}", e));
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::new();
    
    match cli.command {
        Commands::Add { name, user, ip } => {
            handle_add(name, user, ip)?;
        }
        Commands::List { verbose } => {
            handle_list(verbose)?;
        }
        Commands::Connect { identifier, test } => {
            handle_connect(identifier, test)?;
        }
        Commands::Remove { identifier, force } => {
            handle_remove(identifier, force)?;
        }
        Commands::Edit { identifier, name, user, ip } => {
            handle_edit(identifier, name, user, ip)?;
        }
        Commands::Config { path, init } => {
            handle_config(path, init)?;
        }
        Commands::Copy { server, source, destination, from } => {
            handle_copy(server, source, destination, from)?;
        }
        Commands::Exec { server, command } => {
            handle_exec(server, command)?;
        }
        Commands::Import { file, merge } => {
            handle_import(file, merge)?;
        }
        Commands::Export { file, pretty } => {
            handle_export(file, pretty)?;
        }
    }
    
    Ok(())
}

fn handle_add(name: String, user: String, ip: String) -> Result<()> {
    // Validate inputs
    if !is_valid_server_name(&name) {
        return Err(anyhow::anyhow!("Invalid server name. Use only alphanumeric characters, hyphens, and underscores."));
    }
    
    if !is_valid_ip(&ip) {
        return Err(anyhow::anyhow!("Invalid IP address format."));
    }
    
    let mut config = load_config()?;
    let server = Server::new(name, user, ip);
    
    config.add_server(server.clone())?;
    save_config(&config)?;
    
    print_success(&format!("Added server: {}", server));
    Ok(())
}

fn handle_list(verbose: bool) -> Result<()> {
    let config = load_config()?;
    
    if config.is_empty() {
        print_info("No servers configured. Use 'hop add' to add a server.");
        return Ok(());
    }
    
    println!("{}", "Configured servers:".bold());
    println!();
    
    for server in config.list_servers() {
        if verbose {
            println!("  {}", server.name.green().bold());
            println!("    User: {}", server.user);
            println!("    IP: {}", server.ip);
            println!("    SSH Command: {}", server.ssh_command().yellow());
            println!();
        } else {
            println!("  {}", server);
        }
    }
    
    Ok(())
}

fn handle_connect(identifier: String, test: bool) -> Result<()> {
    let config = load_config()?;
    
    let server = config.find_server(&identifier)
        .ok_or_else(|| anyhow::anyhow!("Server '{}' not found", identifier))?;
    
    let ssh_client = SshClient::new();
    
    if test {
        ssh_client.test_connection(server)?;
    } else {
        ssh_client.connect(server)?;
    }
    
    Ok(())
}

fn handle_remove(identifier: String, force: bool) -> Result<()> {
    let mut config = load_config()?;
    
    let server = config.find_server(&identifier)
        .ok_or_else(|| anyhow::anyhow!("Server '{}' not found", identifier))?;
    
    if !force && !confirm_action(&format!("Remove server '{}'?", server)) {
        print_info("Operation cancelled.");
        return Ok(());
    }
    
    let removed_server = config.remove_server(&identifier)?;
    save_config(&config)?;
    
    print_success(&format!("Removed server: {}", removed_server));
    Ok(())
}

fn handle_edit(identifier: String, name: Option<String>, user: Option<String>, ip: Option<String>) -> Result<()> {
    let mut config = load_config()?;
    
    let mut changed = false;
    let updated_server = {
        let server = config.find_server_mut(&identifier)
            .ok_or_else(|| anyhow::anyhow!("Server '{}' not found", identifier))?;
        
        if let Some(new_name) = name {
            if !is_valid_server_name(&new_name) {
                return Err(anyhow::anyhow!("Invalid server name. Use only alphanumeric characters, hyphens, and underscores."));
            }
            server.name = new_name;
            changed = true;
        }
        
        if let Some(new_user) = user {
            server.user = new_user;
            changed = true;
        }
        
        if let Some(new_ip) = ip {
            if !is_valid_ip(&new_ip) {
                return Err(anyhow::anyhow!("Invalid IP address format."));
            }
            server.ip = new_ip;
            changed = true;
        }
        
        server.clone()
    };
    
    if !changed {
        print_warning("No changes specified. Use --name, --user, or --ip to edit the server.");
        return Ok(());
    }
    
    save_config(&config)?;
    print_success(&format!("Updated server: {}", updated_server));
    Ok(())
}

fn handle_config(path: bool, init: bool) -> Result<()> {
    if init {
        init_config()?;
        print_success("Configuration initialized successfully.");
    }
    
    if path {
        let config_path = get_config_file_path()?;
        println!("Configuration file: {}", config_path.display());
    }
    
    if !path && !init {
        let config_path = get_config_file_path()?;
        println!("Configuration file: {}", config_path.display());
        
        if config_path.exists() {
            let config = load_config()?;
            println!("Servers configured: {}", config.list_servers().len());
        } else {
            println!("Configuration file does not exist. Use --init to create it.");
        }
    }
    
    Ok(())
}

fn handle_copy(server_id: String, source: String, destination: String, from: bool) -> Result<()> {
    let config = load_config()?;
    
    let server = config.find_server(&server_id)
        .ok_or_else(|| anyhow::anyhow!("Server '{}' not found", server_id))?;
    
    let ssh_client = SshClient::new();
    
    if from {
        ssh_client.copy_file_from(server, &source, &destination)?;
    } else {
        ssh_client.copy_file(server, &source, &destination)?;
    }
    
    Ok(())
}

fn handle_exec(server_id: String, command: String) -> Result<()> {
    let config = load_config()?;
    
    let server = config.find_server(&server_id)
        .ok_or_else(|| anyhow::anyhow!("Server '{}' not found", server_id))?;
    
    let ssh_client = SshClient::new();
    let output = ssh_client.execute_command(server, &command)?;
    
    print!("{}", output);
    Ok(())
}

fn handle_import(file: String, merge: bool) -> Result<()> {
    let content = fs::read_to_string(&file)
        .map_err(|e| anyhow::anyhow!("Failed to read file '{}': {}", file, e))?;
    
    let imported_servers: Vec<Server> = serde_json::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}", e))?;
    
    let mut config = if merge {
        load_config()?
    } else {
        config::Config::new()
    };
    
    let mut added_count = 0;
    let mut skipped_count = 0;
    
    for server in imported_servers {
        match config.add_server(server.clone()) {
            Ok(_) => {
                added_count += 1;
                print_success(&format!("Imported: {}", server));
            }
            Err(_) => {
                skipped_count += 1;
                print_warning(&format!("Skipped (already exists): {}", server));
            }
        }
    }
    
    save_config(&config)?;
    
    print_success(&format!("Import complete. Added: {}, Skipped: {}", added_count, skipped_count));
    Ok(())
}

fn handle_export(file: String, pretty: bool) -> Result<()> {
    let config = load_config()?;
    
    let json = if pretty {
        serde_json::to_string_pretty(&config.list_servers())?
    } else {
        serde_json::to_string(&config.list_servers())?
    };
    
    fs::write(&file, json)
        .map_err(|e| anyhow::anyhow!("Failed to write file '{}': {}", file, e))?;
    
    print_success(&format!("Exported {} servers to '{}'", config.list_servers().len(), file));
    Ok(())
} 