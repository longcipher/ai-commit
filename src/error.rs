use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not in a Git repository")]
    NotInGitRepo,

    #[error("Configuration directory not found")]
    ConfigDirNotFound,

    #[error("Unsupported provider: {0}")]
    UnsupportedProvider(String),

    #[error("Invalid temperature value. Must be between 0.0 and 2.0")]
    InvalidTemperature,

    #[error("No response received from AI")]
    NoResponseFromAi,

    #[error("GitHub CLI (gh) not found. Please install GitHub CLI first")]
    GitHubCliNotFound,

    #[error("Authentication failed")]
    AuthenticationFailed,

    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parsing error: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("TOML serialization error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("GenAI error: {0}")]
    GenAi(#[from] genai::Error),
}
