# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of ai-commit
- Support for multiple AI providers (OpenAI, Anthropic, Gemini, GitHub Copilot, Ollama, Groq, DeepSeek, xAI, Cohere)
- Command-line interface with clap
- Configuration management with TOML
- Git repository operations
- Interactive commit message editing
- Conventional commit format support
- Provider-specific model listing
- Environment variable substitution in config
- Comprehensive error handling

### Features
- **Multi-provider AI support**: Choose from 9 different AI providers
- **Configuration system**: TOML-based configuration with environment variable support
- **Interactive mode**: Edit commit messages before committing
- **Git integration**: Detect changes, show diffs, stage files
- **Conventional commits**: Generate properly formatted commit messages
- **Model selection**: List and choose from provider-specific models
- **Local AI support**: Use Ollama for local model inference
- **Cross-platform**: Works on Linux, macOS, and Windows

### Commands
- `ai-commit`: Generate and commit with AI-generated message
- `ai-commit config`: Manage configuration settings
- `ai-commit models`: List available models for current provider

### Configuration
- Provider selection and API key management
- Model and generation parameter customization
- Git behavior configuration
- UI preferences
- Custom prompts

## [0.1.0] - 2024-12-XX

### Added
- Initial implementation
- Basic functionality for AI-powered commit message generation
- Multi-provider support
- Configuration system
- CLI interface
