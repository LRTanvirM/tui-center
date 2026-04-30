mod main_ui;
mod onboarding_ui;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};
use sysinfo::System;

use crate::types::{AppMode, MenuApp};

/// Top-level UI dispatcher.
pub fn ui(f: &mut Frame, sys: &System, app: &mut MenuApp) {
    let size = f.size();

    // Onboarding screens take over the entire viewport
    match app.mode {
        AppMode::OnboardingStart
        | AppMode::OnboardingChaoticAur
        | AppMode::OnboardingAurHelper
        | AppMode::OnboardingTheme
        | AppMode::OnboardingLayout
        | AppMode::OnboardingApps
        | AppMode::OnboardingComplete
        | AppMode::OnboardingInstalling => {
            onboarding_ui::draw_onboarding(f, app, size);
            return;
        }
        _ => {}
    }

    // Normal dashboard
    main_ui::draw_main(f, sys, app, size);
}

/// Creates a centered rectangle for popups.
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
