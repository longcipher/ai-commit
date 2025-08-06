mod ai;
mod cli;
mod config;
mod error;
mod git;
mod prompts;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use tracing::debug;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("ai_commit=info".parse()?),
        )
        .init();

    let cli = Cli::parse();

    debug!("Starting ai-commit with args: {:?}", cli);

    match cli.command {
        Some(Commands::Config { action }) => {
            cli::config::handle_config_command(action)?;
        }
        Some(Commands::Models) => {
            cli::models::handle_models_command().await?;
        }
        None => {
            // Default: commit command
            cli::commit::handle_commit_command(cli.all, cli.yes, cli.model, cli.context).await?;
        }
    }

    Ok(())
}
