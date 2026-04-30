use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};
use sysinfo::System;
use chrono::Local;

use super::centered_rect;
use crate::types::*;

// ═══════════════════════════════════════════════════════════════════════════
//  SHARED HELPERS
// ═══════════════════════════════════════════════════════════════════════════

/// Derive all theme-based styles once to avoid repeated indexing.
struct UiStyles {
    active: Style,
    accent: Style,
    dim: Style,
    normal_fg: Color,
    focus_border: Color,
    unfocus_border: Color,
}

impl UiStyles {
    fn from_app(app: &MenuApp) -> Self {
        let t = &app.themes[app.current_theme];
        Self {
            active: Style::default()
                .bg(t.highlight_bg)
                .fg(t.highlight_fg)
                .add_modifier(Modifier::BOLD),
            accent: Style::default().fg(t.text_accent),
            dim: Style::default().fg(t.unfocus_border),
            normal_fg: t.text_normal,
            focus_border: t.focus_border,
            unfocus_border: t.unfocus_border,
        }
    }

    /// Standard popup block with focused border.
    fn popup_block<'a>(&self, title: &'a str) -> Block<'a> {
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.focus_border))
            .style(Style::default().bg(Color::Black))
            .title(title)
    }

    /// Yes/No button pair with the cursor on the given side (0=No, 1=Yes).
    fn yes_no_spans(&self, cursor: usize) -> Vec<Span<'_>> {
        let (yes_s, no_s) = if cursor == 1 {
            (self.active, Style::default().fg(Color::Green))
        } else {
            (Style::default().fg(Color::Green), self.active)
        };
        vec![
            Span::styled(" [Y]es ", yes_s),
            Span::raw("   /   "),
            Span::styled(" [N]o ", no_s),
        ]
    }
}

/// Renders a one-line hint bar at the bottom edge of the given area.
fn render_hint(f: &mut Frame, area: Rect, text: &str, color: Color) {
    let hint_area = Rect {
        x: area.x + 1,
        y: area.y + area.height.saturating_sub(2),
        width: area.width.saturating_sub(2),
        height: 1,
    };
    let hint = Paragraph::new(text)
        .style(Style::default().fg(color))
        .alignment(Alignment::Center);
    f.render_widget(hint, hint_area);
}

// ═══════════════════════════════════════════════════════════════════════════
//  TOP-LEVEL ENTRY
// ═══════════════════════════════════════════════════════════════════════════

/// Draws the main dashboard (status bar, workspace, app bar) and any overlay popups.
pub fn draw_main(f: &mut Frame, sys: &System, app: &mut MenuApp, size: Rect) {
    let s = UiStyles::from_app(app);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .split(size);

    draw_status_bar(f, sys, app, chunks[0], &s);
    draw_workspace(f, app, chunks[1], &s);
    draw_app_bar(f, app, chunks[2], &s);
    draw_footer(f, app, chunks[3], &s);

    if app.mode != AppMode::Normal {
        draw_popup(f, app, size, &s);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
//  STATUS BAR
// ═══════════════════════════════════════════════════════════════════════════

fn draw_status_bar(f: &mut Frame, sys: &System, app: &MenuApp, area: Rect, s: &UiStyles) {
    let border_color = if app.focus == FocusPane::StatusBar { s.focus_border } else { s.unfocus_border };

    let style_for = |idx: usize| -> Style {
        if app.focus == FocusPane::StatusBar && app.status_index == idx { s.active } else { s.accent }
    };

    let active_modules: Vec<_> = app.config.status_modules.iter().filter(|(_, v)| *v).collect();
    let n = active_modules.len();
    let theme = &app.themes[app.current_theme];

    let mut spans = Vec::new();
    for (i, (module, _)) in active_modules.iter().enumerate() {
        let text = match module {
            StatusModule::Greeting => format!(" 👋 {}, @{} ", app.greeting_text, app.user_name),
            StatusModule::Time     => format!(" 🕒 {} ", Local::now().format("%I:%M %p")),
            StatusModule::Memory   => format!(" 💾 {:.1}/{:.1} GiB ", sys.used_memory() as f64 / 1_073_741_824.0, sys.total_memory() as f64 / 1_073_741_824.0),
            StatusModule::Uptime   => format!(" ⏱ {}h{}m ", System::uptime() / 3600, (System::uptime() % 3600) / 60),
            StatusModule::Theme    => format!(" 🎨 {} ", theme.name),
            StatusModule::SysInfoToggle => if app.show_sys_info { " 🖥 SysInfo ✓ ".into() } else { " 🖥 SysInfo ✗ ".into() },
            StatusModule::Audio    => format!(" {} ", app.audio_vol),
            StatusModule::Network  => format!(" {} ", app.network_info),
            StatusModule::Power    => format!(" {} ", app.battery_info),
        };
        spans.push(Span::styled(text, style_for(i)));
        if i < n - 1 { spans.push(Span::raw(" │ ")); }
    }

    let panel = Paragraph::new(Line::from(spans))
        .alignment(Alignment::Right)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .title(" Status Bar "));
    f.render_widget(panel, area);
}

// ═══════════════════════════════════════════════════════════════════════════
//  WORKSPACE
// ═══════════════════════════════════════════════════════════════════════════

fn draw_workspace(f: &mut Frame, app: &mut MenuApp, area: Rect, s: &UiStyles) {
    let border = if app.focus == FocusPane::Workspace { s.focus_border } else { s.unfocus_border };

    let work_chunks = if app.show_sys_info {
        Layout::default().direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
            .split(area)
    } else {
        Layout::default().direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(area)
    };

    let items: Vec<ListItem> = app.items.iter()
        .map(|item| ListItem::new(format!("  {:<20} │ {}", item.name, item.desc))
            .style(Style::default().fg(s.normal_fg)))
        .collect();

    let highlight = if app.focus == FocusPane::Workspace { s.active } else { Style::default().fg(s.unfocus_border) };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL)
            .border_style(Style::default().fg(border))
            .title(" Workspace "))
        .highlight_style(highlight)
        .highlight_symbol("▸ ");
    f.render_stateful_widget(list, work_chunks[0], &mut app.state);

    if app.show_sys_info {
        let sys_lines: Vec<Line> = app.sys_info_text.lines().map(Line::from).collect();
        let sys_block = Paragraph::new(sys_lines)
            .style(Style::default().fg(s.normal_fg))
            .block(Block::default().borders(Borders::ALL)
                .border_style(Style::default().fg(s.unfocus_border))
                .title(" System Info "));
        f.render_widget(sys_block, work_chunks[1]);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
//  APP BAR
// ═══════════════════════════════════════════════════════════════════════════

fn draw_app_bar(f: &mut Frame, app: &MenuApp, area: Rect, s: &UiStyles) {
    let border = if app.focus == FocusPane::AppBar { s.focus_border } else { s.unfocus_border };

    let mut spans = Vec::new();
    for (i, item) in app.app_bar_items.iter().enumerate() {
        let style = if app.focus == FocusPane::AppBar && app.app_bar_index == i { s.active }
                    else { Style::default().fg(Color::Green) };
        spans.push(Span::styled(format!(" [{}] {} ", i + 1, item.name), style));
        if i < app.app_bar_items.len() - 1 { spans.push(Span::raw("  ")); }
    }

    let bar = Paragraph::new(Line::from(spans))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL)
            .border_style(Style::default().fg(border))
            .title(" App Bar "));
    f.render_widget(bar, area);
}

// ═══════════════════════════════════════════════════════════════════════════
//  FOOTER — context-aware key hints
// ═══════════════════════════════════════════════════════════════════════════

fn draw_footer(f: &mut Frame, app: &MenuApp, area: Rect, s: &UiStyles) {
    let hints: Vec<(&str, &str)> = match app.mode {
        AppMode::Normal => match app.focus {
            FocusPane::StatusBar => vec![("←→","Navigate"),("Enter","Action"),("Tab","Pane"),("Esc","Settings"),("?","Help"),("q","Quit")],
            FocusPane::Workspace => vec![("↑↓","Navigate"),("Enter","Launch"),("Tab","Pane"),("Esc","Settings"),("?","Help"),("q","Quit")],
            FocusPane::AppBar    => vec![("←→","Navigate"),("1-9","Launch"),("Tab","Pane"),("Esc","Settings"),("?","Help"),("q","Quit")],
        },
        AppMode::OptionsPopup => vec![("↑↓","Navigate"),("Enter","Select"),("Esc","Close")],
        AppMode::EditMain | AppMode::EditApp => vec![("↑↓","Navigate"),("a","Add"),("d","Delete"),("Esc","Back")],
        AppMode::CustomizingStatusBar => vec![("↑↓","Navigate"),("Space","Toggle"),("Shift+J/K","Reorder"),("Esc","Back")],
        AppMode::ImportExportMenu => vec![("↑↓","Navigate"),("Enter","Select"),("Esc","Back")],
        AppMode::CheatBrowser => vec![("↑↓","Navigate"),("Enter","Import"),("Esc","Back")],
        AppMode::ThemePopup => vec![("↑↓","Navigate"),("Enter","Apply"),("Esc","Cancel")],
        _ => vec![("Esc","Back"),("Enter","Confirm")],
    };

    let mut spans = Vec::new();
    for (key, desc) in hints {
        spans.push(Span::styled(format!(" {}", key), s.accent));
        spans.push(Span::styled(format!(" {}  ", desc), s.dim));
    }

    let footer = Paragraph::new(Line::from(spans)).alignment(Alignment::Center);
    f.render_widget(footer, area);
}

// ═══════════════════════════════════════════════════════════════════════════
//  POPUPS — single unified dispatcher
// ═══════════════════════════════════════════════════════════════════════════

fn draw_popup(f: &mut Frame, app: &mut MenuApp, size: Rect, s: &UiStyles) {
    match app.mode {
        // ── Settings ────────────────────────────────────────────────────
        AppMode::OptionsPopup => {
            let area = centered_rect(50, 55, size);
            f.render_widget(Clear, area);

            let sys_toggle = if app.show_sys_info { "  ✓ SysInfo visible" } else { "  ✗ SysInfo hidden" };
            let sys_default = if app.default_show_sys_info { "  [X] Show SysInfo on launch" } else { "  [ ] Show SysInfo on launch" };

            let items = vec![
                ListItem::new(""),
                ListItem::new(Span::styled("── Workspace ──", s.accent)),
                ListItem::new("  Edit Main Workspace Apps →"),
                ListItem::new("  Edit App Bar Apps →"),
                ListItem::new(""),
                ListItem::new(Span::styled("── Display ──", s.accent)),
                ListItem::new(sys_toggle),
                ListItem::new(sys_default),
                ListItem::new("  Customize Top Bar →"),
                ListItem::new(""),
                ListItem::new(Span::styled("── Data ──", s.accent)),
                ListItem::new("  Import / Export .cheat →"),
                ListItem::new(""),
                ListItem::new(Span::styled("── System ──", s.accent)),
                ListItem::new("  Re-run Onboarding Wizard →"),
                ListItem::new(""),
                ListItem::new("  ← Close Settings"),
            ];

            // Map visual index → action index (skip non-selectable rows)
            let selectable: Vec<usize> = vec![2, 3, 6, 7, 8, 11, 14, 16];
            let sel_visual = selectable.get(app.options_index).copied().unwrap_or(2);

            let mut state = ratatui::widgets::ListState::default();
            state.select(Some(sel_visual));

            let list = List::new(items)
                .block(s.popup_block(" ⚙ Settings "))
                .highlight_style(s.active)
                .highlight_symbol("▸ ");
            f.render_stateful_widget(list, area, &mut state);
            render_hint(f, area, "↑↓ Navigate  │  Enter Select  │  Esc Close", s.focus_border);
        }

        // ── Theme Selector ──────────────────────────────────────────────
        AppMode::ThemePopup => {
            let area = centered_rect(30, 40, size);
            f.render_widget(Clear, area);
            let items: Vec<ListItem> = app.themes.iter()
                .map(|t| ListItem::new(format!("  {}  ", t.name)).style(Style::default().fg(s.normal_fg)))
                .collect();
            let list = List::new(items)
                .block(s.popup_block(" 🎨 Select Theme "))
                .highlight_style(s.active)
                .highlight_symbol("▸ ");
            f.render_stateful_widget(list, area, &mut app.theme_state);
        }

        // ── Quit Confirmation ───────────────────────────────────────────
        AppMode::Quitting => {
            let area = centered_rect(40, 20, size);
            f.render_widget(Clear, area);
            let content = vec![
                Line::from(""),
                Line::from("  Are you sure you want to quit?"),
                Line::from(""),
                Line::from(s.yes_no_spans(app.quit_index)),
            ];
            let p = Paragraph::new(content).alignment(Alignment::Center)
                .block(s.popup_block(" Exit Confirmation "));
            f.render_widget(p, area);
        }

        // ── Help ────────────────────────────────────────────────────────
        AppMode::HelpPopup => {
            let area = centered_rect(60, 65, size);
            f.render_widget(Clear, area);
            let section = |title: &str| -> Line {
                Line::from(Span::styled(format!("──── {} ────", title), s.accent))
            };
            let help_lines = vec![
                Line::from(""),
                section("Navigation"),
                Line::from("  Tab          Cycle panes (Status → Workspace → AppBar)"),
                Line::from("  ↑↓ / j k     Navigate lists"),
                Line::from("  ←→ / h l     Navigate status bar & app bar"),
                Line::from("  Enter        Launch app / Confirm action"),
                Line::from("  1-9          Launch app bar shortcut"),
                Line::from(""),
                section("Actions"),
                Line::from("  Esc          Open settings menu"),
                Line::from("  q            Quit (with confirmation)"),
                Line::from("  t            Cycle theme"),
                Line::from("  f            Toggle system info panel"),
                Line::from("  ? / F1       Show this help"),
                Line::from(""),
                section("Universal"),
                Line::from("  Esc          Go back / Cancel"),
                Line::from("  Enter        Confirm / Select"),
                Line::from("  Backspace    Go back (same as Esc)"),
                Line::from(""),
            ];
            let p = Paragraph::new(help_lines)
                .style(Style::default().fg(s.normal_fg))
                .block(s.popup_block(" ❓ Keyboard Shortcuts "));
            f.render_widget(p, area);
            render_hint(f, area, "Press Esc to close", s.focus_border);
        }

        // ── Edit Main / App Bar ─────────────────────────────────────────
        AppMode::EditMain | AppMode::EditApp => {
            let is_main = app.mode == AppMode::EditMain;
            let area = centered_rect(70, 60, size);
            f.render_widget(Clear, area);
            let items_vec = if is_main { &app.items } else { &app.app_bar_items };
            let list_items: Vec<ListItem> = items_vec.iter()
                .map(|item| ListItem::new(format!("  {:<20} │ {}", item.name, item.desc))
                    .style(Style::default().fg(s.normal_fg)))
                .collect();
            let title = if is_main { " ✏ Edit Workspace Apps " } else { " ✏ Edit App Bar " };
            let list = List::new(list_items)
                .block(s.popup_block(title))
                .highlight_style(s.active)
                .highlight_symbol("▸ ");
            f.render_stateful_widget(list, area, &mut app.options_state);
            render_hint(f, area, "a: Add  │  d: Delete  │  Esc: Back", s.focus_border);
        }

        // ── Delete Confirmation ─────────────────────────────────────────
        AppMode::DeleteConfirmMain | AppMode::DeleteConfirmApp => {
            let is_main = app.mode == AppMode::DeleteConfirmMain;
            let app_name = if is_main {
                app.items.get(app.options_index).map(|i| i.name.clone()).unwrap_or_default()
            } else {
                app.app_bar_items.get(app.options_index).map(|i| i.name.clone()).unwrap_or_default()
            };
            let area = centered_rect(40, 20, size);
            f.render_widget(Clear, area);
            let content = vec![
                Line::from(""),
                Line::from(format!("  Delete '{}'?", app_name)),
                Line::from(""),
                Line::from(s.yes_no_spans(app.quit_index)),
            ];
            let p = Paragraph::new(content).alignment(Alignment::Center)
                .block(s.popup_block(" Delete Confirmation "));
            f.render_widget(p, area);
        }

        // ── Add App Steps ───────────────────────────────────────────────
        AppMode::AddMainStep(step) | AppMode::AddAppStep(step) => {
            let area = centered_rect(60, 20, size);
            f.render_widget(Clear, area);
            let (title, label) = match step {
                AddField::Name => (" Add App — Step 1/3 ", "  Name: "),
                AddField::Desc => (" Add App — Step 2/3 ", "  Description: "),
                AddField::Cmd  => (" Add App — Step 3/3 ", "  Command: "),
            };
            let content = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled(label, Style::default().fg(Color::Green)),
                    Span::raw(&app.input_buffer),
                    Span::styled("█", Style::default().fg(s.focus_border)),
                ]),
            ];
            let p = Paragraph::new(content).block(s.popup_block(title));
            f.render_widget(p, area);
            render_hint(f, area, "Enter: Next  │  Esc: Cancel", s.focus_border);
        }

        // ── Import/Export Menu ───────────────────────────────────────────
        AppMode::ImportExportMenu => {
            let area = centered_rect(50, 30, size);
            f.render_widget(Clear, area);
            let items = vec![
                ListItem::new("  📥 Import .cheat file →"),
                ListItem::new("  📤 Export workspace to .cheat →"),
                ListItem::new("  ← Back"),
            ];
            let list = List::new(items)
                .block(s.popup_block(" Import / Export "))
                .highlight_style(s.active)
                .highlight_symbol("▸ ");
            f.render_stateful_widget(list, area, &mut app.options_state);

            if !app.cheat_status.is_empty() {
                render_hint(f, area, &app.cheat_status, s.focus_border);
            }
        }

        // ── Cheat File Browser ──────────────────────────────────────────
        AppMode::CheatBrowser => {
            let area = centered_rect(70, 60, size);
            f.render_widget(Clear, area);

            if app.cheat_files.is_empty() {
                let content = vec![
                    Line::from(""),
                    Line::from("  No .cheat files found in:"),
                    Line::from(format!("    • {}", crate::cheat::default_cheat_dir().display())),
                    Line::from(format!("    • {}", crate::cheat::tui_center_cheat_dir().display())),
                    Line::from(""),
                    Line::from("  Place .cheat files in either directory and try again."),
                ];
                let p = Paragraph::new(content)
                    .style(Style::default().fg(s.normal_fg))
                    .block(s.popup_block(" Import .cheat "));
                f.render_widget(p, area);
                render_hint(f, area, "Esc: Back", s.focus_border);
            } else {
                let items: Vec<ListItem> = app.cheat_files.iter().map(|path| {
                    let name = path.file_name().unwrap_or_default().to_string_lossy();
                    let dir = path.parent().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
                    ListItem::new(format!("  {}  ({})", name, dir))
                }).collect();
                let list = List::new(items)
                    .block(s.popup_block(" Select .cheat file "))
                    .highlight_style(s.active)
                    .highlight_symbol("▸ ");
                f.render_stateful_widget(list, area, &mut app.options_state);

                let hint = if app.cheat_status.is_empty() {
                    "↑↓ Navigate  │  Enter: Import  │  Esc: Back"
                } else {
                    &app.cheat_status
                };
                render_hint(f, area, hint, s.focus_border);
            }
        }

        // ── Export Confirmation ──────────────────────────────────────────
        AppMode::CheatExportConfirm => {
            let area = centered_rect(50, 25, size);
            f.render_widget(Clear, area);
            let export_path = crate::cheat::tui_center_cheat_dir().join("workspace.cheat");
            let content = vec![
                Line::from(""),
                Line::from(format!("  Export {} workspace commands to:", app.items.len())),
                Line::from(format!("  {}", export_path.display())),
                Line::from(""),
                Line::from(s.yes_no_spans(app.quit_index)),
            ];
            let p = Paragraph::new(content).alignment(Alignment::Center)
                .block(s.popup_block(" Export to .cheat "));
            f.render_widget(p, area);
        }

        // ── Customize Top Bar ───────────────────────────────────────────
        AppMode::CustomizingStatusBar => {
            let area = centered_rect(50, 55, size);
            f.render_widget(Clear, area);

            let items: Vec<ListItem> = app.config.status_modules.iter().map(|(module, visible)| {
                let icon = if *visible { "✓" } else { "✗" };
                let name = match module {
                    StatusModule::Greeting      => "👋 Greeting & User",
                    StatusModule::Time          => "🕒 Clock",
                    StatusModule::Memory        => "💾 RAM Usage",
                    StatusModule::Uptime        => "⏱ System Uptime",
                    StatusModule::Theme         => "🎨 Theme Selector",
                    StatusModule::SysInfoToggle => "🖥 SysInfo Toggle",
                    StatusModule::Audio         => "🔊 Audio Volume",
                    StatusModule::Network       => "🌐 Network Status",
                    StatusModule::Power         => "🔋 Battery / Power",
                };
                let fg = if *visible { s.normal_fg } else { s.unfocus_border };
                ListItem::new(format!("  [{}] {} ", icon, name)).style(Style::default().fg(fg))
            }).collect();

            let list = List::new(items)
                .block(s.popup_block(" Customize Top Bar "))
                .highlight_style(s.active)
                .highlight_symbol("▸ ");
            f.render_stateful_widget(list, area, &mut app.options_state);
            render_hint(f, area, "Space: Toggle  │  Shift+J/K: Reorder  │  Esc: Back", s.focus_border);
        }

        _ => {}
    }
}
