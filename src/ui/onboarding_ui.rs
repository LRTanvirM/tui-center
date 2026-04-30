use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

use super::centered_rect;
use crate::types::{AppMode, MenuApp};

/// Draws whichever onboarding screen is currently active.
pub fn draw_onboarding(f: &mut Frame, app: &mut MenuApp, size: Rect) {
    let theme = &app.themes[app.current_theme];

    match app.mode {
        AppMode::OnboardingStart => {
            let area = centered_rect(60, 40, size);
            f.render_widget(Clear, area);
            let border = Style::default().fg(theme.focus_border);
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(border)
                .title(" Welcome to TUI Center! ");
            let content = vec![
                Line::from(""),
                Line::from(Span::styled(
                    "  Welcome to TUI Control Center!",
                    Style::default().fg(theme.text_accent).add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from("  This wizard will help you set up:"),
                Line::from("    • Your color theme"),
                Line::from("    • Your preferred layout"),
                Line::from("    • Pre-install useful TUI apps"),
                Line::from(""),
                Line::from(format!("  Detected distro: {}", app.distro_id)),
                Line::from(""),
                Line::from(Span::styled(
                    "  Press Enter to begin  |  Esc to skip",
                    Style::default().fg(theme.text_accent),
                )),
            ];
            let p = Paragraph::new(content)
                .block(block)
                .style(Style::default().fg(theme.text_normal));
            f.render_widget(p, area);
        }

        AppMode::OnboardingChaoticAur => {
            let area = centered_rect(55, 40, size);
            f.render_widget(Clear, area);
            let border = Style::default().fg(theme.focus_border);
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(border)
                .title(" Chaotic AUR ");
            let options = vec!["Yes - Enable Chaotic AUR", "No  - Use AUR helper instead"];
            let items: Vec<ListItem> = options
                .iter()
                .map(|opt| ListItem::new(format!("  {}", opt)))
                .collect();
            let active = Style::default()
                .bg(theme.highlight_bg)
                .fg(theme.highlight_fg)
                .add_modifier(Modifier::BOLD);
            let list = List::new(items)
                .block(block)
                .highlight_style(active)
                .highlight_symbol(">> ");
            f.render_stateful_widget(list, area, &mut app.options_state);
            render_hint(
                f,
                area,
                "Chaotic AUR provides pre-built AUR packages. Enter to select.",
                theme.text_accent,
            );
        }

        AppMode::OnboardingAurHelper => {
            let area = centered_rect(55, 45, size);
            f.render_widget(Clear, area);
            let border = Style::default().fg(theme.focus_border);
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(border)
                .title(" Select AUR Helper ");
            let items: Vec<ListItem> = app
                .aur_helper_choices
                .iter()
                .map(|name| ListItem::new(format!("  {}", name)))
                .collect();
            let active = Style::default()
                .bg(theme.highlight_bg)
                .fg(theme.highlight_fg)
                .add_modifier(Modifier::BOLD);
            let list = List::new(items)
                .block(block)
                .highlight_style(active)
                .highlight_symbol(">> ");
            f.render_stateful_widget(list, area, &mut app.options_state);

            let hint_text = if app.aur_helper.is_some() {
                format!(
                    "Detected: {}. Press Enter to confirm or pick another.",
                    app.aur_helper.as_ref().unwrap()
                )
            } else {
                "No AUR helper found. Pick one to install.".to_string()
            };
            render_hint(f, area, &hint_text, theme.text_accent);
        }

        AppMode::OnboardingTheme => {
            let area = centered_rect(40, 50, size);
            f.render_widget(Clear, area);
            let border = Style::default().fg(theme.focus_border);
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(border)
                .title(" Select Your Theme ");
            let theme_items: Vec<ListItem> = app
                .themes
                .iter()
                .map(|t| ListItem::new(format!("  {}  ", t.name)))
                .collect();
            let active = Style::default()
                .bg(theme.highlight_bg)
                .fg(theme.highlight_fg)
                .add_modifier(Modifier::BOLD);
            let list = List::new(theme_items)
                .block(block)
                .highlight_style(active)
                .highlight_symbol(">> ");
            f.render_stateful_widget(list, area, &mut app.theme_state);
            render_hint(
                f,
                area,
                "↑↓ Navigate  |  Enter: Apply  |  Tab: Next →",
                theme.text_accent,
            );
        }

        AppMode::OnboardingLayout => {
            let area = centered_rect(50, 50, size);
            f.render_widget(Clear, area);
            let border = Style::default().fg(theme.focus_border);
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(border)
                .title(" Configure Layout ");
            let layout_items: Vec<ListItem> = app
                .layouts
                .iter()
                .map(|l| ListItem::new(format!("  {} - {}", l.name, l.description)))
                .collect();
            let active = Style::default()
                .bg(theme.highlight_bg)
                .fg(theme.highlight_fg)
                .add_modifier(Modifier::BOLD);
            let list = List::new(layout_items)
                .block(block)
                .highlight_style(active)
                .highlight_symbol(">> ");
            f.render_stateful_widget(list, area, &mut app.options_state);
            render_hint(
                f,
                area,
                "↑↓ Navigate  |  Enter: Apply  |  Tab: Next →",
                theme.text_accent,
            );
        }

        AppMode::OnboardingApps => {
            let area = centered_rect(60, 65, size);
            f.render_widget(Clear, area);
            let border = Style::default().fg(theme.focus_border);
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(border)
                .title(" Select Apps ");
            let app_items: Vec<ListItem> = app
                .suggested_apps
                .iter()
                .map(|a| {
                    let checkbox = if a.selected { "[X]" } else { "[ ]" };
                    let note = if a.repo_note.is_empty() {
                        String::new()
                    } else {
                        format!(" ({})", a.repo_note)
                    };
                    let bar_tag = if a.is_appbar { " [AppBar]" } else { "" };
                    ListItem::new(format!(
                        "{} {}: {}{}{}",
                        checkbox, a.name, a.description, bar_tag, note
                    ))
                })
                .collect();
            let active = Style::default()
                .bg(theme.highlight_bg)
                .fg(theme.highlight_fg)
                .add_modifier(Modifier::BOLD);
            let list = List::new(app_items)
                .block(block)
                .highlight_style(active)
                .highlight_symbol(">> ");
            f.render_stateful_widget(list, area, &mut app.options_state);
            render_hint(
                f,
                area,
                "Space: Toggle  |  Enter: Finish & Continue →",
                theme.text_accent,
            );
        }

        AppMode::OnboardingComplete => {
            let area = centered_rect(60, 40, size);
            f.render_widget(Clear, area);
            let border = Style::default().fg(theme.focus_border);
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(border)
                .title(" Setup Complete! ");
            let content = vec![
                Line::from(""),
                Line::from(Span::styled(
                    "  ✓ Setup finished!",
                    Style::default().fg(theme.text_accent).add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(format!("  Theme: {}", app.themes[app.current_theme].name)),
                Line::from(format!("  Layout: {}", app.layouts[app.current_layout].name)),
                Line::from(format!(
                    "  Apps added: {}",
                    app.suggested_apps.iter().filter(|a| a.selected).count()
                )),
                Line::from(""),
                Line::from("  Press any key to start using TUI Center..."),
            ];
            let p = Paragraph::new(content)
                .block(block)
                .style(Style::default().fg(theme.text_normal));
            f.render_widget(p, area);
        }

        AppMode::OnboardingInstalling => {
            let area = centered_rect(50, 30, size);
            f.render_widget(Clear, area);
            let border = Style::default().fg(theme.focus_border);
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(border)
                .title(" Installing Apps ");
            let content = vec![
                Line::from(""),
                Line::from("Installation progress:"),
                Line::from(app.install_status.clone()),
                Line::from(""),
                Line::from("Press any key to continue..."),
            ];
            let p = Paragraph::new(content)
                .block(block)
                .style(Style::default().fg(theme.text_normal));
            f.render_widget(p, area);
        }

        _ => {}
    }
}

/// Renders a one-line hint at the bottom edge of the given area.
fn render_hint(f: &mut Frame, area: Rect, text: &str, color: ratatui::style::Color) {
    let hint = Paragraph::new(text)
        .style(Style::default().fg(color))
        .alignment(Alignment::Center);
    let hint_area = Rect {
        x: area.x,
        y: area.y + area.height.saturating_sub(1),
        width: area.width,
        height: 1,
    };
    f.render_widget(hint, hint_area);
}
