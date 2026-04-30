use ratatui::widgets::ListState;
use std::io;
use std::process::Command as ProcessCommand;

use crate::config;
use crate::theme;
use crate::types::*;

impl MenuApp {
    pub fn new() -> Self {
        let config_path = config::get_config_path();
        let cfg = config::load_config(&config_path).unwrap_or_else(|| Config {
            first_launch: true,
            current_theme: "Nord".to_string(),
            default_show_sys_info: true,
            status_modules: vec![
                (StatusModule::Greeting, true),
                (StatusModule::Time, true),
                (StatusModule::Memory, true),
                (StatusModule::Uptime, true),
                (StatusModule::Theme, true),
                (StatusModule::SysInfoToggle, true),
                (StatusModule::Audio, true),
                (StatusModule::Network, true),
                (StatusModule::Power, true),
            ],
        });

        let mut state = ListState::default();
        state.select(Some(0));
        let mut options_state = ListState::default();
        options_state.select(Some(0));
        let mut theme_state = ListState::default();
        theme_state.select(Some(0));
        let mut layout_state = ListState::default();
        layout_state.select(Some(0));
        let mut suggested_state = ListState::default();
        suggested_state.select(Some(0));

        // ── Distro / AUR detection ──────────────────────────────────────
        let distro_id = config::detect_distro();
        let is_arch = distro_id == "arch";

        let aur_helper_choices = vec![
            "yay".to_string(),
            "paru".to_string(),
            "aurman".to_string(),
        ];
        let mut aur_helper = None;
        let mut aur_helper_index = 0;
        if is_arch {
            for (i, helper) in aur_helper_choices.iter().enumerate() {
                if ProcessCommand::new("sh")
                    .arg("-c")
                    .arg(format!("command -v {}", helper))
                    .output()
                    .map(|o| o.status.success())
                    .unwrap_or(false)
                {
                    aur_helper = Some(helper.clone());
                    aur_helper_index = i;
                    break;
                }
            }
        }

        // ── Themes ──────────────────────────────────────────────────────
        let themes = theme::default_themes();
        let current_theme = themes
            .iter()
            .position(|t| t.name == cfg.current_theme)
            .unwrap_or(0);

        // ── Layouts ─────────────────────────────────────────────────────
        let layouts = vec![
            PresetLayout { name: "Default".to_string(), description: "Full featured layout with all panes".to_string() },
            PresetLayout { name: "Compact".to_string(), description: "Minimal, focused workspace".to_string() },
            PresetLayout { name: "Spacious".to_string(), description: "Large text and generous spacing".to_string() },
        ];

        // ── Suggested apps with distro-aware repo notes ─────────────────
        let suggested_apps = Self::build_suggested_apps(is_arch, &distro_id);

        let default_show_sys_info = cfg.default_show_sys_info;
        let sys_info_text = config::fetch_sys_info();
        let initial_mode = if cfg.first_launch {
            AppMode::OnboardingStart
        } else {
            AppMode::Normal
        };

        let mut app = Self {
            items: vec![],
            state,
            mode: initial_mode,
            focus: FocusPane::Workspace,
            status_index: 0,
            app_bar_index: 0,
            app_bar_items: vec![],
            options_index: 0,
            options_state,
            themes,
            current_theme,
            theme_state,
            show_sys_info: default_show_sys_info,
            default_show_sys_info,
            sys_info_text,
            quit_index: 0,
            input_buffer: String::new(),
            add_name: String::new(),
            add_desc: String::new(),
            add_cmd: String::new(),
            config: cfg,
            config_path,
            layouts,
            current_layout: 0,
            layout_state,
            suggested_apps,
            suggested_state,
            distro_id,
            is_arch,
            chaotic_aur_enabled: false,
            chaotic_aur_index: 0,
            aur_helper,
            aur_helper_choices,
            aur_helper_index,
            onboarding_focus: 0,
            install_status: String::new(),
            cheat_files: Vec::new(),
            cheat_file_index: 0,
            cheat_status: String::new(),
            import_export_index: 0,
            greeting_text: String::new(),
            user_name: std::env::var("USER").unwrap_or_else(|_| "User".to_string()),
            audio_vol: String::new(),
            network_info: String::new(),
            battery_info: String::new(),
            status_custom_index: 0,
            last_refresh: std::time::Instant::now(),
        };
        app.refresh_status_bar_data();
        app
    }

    /// Periodically refresh dynamic data like battery and audio that isn't provided natively by sysinfo.
    pub fn refresh_status_bar_data(&mut self) {
        // Battery status (basic Linux sysfs reading)
        let bat_cap = std::fs::read_to_string("/sys/class/power_supply/BAT0/capacity")
            .unwrap_or_else(|_| "N/A".to_string())
            .trim()
            .to_string();
        let bat_stat = std::fs::read_to_string("/sys/class/power_supply/BAT0/status")
            .unwrap_or_else(|_| "".to_string())
            .trim()
            .to_string();
        self.battery_info = if bat_cap == "N/A" {
            "🔌 AC".to_string()
        } else {
            let icon = if bat_stat == "Charging" { "⚡" } else { "🔋" };
            format!("{} {}%", icon, bat_cap)
        };

        // Greeting
        let hour = chrono::Local::now().format("%H").to_string().parse::<u32>().unwrap_or(12);
        self.greeting_text = match hour {
            5..=11 => "Good Morning",
            12..=16 => "Good Afternoon",
            17..=21 => "Good Evening",
            _ => "Good Night",
        }.to_string();

        // Audio volume (using wpctl as common on modern desktop environments, or amixer)
        // This is a rough heuristic.
        let out = ProcessCommand::new("sh").arg("-c").arg("wpctl get-volume @DEFAULT_AUDIO_SINK@ || amixer sget Master").output();
        if let Ok(output) = out {
            let s = String::from_utf8_lossy(&output.stdout);
            if s.contains("Volume:") {
                // wpctl output: "Volume: 0.50 [MUTED]"
                let parts: Vec<&str> = s.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Ok(vol_f) = parts[1].parse::<f32>() {
                        let vol_pct = (vol_f * 100.0) as i32;
                        let is_muted = s.contains("MUTED");
                        self.audio_vol = if is_muted { format!("🔇 {}%", vol_pct) } else { format!("🔊 {}%", vol_pct) };
                    }
                }
            } else if s.contains("[%]") {
                // amixer output has something like [50%]
                if let Some(start) = s.find('[') {
                    if let Some(end) = s[start..].find(']') {
                        let inner = &s[start+1..start+end];
                        let is_muted = s.contains("[off]");
                        self.audio_vol = if is_muted { format!("🔇 {}", inner) } else { format!("🔊 {}", inner) };
                    }
                }
            }
        }
        if self.audio_vol.is_empty() {
            self.audio_vol = "🔊 N/A".to_string();
        }

        // Network status (simple check)
        let route_out = ProcessCommand::new("sh").arg("-c").arg("ip route get 1.1.1.1 2>/dev/null").output();
        self.network_info = "🌐 Offline".to_string();
        if let Ok(o) = route_out {
            let s = String::from_utf8_lossy(&o.stdout);
            if s.contains("dev ") {
                let parts: Vec<&str> = s.split("dev ").collect();
                if parts.len() > 1 {
                    let iface = parts[1].split_whitespace().next().unwrap_or("");
                    let icon = if iface.starts_with("wl") || iface.starts_with("wlp") {
                        "📶 WiFi"
                    } else if iface.starts_with("en") || iface.starts_with("eth") {
                        "🖧 LAN"
                    } else {
                        "🌐 Net"
                    };
                    self.network_info = format!("{} ({})", icon, iface);
                }
            }
        }

        self.last_refresh = std::time::Instant::now();
    }

    /// Builds the list of suggested onboarding apps with distro-specific repo notes.
    fn build_suggested_apps(is_arch: bool, distro_id: &str) -> Vec<SuggestedApp> {
        // ── Distro-aware repo availability notes ────────────────────────
        let note_aur = if is_arch {
            String::new()
        } else {
            "⚠ AUR only; not in this distro's repos".to_string()
        };
        let note_aur_ppa = if is_arch {
            String::new()
        } else if distro_id == "ubuntu" || distro_id == "debian" {
            "⚠ Needs PPA or manual install".to_string()
        } else {
            "⚠ Not in standard repos".to_string()
        };
        let note_pacseek = if is_arch {
            String::new()
        } else {
            "⚠ Arch-only (pacman frontend)".to_string()
        };
        let note_cargo = if is_arch {
            String::new()
        } else {
            "⚠ Install via: cargo install <name>".to_string()
        };
        let note_go = if is_arch {
            String::new()
        } else {
            "⚠ Install via go install or release binary".to_string()
        };

        vec![
            // ═══ FILE & DISK ═══════════════════════════════════════════
            SuggestedApp { name: "yazi".into(), description: "Blazing fast file manager (Rust)".into(), command: "yazi".into(), selected: false, repo_note: note_cargo.clone(), is_appbar: false },
            SuggestedApp { name: "ranger".into(), description: "TUI file manager (Python)".into(), command: "ranger".into(), selected: false, repo_note: String::new(), is_appbar: false },
            SuggestedApp { name: "ncdu".into(), description: "Disk usage analyzer".into(), command: "ncdu".into(), selected: false, repo_note: String::new(), is_appbar: false },
            SuggestedApp { name: "dust".into(), description: "Intuitive disk usage (du + rust)".into(), command: "dust".into(), selected: false, repo_note: note_cargo.clone(), is_appbar: false },
            SuggestedApp { name: "fzf".into(), description: "Fuzzy finder for files & history".into(), command: "fzf".into(), selected: true, repo_note: String::new(), is_appbar: false },
            SuggestedApp { name: "zoxide".into(), description: "Smarter cd (z) command".into(), command: "zoxide query -ls".into(), selected: false, repo_note: String::new(), is_appbar: false },

            // ═══ PRODUCTIVITY ══════════════════════════════════════════
            SuggestedApp { name: "navi".into(), description: "Interactive cheatsheet browser".into(), command: "navi".into(), selected: false, repo_note: note_cargo.clone(), is_appbar: false },
            SuggestedApp { name: "lazygit".into(), description: "Git TUI client".into(), command: "lazygit".into(), selected: false, repo_note: note_go.clone(), is_appbar: false },
            SuggestedApp { name: "lazydocker".into(), description: "Docker management TUI".into(), command: "lazydocker".into(), selected: false, repo_note: note_go.clone(), is_appbar: false },
            SuggestedApp { name: "calcurse".into(), description: "Calendar & scheduler".into(), command: "calcurse".into(), selected: false, repo_note: String::new(), is_appbar: false },
            SuggestedApp { name: "newsboat".into(), description: "RSS/Atom feed reader".into(), command: "newsboat".into(), selected: false, repo_note: String::new(), is_appbar: false },
            SuggestedApp { name: "neomutt".into(), description: "Email client".into(), command: "neomutt".into(), selected: false, repo_note: String::new(), is_appbar: false },

            // ═══ SYSTEM MONITORING ═════════════════════════════════════
            SuggestedApp { name: "btop".into(), description: "Resource monitor (C++)".into(), command: "btop".into(), selected: true, repo_note: String::new(), is_appbar: false },
            SuggestedApp { name: "htop".into(), description: "Process viewer".into(), command: "htop".into(), selected: false, repo_note: String::new(), is_appbar: false },
            SuggestedApp { name: "fastfetch".into(), description: "System info fetch tool".into(), command: "fastfetch".into(), selected: true, repo_note: String::new(), is_appbar: false },
            SuggestedApp { name: "alsamixer".into(), description: "Audio mixer (ALSA)".into(), command: "alsamixer".into(), selected: false, repo_note: String::new(), is_appbar: false },
            SuggestedApp { name: "nmtui".into(), description: "Network Manager TUI".into(), command: "nmtui".into(), selected: false, repo_note: String::new(), is_appbar: false },
            SuggestedApp { name: "Weather".into(), description: "Show weather info (curl)".into(), command: "curl -s wttr.in?format=3".into(), selected: true, repo_note: String::new(), is_appbar: false },

            // ═══ MEDIA & MISC ══════════════════════════════════════════
            SuggestedApp { name: "termusic".into(), description: "Terminal music player".into(), command: "termusic".into(), selected: false, repo_note: note_aur.clone(), is_appbar: false },
            SuggestedApp { name: "ncmpcpp".into(), description: "MPD music client".into(), command: "ncmpcpp".into(), selected: false, repo_note: String::new(), is_appbar: false },
            SuggestedApp { name: "cava".into(), description: "Audio visualiser".into(), command: "cava".into(), selected: false, repo_note: note_aur_ppa, is_appbar: false },
            SuggestedApp { name: "viu".into(), description: "Terminal image viewer".into(), command: "viu".into(), selected: false, repo_note: note_aur.clone(), is_appbar: false },
            SuggestedApp { name: "ytfzf".into(), description: "YouTube from terminal (fzf)".into(), command: "ytfzf".into(), selected: false, repo_note: note_aur, is_appbar: false },
            SuggestedApp { name: "pacseek".into(), description: "Package search TUI".into(), command: "pacseek".into(), selected: false, repo_note: note_pacseek, is_appbar: false },

            // ═══ APP BAR (quick-launch) ════════════════════════════════
            SuggestedApp { name: "Browser".into(), description: "Web browser".into(), command: "xdg-open http://".into(), selected: true, repo_note: String::new(), is_appbar: true },
            SuggestedApp { name: "Files".into(), description: "File manager".into(), command: "xdg-open .".into(), selected: true, repo_note: String::new(), is_appbar: true },
            SuggestedApp { name: "Terminal".into(), description: "Extra terminal".into(), command: "$TERMINAL".into(), selected: true, repo_note: String::new(), is_appbar: true },
            SuggestedApp { name: "Settings".into(), description: "System settings".into(), command: "xdg-open settings://".into(), selected: false, repo_note: String::new(), is_appbar: true },
        ]
    }

    // ── Navigation helpers ──────────────────────────────────────────────

    pub fn save_config(&self) -> io::Result<()> {
        config::save_config(&self.config, &self.config_path)
    }

    pub fn next_list(&mut self) {
        let i = match self.state.selected() {
            Some(i) => if i >= self.items.len() - 1 { 0 } else { i + 1 },
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn prev_list(&mut self) {
        let i = match self.state.selected() {
            Some(i) => if i == 0 { self.items.len() - 1 } else { i - 1 },
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn next_opt(&mut self, len: usize) {
        if len == 0 { return; }
        self.options_index = (self.options_index + 1) % len;
        self.options_state.select(Some(self.options_index));
    }

    pub fn prev_opt(&mut self, len: usize) {
        if len == 0 { return; }
        self.options_index = if self.options_index == 0 { len - 1 } else { self.options_index - 1 };
        self.options_state.select(Some(self.options_index));
    }

    pub fn run_app_bar(&mut self, index: usize) -> Option<String> {
        if index < self.app_bar_items.len() {
            Some(self.app_bar_items[index].cmd.clone())
        } else {
            None
        }
    }
}
