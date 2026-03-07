#!/bin/bash

echo "🚀 Starting installation for tui-center..."

# 1. Check if Rust is installed on the target machine
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Rust and Cargo are not installed on this system."
    echo "Please install Rust first using:"
    echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# 2. Build the highly-optimized release version
echo "📦 Compiling the release binary..."
cargo build --release

# Stop if the build fails
if [ $? -ne 0 ]; then
    echo "❌ Build failed. Please check the compiler errors above."
    exit 1
fi

# 3. Copy it to the system binaries directory
echo "🔑 Installing system-wide. You may be prompted for your password..."
sudo cp target/release/tui-center /usr/local/bin/

# 4. Set permissions so anyone can execute it
sudo chmod +x /usr/local/bin/tui-center

echo "✅ Success! tui-center has been installed."
echo "You can now launch it by typing 'tui-center' in your terminal."
