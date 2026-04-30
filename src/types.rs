use ratatui::style::Color;
use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// ── Mode / Focus enums ──────────────────────────────────────────────────────

#[derive(PartialEq, Clone, Copy)]
pub enum AppMode {
    Normal,
    Quitting,
    HelpPopup,
    OptionsPopup,
    ThemePopup,
    EditMain,
    EditApp,
    DeleteConfirmMain,
    DeleteConfirmApp,
    AddAppStep(AddField),
    AddMainStep(AddField),
    OnboardingStart,
    OnboardingChaoticAur,
    OnboardingAurHelper,
    OnboardingTheme,
    OnboardingLayout,
    OnboardingApps,
    OnboardingInstalling,
    OnboardingComplete,
}

#[derive(PartialEq, Clone, Copy)]
pub enum AddField { Name, Desc, Cmd }

#[derive(PartialEq)]
pub enum FocusPane {
    StatusBar,
    Workspace,
    AppBar,
}

// ── Data structs ────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct PresetLayout {
    pub name: String,
    pub description: String,
}

#[derive(Clone)]
pub struct SuggestedApp {
    pub name: String,
    pub description: String,
    pub command: String,
    pub selected: bool,
    pub repo_note: String,
    pub is_appbar: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub first_launch: bool,
    pub current_theme: String,
    pub default_show_sys_info: bool,
}

pub struct Theme {
    pub name: &'static str,
    pub focus_border: Color,
    pub unfocus_border: Color,
    pub highlight_bg: Color,
    pub highlight_fg: Color,
    pub text_normal: Color,
    pub text_accent: Color,
}

pub struct AppEntry {
    pub name: String,
    pub desc: String,
    pub cmd: String,
}

// ── Application state ───────────────────────────────────────────────────────

#[allow(dead_code)]
pub struct MenuApp {
    pub items: Vec<AppEntry>,
    pub state: ListState,
    pub mode: AppMode,
    pub focus: FocusPane,

    pub status_index: usize,
    pub app_bar_index: usize,
    pub app_bar_items: Vec<AppEntry>,

    pub options_index: usize,
    pub options_state: ListState,

    pub themes: Vec<Theme>,
    pub current_theme: usize,
    pub theme_state: ListState,

    pub show_sys_info: bool,
    pub default_show_sys_info: bool,
    pub sys_info_text: String,

    pub quit_index: usize,

    pub input_buffer: String,
    pub add_name: String,
    pub add_desc: String,
    pub add_cmd: String,

    pub config: Config,
    pub config_path: PathBuf,

    pub layouts: Vec<PresetLayout>,
    pub current_layout: usize,
    pub layout_state: ListState,

    pub suggested_apps: Vec<SuggestedApp>,
    pub suggested_state: ListState,

    pub distro_id: String,
    pub is_arch: bool,
    pub chaotic_aur_enabled: bool,
    pub chaotic_aur_index: usize,
    pub aur_helper: Option<String>,
    pub aur_helper_choices: Vec<String>,
    pub aur_helper_index: usize,
    pub onboarding_focus: usize,
    pub install_status: String,
}
