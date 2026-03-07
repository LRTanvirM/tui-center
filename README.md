# 🖥️ TUI Control Center

**A high-performance, themeable terminal dashboard for Linux power users.**

![Dashboard Screenshot](https://raw.githubusercontent.com/LRTanvirM/tui-center/master/assets/screenshot.png)

Built with **Rust** and **Ratatui**, this Control Center transforms your terminal into a fully interactive desktop-style environment. It features a live system status bar, a dynamic application launcher, and integrated system information—all while remaining blazingly fast and light on resources.

---

## ✨ Features

* **Stateful Navigation:** Smoothly switch focus between the **Status Bar**, **Main Workspace**, and **App Bar** using `Tab`.
* **Dynamic Theming:** Instant switching between high-contrast dark themes (**Nord**, **Dracula**, and **Gruvbox**) by pressing `t`.
* **Live System Stats:** Real-time monitoring of local time, RAM usage, and system uptime.
* **Integrated App Launcher:** Launch your favorite TUI and GUI apps (Browser, Pacseek, Wikipedia, etc.) directly from the interface.
* **System Info Pane:** A dedicated toggleable pane (`f`) that displays clean, formatted system information via `fastfetch`.
* **Customizable:** Easily add or delete applications from the Workspace or App Bar through the built-in interactive Settings menu (`Esc`).
* **Global Hotkeys:** Press `1-9` at any time to instantly trigger apps in your bottom dock.

---

## 🛠️ Prerequisites

This dashboard acts as a central hub for other terminal utilities. For the best experience, ensure the following are installed:

| Category | Apps |
| --- | --- |
| **System** | `btop`, `fastfetch`, `curl`, `tclock` |
| **Internet** | `wiki-tui`, `gophertube`, `endcord` |
| **Media** | `termusic`, `soundscope`, `viu-media` |
| **Files** | `spf` (Superfile), `pacseek` (Arch/CachyOS) |

---

## 🚀 Installation

### 1. Clone the Repository

```bash
git clone https://github.com/LRTanvirM/tui-center.git
cd tui-center

```

### 2. Install Dependencies

Run the included script to detect your distro and install core requirements:

```bash
chmod +x prerequisites.sh
./prerequisites.sh

```

### 3. Build and Install System-Wide

Use the installer to compile the optimized Rust binary and move it to your path:

```bash
chmod +x install.sh
./install.sh

```

---

## ⌨️ Controls

| Key | Action |
| --- | --- |
| **Tab** | Switch focus (Status Bar ↔️ Workspace ↔️ App Bar) |
| **Arrows / j,k,h,l** | Navigate within the active pane |
| **Enter** | Launch selected application / Toggle setting |
| **1 - 9** | Quick-launch apps from the Bottom Bar |
| **t** | Cycle through UI Themes |
| **f** | Toggle System Info Pane |
| **Esc** | Open Settings / Customize Apps |
| **q** | Quit (with confirmation) |

---

## 🤝 Credits

* **Core Model:** Tanvir (Vibe coded with Gemini)
* **Frameworks:** [Ratatui](https://ratatui.rs/), [Crossterm](https://github.com/crossterm-rs/crossterm)
* **OS:** Optimized for CachyOS / Arch Linux
