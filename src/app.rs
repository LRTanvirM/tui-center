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

        Self {
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
        }
    }

    /// Builds the list of suggested onboarding apps with distro-specific repo notes.
    fn build_suggested_apps(is_arch: bool, distro_id: &str) -> Vec<SuggestedApp> {
        let note_aur = if is_arch {
            String::new()
        } else {
            "⚠ AUR only (Arch); not in this distro's repos".to_string()
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

        vec![
            // ── Workspace apps ──
            SuggestedApp { name: "Weather".into(), description: "Show weather info".into(), command: "curl -s wttr.in?format=3".into(), selected: true, repo_note: String::new(), is_appbar: false },
            SuggestedApp { name: "btop".into(), description: "System monitor".into(), command: "btop".into(), selected: true, repo_note: String::new(), is_appbar: false },
            SuggestedApp { name: "htop".into(), description: "Process viewer".into(), command: "htop".into(), selected: false, repo_note: String::new(), is_appbar: false },
            SuggestedApp { name: "ranger".into(), description: "TUI file manager".into(), command: "ranger".into(), selected: false, repo_note: String::new(), is_appbar: false },
            SuggestedApp { name: "termusic".into(), description: "Terminal music player".into(), command: "termusic".into(), selected: false, repo_note: note_aur.clone(), is_appbar: false },
            SuggestedApp { name: "cava".into(), description: "Audio visualiser".into(), command: "cava".into(), selected: false, repo_note: note_aur_ppa.clone(), is_appbar: false },
            SuggestedApp { name: "pacseek".into(), description: "Package search TUI".into(), command: "pacseek".into(), selected: false, repo_note: note_pacseek, is_appbar: false },
            SuggestedApp { name: "viu".into(), description: "Terminal image viewer".into(), command: "viu".into(), selected: false, repo_note: note_aur.clone(), is_appbar: false },
            SuggestedApp { name: "newsboat".into(), description: "RSS/Atom feed reader".into(), command: "newsboat".into(), selected: false, repo_note: String::new(), is_appbar: false },
            SuggestedApp { name: "calcurse".into(), description: "Calendar & scheduler".into(), command: "calcurse".into(), selected: false, repo_note: String::new(), is_appbar: false },
            SuggestedApp { name: "ncmpcpp".into(), description: "MPD music client".into(), command: "ncmpcpp".into(), selected: false, repo_note: String::new(), is_appbar: false },
            SuggestedApp { name: "neomutt".into(), description: "Email client".into(), command: "neomutt".into(), selected: false, repo_note: String::new(), is_appbar: false },
            // ── App-bar apps ──
            SuggestedApp { name: "Browser".into(), description: "Web browser".into(), command: "xdg-open http://".into(), selected: true, repo_note: String::new(), is_appbar: true },
            SuggestedApp { name: "Files".into(), description: "File manager (thunar)".into(), command: "thunar".into(), selected: true, repo_note: String::new(), //remove thunar use xdg opne to open users default files. is_appbar: true },
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
