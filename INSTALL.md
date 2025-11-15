# Installation Guide

Complete step-by-step installation guide for Bazzounquester.

## Quick Installation (If You Have Rust)

```bash
# Clone the repository
git clone https://github.com/yourusername/bazzounquester.git
cd bazzounquester

# Run the installation script
./install.sh
```

Follow the on-screen instructions to add the binary to your PATH.

## Complete Installation (From Scratch)

### Step 1: Install Rust

If you don't have Rust installed:

**Linux/macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Windows:**
Download and run the installer from https://rustup.rs/

After installation:
```bash
# Reload your shell environment
source $HOME/.cargo/env

# Verify Rust is installed
rustc --version
cargo --version
```

### Step 2: Clone the Repository

```bash
git clone https://github.com/yourusername/bazzounquester.git
cd bazzounquester
```

### Step 3: Install Bazzounquester

**Using the install script (recommended):**
```bash
./install.sh
```

**Manual installation:**
```bash
cargo install --path .
```

### Step 4: Configure Your PATH

After installation, the binary will be at `~/.cargo/bin/bazzounquester`

**Check if it's in your PATH:**
```bash
which bazzounquester
```

If the command is not found, add Cargo's bin directory to your PATH:

**Bash (Linux/macOS):**
```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

**Zsh (macOS default):**
```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

**Fish:**
```bash
echo 'set -gx PATH $HOME/.cargo/bin $PATH' >> ~/.config/fish/config.fish
source ~/.config/fish/config.fish
```

**Windows PowerShell:**
```powershell
$env:Path += ";$env:USERPROFILE\.cargo\bin"
```

To make it permanent on Windows, add `%USERPROFILE%\.cargo\bin` to your system PATH through System Properties.

### Step 5: Verify Installation

```bash
bazzounquester --version
```

You should see: `bazzounquester 1.0.0`

## Alternative: No Installation Required

If you don't want to install globally, you can run it directly:

```bash
# Build the project
cargo build --release

# Run from the target directory
./target/release/bazzounquester
```

## Alternative: Copy to System Path

**Linux/macOS:**
```bash
# Build
cargo build --release

# Copy to /usr/local/bin
sudo cp target/release/bazzounquester /usr/local/bin/

# Now you can run it from anywhere
bazzounquester
```

**Windows:**
```powershell
# Build
cargo build --release

# Copy to a directory in your PATH
copy target\release\bazzounquester.exe C:\Windows\System32\
```

## Troubleshooting

### "cargo: command not found"

**Issue:** Rust/Cargo is not installed or not in PATH.

**Solution:**
1. Install Rust: https://rustup.rs/
2. Restart your terminal
3. Run: `source $HOME/.cargo/env`

### "bazzounquester: command not found"

**Issue:** The binary is installed but not in your PATH.

**Solution:**
1. Add `~/.cargo/bin` to your PATH (see Step 4)
2. Or run with full path: `~/.cargo/bin/bazzounquester`

### Build Errors

**Issue:** Compilation fails.

**Solutions:**
```bash
# Update Rust to the latest version
rustup update

# Clean build artifacts and retry
cargo clean
cargo build --release

# Check Rust version (should be 1.70+)
rustc --version
```

### Permission Denied

**Issue:** Cannot execute the install script.

**Solution:**
```bash
chmod +x install.sh
./install.sh
```

### Slow Build Time

**Issue:** First build takes a long time.

**Explanation:** This is normal. Rust compiles all dependencies from source. Subsequent builds will be much faster.

## Uninstallation

To remove Bazzounquester:

```bash
cargo uninstall bazzounquester
```

## Getting Help

If you encounter any issues:

1. Check this guide
2. See the main README.md
3. Open an issue on GitHub: https://github.com/yourusername/bazzounquester/issues
4. Email: hassan.bazzoundev@gmail.com

## Next Steps

Once installed, check out:
- `bazzounquester --help` - View help
- `bazzounquester` - Start interactive mode
- See README.md for usage examples
