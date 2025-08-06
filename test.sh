#!/bin/bash

# Test script for ai-commit

set -e

echo "🧪 Testing ai-commit..."

# Build the project
echo "📦 Building project..."
cargo build --release

# Test basic help
echo "📝 Testing help command..."
./target/release/ai-commit --help

# Test config commands
echo "⚙️ Testing config commands..."
./target/release/ai-commit config show

# Test models command (this will fail without API key, but we test the command parsing)
echo "🤖 Testing models command..."
if ./target/release/ai-commit models 2>/dev/null; then
    echo "✓ Models command succeeded"
else
    echo "⚠️ Models command failed (expected without API key)"
fi

echo "✅ Basic tests completed!"
echo ""
echo "📖 To use ai-commit:"
echo "1. Set up your AI provider:"
echo "   ./target/release/ai-commit config set-provider openai"
echo "   ./target/release/ai-commit config set-api-key YOUR_API_KEY"
echo ""
echo "2. In a git repository with staged changes:"
echo "   ./target/release/ai-commit"
