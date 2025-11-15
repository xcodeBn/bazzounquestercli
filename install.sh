#!/bin/bash

# Bazzounquester Installation Script
# Author: Hassan Bazzoun <hassan.bazzoundev@gmail.com>

set -e

echo ""
echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║              Bazzounquester - Installation Script            ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust is not installed."
    echo ""
    echo "Please install Rust from: https://rustup.rs/"
    echo ""
    echo "Run this command to install Rust:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo ""
    exit 1
fi

echo "✓ Rust is installed"
echo ""

# Build and install
echo "Building and installing bazzounquester..."
echo ""
cargo install --path .

echo ""
echo "✓ Installation complete!"
echo ""

# Check if ~/.cargo/bin is in PATH
CARGO_BIN="$HOME/.cargo/bin"

if [[ ":$PATH:" == *":$CARGO_BIN:"* ]]; then
    echo "✓ $CARGO_BIN is already in your PATH"
    echo ""
    echo "You can now run: bazzounquester"
    echo ""
else
    echo "⚠ $CARGO_BIN is NOT in your PATH"
    echo ""
    echo "To use bazzounquester from anywhere, add this line to your shell config:"
    echo ""

    # Detect shell
    if [ -n "$BASH_VERSION" ]; then
        SHELL_CONFIG="~/.bashrc"
    elif [ -n "$ZSH_VERSION" ]; then
        SHELL_CONFIG="~/.zshrc"
    else
        SHELL_CONFIG="~/.profile"
    fi

    echo "  export PATH=\"\$HOME/.cargo/bin:\$PATH\""
    echo ""
    echo "Add it to your $SHELL_CONFIG file:"
    echo "  echo 'export PATH=\"\$HOME/.cargo/bin:\$PATH\"' >> $SHELL_CONFIG"
    echo "  source $SHELL_CONFIG"
    echo ""
    echo "Or run it directly from:"
    echo "  $CARGO_BIN/bazzounquester"
    echo ""
fi

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║                     Installation Complete!                   ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""
echo "Try it out:"
echo "  bazzounquester                    # Start interactive mode"
echo "  bazzounquester --help             # Show help"
echo "  bazzounquester --version          # Show version"
echo ""
echo "For more information, see README.md"
echo ""
