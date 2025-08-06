# Contributing to AI Commit

Thank you for your interest in contributing to AI Commit! This document provides guidelines for contributing to the project.

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Git
- A code editor (VS Code with rust-analyzer recommended)

### Setting up the development environment

1. Clone the repository:
```bash
git clone https://github.com/longcipher/ai-commit.git
cd ai-commit
```

2. Build the project:
```bash
cargo build
```

3. Run tests:
```bash
cargo test
```

4. Run the development version:
```bash
cargo run -- --help
```

## Development Workflow

### Code Structure

```
src/
├── main.rs          # Entry point
├── cli.rs           # Command-line interface
├── config.rs        # Configuration management
├── git.rs           # Git operations
├── ai.rs            # AI provider integration
├── error.rs         # Error handling
└── lib.rs           # Library exports

prompts/
└── system.md        # Default system prompt

config.example.toml  # Example configuration
```

### Making Changes

1. Create a new branch for your feature:
```bash
git checkout -b feature/your-feature-name
```

2. Make your changes following the coding standards below.

3. Add tests for new functionality:
```bash
cargo test
```

4. Ensure your code compiles without warnings:
```bash
cargo clippy
```

5. Format your code:
```bash
cargo fmt
```

6. Commit your changes with a conventional commit message:
```bash
git commit -m "feat(component): add new feature"
```

## Coding Standards

### Rust Style

- Follow the official Rust style guide
- Use `cargo fmt` to format code
- Use `cargo clippy` to catch common mistakes
- Prefer explicit error handling over panics
- Use meaningful variable and function names

### Documentation

- Add doc comments for public functions and structs
- Include examples in doc comments where helpful
- Update README.md for user-facing changes

### Error Handling

- Use the `AppError` enum for all error types
- Provide meaningful error messages
- Use `anyhow::Context` to add context to errors

### Testing

- Write unit tests for new functions
- Add integration tests for major features
- Test error conditions and edge cases
- Use descriptive test names

## Adding a New AI Provider

To add support for a new AI provider:

1. Add the provider to the `Provider` enum in `src/config.rs`
2. Update the `models()` method in `src/ai.rs` to return models for your provider
3. Update the configuration example and documentation
4. Add any provider-specific configuration options

Example:
```rust
// In src/config.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    // ... existing providers
    YourProvider,
}

// In src/ai.rs
impl AiClient {
    pub fn models(&self) -> Vec<String> {
        match self.config.ai.provider {
            // ... existing providers
            Provider::YourProvider => vec![
                "your-model-1".to_string(),
                "your-model-2".to_string(),
            ],
        }
    }
}
```

## Submitting Changes

### Pull Request Process

1. Fork the repository
2. Create a feature branch from `main`
3. Make your changes following the guidelines above
4. Push to your fork and submit a pull request

### Pull Request Guidelines

- Use a clear and descriptive title
- Include a detailed description of changes
- Reference any related issues
- Include tests for new functionality
- Ensure CI passes

### Commit Message Format

Use conventional commit format:

```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Examples:
- `feat(ai): add support for new provider`
- `fix(git): handle repositories without initial commit`
- `docs(readme): update installation instructions`

## Issues and Bug Reports

### Reporting Bugs

When reporting bugs, please include:

- Operating system and version
- Rust version (`rustc --version`)
- AI Commit version
- Steps to reproduce the issue
- Expected vs. actual behavior
- Relevant configuration

### Requesting Features

When requesting features:

- Describe the use case
- Explain why the feature would be valuable
- Consider implementation complexity
- Suggest possible solutions

## Code Review

All submissions require code review. Here's what reviewers look for:

- Correctness and functionality
- Code quality and style
- Test coverage
- Documentation
- Performance implications
- Security considerations

## Release Process

Releases follow semantic versioning (semver):

- `MAJOR`: Breaking changes
- `MINOR`: New features (backward compatible)
- `PATCH`: Bug fixes (backward compatible)

## Getting Help

- Open an issue for bugs or feature requests
- Start a discussion for questions or ideas
- Check existing issues and discussions first

## License

By contributing to AI Commit, you agree that your contributions will be licensed under the MIT License.

## Recognition

Contributors will be recognized in the project's credits. Significant contributions may be highlighted in release notes.

Thank you for contributing to AI Commit!
