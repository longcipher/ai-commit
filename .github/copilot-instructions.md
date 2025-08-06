# AI Commit - Copilot Instructions

## Architecture Overview

**ai-commit** is a Rust CLI tool that generates conventional commit messages using multiple AI providers. The architecture follows a clean modular design:

- **`src/main.rs`**: Tokio async entry point with clap CLI parsing
- **`src/cli.rs`**: Command handlers organized in modules (`commit`, `config`, `models`)
- **`src/ai.rs`**: AI client using `genai` crate for multi-provider LLM integration
- **`src/config.rs`**: TOML-based configuration with environment variable substitution
- **`src/git.rs`**: Git operations via `git2` crate (status, diff, staging, commits)
- **`src/error.rs`**: Centralized error handling with `thiserror`

## Key Technologies & Patterns

### Core Dependencies
- **clap**: CLI with derive features (`#[command]`, `#[arg]`)
- **genai**: Multi-provider AI client (OpenAI, Anthropic, Gemini, etc.)
- **git2**: Git operations (wrap Repository in `GitRepo` struct)
- **tokio**: Async runtime only for AI calls
- **serde + toml**: Configuration serialization
- **dialoguer**: Interactive prompts (`Confirm`, `Editor`)

### Configuration System
Config lives at `~/.config/ai-commit/config.toml` with automatic creation:
```rust
// Always use AppConfig::load().unwrap_or_default()
let config = AppConfig::load()?;
config.save()?; // Auto-creates directories
```

Environment variables expand with `${VAR_NAME}` syntax in TOML.

## Development Workflows

### Build & Test
```bash
cargo build --release          # Release build
cargo clippy                   # Linting
cargo fmt                      # Formatting
./test.sh                      # Custom test script
```

### Local Testing
```bash
# Test without API keys (uses test-repo/)
./target/release/ai-commit config set-provider ollama
git init test-repo && cd test-repo
git add . && ../target/release/ai-commit --yes
```

## Critical Code Patterns

### Error Handling
Always use the `AppError` enum with `anyhow::Result`:
```rust
// In functions returning Result<()>
config.save()?;
// In CLI handlers
return Err(AppError::InvalidTemperature.into());
```

### AI Provider Integration
New providers require updates in two places:
1. **`src/ai.rs`** - Add to `list_models()` match statement
2. **`src/config.rs`** - Provider string validation

### CLI Module Pattern
Each command lives in a module with specific import pattern:
```rust
pub mod commit {
    use crate::{ai::AiClient, config::AppConfig, git::GitRepo};
    
    pub async fn handle_commit_command(...) -> Result<()> {
        // Only commit command is async (AI calls)
    }
}
```

### Git Operations
Always wrap `git2::Repository` in `GitRepo` struct:
```rust
let repo = GitRepo::new(".")?;
let status = repo.get_status()?;  // Returns GitStatus with .staged, .modified, .untracked
```

## AI Provider Notes

- **GitHub Copilot**: Uses GitHub CLI auth (no API key needed)
- **Ollama**: Local models (no API key needed) 
- **Others**: Require environment variables (`OPENAI_API_KEY`, etc.)

The `genai` crate handles provider abstraction - don't implement custom HTTP clients.

## Interactive Flow

1. Check git status → prompt for staging if needed
2. Generate diff → send to AI with system prompt from `prompts/system.md`
3. Show generated message → allow editing via `dialoguer::Editor`
4. Commit with `git2` (not shell commands)

## Debugging

Set `RUST_LOG=ai_commit=debug` for detailed logging. The tool gracefully handles missing API keys and git repo detection.
