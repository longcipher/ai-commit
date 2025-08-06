# GitHub Copilot Integration

## Overview

ai-commit now supports GitHub Copilot as a dedicated AI provider, offering seamless integration with GitHub's AI capabilities. This integration uses the official GitHub Copilot API and provides access to multiple models including GPT-4o, Claude, and Gemini.

## Features

- **Native GitHub Copilot Support**: Direct integration with GitHub Copilot API
- **Multiple Models**: Access to GPT-4o, GPT-4o-mini, Claude 3.5 Sonnet, Claude 3 Haiku, and Gemini 2.0 Flash
- **Authentication**: Simple authentication via GitHub CLI
- **Token Management**: Automatic token retrieval and refresh

## Setup

### 1. Prerequisites

- Active GitHub Copilot subscription
- GitHub CLI (`gh`) installed and accessible

### 2. Authentication

First, authenticate with GitHub Copilot:

```bash
ai-commit auth
```

This command will:
- Check if GitHub CLI is installed
- Verify your authentication status
- Launch the GitHub authentication flow if needed
- Set up the necessary scopes for Copilot access

### 3. Configuration

Set GitHub as your AI provider:

```bash
ai-commit config set-provider github
```

Choose a model (optional - defaults to gpt-4o-mini):

```bash
ai-commit config set-model gpt-4o
```

Available models:
- `gpt-4o` - OpenAI's latest and most capable model
- `gpt-4o-mini` - Faster, more cost-effective version
- `claude-3-5-sonnet` - Anthropic's most capable model
- `claude-3-haiku` - Fast and cost-effective from Anthropic
- `gemini-2.0-flash-001` - Google's latest multimodal model

## Usage

### Basic Usage

```bash
# Generate and commit with GitHub Copilot
ai-commit

# Auto-accept the generated message
ai-commit --yes

# Stage all files and commit
ai-commit --all --yes

# Use a specific model
ai-commit --model claude-3-5-sonnet

# Provide additional context
ai-commit --context "Fix performance issue in user authentication"
```

### List Available Models

```bash
ai-commit models
```

### Check Configuration

```bash
ai-commit config show
```

## Authentication Details

The GitHub Copilot integration uses the `copilot-client` Rust crate, which:

1. Retrieves your GitHub token from:
   - `GITHUB_TOKEN` environment variable
   - GitHub CLI configuration files
   - GitHub Codespaces environment

2. Exchanges the GitHub token for a Copilot-specific token

3. Uses the Copilot token to make API requests to `api.githubcopilot.com`

## Troubleshooting

### Authentication Issues

If you encounter authentication errors:

1. Ensure you have an active GitHub Copilot subscription
2. Re-run the authentication command: `ai-commit auth`
3. Check your GitHub CLI status: `gh auth status`
4. Verify Copilot access in your GitHub settings

### Token Errors

If you see token-related errors:

1. Your Copilot subscription may have expired
2. Re-authenticate: `ai-commit auth`
3. Check environment variables if using `GITHUB_TOKEN`

### Model Availability

Some models may not be available in all regions or for all users. If you encounter model errors:

1. Try a different model: `ai-commit --model gpt-4o-mini`
2. Check available models: `ai-commit models`
3. Contact GitHub support if issues persist

## Implementation Details

The GitHub Copilot integration is implemented using:

- **copilot-client**: A Rust client for the GitHub Copilot API
- **Direct API Access**: Uses the official Copilot endpoints
- **Token Management**: Automatic token retrieval and refresh
- **Error Handling**: Comprehensive error handling for authentication and API issues

## Benefits

- **Official Support**: Uses the official GitHub Copilot API
- **Better Models**: Access to latest models as they become available
- **Enterprise Ready**: Works with GitHub Enterprise Cloud
- **Reliable**: Built on GitHub's infrastructure
- **Cost Effective**: Uses your existing Copilot subscription

## Migration from Other Providers

If you're migrating from another AI provider:

1. Authenticate with GitHub: `ai-commit auth`
2. Switch provider: `ai-commit config set-provider github`
3. Choose a model: `ai-commit config set-model gpt-4o-mini`
4. Test: `ai-commit --yes`

Your configuration for other settings (temperature, max tokens, etc.) will be preserved.
