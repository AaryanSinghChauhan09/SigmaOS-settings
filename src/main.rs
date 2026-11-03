//! SigmaOS-settings - SigmaOS Component CLI
//!
//! Command-line interface for SigmaOS-settings component.

use anyhow::Result;
use clap::{Parser, Subcommand};
use log::info;
use settings::{Component, ComponentStatus, Config, settings};

#[derive(Parser)]
#[command(name = "settings")]
#[command(about = "SigmaOS SigmaOS-settings component", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the component
    Init {
        /// Enable debug mode
        #[arg(short, long)]
        debug: bool,
    },
    /// Start the component
    Start,
    /// Stop the component
    Stop,
    /// Get component status
    Status,
    /// Run the component
    Run {
        /// Enable debug mode
        #[arg(short, long)]
        debug: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let config = Config::default();
    let mut component = settings::new(config);

    match cli.command {
        Commands::Init { debug } => {
            if debug {
                println!("Initializing SigmaOS-settings in debug mode...");
            } else {
                println!("Initializing SigmaOS-settings...");
            }
            component.initialize()?;
            println!("SigmaOS-settings initialized successfully");
        }
        Commands::Start => {
            component.initialize()?;
            component.start()?;
            println!("SigmaOS-settings started successfully");
        }
        Commands::Stop => {
            component.stop()?;
            println!("SigmaOS-settings stopped successfully");
        }
        Commands::Status => {
            let status = component.status();
            println!("SigmaOS-settings status: {:?}", status);
        }
        Commands::Run { debug } => {
            if debug {
                println!("Running SigmaOS-settings in debug mode...");
            } else {
                println!("Running SigmaOS-settings...");
            }
            component.initialize()?;
            component.start()?;
            println!("SigmaOS-settings is running. Press Ctrl+C to stop.");
            
            // Keep running
            tokio::runtime::Runtime::new()?.block_on(async {
                tokio::signal::ctrl_c().await.unwrap();
                info!("Received Ctrl+C, shutting down...");
            });
            
            component.stop()?;
            println!("SigmaOS-settings stopped");
        }
    }

    Ok(())
}
