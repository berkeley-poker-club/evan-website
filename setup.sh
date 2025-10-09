#!/bin/bash
set -e

echo "Poker at Berkeley web dev setup"
echo "=================================="
echo ""

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

command_exists() {
    command -v "$1" >/dev/null 2>&1
}

print_status() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

OS="$(uname -s)"
case "${OS}" in
    Linux*)     MACHINE=Linux;;
    Darwin*)    MACHINE=Mac;;
    *)          MACHINE="UNKNOWN:${OS}"
esac


if [ "$MACHINE" = "Mac" ] || [ "$MACHINE" = "Linux" ]; then
    if ! command_exists brew; then
        print_warning "Homebrew not found. Installing Homebrew"
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

        if [ "$MACHINE" = "Mac" ]; then
            if [ -f "/opt/homebrew/bin/brew" ]; then
                eval "$(/opt/homebrew/bin/brew shellenv)"
            elif [ -f "/usr/local/bin/brew" ]; then
                eval "$(/usr/local/bin/brew shellenv)"
            fi
        fi
    else
        print_status "Homebrew found"
    fi
fi

if ! command_exists rustc; then
    print_warning "Rust not found. Installing Rust"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    print_status "Rust found ($(rustc --version))"
fi

if ! command_exists cargo; then
    source "$HOME/.cargo/env"
fi

RUST_VERSION=$(rustc --version | awk '{print $2}')
print_status "Rust version: $RUST_VERSION"

print_status "Adding wasm32-unknown-unknown target"
rustup target add wasm32-unknown-unknown

if ! command_exists trunk; then
    print_warning "Trunk not found. Installing Trunk"
    cargo install trunk
else
    print_status "Trunk found ($(trunk --version))"
fi

if ! command_exists node; then
    print_warning "Node.js not found. Installing Node.js"
    if [ "$MACHINE" = "Mac" ]; then
        brew install node
    elif [ "$MACHINE" = "Linux" ]; then
        curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
        sudo apt-get install -y nodejs
    fi
else
    print_status "Node.js found ($(node --version))"
fi

if ! command_exists npm; then
    print_error "npm not found. Please install Node.js manually."
    exit 1
else
    print_status "npm found ($(npm --version))"
fi

if [ -f "package.json" ]; then
    print_status "Installing npm dependencies"
    npm install
else
    print_warning "No package.json found, skipping npm install"
fi


if ! cargo tauri --version >/dev/null 2>&1; then
    read -p "Would you like to install Tauri CLI for desktop app development? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        print_status "Installing Tauri CLI"
        cargo install tauri-cli

        if [ "$MACHINE" = "Mac" ]; then
            print_status "Tauri system dependencies already satisfied on macOS"
        elif [ "$MACHINE" = "Linux" ]; then
            print_warning "Installing Tauri system dependencies for Linux"
            sudo apt-get update
            sudo apt-get install -y \
                libwebkit2gtk-4.0-dev \
                build-essential \
                curl \
                wget \
                file \
                libssl-dev \
                libgtk-3-dev \
                libayatana-appindicator3-dev \
                librsvg2-dev
        fi
    fi
else
    print_status "Tauri CLI found"
fi

echo ""
echo "=================================="
echo "setup complete"
echo ""
echo "Available commands:"
echo "  npm run dev          - Start development server"
echo "  npm run build        - Build for production"
echo "  npm run deploy       - Deploy to OCF"
echo "  npm run deploy:all   - Deploy to both GitHub Pages and OCF"

if cargo tauri --version >/dev/null 2>&1; then
    echo ""
    echo "Tauri commands:"
    echo "  npm run tauri:dev    - Run desktop app in development"
    echo "  npm run tauri:build  - Build desktop app for distribution"
fi
