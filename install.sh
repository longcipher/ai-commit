#!/usr/bin/env bash

# AI Commit Installation Script
# This script installs ai-commit to your system

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default installation directory
INSTALL_DIR="/usr/local/bin"
CONFIG_DIR="$HOME/.config/ai-commit"

# Function to print colored output
print_color() {
    printf "${1}%s${NC}\n" "$2"
}

print_info() {
    print_color "$BLUE" "ℹ️  $1"
}

print_success() {
    print_color "$GREEN" "✅ $1"
}

print_warning() {
    print_color "$YELLOW" "⚠️  $1"
}

print_error() {
    print_color "$RED" "❌ $1"
}

# Check if running as root for system installation
check_permissions() {
    if [[ "$INSTALL_DIR" == "/usr/local/bin" ]] && [[ $EUID -ne 0 ]]; then
        print_warning "Installing to $INSTALL_DIR requires sudo privileges"
        return 1
    fi
    return 0
}

# Check if Rust and Cargo are installed
check_rust() {
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo is not installed. Please install Rust first:"
        print_info "Visit https://rustup.rs/ or run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
    print_success "Rust/Cargo found"
}

# Check if Git is installed
check_git() {
    if ! command -v git &> /dev/null; then
        print_error "Git is not installed. Please install Git first."
        exit 1
    fi
    print_success "Git found"
}

# Clone or update repository
setup_repo() {
    local repo_dir="/tmp/ai-commit"
    
    print_info "Setting up repository..."
    
    if [[ -d "$repo_dir" ]]; then
        print_info "Updating existing repository..."
        cd "$repo_dir"
        git pull origin main || git pull origin master
    else
        print_info "Cloning repository..."
        git clone https://github.com/longcipher/ai-commit.git "$repo_dir"
        cd "$repo_dir"
    fi
    
    print_success "Repository ready"
}

# Build the project
build_project() {
    print_info "Building ai-commit..."
    cargo build --release
    
    if [[ ! -f "target/release/ai-commit" ]]; then
        print_error "Build failed - binary not found"
        exit 1
    fi
    
    print_success "Build completed"
}

# Install binary
install_binary() {
    local binary_path="target/release/ai-commit"
    
    print_info "Installing binary to $INSTALL_DIR..."
    
    if [[ "$INSTALL_DIR" == "/usr/local/bin" ]]; then
        sudo cp "$binary_path" "$INSTALL_DIR/"
        sudo chmod +x "$INSTALL_DIR/ai-commit"
    else
        cp "$binary_path" "$INSTALL_DIR/"
        chmod +x "$INSTALL_DIR/ai-commit"
    fi
    
    print_success "Binary installed to $INSTALL_DIR/ai-commit"
}

# Setup configuration directory
setup_config() {
    print_info "Setting up configuration directory..."
    
    if [[ ! -d "$CONFIG_DIR" ]]; then
        mkdir -p "$CONFIG_DIR"
        print_success "Created config directory: $CONFIG_DIR"
    else
        print_info "Config directory already exists: $CONFIG_DIR"
    fi
    
    # Copy example config if it doesn't exist
    if [[ ! -f "$CONFIG_DIR/config.toml" ]] && [[ -f "config.example.toml" ]]; then
        cp "config.example.toml" "$CONFIG_DIR/config.toml"
        print_success "Copied example configuration to $CONFIG_DIR/config.toml"
        print_warning "Please edit $CONFIG_DIR/config.toml to add your API keys"
    fi
}

# Verify installation
verify_installation() {
    print_info "Verifying installation..."
    
    if command -v ai-commit &> /dev/null; then
        local version
        version=$(ai-commit --version 2>/dev/null || echo "unknown")
        print_success "ai-commit installed successfully! Version: $version"
        return 0
    else
        print_error "Installation verification failed"
        print_info "Make sure $INSTALL_DIR is in your PATH"
        return 1
    fi
}

# Show next steps
show_next_steps() {
    print_info "🎉 Installation complete!"
    echo
    print_info "Next steps:"
    echo "1. Configure your AI provider:"
    echo "   ai-commit config set-provider openai"
    echo "   ai-commit config set-api-key YOUR_API_KEY"
    echo
    echo "2. Or edit the config file directly:"
    echo "   \$EDITOR $CONFIG_DIR/config.toml"
    echo
    echo "3. Test it out:"
    echo "   cd your-git-repo"
    echo "   git add ."
    echo "   ai-commit"
    echo
    print_info "For more information, visit: https://github.com/longcipher/ai-commit"
}

# Main installation function
main() {
    print_info "🚀 Installing ai-commit..."
    echo
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --install-dir)
                INSTALL_DIR="$2"
                shift 2
                ;;
            --help|-h)
                echo "Usage: $0 [--install-dir DIR]"
                echo "  --install-dir DIR    Install binary to DIR (default: /usr/local/bin)"
                exit 0
                ;;
            *)
                print_error "Unknown option: $1"
                exit 1
                ;;
        esac
    done
    
    # Check prerequisites
    check_rust
    check_git
    
    # Check permissions for installation directory
    if ! check_permissions; then
        print_info "You can also install to a user directory:"
        print_info "$0 --install-dir \$HOME/.local/bin"
        exit 1
    fi
    
    # Create installation directory if it doesn't exist
    if [[ ! -d "$INSTALL_DIR" ]]; then
        if [[ "$INSTALL_DIR" == "/usr/local/bin" ]]; then
            sudo mkdir -p "$INSTALL_DIR"
        else
            mkdir -p "$INSTALL_DIR"
        fi
    fi
    
    # Save current directory
    local original_dir
    original_dir=$(pwd)
    
    # Setup, build, and install
    setup_repo
    build_project
    install_binary
    setup_config
    
    # Return to original directory
    cd "$original_dir"
    
    # Verify and show next steps
    if verify_installation; then
        show_next_steps
    else
        exit 1
    fi
}

# Run main function
main "$@"
