# 🖥️ TUI Control Center

A high-performance, themeable terminal dashboard for Linux power users.

![Dashboard Screenshot](https://raw.githubusercontent.com/LRTanvirM/tui-center/master/assets/screenshot.png)

TUI Control Center is a feature-rich terminal interface built with **Rust** and **Ratatui** that transforms your terminal into a fully interactive dashboard environment. It provides real-time system monitoring, application launching, and deep customization — all while maintaining exceptional performance and minimal resource usage.

## ✨ Features

### 🧭 Dashboard

- **Three-pane layout** — Status Bar, Main Workspace, and App Bar with `Tab` cycling
- **Stateful navigation** — Vi-style (`hjkl`) and arrow key navigation across all panes
- **Context-aware footer** — Dynamic key hints that update based on the active pane and popup

### 📊 Smart Status Bar

The top bar is fully modular — each widget can be toggled on/off and reordered:

| Module       | Description                                                       |
| ------------ | ----------------------------------------------------------------- |
| 👋 Greeting  | Time-of-day greeting with `@username`                             |
| 🕒 Clock     | 12-hour system time                                               |
| 💾 Memory    | Live RAM usage (GiB)                                              |
| ⏱ Uptime     | System uptime (hours/minutes)                                     |
| 🎨 Theme     | Active color theme name                                           |
| 🖥 SysInfo   | Toggle the system info panel                                      |
| 🔊 Audio     | Volume level via `wpctl` or `amixer` — click to open `alsamixer`  |
| 🌐 Network   | WiFi/LAN detection with interface name — click to open `nmtui`    |
| 🔋 Power     | Battery percentage and charging state (or `🔌 AC` on desktops)    |

### 🎨 Theming

Twelve built-in high-contrast themes with instant switching (`t` key):

- **Nord** — Cyan accent
  ![Nord Theme Screenshot](https://raw.githubusercontent.com/LRTanvirM/tui-center/master/assets/nord.png)

- **Dracula** — Magenta accent
  ![Dracula Theme Screenshot](https://raw.githubusercontent.com/LRTanvirM/tui-center/master/assets/dracula.png)

- **Gruvbox** — Yellow accent
  ![Gruvbox Theme Screenshot](https://raw.githubusercontent.com/LRTanvirM/tui-center/master/assets/gruvbox.png)

- **Catppuccin Mocha** — Mauve accent (Dark)
  ![Catppuccin Mocha Theme Screenshot](https://raw.githubusercontent.com/LRTanvirM/tui-center/master/assets/catppuccin-mocha.png)

- **Catppuccin Macchiato** — Blue accent (Medium Dark)
  ![Catppuccin Macchiato Theme Screenshot](https://raw.githubusercontent.com/LRTanvirM/tui-center/master/assets/catppuccin-macchiato.png)

- **Catppuccin Frappé** — Pink accent (Soft Dark)
  ![Catppuccin Frappé Theme Screenshot](https://raw.githubusercontent.com/LRTanvirM/tui-center/master/assets/catppuccin-frappe.png)

- **Catppuccin Latte** — Teal accent (Light)
  ![Catppuccin Latte Theme Screenshot](https://raw.githubusercontent.com/LRTanvirM/tui-center/master/assets/catppuccin-latte.png)

- **Tokyo Night** — Purple accent
  ![Tokyo Night Theme Screenshot](https://raw.githubusercontent.com/LRTanvirM/tui-center/master/assets/tokyo-night.png)

- **Solarized Dark** — Cyan accent
  ![Solarized Dark Theme Screenshot](https://raw.githubusercontent.com/LRTanvirM/tui-center/master/assets/solarized-dark.png)

- **Solarized Light** — Cyan accent
  ![Solarized Light Theme Screenshot](https://raw.githubusercontent.com/LRTanvirM/tui-center/master/assets/solarized-light.png)

- **Monokai** — Pink/Green accent
  ![Monokai Theme Screenshot](https://raw.githubusercontent.com/LRTanvirM/tui-center/master/assets/monokai.png)

- **One Dark** — Blue/Purple accent
  ![One Dark Theme Screenshot](https://raw.githubusercontent.com/LRTanvirM/tui-center/master/assets/one-dark.png)

### 🚀 Onboarding Wizard

A guided first-run setup (re-runnable from Settings) that configures:

1. **Chaotic AUR** integration (Arch Linux only)
2. **AUR helper** detection/installation (`yay`, `paru`, `pikaur`, `trizen`)
3. **Theme** selection with live preview
4. **Layout** configuration
5. **App selection** from a curated registry of 28+ TUI apps

### 📦 Comprehensive TUI App Registry

The onboarding wizard includes distro-aware availability notes for every app:

| Category          | Apps                                                         |
| ----------------- | ------------------------------------------------------------ |
| **File & Disk**   | `yazi`, `ranger`, `ncdu`, `dust`, `fzf`, `zoxide`           |
| **Productivity**  | `navi`, `lazygit`, `lazydocker`, `calcurse`, `newsboat`, `neomutt` |
| **System**        | `btop`, `htop`, `fastfetch`, `alsamixer`, `nmtui`, Weather   |
| **Media & Misc**  | `termusic`, `ncmpcpp`, `cava`, `viu`, `ytfzf`, `pacseek`    |
| **App Bar**       | Browser, Files, Terminal, System Settings                    |

### 📂 Navi `.cheat` Integration

Full bidirectional support for [navi](https://github.com/denisidoro/navi) cheat sheets:

- **Import** `.cheat` files from `~/.local/share/navi/cheats/` or `~/.config/tui-center/cheats/`
- **Export** your workspace commands to `.cheat` format
- **Parser** supports `%` tags, `#` descriptions, `;` comments, `$` variables, `@` annotations

### ⚙️ Settings Panel

Organized into four sections accessible via `Esc`:

- **Workspace** — Edit main workspace apps, edit app bar apps
- **Display** — Toggle SysInfo, set SysInfo default, customize top bar modules
- **Data** — Import/export `.cheat` files
- **System** — Re-run onboarding wizard

### 🌍 Cross-Platform Package Manager Support

Automatic detection and installation support for:

| Distro             | Package Manager      |
| ------------------ | -------------------- |
| Arch / Manjaro     | `pacman` + AUR       |
| Ubuntu / Debian    | `apt`                |
| Fedora / RHEL      | `dnf`                |
| openSUSE           | `zypper`             |
| Void Linux         | `xbps-install`       |
| macOS              | `brew`               |

## 📋 Prerequisites

| Package          | Purpose                                |
| ---------------- | -------------------------------------- |
| `curl`           | 🌐 Used by the Weather widget          |
| `fastfetch`      | 📱 Powers the System Information pane  |
| `rust` / `cargo` | 🦀 Required to compile the application |

> **Note:** All other TUI apps (btop, lazygit, yazi, etc.) are optional and can be installed through the onboarding wizard.

## 🚀 Installation

### ⚡ Option 1: Automated Installation (Recommended)

```bash
curl -sSf https://raw.githubusercontent.com/LRTanvirM/tui-center/master/install.sh | bash
```

The installer automatically detects your Linux distribution, installs dependencies, compiles, and installs the binary system-wide.

### 📦 Option 2: Clone & Install

```bash
git clone https://github.com/LRTanvirM/tui-center.git
cd tui-center
chmod +x install.sh
./install.sh
```

### 🔧 Option 3: Manual Installation

**1. Clone the repository:**

```bash
git clone https://github.com/LRTanvirM/tui-center.git
cd tui-center
```

**2. Install system dependencies:**

```bash
# Arch Linux / Manjaro
sudo pacman -S --needed curl fastfetch

# Debian / Ubuntu
sudo apt update && sudo apt install -y curl fastfetch

# Fedora / RHEL
sudo dnf install -y curl fastfetch

# openSUSE
sudo zypper install -y curl fastfetch
```

**3. Install Rust (if not already installed):**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
```

**4. Compile and install:**

```bash
cargo build --release
sudo cp target/release/tui-center /usr/local/bin/
sudo chmod +x /usr/local/bin/tui-center
```

## ⌨️ Keyboard Controls

### Global

| Key           | Action                                          |
| ------------- | ----------------------------------------------- |
| `Tab`         | Cycle focus: Status Bar → Workspace → App Bar   |
| `↑↓` / `jk`  | Navigate lists                                  |
| `←→` / `hl`  | Navigate status bar and app bar                 |
| `Enter`       | Launch app / confirm action                     |
| `1-9`         | Quick-launch from app bar                       |
| `t`           | Cycle through themes                            |
| `f`           | Toggle system info panel                        |
| `Esc`         | Open settings / go back                         |
| `?` / `F1`    | Show help                                       |
| `q`           | Quit (with confirmation)                        |

### In Settings / Popups

| Key           | Action                                          |
| ------------- | ----------------------------------------------- |
| `↑↓` / `jk`  | Navigate options                                |
| `Enter`       | Select / confirm                                |
| `Space`       | Toggle checkboxes                               |
| `Shift+J/K`   | Reorder items (top bar customization)           |
| `a`           | Add new app (in edit mode)                      |
| `d`           | Delete selected app (in edit mode)              |
| `Esc`         | Go back to previous screen                      |
| `Backspace`   | Go back (same as Esc)                           |

## 📁 Configuration

All configuration is stored in `~/.config/tui-center/`:

| File              | Purpose                              |
| ----------------- | ------------------------------------ |
| `config.json`     | Theme, first-launch flag, status bar module order & visibility |
| `cheats/`         | Exported `.cheat` files              |

## 🏗️ Project Structure

```
src/
├── main.rs          # Entry point, event loop, child process spawning
├── app.rs           # MenuApp state, initialization, system data refresh
├── types.rs         # All shared types (AppMode, Config, StatusModule, etc.)
├── config.rs        # Config file I/O, distro detection
├── theme.rs         # Built-in theme definitions
├── handlers.rs      # All keyboard event handling
├── onboarding.rs    # Onboarding wizard logic & package installation
├── cheat.rs         # Navi .cheat file parser & converter
└── ui/
    ├── mod.rs           # Top-level UI dispatcher
    ├── main_ui.rs       # Dashboard rendering (status bar, workspace, app bar, popups)
    └── onboarding_ui.rs # Onboarding wizard screens
```

## 🗑️ Uninstallation

```bash
chmod +x uninstall.sh
./uninstall.sh
```

The uninstall script removes the binary, optionally removes core prerequisites, and cleans up build artifacts.

## 🛠️ Built With

- [Ratatui](https://ratatui.rs/) — Terminal UI framework
- [Crossterm](https://github.com/crossterm-rs/crossterm) — Terminal manipulation
- [sysinfo](https://crates.io/crates/sysinfo) — System monitoring
- [chrono](https://crates.io/crates/chrono) — Date/time handling
- [serde](https://serde.rs/) — Configuration serialization

## 🤝 Credits

- **Author**: [Tanvir](https://github.com/LRTanvirM) & [TokiTauhid](https://github.com/tokitauhid/)
- **Optimized for**: CachyOS / Arch Linux (works on all major distributions)
