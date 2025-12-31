use clap::{Parser, Subcommand};
use reqwest::Client;
use std::env;
use tokio::fs::create_dir_all;
use zpm::commands::*;
use zpm::utils::{get_cache_dir, get_versions_dir};

#[derive(Parser, Debug)]
#[command(name = "zpm")]
#[command(about = "Zig Package Manager - A version manager for Zig")]
#[command(long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Install a Zig version")]
    #[command(alias = "i")]
    Install {
        #[arg(help = "Version to install (latest, master, stable, or specific version)")]
        version: Option<String>,
        #[arg(long, short, help = "Set as default version")]
        default: bool,
    },
    #[command(about = "Uninstall a Zig version")]
    #[command(alias = "rm")]
    Uninstall {
        #[arg(help = "Version to uninstall")]
        version: String,
    },
    #[command(about = "Set a version as default")]
    Use {
        #[arg(help = "Version to use")]
        version: String,
    },
    #[command(about = "List installed versions")]
    #[command(alias = "ls")]
    List {
        #[arg(long, short, help = "List available versions")]
        remote: bool,
    },
    #[command(about = "Install ZLS for the current Zig version")]
    InstallZls {},
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let client = Client::new();
    let home_dir = env::var("HOME").expect("HOME environment variable not set");
    let versions_dir = get_versions_dir(&home_dir);
    let cache_dir = get_cache_dir(&home_dir);
    
    // Create necessary directories if they don't exist
    create_dir_all(&versions_dir).await?;
    create_dir_all(&cache_dir).await?;

    match &cli.command {
        Commands::Install { version, default } => {
            let version = version.as_deref().unwrap_or("latest");
            install(&client, &home_dir, version, *default).await?;
        }
        Commands::Uninstall { version } => {
            uninstall(&home_dir, version).await?;
        }
        Commands::Use { version } => {
            set_default(&home_dir, version).await?;
        }
        Commands::List { remote } => {
            if *remote {
                // For compatibility, keep the remote option working
                list_versions(&client, &home_dir).await?;
            } else {
                list_versions(&client, &home_dir).await?;
            }
        }
        Commands::InstallZls {} => {
            install_zls(&client, &home_dir).await?;
        }
    }

    Ok(())
}
