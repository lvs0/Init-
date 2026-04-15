#!/bin/bash

# INIT One-Line Installer

set -e

echo "⬡ Installing INIT..."

# Check Rust
if ! command -v cargo &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Build
cargo build --release

# Install
sudo cp target/release/init /usr/local/bin/init
chmod +x /usr/local/bin/init

echo "✓ INIT installed!"
echo "Run 'init init' to start."
