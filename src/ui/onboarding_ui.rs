use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

use super::centered_rect;
use crate::types::{AppMode, MenuApp};

/// Draws whichever onboarding screen is currently active.
pub fn draw_onboarding(f: &mut Frame, app: &mut MenuApp, size: Rect) {
    let theme = &app.themes[app.current_theme];
    let active = Style::default()
        .bg(theme.highlight_bg)
        .fg(theme.highlight_fg)
        .add_modifier(Modifier::BOLD);
    let accent_color = theme.text_accent;
    let normal_fg = theme.text_normal;

    let border_color = theme.focus_border;

    match app.mode {
        AppMode::OnboardingStart => {
            let area = centered_rect(60, 40, size);
            f.render_widget(Clear, area);
            let content = vec![
                Line::from(""),
                Line::from(Span::styled(
                    "  Welcome to TUI Control Center!",
                    Style::default().fg(accent_color).add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from("  This wizard will help you set up:"),
                Line::from("    • Your color theme"),
                Line::from("    • Your preferred layout"),
                Line::from("    • Pre-install useful TUI apps"),
                Line::from(""),
                Line::from(format!("  Detected distro: {}", app.distro_id)),
                Line::from(""),
            ];
            let p = Paragraph::new(content)
                .block(make_popup_block(border_color," 🚀 Welcome to TUI Center! "))
                .style(Style::default().fg(normal_fg));
            f.render_widget(p, area);
            render_hint(f, area, "Enter: Begin  │  Esc: Skip", accent_color);
        }

        AppMode::OnboardingChaoticAur => {
            let area = centered_rect(55, 35, size);
            f.render_widget(Clear, area);
            let items = vec![
                ListItem::new("  Yes — Enable Chaotic AUR"),
                ListItem::new("  No  — Use AUR helper instead"),
            ];
            let list = List::new(items)
                .block(make_popup_block(border_color," Chaotic AUR "))
                .highlight_style(active)
                .highlight_symbol("▸ ");
            f.render_stateful_widget(list, area, &mut app.options_state);
            render_hint(f, area, "↑↓ Navigate  │  Enter: Select  │  Esc: Back", accent_color);
        }

        AppMode::OnboardingAurHelper => {
            let area = centered_rect(55, 40, size);
            f.render_widget(Clear, area);
            let items: Vec<ListItem> = app.aur_helper_choices.iter()
                .map(|name| ListItem::new(format!("  {}", name)))
                .collect();
            let list = List::new(items)
                .block(make_popup_block(border_color," Select AUR Helper "))
                .highlight_style(active)
                .highlight_symbol("▸ ");
            f.render_stateful_widget(list, area, &mut app.options_state);

            let hint_text = if let Some(ref helper) = app.aur_helper {
                format!("Detected: {}  │  Enter: Confirm  │  Esc: Back", helper)
            } else {
                "↑↓ Navigate  │  Enter: Install  │  Esc: Back".to_string()
            };
            render_hint(f, area, &hint_text, accent_color);
        }

        AppMode::OnboardingTheme => {
            let area = centered_rect(40, 45, size);
            f.render_widget(Clear, area);
            let items: Vec<ListItem> = app.themes.iter()
                .map(|t| ListItem::new(format!("  {}  ", t.name)))
                .collect();
            let list = List::new(items)
                .block(make_popup_block(border_color," 🎨 Select Your Theme "))
                .highlight_style(active)
                .highlight_symbol("▸ ");
            f.render_stateful_widget(list, area, &mut app.theme_state);
            render_hint(f, area, "↑↓ Navigate  │  Enter: Apply  │  Tab: Next →  │  Esc: Back", accent_color);
        }

        AppMode::OnboardingLayout => {
            let area = centered_rect(50, 45, size);
            f.render_widget(Clear, area);
            let items: Vec<ListItem> = app.layouts.iter()
                .map(|l| ListItem::new(format!("  {} — {}", l.name, l.description)))
                .collect();
            let list = List::new(items)
                .block(make_popup_block(border_color," 📐 Configure Layout "))
                .highlight_style(active)
                .highlight_symbol("▸ ");
            f.render_stateful_widget(list, area, &mut app.options_state);
            render_hint(f, area, "↑↓ Navigate  │  Enter: Apply  │  Tab: Next →  │  Esc: Back", accent_color);
        }

        AppMode::OnboardingApps => {
            let area = centered_rect(65, 70, size);
            f.render_widget(Clear, area);
            let items: Vec<ListItem> = app.suggested_apps.iter().map(|a| {
                let checkbox = if a.selected { "✓" } else { "✗" };
                let note = if a.repo_note.is_empty() { String::new() } else { format!(" ({})", a.repo_note) };
                let bar_tag = if a.is_appbar { " [AppBar]" } else { "" };
                let fg = if a.selected { normal_fg } else { theme.unfocus_border };
                ListItem::new(format!(" [{}] {}: {}{}{}", checkbox, a.name, a.description, bar_tag, note))
                    .style(Style::default().fg(fg))
            }).collect();
            let list = List::new(items)
                .block(make_popup_block(border_color," 📦 Select Apps "))
                .highlight_style(active)
                .highlight_symbol("▸ ");
            f.render_stateful_widget(list, area, &mut app.options_state);
            render_hint(f, area, "Space: Toggle  │  Enter: Finish & Continue →  │  Esc: Back", accent_color);
        }

        AppMode::OnboardingComplete => {
            let area = centered_rect(60, 35, size);
            f.render_widget(Clear, area);
            let content = vec![
                Line::from(""),
                Line::from(Span::styled(
                    "  ✓ Setup finished!",
                    Style::default().fg(accent_color).add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(format!("  Theme:      {}", app.themes[app.current_theme].name)),
                Line::from(format!("  Layout:     {}", app.layouts[app.current_layout].name)),
                Line::from(format!("  Apps added: {}", app.suggested_apps.iter().filter(|a| a.selected).count())),
                Line::from(""),
            ];
            let p = Paragraph::new(content)
                .block(make_popup_block(border_color," ✅ Setup Complete! "))
                .style(Style::default().fg(normal_fg));
            f.render_widget(p, area);
            render_hint(f, area, "Press any key to start using TUI Center...", accent_color);
        }

        AppMode::OnboardingInstalling => {
            let area = centered_rect(55, 30, size);
            f.render_widget(Clear, area);
            let content = vec![
                Line::from(""),
                Line::from("  Installation progress:"),
                Line::from(format!("  {}", app.install_status)),
                Line::from(""),
            ];
            let p = Paragraph::new(content)
                .block(make_popup_block(border_color," ⏳ Installing Apps "))
                .style(Style::default().fg(normal_fg));
            f.render_widget(p, area);
            render_hint(f, area, "Press any key to continue...", accent_color);
        }

        _ => {}
    }
}

/// Creates a standard popup block with the given border color and title.
fn make_popup_block<'a>(border_color: Color, title: &'a str) -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(Color::Black))
        .title(title)
}

/// Renders a one-line hint bar inside the bottom of the popup area.
fn render_hint(f: &mut Frame, area: Rect, text: &str, color: ratatui::style::Color) {
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
