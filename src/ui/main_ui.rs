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

/// Draws the main dashboard (status bar, workspace, app bar) and any overlay popups.
pub fn draw_main(f: &mut Frame, sys: &System, app: &mut MenuApp, size: Rect) {
    // Copy theme-derived styles upfront to avoid borrow conflicts
    let theme_idx = app.current_theme;
    let active_style = Style::default()
        .bg(app.themes[theme_idx].highlight_bg)
        .fg(app.themes[theme_idx].highlight_fg)
        .add_modifier(Modifier::BOLD);
    let inactive_accent = Style::default().fg(app.themes[theme_idx].text_accent);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .split(size);

    // ── Status bar ──────────────────────────────────────────────────────
    draw_status_bar(f, sys, app, chunks[0], active_style, inactive_accent);

    // ── Workspace + SysInfo ─────────────────────────────────────────────
    draw_workspace(f, app, chunks[1], active_style);

    // ── App bar ─────────────────────────────────────────────────────────
    draw_app_bar(f, app, chunks[2], active_style);

    // ── Persistent key hints footer ─────────────────────────────────────
    draw_footer(f, app, chunks[3]);

    // ── Overlay popups ──────────────────────────────────────────────────
    if app.mode != AppMode::Normal {
        draw_popup(f, app, size, active_style);
    }
}

// ── Status bar ──────────────────────────────────────────────────────────────

fn draw_status_bar(
    f: &mut Frame,
    sys: &System,
    app: &MenuApp,
    area: Rect,
    active: Style,
    inactive: Style,
) {
    let theme = &app.themes[app.current_theme];
    let border_color = if app.focus == FocusPane::StatusBar {
        theme.focus_border
    } else {
        theme.unfocus_border
    };

    let time = Local::now().format("%I:%M %p").to_string();
    let used_mem = sys.used_memory() as f64 / 1_073_741_824.0;
    let total_mem = sys.total_memory() as f64 / 1_073_741_824.0;
    let up_secs = System::uptime();
    let uptime_str = format!("{}h {}m", up_secs / 3600, (up_secs % 3600) / 60);

    let style_for = |idx| {
        if app.focus == FocusPane::StatusBar && app.status_index == idx { active } else { inactive }
    };

    let status_line = Line::from(vec![
        Span::styled(format!(" 🕒 {} ", time), style_for(0)),
        Span::raw("  |  "),
        Span::styled(format!(" 💾 {:.2} GiB / {:.2} GiB ", used_mem, total_mem), style_for(1)),
        Span::raw("  |  "),
        Span::styled(format!(" ⏱ Up: {} ", uptime_str), style_for(2)),
        Span::raw("  |  "),
        Span::styled(format!(" 🎨 Theme: {} ", theme.name), style_for(3)),
        Span::raw("  |  "),
        Span::styled(
            if app.show_sys_info { " 🖥 Hide SysInfo " } else { " 🖥 Show SysInfo " },
            style_for(4),
        ),
    ]);

    let panel = Paragraph::new(status_line)
        .alignment(Alignment::Right)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color))
                .title(" Status Bar (Tab to switch panes) "),
        );
    f.render_widget(panel, area);
}

// ── Workspace ───────────────────────────────────────────────────────────────

fn draw_workspace(
    f: &mut Frame,
    app: &mut MenuApp,
    area: Rect,
    active_style: Style,
) {
    let theme = &app.themes[app.current_theme];
    let work_border = if app.focus == FocusPane::Workspace {
        theme.focus_border
    } else {
        theme.unfocus_border
    };

    let work_chunks = if app.show_sys_info {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
            .split(area)
    } else {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(area)
    };

    let items: Vec<ListItem> = app
        .items
        .iter()
        .map(|item| {
            ListItem::new(format!("{:<20} │ {}", item.name, item.desc))
                .style(Style::default().fg(theme.text_normal))
        })
        .collect();

    let highlight = if app.focus == FocusPane::Workspace {
        active_style
    } else {
        Style::default().fg(theme.unfocus_border)
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(work_border))
                .title(" Main Workspace (F1 for help, Esc for settings) "),
        )
        .highlight_style(highlight)
        .highlight_symbol(">> ");
    f.render_stateful_widget(list, work_chunks[0], &mut app.state);

    if app.show_sys_info {
        let sys_lines: Vec<Line> = app.sys_info_text.lines().map(Line::from).collect();
        let sys_block = Paragraph::new(sys_lines)
            .style(Style::default().fg(theme.text_normal))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme.unfocus_border))
                    .title(" System Info "),
            );
        f.render_widget(sys_block, work_chunks[1]);
    }
}

// ── App bar ─────────────────────────────────────────────────────────────────

fn draw_app_bar(
    f: &mut Frame,
    app: &MenuApp,
    area: Rect,
    active_style: Style,
) {
    let theme = &app.themes[app.current_theme];
    let app_border = if app.focus == FocusPane::AppBar {
        theme.focus_border
    } else {
        theme.unfocus_border
    };
    let inactive_green = Style::default().fg(Color::Green);

    let mut spans = Vec::new();
    for (i, item) in app.app_bar_items.iter().enumerate() {
        let style = if app.focus == FocusPane::AppBar && app.app_bar_index == i {
            active_style
        } else {
            inactive_green
        };
        spans.push(Span::styled(format!(" [{}] {} ", i + 1, item.name), style));
        if i < app.app_bar_items.len() - 1 {
            spans.push(Span::raw("   "));
        }
    }

    let bar = Paragraph::new(Line::from(spans))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app_border))
                .title(" App Bar (Press 1-9 to launch) "),
        );
    f.render_widget(bar, area);
}

// ── Persistent footer ───────────────────────────────────────────────────────

fn draw_footer(f: &mut Frame, app: &MenuApp, area: Rect) {
    let theme = &app.themes[app.current_theme];
    let dim = Style::default().fg(theme.unfocus_border);
    let accent = Style::default().fg(theme.text_accent);

    let hints = match app.focus {
        FocusPane::StatusBar => Line::from(vec![
            Span::styled(" ←→", accent), Span::styled(" Navigate  ", dim),
            Span::styled("Enter", accent), Span::styled(" Select  ", dim),
            Span::styled("Tab", accent), Span::styled(" Switch pane  ", dim),
            Span::styled("Esc", accent), Span::styled(" Settings  ", dim),
            Span::styled("?", accent), Span::styled(" Help  ", dim),
            Span::styled("q", accent), Span::styled(" Quit", dim),
        ]),
        FocusPane::Workspace => Line::from(vec![
            Span::styled(" ↑↓", accent), Span::styled(" Navigate  ", dim),
            Span::styled("Enter", accent), Span::styled(" Launch  ", dim),
            Span::styled("Tab", accent), Span::styled(" Switch pane  ", dim),
            Span::styled("Esc", accent), Span::styled(" Settings  ", dim),
            Span::styled("?", accent), Span::styled(" Help  ", dim),
            Span::styled("q", accent), Span::styled(" Quit", dim),
        ]),
        FocusPane::AppBar => Line::from(vec![
            Span::styled(" ←→", accent), Span::styled(" Navigate  ", dim),
            Span::styled("1-9", accent), Span::styled(" Launch  ", dim),
            Span::styled("Tab", accent), Span::styled(" Switch pane  ", dim),
            Span::styled("Esc", accent), Span::styled(" Settings  ", dim),
            Span::styled("?", accent), Span::styled(" Help  ", dim),
            Span::styled("q", accent), Span::styled(" Quit", dim),
        ]),
    };

    let footer = Paragraph::new(hints).alignment(Alignment::Center);
    f.render_widget(footer, area);
}

// ── Popups (non-onboarding) ─────────────────────────────────────────────────


fn draw_popup(f: &mut Frame, app: &mut MenuApp, size: Rect, active_style: Style) {
    let theme = &app.themes[app.current_theme];
    let popup_border_style = Style::default().fg(theme.focus_border);
    let popup_block = Block::default()
        .borders(Borders::ALL)
        .border_style(popup_border_style)
        .style(Style::default().bg(Color::Black));
    let inactive_green = Style::default().fg(Color::Green);

    match app.mode {
        AppMode::ThemePopup => {
            let area = centered_rect(30, 40, size);
            f.render_widget(Clear, area);
            let items: Vec<ListItem> = app
                .themes
                .iter()
                .map(|t| {
                    ListItem::new(format!("  {}  ", t.name))
                        .style(Style::default().fg(theme.text_normal))
                })
                .collect();
            let list = List::new(items)
                .block(popup_block.title(" Select Theme "))
                .highlight_style(active_style)
                .highlight_symbol(">> ");
            f.render_stateful_widget(list, area, &mut app.theme_state);
        }

        AppMode::Quitting => {
            let area = centered_rect(40, 20, size);
            f.render_widget(Clear, area);
            let (yes_style, no_style) = if app.quit_index == 1 {
                (active_style, inactive_green)
            } else {
                (inactive_green, active_style)
            };
            let content = vec![
                Line::from(""),
                Line::from("Are you sure you want to quit?"),
                Line::from(""),
                Line::from(vec![
                    Span::styled(" [Y]es ", yes_style),
                    Span::raw("   /   "),
                    Span::styled(" [N]o ", no_style),
                ]),
            ];
            let p = Paragraph::new(content)
                .alignment(Alignment::Center)
                .block(popup_block.title(" Exit Confirmation "));
            f.render_widget(p, area);
        }

        AppMode::HelpPopup => {
            let area = centered_rect(60, 60, size);
            f.render_widget(Clear, area);
            let help_lines = vec![
                Line::from(""),
                Line::from(Span::styled(
                    "──── Navigation ────",
                    Style::default().fg(theme.text_accent).add_modifier(Modifier::BOLD),
                )),
                Line::from("  Tab          Switch pane (Status → Workspace → AppBar)"),
                Line::from("  ↑↓ / j k     Navigate lists"),
                Line::from("  ←→ / h l     Navigate status bar & app bar"),
                Line::from("  Enter        Launch app / Confirm action"),
                Line::from("  1-9          Launch app bar shortcut"),
                Line::from(""),
                Line::from(Span::styled(
                    "──── Actions ────",
                    Style::default().fg(theme.text_accent).add_modifier(Modifier::BOLD),
                )),
                Line::from("  Esc          Open settings menu"),
                Line::from("  q            Quit (with confirmation)"),
                Line::from("  t            Cycle theme"),
                Line::from("  f            Toggle system info panel"),
                Line::from("  ? / F1       Show this help"),
                Line::from(""),
                Line::from(Span::styled(
                    "──── Universal ────",
                    Style::default().fg(theme.text_accent).add_modifier(Modifier::BOLD),
                )),
                Line::from("  Esc          Go back / Cancel"),
                Line::from("  Enter        Confirm / Select"),
                Line::from("  Backspace    Go back (same as Esc)"),
                Line::from(""),
                Line::from(Span::styled(
                    "  Press Esc to close",
                    Style::default().fg(theme.text_accent),
                )),
            ];
            let p = Paragraph::new(help_lines)
                .style(Style::default().fg(theme.text_normal))
                .block(popup_block.title(" Keyboard Shortcuts "));
            f.render_widget(p, area);
        }

        AppMode::OptionsPopup => {
            let area = centered_rect(50, 50, size);
            f.render_widget(Clear, area);
            let def_sys_text = if app.default_show_sys_info {
                "[X] Default Show SysInfo"
            } else {
                "[ ] Default Show SysInfo"
            };
            let toggle_sys_text = if app.show_sys_info {
                "[-] Toggle SysInfo (F)"
            } else {
                "[+] Toggle SysInfo (F)"
            };
            let items = vec![
                ListItem::new(" Customize Main Workspace Apps -> "),
                ListItem::new(" Customize App Bar Apps -> "),
                ListItem::new(toggle_sys_text),
                ListItem::new(def_sys_text),
                ListItem::new(" Run Onboarding Setup -> "),
                ListItem::new(" <- Back "),
            ];
            let list = List::new(items)
                .block(popup_block.title(" Settings "))
                .highlight_style(active_style)
                .highlight_symbol(">> ");
            f.render_stateful_widget(list, area, &mut app.options_state);
        }

        AppMode::EditMain | AppMode::EditApp => {
            let is_main = app.mode == AppMode::EditMain;
            let area = centered_rect(70, 60, size);
            f.render_widget(Clear, area);
            let items_vec = if is_main { &app.items } else { &app.app_bar_items };
            let list_items: Vec<ListItem> = items_vec
                .iter()
                .map(|item| ListItem::new(format!("{:<20} │ {}", item.name, item.desc)))
                .collect();
            let title = if is_main {
                " Customize Main Apps ([a]dd / [d]elete, Esc to back) "
            } else {
                " Customize App Bar ([a]dd / [d]elete, Esc to back) "
            };
            let list = List::new(list_items)
                .block(popup_block.title(title))
                .highlight_style(active_style)
                .highlight_symbol(">> ");
            f.render_stateful_widget(list, area, &mut app.options_state);
        }

        AppMode::DeleteConfirmMain | AppMode::DeleteConfirmApp => {
            let is_main = app.mode == AppMode::DeleteConfirmMain;
            let app_name = if is_main {
                app.items[app.options_index].name.clone()
            } else {
                app.app_bar_items[app.options_index].name.clone()
            };
            let area = centered_rect(40, 20, size);
            f.render_widget(Clear, area);
            let (yes_style, no_style) = if app.quit_index == 1 {
                (active_style, inactive_green)
            } else {
                (inactive_green, active_style)
            };
            let content = vec![
                Line::from(""),
                Line::from(format!("Delete app '{}'?", app_name)),
                Line::from(""),
                Line::from(vec![
                    Span::styled(" [Y]es ", yes_style),
                    Span::raw("   /   "),
                    Span::styled(" [N]o ", no_style),
                ]),
            ];
            let p = Paragraph::new(content)
                .alignment(Alignment::Center)
                .block(popup_block.title(" Delete Confirmation "));
            f.render_widget(p, area);
        }

        AppMode::AddMainStep(step) | AppMode::AddAppStep(step) => {
            let area = centered_rect(60, 20, size);
            f.render_widget(Clear, area);
            let title = match step {
                AddField::Name => " Add New App - Step 1: Name ",
                AddField::Desc => " Add New App - Step 2: Description ",
                AddField::Cmd => " Add New App - Step 3: Command ",
            };
            let label = match step {
                AddField::Name => " Enter Name: ",
                AddField::Desc => " Enter Description: ",
                AddField::Cmd => " Enter System Command: ",
            };
            let content = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled(label, inactive_green),
                    Span::raw(app.input_buffer.clone()),
                    Span::raw("█"),
                ]),
            ];
            let p = Paragraph::new(content).block(popup_block.title(title));
            f.render_widget(p, area);
        }

        _ => {}
    }
}
