#!/bin/bash

echo "Uninstalling tui-center..."

# ── Step 1: Remove the binary ──────────────────────────────────────────────
if [ -f /usr/local/bin/tui-center ]; then
    echo "Removing system binary. You may be prompted for your password..."
    sudo rm /usr/local/bin/tui-center

    if [ $? -eq 0 ]; then
        echo "Binary removed."
    else
        echo "Error: Failed to remove binary. Please check permissions."
        exit 1
    fi
else
    echo "tui-center is not installed in /usr/local/bin/."
fi

# ── Step 2: Remove core prerequisites ──────────────────────────────────────
read -p "Remove core prerequisites (curl, fastfetch)? [y/N]: " remove_core
if [[ "$remove_core" =~ ^[Yy]$ ]]; then
    if command -v pacman &> /dev/null; then
        sudo pacman -Rns --noconfirm curl fastfetch 2>/dev/null
    elif command -v apt &> /dev/null; then
        sudo apt remove -y curl fastfetch 2>/dev/null
    elif command -v dnf &> /dev/null; then
        sudo dnf remove -y curl fastfetch 2>/dev/null
    fi
    echo "Core prerequisites removed."
fi

# ── Step 3: Clean up build artifacts ───────────────────────────────────────
if [ -d "target" ]; then
    read -p "Remove build artifacts (target/ directory)? [y/N]: " remove_target
    if [[ "$remove_target" =~ ^[Yy]$ ]]; then
        rm -rf target/
        echo "Build artifacts removed."
    fi
fi

echo ""
echo "Uninstallation complete!"
echo "tui-center has been uninstalled."
echo "You may also delete this repository folder if desired."
