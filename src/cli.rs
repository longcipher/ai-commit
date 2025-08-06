use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "ai-commit")]
#[command(about = "AI-powered Git commit message generator")]
#[command(version)]
pub struct Cli {
    /// Stage all files before committing
    #[arg(short, long)]
    pub all: bool,

    /// Automatically accept the generated commit message
    #[arg(short, long)]
    pub yes: bool,

    /// Specify the AI model to use
    #[arg(short, long)]
    pub model: Option<String>,

    /// Additional context to guide the AI
    #[arg(short, long)]
    pub context: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// List available models for the current provider
    Models,
}

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Show current configuration
    Show,
    /// Set the AI provider
    SetProvider { provider: String },
    /// Set the API key
    SetApiKey { api_key: String },
    /// Set the default model
    SetModel { model: String },
    /// Set temperature (0.0-2.0)
    SetTemperature { temperature: f32 },
    /// Set maximum tokens
    SetMaxTokens { max_tokens: u32 },
    /// Enable/disable interactive mode
    SetInteractive { interactive: bool },
    /// Enable/disable conventional commits
    SetConventional { conventional: bool },
}

pub mod commit {
    use anyhow::Result;
    use console::style;
    use dialoguer::{Confirm, Editor};
    use indicatif::{ProgressBar, ProgressStyle};

    use crate::{ai::AiClient, config::AppConfig, error::AppError, git::GitRepo};

    #[allow(clippy::too_many_lines)]
    pub async fn handle_commit_command(
        all: bool,
        yes: bool,
        model: Option<String>,
        context: Option<String>,
    ) -> Result<()> {
        let config = AppConfig::load()?;
        let mut repo = GitRepo::new(".")?;

        // Check if we're in a git repository
        if !repo.is_git_repo() {
            return Err(AppError::NotInGitRepo.into());
        }

        // Stage files if requested
        if all {
            repo.stage_all()?;
            println!("{}", style("✓ Staged all files").green());
        }

        // Check for staged changes
        let status = repo.get_status()?;
        if status.staged.is_empty() {
            if status.modified.is_empty() && status.untracked.is_empty() {
                println!("{}", style("No changes to commit").yellow());
                return Ok(());
            }

            // Prompt to stage files
            if !status.modified.is_empty() {
                let should_stage = Confirm::new()
                    .with_prompt("Stage modified files?")
                    .default(true)
                    .interact()?;

                if should_stage {
                    repo.stage_modified()?;
                    println!("{}", style("✓ Staged modified files").green());
                }
            }

            if !status.untracked.is_empty() {
                let should_stage = Confirm::new()
                    .with_prompt("Stage untracked files?")
                    .default(false)
                    .interact()?;

                if should_stage {
                    repo.stage_untracked()?;
                    println!("{}", style("✓ Staged untracked files").green());
                }
            }

            // Refresh status
            let status = repo.get_status()?;
            if status.staged.is_empty() {
                println!("{}", style("No staged changes to commit").yellow());
                return Ok(());
            }
        }

        // Show diff if configured
        if config.ui.show_diff {
            println!("\n{}", style("Staged changes:").bold());
            let diff = repo.get_staged_diff()?;
            println!("{diff}");
        }

        // Generate commit message
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .expect("Failed to create progress bar template"),
        );
        pb.set_message("Generating commit message...");
        pb.enable_steady_tick(std::time::Duration::from_millis(100));

        let ai_client = AiClient::new(&config);
        let diff = repo.get_staged_diff()?;
        let status_output = repo.get_status_porcelain()?;

        let commit_message = ai_client
            .generate_commit_message(&diff, &status_output, context.as_deref(), model.as_deref())
            .await?;

        pb.finish_and_clear();

        println!("\n{}", style("Generated commit message:").bold());
        println!("{}", style(&commit_message).cyan());

        // Handle commit confirmation
        if yes {
            repo.commit(&commit_message)?;
            println!("\n{}", style("✓ Committed successfully").green());
        } else if config.ui.interactive {
            let choice = dialoguer::Select::new()
                .with_prompt("What would you like to do?")
                .items(&["Commit", "Edit message", "Cancel"])
                .default(0)
                .interact()?;

            match choice {
                0 => {
                    repo.commit(&commit_message)?;
                    println!("\n{}", style("✓ Committed successfully").green());
                }
                1 => {
                    if let Some(edited_message) =
                        Editor::new().extension(".txt").edit(&commit_message)?
                    {
                        repo.commit(&edited_message)?;
                        println!(
                            "\n{}",
                            style("✓ Committed successfully with edited message").green()
                        );
                    } else {
                        println!("{}", style("Commit cancelled").yellow());
                    }
                }
                2 => {
                    println!("{}", style("Commit cancelled").yellow());
                }
                _ => unreachable!(),
            }
        } else {
            let should_commit = Confirm::new()
                .with_prompt("Commit with this message?")
                .default(true)
                .interact()?;

            if should_commit {
                repo.commit(&commit_message)?;
                println!("\n{}", style("✓ Committed successfully").green());
            } else {
                println!("{}", style("Commit cancelled").yellow());
            }
        }

        Ok(())
    }
}

pub mod config {
    use anyhow::Result;
    use console::style;

    use super::ConfigAction;
    use crate::{config::AppConfig, error::AppError};

    pub fn handle_config_command(action: ConfigAction) -> Result<()> {
        let mut config = AppConfig::load().unwrap_or_default();

        match action {
            ConfigAction::Show => {
                println!("{}", style("Current configuration:").bold());
                println!("Provider: {}", style(&config.ai.provider).cyan());
                println!("Model: {}", style(&config.ai.model).cyan());
                println!("Temperature: {}", style(config.ai.temperature).cyan());
                println!("Max tokens: {}", style(config.ai.max_tokens).cyan());
                println!("Interactive: {}", style(config.ui.interactive).cyan());
                println!(
                    "Conventional commits: {}",
                    style(config.git.conventional_commits).cyan()
                );
                println!("Auto stage: {}", style(config.git.auto_stage).cyan());
                println!(
                    "Config file: {}",
                    style(AppConfig::get_config_path().display()).dim()
                );
            }
            ConfigAction::SetProvider { provider } => {
                config.ai.provider.clone_from(&provider);
                config.save()?;
                println!(
                    "{} {}",
                    style("✓ Set provider to:").green(),
                    style(provider).cyan()
                );
            }
            ConfigAction::SetApiKey { api_key } => {
                config.ai.api_key = Some(api_key);
                config.save()?;
                println!("{}", style("✓ API key updated").green());
            }
            ConfigAction::SetModel { model } => {
                config.ai.model.clone_from(&model);
                config.save()?;
                println!(
                    "{} {}",
                    style("✓ Set model to:").green(),
                    style(model).cyan()
                );
            }
            ConfigAction::SetTemperature { temperature } => {
                if !(0.0..=2.0).contains(&temperature) {
                    return Err(AppError::InvalidTemperature.into());
                }
                config.ai.temperature = temperature;
                config.save()?;
                println!(
                    "{} {}",
                    style("✓ Set temperature to:").green(),
                    style(temperature).cyan()
                );
            }
            ConfigAction::SetMaxTokens { max_tokens } => {
                config.ai.max_tokens = max_tokens;
                config.save()?;
                println!(
                    "{} {}",
                    style("✓ Set max tokens to:").green(),
                    style(max_tokens).cyan()
                );
            }
            ConfigAction::SetInteractive { interactive } => {
                config.ui.interactive = interactive;
                config.save()?;
                println!(
                    "{} {}",
                    style("✓ Set interactive mode to:").green(),
                    style(interactive).cyan()
                );
            }
            ConfigAction::SetConventional { conventional } => {
                config.git.conventional_commits = conventional;
                config.save()?;
                println!(
                    "{} {}",
                    style("✓ Set conventional commits to:").green(),
                    style(conventional).cyan()
                );
            }
        }

        Ok(())
    }
}

pub mod models {
    use anyhow::Result;
    use console::style;

    use crate::{ai::AiClient, config::AppConfig};

    pub async fn handle_models_command() -> Result<()> {
        let config = AppConfig::load()?;
        let ai_client = AiClient::new(&config);

        println!("{}", style("Available models:").bold());

        let models = ai_client.list_models()?;

        for model in models {
            if model == config.ai.model {
                println!("  {} {}", style("●").green(), style(&model).cyan().bold());
            } else {
                println!("  {} {}", style("○").dim(), style(&model).cyan());
            }
        }

        Ok(())
    }
}
