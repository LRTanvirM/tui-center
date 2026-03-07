#!/bin/bash

echo "🛠️  Checking and installing prerequisites for TUI Control Center..."

# 1. Define the universal core packages every distro has
CORE_PKGS="curl less btop fastfetch"

# 2. Detect the package manager and install core packages
if command -v pacman &> /dev/null; then
    echo "🟢 Arch/CachyOS detected. Installing core packages..."
    sudo pacman -S --needed $CORE_PKGS

    # Arch-specific AUR/Custom packages from your dashboard
    echo "🔵 Checking for an AUR helper (yay or paru)..."
    if command -v yay &> /dev/null; then
        yay -S --needed pacseek brave-bin wiki-tui termusic
    elif command -v paru &> /dev/null; then
        paru -S --needed pacseek brave-bin wiki-tui termusic
    else
        echo "⚠️  No AUR helper found. You may need to manually install: pacseek, wiki-tui, termusic."
    fi

elif command -v apt &> /dev/null; then
    echo "🟠 Debian/Ubuntu detected. Installing core packages..."
    sudo apt update
    sudo apt install -y $CORE_PKGS
    echo "⚠️  Note: Apps like 'pacseek' are not available on Debian."

elif command -v dnf &> /dev/null; then
    echo "🔴 Fedora/RHEL detected. Installing core packages..."
    sudo dnf install -y $CORE_PKGS
    echo "⚠️  Note: Apps like 'pacseek' are not available on Fedora."

else
    echo "❌ Unsupported package manager. Please manually install: $CORE_PKGS"
fi

# 3. Check for Rust/Cargo specific TUI apps
echo "🦀 Checking for Cargo to install Rust-based TUI tools..."
if command -v cargo &> /dev/null; then
    # tclock is a popular rust crate
    cargo install tclock
else
    echo "⚠️  Cargo not found. Skipping Rust-based apps like tclock."
fi

echo "✅ Prerequisite check complete!"
echo "Note: Some niche apps (like spf, soundscope, endcord, gophertube) may need to be installed manually from their specific GitHub repositories or via Go/Cargo."
