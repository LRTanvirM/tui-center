#!/bin/bash

echo "🚀 Installing tui-center..."

# ── Step 1: Detect package manager and install bare minimum prerequisites ──
# Only installs what's absolutely needed to compile and run the TUI:
#   • curl       — used by the Weather widget
#   • fastfetch  — used by the System Info pane

if command -v pacman &> /dev/null; then
    echo "🟢 Arch Linux detected."
    sudo pacman -S --needed curl fastfetch

elif command -v apt &> /dev/null; then
    echo "🟠 Debian/Ubuntu detected."
    sudo apt update
    sudo apt install -y curl fastfetch

elif command -v dnf &> /dev/null; then
    echo "🔴 Fedora/RHEL detected."
    sudo dnf install -y curl fastfetch

else
    echo "❌ Unsupported package manager. Please manually install: curl, fastfetch"
    exit 1
fi

# ── Step 2: Ensure Rust/Cargo is available ─────────────────────────────────
if ! command -v cargo &> /dev/null; then
    echo "🦀 Rust is not installed. Installing via rustup (recommended)..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"

    if ! command -v cargo &> /dev/null; then
        echo "❌ Rust installation failed. Please install manually: https://rustup.rs"
        exit 1
    fi
fi

echo "🦀 Rust $(rustc --version | awk '{print $2}') found."

# ── Step 3: Build the release binary ───────────────────────────────────────
echo "📦 Compiling tui-center (release mode)..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed. Please check the compiler errors above."
    exit 1
fi

# ── Step 4: Install system-wide ────────────────────────────────────────────
echo "🔑 Installing system-wide. You may be prompted for your password..."
sudo cp target/release/tui-center /usr/local/bin/
sudo chmod +x /usr/local/bin/tui-center

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  ✅ tui-center has been installed successfully!"
echo "  Launch it by typing: tui-center"
echo ""
echo "  💡 Run the onboarding setup inside the TUI to install"
echo "     optional apps (btop, termusic, pacseek, etc.)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
