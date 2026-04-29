use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, stdout};
use std::path::PathBuf;
use std::process::Command as ProcessCommand;
use sysinfo::System;
use chrono::Local;

#[derive(PartialEq, Clone, Copy)]
enum AppMode {
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
    OnboardingTheme,
    OnboardingLayout,
    OnboardingApps,
    OnboardingComplete,
}

#[derive(PartialEq, Clone, Copy)]
enum AddField { Name, Desc, Cmd }

#[derive(PartialEq)]
enum FocusPane {
    StatusBar,
    Workspace,
    AppBar,
}

#[derive(Clone)]
struct PresetLayout {
    name: String,
    description: String,
}

#[derive(Clone)]
struct SuggestedApp {
    name: String,
    description: String,
    command: String,
    selected: bool,
}

#[derive(Serialize, Deserialize, Clone)]
struct Config {
    first_launch: bool,
    current_theme: String,
    default_show_sys_info: bool,
}

struct Theme {
    name: &'static str,
    focus_border: Color,
    unfocus_border: Color,
    highlight_bg: Color,
    highlight_fg: Color,
    text_normal: Color,
    text_accent: Color,
}

struct AppEntry {
    name: String,
    desc: String,
    cmd: String,
}

struct MenuApp {
    items: Vec<AppEntry>,
    state: ListState,
    mode: AppMode,
    focus: FocusPane,
    
    status_index: usize,
    app_bar_index: usize,
    app_bar_items: Vec<AppEntry>,
    
    options_index: usize,
    options_state: ListState,
    
    themes: Vec<Theme>,
    current_theme: usize,
    theme_state: ListState,

    show_sys_info: bool,
    default_show_sys_info: bool,
    sys_info_text: String,

    quit_index: usize,

    input_buffer: String,
    add_name: String,
    add_desc: String,
    add_cmd: String,

    config: Config,
    config_path: PathBuf,
    
    layouts: Vec<PresetLayout>,
    current_layout: usize,
    layout_state: ListState,
    
    suggested_apps: Vec<SuggestedApp>,
    suggested_state: ListState,
}

impl MenuApp {
    fn new() -> Self {
        let config_path = Self::get_config_path();
        let config = Self::load_config(&config_path).unwrap_or_else(|| Config {
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

        let themes = vec![
            Theme {
                name: "Nord",
                focus_border: Color::Cyan,
                unfocus_border: Color::DarkGray,
                highlight_bg: Color::Cyan,
                highlight_fg: Color::Black,
                text_normal: Color::White,
                text_accent: Color::LightCyan,
            },
            Theme {
                name: "Dracula",
                focus_border: Color::Magenta,
                unfocus_border: Color::DarkGray,
                highlight_bg: Color::Magenta,
                highlight_fg: Color::Black,
                text_normal: Color::White,
                text_accent: Color::LightMagenta,
            },
            Theme {
                name: "Gruvbox",
                focus_border: Color::Yellow,
                unfocus_border: Color::DarkGray,
                highlight_bg: Color::Yellow,
                highlight_fg: Color::Black,
                text_normal: Color::White,
                text_accent: Color::LightYellow,
            },
        ];

        let current_theme = themes.iter().position(|t| t.name == config.current_theme).unwrap_or(0);

        let items = vec![
            AppEntry{ name: "Weather".to_string(), desc: "Show weather info".to_string(), cmd: "curl -s wttr.in/Tangail | less -R".to_string() },
            AppEntry{ name: "Clock".to_string(), desc: "Terminal Clock".to_string(), cmd: "tclock".to_string() },
            AppEntry{ name: "Pacseek".to_string(), desc: "yay but in TUI".to_string(), cmd: "pacseek".to_string() },
            AppEntry{ name: "App Launcher".to_string(), desc: "App Launcher based on TUI".to_string(), cmd: "fsel".to_string() },
            AppEntry{ name: "Wikipedia".to_string(), desc: "Wikipedia - TUI".to_string(), cmd: "wiki-tui".to_string() },
            AppEntry{ name: "Notepad".to_string(), desc: "Notepad - TUI".to_string(), cmd: "tjournal".to_string() },
            AppEntry{ name: "Audio Visualizer".to_string(), desc: "Audio Visualizer".to_string(), cmd: "soundscope".to_string() },
            AppEntry{ name: "Termusic".to_string(), desc: "Music Player".to_string(), cmd: "termusic".to_string() },
            AppEntry{ name: "Discord".to_string(), desc: "Discord in Terminal".to_string(), cmd: "endcord".to_string() },
            AppEntry{ name: "YouTube".to_string(), desc: "YouTube Client".to_string(), cmd: "gophertube".to_string() },
            AppEntry{ name: "Watch Anime".to_string(), desc: "Viu - Watch Anime".to_string(), cmd: "viu-media".to_string() },
            AppEntry{ name: "Nyaa".to_string(), desc: "Nyaa torrents".to_string(), cmd: "nyaa".to_string() },
        ];

        let app_bar_items = vec![
            AppEntry{ name: "Browser".to_string(), desc: "brave".to_string(), cmd: "brave".to_string() }, 
            AppEntry{ name: "Files".to_string(), desc: "spf".to_string(), cmd: "spf".to_string() },
            AppEntry{ name: "Settings".to_string(), desc: "systemsettings".to_string(), cmd: "systemsettings".to_string() }, 
        ];

        let layouts = vec![
            PresetLayout { name: "Default".to_string(), description: "Full featured layout with all panes".to_string() },
            PresetLayout { name: "Compact".to_string(), description: "Minimal, focused workspace".to_string() },
            PresetLayout { name: "Spacious".to_string(), description: "Large text and generous spacing".to_string() },
        ];

        let suggested_apps = vec![
            SuggestedApp { name: "btop".to_string(), description: "System monitor".to_string(), command: "btop".to_string(), selected: true },
            SuggestedApp { name: "pacseek".to_string(), description: "Package manager TUI".to_string(), command: "pacseek".to_string(), selected: true },
            SuggestedApp { name: "termusic".to_string(), description: "Music player".to_string(), command: "termusic".to_string(), selected: false },
            SuggestedApp { name: "viu".to_string(), description: "Image viewer".to_string(), command: "viu".to_string(), selected: false },
        ];

        let default_show_sys_info = config.default_show_sys_info;
        let sys_info_text = Self::fetch_sys_info();

        let initial_mode = if config.first_launch { AppMode::OnboardingStart } else { AppMode::Normal };

        Self {
            items,
            state,
            mode: initial_mode,
            focus: FocusPane::Workspace,
            status_index: 0,
            app_bar_index: 0,
            app_bar_items,
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
            config,
            config_path,
            layouts,
            current_layout: 0,
            layout_state,
            suggested_apps,
            suggested_state,
        }
    }

    fn get_config_path() -> PathBuf {
        let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        let app_config_dir = config_dir.join("tui-center");
        if !app_config_dir.exists() {
            let _ = fs::create_dir_all(&app_config_dir);
        }
        app_config_dir.join("config.json")
    }

    fn load_config(path: &PathBuf) -> Option<Config> {
        fs::read_to_string(path).ok().and_then(|content| serde_json::from_str(&content).ok())
    }

    fn save_config(&self) -> io::Result<()> {
        let config_json = serde_json::to_string_pretty(&self.config)?;
        fs::write(&self.config_path, config_json)?;
        Ok(())
    }
    
    // Fixed: Stripping ANSI escape codes so layout isn't destroyed
    fn fetch_sys_info() -> String {
        let output = match ProcessCommand::new("fastfetch").output() {
            Ok(o) => o.stdout,
            Err(_) => match ProcessCommand::new("cachyos-fetch").output() {
                Ok(o) => o.stdout,
                Err(_) => match ProcessCommand::new("neofetch").output() {
                    Ok(o) => o.stdout,
                    Err(_) => b"Error: fastfetch/cachyos-fetch not found.".to_vec(),
                },
            },
        };
        // Strip invisible ANSI characters and convert to String
        let stripped = strip_ansi_escapes::strip(&output);
        String::from_utf8_lossy(&stripped).to_string()
    }

    fn next_list(&mut self) {
        let i = match self.state.selected() {
            Some(i) => if i >= self.items.len() - 1 { 0 } else { i + 1 },
            None => 0,
        };
        self.state.select(Some(i));
    }
    fn prev_list(&mut self) {
        let i = match self.state.selected() {
            Some(i) => if i == 0 { self.items.len() - 1 } else { i - 1 },
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn next_opt(&mut self, len: usize) {
        if len == 0 { return; }
        self.options_index = (self.options_index + 1) % len;
        self.options_state.select(Some(self.options_index));
    }
    fn prev_opt(&mut self, len: usize) {
        if len == 0 { return; }
        self.options_index = if self.options_index == 0 { len - 1 } else { self.options_index - 1 };
        self.options_state.select(Some(self.options_index));
    }

    fn run_app_bar(&mut self, index: usize) -> Option<String> {
        if index < self.app_bar_items.len() {
            Some(self.app_bar_items[index].cmd.clone())
        } else {
            None
        }
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage((100 - percent_y) / 2), Constraint::Percentage(percent_y), Constraint::Percentage((100 - percent_y) / 2)])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage((100 - percent_x) / 2), Constraint::Percentage(percent_x), Constraint::Percentage((100 - percent_x) / 2)])
        .split(popup_layout[1])[1]
}

fn main() -> io::Result<()> {
    // Tell the dashboard to ignore OS-level Ctrl+C kill signals!
    let _ = ctrlc::set_handler(|| {});

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut sys = System::new_all();
    let mut app = MenuApp::new();
    let mut should_quit = false;

    while !should_quit {
        sys.refresh_all();
        terminal.draw(|f| ui(f, &sys, &mut app))?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                
                // GLOBAL HOTKEYS
                if app.mode == AppMode::Normal {
                    if let KeyCode::Char(c) = key.code {
                        if c.is_digit(10) {
                            let digit = c.to_digit(10).unwrap() as usize;
                            if let Some(cmd) = app.run_app_bar(digit.saturating_sub(1)) {
                                disable_raw_mode()?;
                                stdout().execute(LeaveAlternateScreen)?;
                                let mut child = ProcessCommand::new("sh").arg("-c").arg(&cmd).spawn()?;
                                child.wait()?;
                                enable_raw_mode()?;
                                stdout().execute(EnterAlternateScreen)?;
                                terminal.clear()?;
                                continue;
                            }
                        }
                    }
                }

                match &app.mode {
                    AppMode::OnboardingStart => match key.code {
                        KeyCode::Esc => app.mode = AppMode::Normal,
                        KeyCode::Enter => app.mode = AppMode::OnboardingTheme,
                        _ => {}
                    },
                    AppMode::OnboardingTheme => match key.code {
                        KeyCode::Esc => app.mode = AppMode::Normal,
                        KeyCode::Up | KeyCode::Char('k') => {
                            let i = app.theme_state.selected().unwrap_or(0);
                            app.theme_state.select(Some(if i == 0 { app.themes.len() - 1 } else { i - 1 }));
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            let i = app.theme_state.selected().unwrap_or(0);
                            app.theme_state.select(Some((i + 1) % app.themes.len()));
                        }
                        KeyCode::Enter => {
                            if let Some(i) = app.theme_state.selected() { app.current_theme = i; }
                            app.mode = AppMode::OnboardingLayout;
                            app.layout_state.select(Some(0));
                        }
                        _ => {}
                    },
                    AppMode::OnboardingLayout => match key.code {
                        KeyCode::Esc => app.mode = AppMode::OnboardingTheme,
                        KeyCode::Up | KeyCode::Char('k') => app.prev_opt(app.layouts.len()),
                        KeyCode::Down | KeyCode::Char('j') => app.next_opt(app.layouts.len()),
                        KeyCode::Enter => {
                            app.current_layout = app.options_index;
                            app.mode = AppMode::OnboardingApps;
                            app.suggested_state.select(Some(0));
                            app.options_index = 0;
                        }
                        _ => {}
                    },
                    AppMode::OnboardingApps => match key.code {
                        KeyCode::Esc => app.mode = AppMode::OnboardingLayout,
                        KeyCode::Up | KeyCode::Char('k') => app.prev_opt(app.suggested_apps.len()),
                        KeyCode::Down | KeyCode::Char('j') => app.next_opt(app.suggested_apps.len()),
                        KeyCode::Char(' ') => {
                            app.suggested_apps[app.options_index].selected = !app.suggested_apps[app.options_index].selected;
                        }
                        KeyCode::Enter => {
                            for suggested in &app.suggested_apps {
                                if suggested.selected {
                                    app.items.push(AppEntry {
                                        name: suggested.name.clone(),
                                        desc: suggested.description.clone(),
                                        cmd: suggested.command.clone(),
                                    });
                                }
                            }
                            app.mode = AppMode::OnboardingComplete;
                        }
                        _ => {}
                    },
                    AppMode::OnboardingComplete => match key.code {
                        KeyCode::Esc | KeyCode::Enter => {
                            app.config.first_launch = false;
                            app.config.current_theme = app.themes[app.current_theme].name.to_string();
                            let _ = app.save_config();
                            app.mode = AppMode::Normal;
                        }
                        _ => {}
                    },
                    AppMode::Normal => match key.code {
                        KeyCode::Char('q') => { app.mode = AppMode::Quitting; app.quit_index = 0; },
                        KeyCode::Esc => { app.mode = AppMode::OptionsPopup; app.options_index = 0; app.options_state.select(Some(0)); },
                        KeyCode::F(1) => app.mode = AppMode::HelpPopup,
                        KeyCode::Char('f') => app.show_sys_info = !app.show_sys_info,
                        KeyCode::Char('t') => app.current_theme = (app.current_theme + 1) % app.themes.len(),
                        KeyCode::Tab => {
                            app.focus = match app.focus {
                                FocusPane::StatusBar => FocusPane::Workspace,
                                FocusPane::Workspace => FocusPane::AppBar,
                                FocusPane::AppBar => FocusPane::StatusBar,
                            };
                        }

                        KeyCode::Down | KeyCode::Char('j') => if app.focus == FocusPane::Workspace { app.next_list() },
                        KeyCode::Up | KeyCode::Char('k') => if app.focus == FocusPane::Workspace { app.prev_list() },
                        KeyCode::Left | KeyCode::Char('h') => {
                            if app.focus == FocusPane::StatusBar { app.status_index = app.status_index.saturating_sub(1); }
                            if app.focus == FocusPane::AppBar { app.app_bar_index = app.app_bar_index.saturating_sub(1); }
                        }
                        KeyCode::Right | KeyCode::Char('l') => {
                            // Status bar now has 5 slots (0 to 4)
                            if app.focus == FocusPane::StatusBar && app.status_index < 4 { app.status_index += 1; }
                            if app.focus == FocusPane::AppBar && app.app_bar_index < app.app_bar_items.len().saturating_sub(1) { app.app_bar_index += 1; }
                        }

                        KeyCode::Enter => {
                            let mut cmd = String::new();
                            match app.focus {
                                FocusPane::StatusBar => {
                                    match app.status_index {
                                        0 => cmd = "tclock".to_string(),
                                        1 => cmd = "btop".to_string(),
                                        2 => cmd = "uptime".to_string(),
                                        3 => { 
                                            // Open Theme Popup Window
                                            app.mode = AppMode::ThemePopup; 
                                            app.theme_state.select(Some(app.current_theme));
                                        },
                                        4 => app.show_sys_info = !app.show_sys_info,
                                        _ => {}
                                    }
                                },
                                FocusPane::Workspace => {
                                    if let Some(i) = app.state.selected() { cmd = app.items[i].cmd.clone() }
                                },
                                FocusPane::AppBar => {
                                    if !app.app_bar_items.is_empty() { cmd = app.app_bar_items[app.app_bar_index].cmd.clone() }
                                },
                            };

                            if !cmd.is_empty() {
                                // 1. Suspend the dashboard's terminal control
                                // We use .ok() or let _ = to prevent a crash from stopping the recovery chain
                                let _ = disable_raw_mode();
                                let _ = stdout().execute(LeaveAlternateScreen);

                                // 2. Launch the child process
                                let mut child = ProcessCommand::new("sh").arg("-c").arg(&cmd).spawn()?;
                                let _ = child.wait();

                                // 3. MANDATORY RECOVERY: Reset terminal regardless of how the app exited
                                let _ = enable_raw_mode();
                                let _ = stdout().execute(EnterAlternateScreen);

                                // 4. Force a full hardware redraw to clear any "curses" artifacts
                                terminal.clear()?;
                            }
                        }
                        _ => {}
                    },
                    AppMode::ThemePopup => match key.code {
                        KeyCode::Esc | KeyCode::Backspace => app.mode = AppMode::Normal,
                        KeyCode::Up | KeyCode::Char('k') => {
                            let i = app.theme_state.selected().unwrap_or(0);
                            app.theme_state.select(Some(if i == 0 { app.themes.len() - 1 } else { i - 1 }));
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            let i = app.theme_state.selected().unwrap_or(0);
                            app.theme_state.select(Some((i + 1) % app.themes.len()));
                        }
                        KeyCode::Enter => {
                            if let Some(i) = app.theme_state.selected() { app.current_theme = i; }
                            app.mode = AppMode::Normal;
                        }
                        _ => {}
                    },
                    AppMode::Quitting => match key.code {
                        KeyCode::Left | KeyCode::Right | KeyCode::Char('h') | KeyCode::Char('l') => {
                            app.quit_index = if app.quit_index == 0 { 1 } else { 0 };
                        }
                        KeyCode::Enter => {
                            if app.quit_index == 1 { should_quit = true; } else { app.mode = AppMode::Normal; }
                        }
                        KeyCode::Char('y') | KeyCode::Char('Y') => should_quit = true,
                        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc | KeyCode::Char('q') => app.mode = AppMode::Normal,
                        _ => {}
                    },
                    AppMode::HelpPopup => match key.code {
                        KeyCode::Esc | KeyCode::Enter | KeyCode::Backspace => app.mode = AppMode::Normal,
                        _ => {}
                    },
                    AppMode::OptionsPopup => {
                        let opts_len = 6;
                        match key.code {
                            KeyCode::Esc | KeyCode::Backspace => app.mode = AppMode::Normal,
                            KeyCode::Up | KeyCode::Char('k') => app.prev_opt(opts_len),
                            KeyCode::Down | KeyCode::Char('j') => app.next_opt(opts_len),
                            KeyCode::Enter => {
                                match app.options_index {
                                    0 => { app.mode = AppMode::EditMain; app.options_index = 0; app.options_state.select(Some(0)); }
                                    1 => { app.mode = AppMode::EditApp; app.options_index = 0; app.options_state.select(Some(0)); }
                                    2 => app.show_sys_info = !app.show_sys_info,
                                    3 => { app.default_show_sys_info = !app.default_show_sys_info; app.show_sys_info = app.default_show_sys_info; }
                                    4 => app.mode = AppMode::OnboardingStart,
                                    5 => app.mode = AppMode::Normal,
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    },
                    AppMode::EditMain | AppMode::EditApp => {
                        let is_main = app.mode == AppMode::EditMain;
                        let list_len = if is_main { app.items.len() } else { app.app_bar_items.len() };
                        match key.code {
                            KeyCode::Esc | KeyCode::Backspace => {
                                app.mode = AppMode::OptionsPopup;
                                app.options_index = 0; app.options_state.select(Some(0));
                            }
                            KeyCode::Up | KeyCode::Char('k') => app.prev_opt(list_len),
                            KeyCode::Down | KeyCode::Char('j') => app.next_opt(list_len),
                            KeyCode::Char('a') => {
                                app.mode = if is_main { AppMode::AddMainStep(AddField::Name) } else { AppMode::AddAppStep(AddField::Name) };
                                app.input_buffer.clear(); app.add_name.clear(); app.add_desc.clear(); app.add_cmd.clear();
                            }
                            KeyCode::Char('d') => {
                                if list_len > 0 {
                                    app.mode = if is_main { AppMode::DeleteConfirmMain } else { AppMode::DeleteConfirmApp };
                                    app.quit_index = 0;
                                }
                            }
                            _ => {}
                        }
                    },
                    AppMode::DeleteConfirmMain | AppMode::DeleteConfirmApp => {
                        let is_main = app.mode == AppMode::DeleteConfirmMain;
                        match key.code {
                            KeyCode::Left | KeyCode::Right | KeyCode::Char('h') | KeyCode::Char('l') => {
                                app.quit_index = if app.quit_index == 0 { 1 } else { 0 };
                            }
                            KeyCode::Enter => {
                                if app.quit_index == 1 { 
                                    if is_main { app.items.remove(app.options_index); app.mode = AppMode::EditMain; } 
                                    else { app.app_bar_items.remove(app.options_index); app.mode = AppMode::EditApp; }
                                    app.options_index = 0; app.options_state.select(Some(0));
                                } else {
                                    app.mode = if is_main { AppMode::EditMain } else { AppMode::EditApp };
                                }
                            }
                            KeyCode::Char('y') | KeyCode::Char('Y') => {
                                if is_main { app.items.remove(app.options_index); app.mode = AppMode::EditMain; } 
                                else { app.app_bar_items.remove(app.options_index); app.mode = AppMode::EditApp; }
                                app.options_index = 0; app.options_state.select(Some(0));
                            }
                            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc | KeyCode::Backspace => app.mode = if is_main { AppMode::EditMain } else { AppMode::EditApp },
                            _ => {}
                        }
                    }
                    AppMode::AddMainStep(step) | AppMode::AddAppStep(step) => {
                        let is_main = if let AppMode::AddMainStep(_) = app.mode { true } else { false };
                        match key.code {
                            KeyCode::Esc => { app.mode = if is_main { AppMode::EditMain } else { AppMode::EditApp }; }
                            KeyCode::Backspace => { 
                                if app.input_buffer.is_empty() { app.mode = if is_main { AppMode::EditMain } else { AppMode::EditApp }; } 
                                else { app.input_buffer.pop(); }
                            }
                            KeyCode::Char(c) => { app.input_buffer.push(c); }
                            KeyCode::Enter => {
                                match step {
                                    AddField::Name => {
                                        app.add_name = app.input_buffer.clone(); app.input_buffer.clear();
                                        app.mode = if is_main { AppMode::AddMainStep(AddField::Desc) } else { AppMode::AddAppStep(AddField::Desc) };
                                    }
                                    AddField::Desc => {
                                        app.add_desc = app.input_buffer.clone(); app.input_buffer.clear();
                                        app.mode = if is_main { AppMode::AddMainStep(AddField::Cmd) } else { AppMode::AddAppStep(AddField::Cmd) };
                                    }
                                    AddField::Cmd => {
                                        app.add_cmd = app.input_buffer.clone(); app.input_buffer.clear();
                                        let new_entry = AppEntry { name: app.add_name.clone(), desc: app.add_desc.clone(), cmd: app.add_cmd.clone() };
                                        if is_main { app.items.push(new_entry); app.mode = AppMode::EditMain; } else { app.app_bar_items.push(new_entry); app.mode = AppMode::EditApp; }
                                        app.options_index = 0; app.options_state.select(Some(0));
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    let _ = app.save_config();
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn ui(f: &mut Frame, sys: &System, app: &mut MenuApp) {
    let size = f.size();
    let theme = &app.themes[app.current_theme];

    match app.mode {
        AppMode::OnboardingStart => {
            let area = centered_rect(60, 50, size);
            f.render_widget(Clear, area);
            let popup_border_style = Style::default().fg(theme.focus_border);
            let popup_block = Block::default()
                .borders(Borders::ALL)
                .border_style(popup_border_style)
                .title(" Welcome to TUI Control Center ");
            let content = vec![
                Line::from(""),
                Line::from("Welcome! Let's set up your environment."),
                Line::from(""),
                Line::from("This wizard will guide you through:"),
                Line::from("  - Selecting a theme"),
                Line::from("  - Configuring your workspace layout"),
                Line::from("  - Adding your favorite applications"),
                Line::from(""),
                Line::from("Press Enter to continue or Esc to skip"),
            ];
            let p = Paragraph::new(content).block(popup_block).style(Style::default().fg(theme.text_normal));
            f.render_widget(p, area);
            return;
        }
        AppMode::OnboardingTheme => {
            let area = centered_rect(40, 50, size);
            f.render_widget(Clear, area);
            let popup_border_style = Style::default().fg(theme.focus_border);
            let popup_block = Block::default()
                .borders(Borders::ALL)
                .border_style(popup_border_style)
                .title(" Select Your Theme ");
            let theme_items: Vec<ListItem> = app.themes.iter().map(|t| ListItem::new(format!("  {}  ", t.name))).collect();
            let active_style = Style::default().bg(theme.highlight_bg).fg(theme.highlight_fg).add_modifier(Modifier::BOLD);
            let list = List::new(theme_items).block(popup_block).highlight_style(active_style).highlight_symbol(">> ");
            f.render_stateful_widget(list, area, &mut app.theme_state);
            return;
        }
        AppMode::OnboardingLayout => {
            let area = centered_rect(50, 50, size);
            f.render_widget(Clear, area);
            let popup_border_style = Style::default().fg(theme.focus_border);
            let popup_block = Block::default()
                .borders(Borders::ALL)
                .border_style(popup_border_style)
                .title(" Configure Layout ");
            let layout_items: Vec<ListItem> = app.layouts.iter().map(|l| ListItem::new(format!("  {}  ", l.name))).collect();
            let active_style = Style::default().bg(theme.highlight_bg).fg(theme.highlight_fg).add_modifier(Modifier::BOLD);
            let list = List::new(layout_items).block(popup_block).highlight_style(active_style).highlight_symbol(">> ");
            f.render_stateful_widget(list, area, &mut app.options_state);
            return;
        }
        AppMode::OnboardingApps => {
            let area = centered_rect(50, 60, size);
            f.render_widget(Clear, area);
            let popup_border_style = Style::default().fg(theme.focus_border);
            let popup_block = Block::default()
                .borders(Borders::ALL)
                .border_style(popup_border_style)
                .title(" Select Apps (Space to toggle, Enter to finish) ");
            let app_items: Vec<ListItem> = app.suggested_apps.iter().map(|a| {
                let checkbox = if a.selected { "[X]" } else { "[ ]" };
                ListItem::new(format!("{} {}", checkbox, a.name))
            }).collect();
            let active_style = Style::default().bg(theme.highlight_bg).fg(theme.highlight_fg).add_modifier(Modifier::BOLD);
            let list = List::new(app_items).block(popup_block).highlight_style(active_style).highlight_symbol(">> ");
            f.render_stateful_widget(list, area, &mut app.options_state);
            return;
        }
        AppMode::OnboardingComplete => {
            let area = centered_rect(60, 40, size);
            f.render_widget(Clear, area);
            let popup_border_style = Style::default().fg(theme.focus_border);
            let popup_block = Block::default()
                .borders(Borders::ALL)
                .border_style(popup_border_style)
                .title(" Setup Complete! ");
            let content = vec![
                Line::from(""),
                Line::from(format!("Theme: {}", app.themes[app.current_theme].name)),
                Line::from(format!("Layout: {}", app.layouts[app.current_layout].name)),
                Line::from(format!("Apps added: {}", app.suggested_apps.iter().filter(|a| a.selected).count())),
                Line::from(""),
                Line::from("Press any key to continue..."),
            ];
            let p = Paragraph::new(content).block(popup_block).style(Style::default().fg(theme.text_normal));
            f.render_widget(p, area);
            return;
        }
        _ => {}
    }
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1), Constraint::Length(3)])
        .split(size);

    let theme = &app.themes[app.current_theme];

    let status_border = if app.focus == FocusPane::StatusBar { theme.focus_border } else { theme.unfocus_border };
    let work_border = if app.focus == FocusPane::Workspace { theme.focus_border } else { theme.unfocus_border };
    let app_border = if app.focus == FocusPane::AppBar { theme.focus_border } else { theme.unfocus_border };

    // FIXED: Properly apply the Theme's highlight colors instead of just reversing
    let active_style = Style::default().bg(theme.highlight_bg).fg(theme.highlight_fg).add_modifier(Modifier::BOLD);
    let inactive_accent = Style::default().fg(theme.text_accent);

    let time = Local::now().format("%I:%M %p").to_string();
    let used_mem = sys.used_memory() as f64 / 1_073_741_824.0; 
    let total_mem = sys.total_memory() as f64 / 1_073_741_824.0;
    
    // Calculate Uptime (sysinfo returns seconds)
    let up_secs = System::uptime();
    let uptime_str = format!("{}h {}m", up_secs / 3600, (up_secs % 3600) / 60);

    // Apply active_style directly based on index
    let clock_style = if app.focus == FocusPane::StatusBar && app.status_index == 0 { active_style } else { inactive_accent };
    let ram_style = if app.focus == FocusPane::StatusBar && app.status_index == 1 { active_style } else { inactive_accent };
    let uptime_style = if app.focus == FocusPane::StatusBar && app.status_index == 2 { active_style } else { inactive_accent };
    let theme_style = if app.focus == FocusPane::StatusBar && app.status_index == 3 { active_style } else { inactive_accent };
    let sys_style = if app.focus == FocusPane::StatusBar && app.status_index == 4 { active_style } else { inactive_accent };

    let status_line = Line::from(vec![
        Span::styled(format!(" 🕒 {} ", time), clock_style),
        Span::raw("  |  "),
        Span::styled(format!(" 💾 {:.2} GiB / {:.2} GiB ", used_mem, total_mem), ram_style),
        Span::raw("  |  "),
        Span::styled(format!(" ⏱ Up: {} ", uptime_str), uptime_style),
        Span::raw("  |  "),
        Span::styled(format!(" 🎨 Theme: {} ", theme.name), theme_style),
        Span::raw("  |  "),
        Span::styled(if app.show_sys_info { " 🖥 Hide SysInfo " } else { " 🖥 Show SysInfo " }, sys_style),
    ]);

    let top_panel = Paragraph::new(status_line)
        .alignment(Alignment::Right)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(status_border)).title(" Status Bar (Tab to switch panes) "));
    f.render_widget(top_panel, chunks[0]);

    let work_chunks = if app.show_sys_info {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
            .split(chunks[1])
    } else {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(chunks[1])
    };

    let items: Vec<ListItem> = app.items.iter().map(|item| {
        ListItem::new(format!("{:<20} │ {}", item.name, item.desc)).style(Style::default().fg(theme.text_normal))
    }).collect();

    // FIXED: Main Workspace highlighting uses custom theme colors
    let highlight_style = if app.focus == FocusPane::Workspace { active_style } else { Style::default().fg(theme.unfocus_border) };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(work_border)).title(" Main Workspace (F1 for help, Esc for settings) "))
        .highlight_style(highlight_style)
        .highlight_symbol(">> ");
    f.render_stateful_widget(list, work_chunks[0], &mut app.state);

    if app.show_sys_info {
        // FIXED: Parsing the string into proper lines so Ratatui handles newlines flawlessly
        let sys_lines: Vec<Line> = app.sys_info_text.lines().map(|l| Line::from(l)).collect();
        let sys_info_block = Paragraph::new(sys_lines)
            .style(Style::default().fg(theme.text_normal))
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(theme.unfocus_border)).title(" System Info "));
        f.render_widget(sys_info_block, work_chunks[1]);
    }

    let mut app_bar_spans = Vec::new();
    let inactive_green = Style::default().fg(Color::Green);
    for (i, item) in app.app_bar_items.iter().enumerate() {
        let style = if app.focus == FocusPane::AppBar && app.app_bar_index == i { active_style } else { inactive_green };
        app_bar_spans.push(Span::styled(format!(" [{}] {} ", i + 1, item.name), style));
        if i < app.app_bar_items.len() - 1 { app_bar_spans.push(Span::raw("   ")); }
    }

    let bottom_app_bar = Paragraph::new(Line::from(app_bar_spans))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(app_border)).title(" App Bar (Press 1-9 to launch) "));
    f.render_widget(bottom_app_bar, chunks[2]);

    if app.mode != AppMode::Normal {
        let popup_border_style = Style::default().fg(theme.focus_border);
        let popup_block = Block::default()
            .borders(Borders::ALL)
            .border_style(popup_border_style)
            .style(Style::default().bg(Color::Black));

        match app.mode {
            AppMode::ThemePopup => {
                let area = centered_rect(30, 40, size); f.render_widget(Clear, area); 
                let theme_items: Vec<ListItem> = app.themes.iter().map(|t| ListItem::new(format!("  {}  ", t.name)).style(Style::default().fg(theme.text_normal))).collect();
                let list = List::new(theme_items).block(popup_block.title(" Select Theme ")).highlight_style(active_style).highlight_symbol(">> ");
                f.render_stateful_widget(list, area, &mut app.theme_state);
            },
            AppMode::Quitting => {
                let area = centered_rect(40, 20, size); f.render_widget(Clear, area); 
                let (yes_style, no_style) = if app.quit_index == 1 { (active_style, inactive_green) } else { (inactive_green, active_style) };
                
                // FIXED: Adding spacer lines vertically centers the text inside the popup
                let content = vec![
                    Line::from(""),
                    Line::from("Are you sure you want to quit?"),
                    Line::from(""),
                    Line::from(vec![ Span::styled(" [Y]es ", yes_style), Span::raw("   /   "), Span::styled(" [N]o ", no_style) ])
                ];
                let p = Paragraph::new(content).alignment(Alignment::Center).block(popup_block.title(" Exit Confirmation "));
                f.render_widget(p, area);
            },
            AppMode::HelpPopup => {
                let area = centered_rect(50, 40, size); f.render_widget(Clear, area); 
                let p = Paragraph::new("\n--- HELP ---\n\nTab: Switch Focus Pane\nArrows: Navigate\nEnter: Launch App\n1-9: App Bar Hotkeys\nq: Quit\n\nPress Esc to close.")
                    .alignment(Alignment::Center).style(Style::default().fg(theme.text_normal)).block(popup_block.title(" Commands "));
                f.render_widget(p, area);
            },
            AppMode::OptionsPopup => {
                let area = centered_rect(50, 50, size); f.render_widget(Clear, area); 
                let def_sys_text = if app.default_show_sys_info { "[X] Default Show SysInfo" } else { "[ ] Default Show SysInfo" };
                let toggle_sys_text = if app.show_sys_info { "[-] Toggle SysInfo (F)" } else { "[+] Toggle SysInfo (F)" };
                let settings_items = vec![ 
                    ListItem::new(" Customize Main Workspace Apps -> "), 
                    ListItem::new(" Customize App Bar Apps -> "), 
                    ListItem::new(toggle_sys_text), 
                    ListItem::new(def_sys_text), 
                    ListItem::new(" Run Onboarding Setup -> "),
                    ListItem::new(" <- Back ") 
                ];
                let list = List::new(settings_items).block(popup_block.title(" Settings "))
                    .highlight_style(active_style).highlight_symbol(">> ");
                f.render_stateful_widget(list, area, &mut app.options_state);
            },
            AppMode::EditMain | AppMode::EditApp => {
                let is_main = app.mode == AppMode::EditMain;
                let area = centered_rect(70, 60, size); f.render_widget(Clear, area); 
                let items_vec = if is_main { &app.items } else { &app.app_bar_items };
                let list_items: Vec<ListItem> = items_vec.iter().map(|item| { ListItem::new(format!("{:<20} │ {}", item.name, item.desc)) }).collect();
                let list = List::new(list_items)
                    .block(popup_block.title(if is_main { " Customize Main Apps ([a]dd / [d]elete, Esc to back) " } else { " Customize App Bar ([a]dd / [d]elete, Esc to back) " }))
                    .highlight_style(active_style).highlight_symbol(">> ");
                f.render_stateful_widget(list, area, &mut app.options_state);
            }
            AppMode::DeleteConfirmMain | AppMode::DeleteConfirmApp => {
                let is_main = app.mode == AppMode::DeleteConfirmMain;
                let app_name = if is_main { app.items[app.options_index].name.clone() } else { app.app_bar_items[app.options_index].name.clone() };
                let area = centered_rect(40, 20, size); f.render_widget(Clear, area); 
                let (yes_style, no_style) = if app.quit_index == 1 { (active_style, inactive_green) } else { (inactive_green, active_style) };
                let content = vec![ Line::from(""), Line::from(format!("Delete app '{}'?", app_name)), Line::from(""), Line::from(vec![ Span::styled(" [Y]es ", yes_style), Span::raw("   /   "), Span::styled(" [N]o ", no_style) ]) ];
                let p = Paragraph::new(content).alignment(Alignment::Center).block(popup_block.title(" Delete Confirmation "));
                f.render_widget(p, area);
            }
            AppMode::AddMainStep(step) | AppMode::AddAppStep(step) => {
                let area = centered_rect(60, 20, size); f.render_widget(Clear, area); 
                let title = match step { AddField::Name => " Add New App - Step 1: Name ", AddField::Desc => " Add New App - Step 2: Description ", AddField::Cmd => " Add New App - Step 3: Command " };
                let label = match step { AddField::Name => " Enter Name: ", AddField::Desc => " Enter Description: ", AddField::Cmd => " Enter System Command: " };
                let content = vec![Line::from(""), Line::from(vec![ Span::styled(label, inactive_green), Span::raw(app.input_buffer.clone()), Span::raw("█") ])];
                let p = Paragraph::new(content).block(popup_block.title(title));
                f.render_widget(p, area);
            }
            _ => {}
        }
    }
}
