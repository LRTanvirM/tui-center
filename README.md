# 🖥️ TUI Control Center

A high-performance, themeable terminal dashboard for Linux power users.

![Dashboard Screenshot](https://raw.githubusercontent.com/LRTanvirM/tui-center/master/assets/screenshot.png)

TUI Control Center is a feature-rich terminal interface built with **Rust** and **Ratatui** that transforms your terminal into a fully interactive dashboard environment. It provides real-time system monitoring, application launching, and system information display while maintaining exceptional performance and minimal resource usage.

## ✨ Features

- 🧭 **Stateful Navigation**: Switch focus between the Status Bar, Main Workspace, and App Bar using the Tab key
- 🎨 **Dynamic Theming**: Select from multiple high-contrast themes (Nord, Dracula, Gruvbox) with instant switching
- 📊 **Live System Monitoring**: Real-time display of system time, RAM usage, and system uptime
- 🚀 **Integrated Application Launcher**: Launch both TUI and GUI applications directly from the interface
- ℹ️ **System Information Pane**: Dedicated toggleable pane powered by fastfetch for detailed system information
- ⚙️ **Customizable Applications**: Add or remove applications through the built-in Settings interface
- ⌨️ **Global Hotkeys**: Quick access to applications using number keys 1-9 from the bottom dock

## 📋 Prerequisites

The following packages are required to build and run TUI Control Center:

| Package          | Purpose                                |
| ---------------- | -------------------------------------- |
| `curl`           | 🌐 Used by the Weather widget          |
| `fastfetch`      | 📱 Powers the System Information pane  |
| `rust` / `cargo` | 🦀 Required to compile the application |

Note: Optional applications (btop, pacseek, termusic, etc.) can be installed through the onboarding setup inside the application.

## 🚀 Installation

TUI Control Center supports three installation methods. Choose the approach that best fits your workflow.

### ⚡ Option 1: Automated Installation (Recommended)

For the quickest setup, use the automated installer which handles all configuration steps:

```bash
curl -sSf https://raw.githubusercontent.com/LRTanvirM/tui-center/master/install.sh | bash
```

The installer automatically:

- Detects your Linux distribution
- Installs required dependencies
- Installs the Rust compiler (if necessary)
- Compiles the application
- Installs the binary system-wide

### 📦 Option 2: Installation Script Method

Clone the repository and execute the provided installation script:

```bash
git clone https://github.com/LRTanvirM/tui-center.git
cd tui-center
chmod +x install.sh
./install.sh
```

This method combines the benefits of automation while working from a local repository, allowing you to review the installation script before execution.

### 🔧 Option 3: Manual Installation

For complete control over the installation process, follow these steps:

**1. Clone the Repository**

```bash
git clone https://github.com/LRTanvirM/tui-center.git
cd tui-center
```

**2. Install System Dependencies**

Select the appropriate command for your distribution:

**Arch Linux / Manjaro:**

```bash
sudo pacman -S --needed curl fastfetch
```

**Debian / Ubuntu:**

```bash
sudo apt update
sudo apt install -y curl fastfetch
```

**Fedora / RHEL / CentOS:**

```bash
sudo dnf install -y curl fastfetch
```

**Alpine Linux:**

```bash
sudo apk add curl fastfetch
```

**3. Install Rust Compiler**

If Rust is not already installed on your system:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
```

Verify the installation:

```bash
rustc --version
```

**4. Compile and Install**

```bash
cargo build --release
sudo cp target/release/tui-center /usr/local/bin/
sudo chmod +x /usr/local/bin/tui-center
```

Verify the installation:

```bash
tui-center --version
```

## ⌨️ Keyboard Controls

| Key                  | Action                                                     |
| -------------------- | ---------------------------------------------------------- |
| Tab                  | ↔️ Switch focus between Status Bar, Workspace, and App Bar |
| Arrow Keys / j,k,h,l | 🔀 Navigate within the active pane                         |
| Enter                | ▶️ Launch selected application                             |
| 1 - 9                | ⚡ Quick-launch applications from the dock                 |
| t                    | 🎨 Cycle through available themes                          |
| f                    | ℹ️ Toggle the System Information pane                      |
| Esc                  | ⚙️ Open Settings and application customization             |
| q                    | 🚪 Exit the application (with confirmation)                |

## 🗑️ Uninstallation

To remove TUI Control Center from your system:

```bash
chmod +x uninstall.sh
./uninstall.sh
```

The uninstall script will remove the binary from system directories and clean up related files.

## 🤝 Credits

- **Author**: [Tanvir](https://github.com/LRTanvirM) & [TokiTauhid](https://github.com/tokitauhid/)
- **Built with**: [Ratatui](https://ratatui.rs/), [Crossterm](https://github.com/crossterm-rs/crossterm)
- **Optimized for**: CachyOS / Arch Linux
