use anyhow::Result;
use genai::{
    Client,
    chat::{ChatMessage, ChatOptions, ChatRequest},
};
use tracing::{debug, info};

use crate::{config::AppConfig, error::AppError};

pub struct AiClient {
    client: Client,
    config: AppConfig,
}

impl AiClient {
    pub fn new(config: &AppConfig) -> Self {
        let client = Client::default();

        Self {
            client,
            config: config.clone(),
        }
    }

    pub async fn generate_commit_message(
        &self,
        diff: &str,
        status: &str,
        context: Option<&str>,
        model_override: Option<&str>,
    ) -> Result<String> {
        let model = model_override.unwrap_or(&self.config.ai.model);

        debug!("Generating commit message with model: {}", model);

        let mut messages = vec![ChatMessage::system(&self.config.prompts.system_prompt)];

        // Add context if provided
        if let Some(ctx) = context {
            messages.push(ChatMessage::user(format!("Context: {ctx}\n\n")));
        }

        // Add git status
        messages.push(ChatMessage::user(format!(
            "`git status`:\n```\n{}\n```\n\n",
            status.trim()
        )));

        // Add git diff
        if !diff.trim().is_empty() {
            messages.push(ChatMessage::user(format!(
                "`git diff --staged`:\n```diff\n{}\n```\n\n",
                diff.trim()
            )));
        }

        messages.push(ChatMessage::user(
            "Generate a conventional commit message based on the changes above:",
        ));

        let chat_request = ChatRequest::new(messages);

        let chat_options = ChatOptions {
            temperature: Some(f64::from(self.config.ai.temperature)),
            max_tokens: Some(self.config.ai.max_tokens),
            ..Default::default()
        };

        debug!(
            "Sending request to AI provider: {}",
            self.config.ai.provider
        );

        let response = self
            .client
            .exec_chat(model, chat_request, Some(&chat_options))
            .await?;

        let commit_message = response
            .first_text()
            .ok_or(AppError::NoResponseFromAi)?
            .trim()
            .to_string();

        info!("Generated commit message: {}", commit_message);

        Ok(commit_message)
    }

    pub fn list_models(&self) -> Result<Vec<String>> {
        // This is a simplified implementation
        // In a real implementation, you'd query the provider for available models
        let models = match self.config.ai.provider.as_str() {
            "openai" => vec![
                "gpt-4o".to_string(),
                "gpt-4o-mini".to_string(),
                "gpt-4-turbo".to_string(),
                "gpt-3.5-turbo".to_string(),
            ],
            "anthropic" => vec![
                "claude-3-5-sonnet-20241022".to_string(),
                "claude-3-haiku-20240307".to_string(),
                "claude-3-opus-20240229".to_string(),
            ],
            "gemini" => vec![
                "gemini-2.0-flash".to_string(),
                "gemini-1.5-pro".to_string(),
                "gemini-1.5-flash".to_string(),
            ],
            "groq" => vec![
                "llama-3.1-8b-instant".to_string(),
                "llama-3.1-70b-versatile".to_string(),
                "mixtral-8x7b-32768".to_string(),
            ],
            "deepseek" => vec!["deepseek-chat".to_string(), "deepseek-coder".to_string()],
            "xai" => vec!["grok-beta".to_string()],
            "cohere" => vec![
                "command-r-plus".to_string(),
                "command-r".to_string(),
                "command-light".to_string(),
            ],
            "ollama" => vec![
                "llama3.2:3b".to_string(),
                "llama3.1:8b".to_string(),
                "codellama:7b".to_string(),
                "gemma:2b".to_string(),
                "mistral:7b".to_string(),
            ],
            "github" => vec![
                "gpt-4o".to_string(),
                "gpt-4o-mini".to_string(),
                "claude-3-5-sonnet".to_string(),
            ],
            _ => return Err(AppError::UnsupportedProvider(self.config.ai.provider.clone()).into()),
        };

        Ok(models)
    }
}
