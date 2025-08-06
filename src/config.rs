use std::path::PathBuf;

use anyhow::Result;
use dirs::config_dir;
use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub ai: AiConfig,
    pub git: GitConfig,
    pub ui: UiConfig,
    pub prompts: PromptsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub provider: String,
    pub model: String,
    pub api_key: Option<String>,
    pub temperature: f32,
    pub max_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitConfig {
    pub auto_stage: bool,
    pub conventional_commits: bool,
    pub diff_context: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub interactive: bool,
    pub show_diff: bool,
    pub editor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptsConfig {
    pub system_prompt: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            ai: AiConfig {
                provider: "openai".to_string(),
                model: "gpt-4o-mini".to_string(),
                api_key: None,
                temperature: 0.1,
                max_tokens: 150,
            },
            git: GitConfig {
                auto_stage: false,
                conventional_commits: true,
                diff_context: 3,
            },
            ui: UiConfig {
                interactive: true,
                show_diff: true,
                editor: None,
            },
            prompts: PromptsConfig {
                system_prompt: crate::prompts::get_system_prompt(),
            },
        }
    }
}

impl AppConfig {
    pub fn config_dir() -> Result<PathBuf> {
        Ok(config_dir()
            .ok_or(AppError::ConfigDirNotFound)?
            .join("ai-commit"))
    }

    pub fn config_path() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.toml"))
    }

    pub fn get_config_path() -> PathBuf {
        Self::config_path().unwrap_or_else(|_| PathBuf::from("config.toml"))
    }

    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            // Create default config if it doesn't exist
            let default_config = Self::default();
            default_config.save()?;
            return Ok(default_config);
        }

        let content = std::fs::read_to_string(&config_path)?;
        let mut config: Self = toml::from_str(&content)?;

        // Expand environment variables
        config.expand_env_vars();

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_dir = Self::config_dir()?;
        std::fs::create_dir_all(&config_dir)?;

        let config_path = Self::config_path()?;
        let content = toml::to_string_pretty(self)?;
        std::fs::write(config_path, content)?;

        Ok(())
    }

    fn expand_env_vars(&mut self) {
        if let Some(ref api_key) = self.ai.api_key
            && api_key.starts_with("${")
            && api_key.ends_with('}')
        {
            let env_var = &api_key[2..api_key.len() - 1];
            if let Ok(value) = std::env::var(env_var) {
                self.ai.api_key = Some(value);
            }
        }

        if let Some(ref editor) = self.ui.editor
            && editor.starts_with("${")
            && editor.ends_with('}')
        {
            let env_var = &editor[2..editor.len() - 1];
            if let Ok(value) = std::env::var(env_var) {
                self.ui.editor = Some(value);
            }
        }
    }
}
