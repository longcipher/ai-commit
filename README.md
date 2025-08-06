# AI Commit

AI-powered Git commit message generator with support for multiple LLM providers.

## Features

- 🤖 Support for multiple AI providers (OpenAI, Anthropic, Gemini, GitHub Copilot, Ollama, etc.)
- 📝 Generates conventional commit messages
- ⚙️ Configurable via `~/.config/ai-commit/config.toml`
- 🔄 Interactive mode with commit message editing
- 📊 Git diff analysis for accurate message generation
- 🎯 Staging support with interactive prompts

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/longcipher/ai-commit.git
cd ai-commit

# Build and install
cargo build --release

# Optional: Install to system PATH
sudo cp target/release/ai-commit /usr/local/bin/
```

### Using Cargo

```bash
# Install from crates.io (when published)
cargo install ai-commit

# Or install from git
cargo install --git https://github.com/longcipher/ai-commit.git
```

## Quick Start

### Option 1: GitHub Copilot (Recommended)

1. Authenticate with GitHub Copilot:

```bash
ai-commit auth
```

1. Set GitHub as your provider:

```bash
ai-commit config set-provider github
ai-commit config set-model gpt-4o-mini
```

1. Stage some changes and generate a commit:

```bash
git add .
ai-commit
```

### Option 2: Other AI Providers

1. Configure your AI provider:

```bash
ai-commit config set-provider openai
ai-commit config set-api-key YOUR_API_KEY
```

1. Stage some changes and generate a commit:

```bash
git add .
ai-commit
```

## Configuration

Configuration file is located at `~/.config/ai-commit/config.toml`:

```toml
[ai]
provider = "openai"
model = "gpt-4o-mini"
api_key = "your-api-key"
temperature = 0.1
max_tokens = 150

[git]
auto_stage = false
conventional_commits = true

[ui]
interactive = true
show_diff = true
```

## Supported Providers

| Provider | Models | API Key Environment Variable |
|----------|--------|------------------------------|
| **OpenAI** | gpt-4o, gpt-4o-mini, gpt-4-turbo, gpt-3.5-turbo | `OPENAI_API_KEY` |
| **Anthropic** | claude-3-5-sonnet, claude-3-haiku, claude-3-opus | `ANTHROPIC_API_KEY` |
| **Google Gemini** | gemini-2.0-flash, gemini-1.5-pro, gemini-1.5-flash | `GEMINI_API_KEY` |
| **Groq** | llama-3.1-8b-instant, llama-3.1-70b-versatile | `GROQ_API_KEY` |
| **DeepSeek** | deepseek-chat, deepseek-coder | `DEEPSEEK_API_KEY` |
| **xAI** | grok-beta | `XAI_API_KEY` |
| **Cohere** | command-r-plus, command-r, command-light | `COHERE_API_KEY` |
| **Ollama** | llama3.2:3b, llama3.1:8b, codellama:7b, gemma:2b | Local (no API key needed) |
| **GitHub Copilot** | gpt-4o, claude-3-5-sonnet | Uses GitHub CLI authentication |

## Usage

```bash
# Basic usage
ai-commit

# Stage all files and commit
ai-commit --all

# Use specific model
ai-commit --model gpt-4o

# Skip interactive mode
ai-commit --yes

# Add context to guide the AI
ai-commit --context "refactoring authentication system"

# Show configuration
ai-commit config show

# Set configuration values
ai-commit config set-provider anthropic
ai-commit config set-model claude-3-haiku-20240307
ai-commit config set-api-key sk-...
```

## Commands

### `ai-commit` (default)

Generate and commit with AI-generated message

### `ai-commit config`

Manage configuration settings

### `ai-commit models`

List available models for the current provider

## Examples

```bash
# Configure for OpenAI
ai-commit config set-provider openai
ai-commit config set-api-key sk-...
ai-commit config set-model gpt-4o-mini

# Configure for GitHub Copilot
ai-commit config set-provider github
# GitHub Copilot uses GitHub CLI authentication

# Configure for local Ollama
ai-commit config set-provider ollama
ai-commit config set-model llama3.2:3b

# Generate commit with context
ai-commit --context "implementing user authentication with JWT tokens"

# Quick commit without interaction
git add .
ai-commit --yes
```

## Configuration File

The configuration file supports environment variable substitution:

```toml
[ai]
provider = "openai"
model = "gpt-4o-mini"
api_key = "${OPENAI_API_KEY}"  # Uses environment variable
temperature = 0.1
max_tokens = 150

[git]
auto_stage = false
conventional_commits = true
diff_context = 3

[ui]
interactive = true
show_diff = true
editor = "${EDITOR}"  # Uses system editor

[prompts]
system_prompt = """
You are a Git commit message generator. Generate a concise, conventional commit message based on the provided git diff.

Rules:
- Use conventional commit format: type(scope): description
- Types: feat, fix, docs, style, refactor, perf, test, chore
- Keep under 72 characters
- Be specific and descriptive
- Focus on what changed, not why
"""
```

## License

Apache-2.0 License
