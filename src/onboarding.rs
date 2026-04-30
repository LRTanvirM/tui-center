use crossterm::event::KeyCode;
use std::process::Command as ProcessCommand;

use crate::types::*;

/// Handles key events for all onboarding-mode states.
/// Returns `true` if the event was consumed (mode was an onboarding mode), `false` otherwise.
pub fn handle_onboarding_key(app: &mut MenuApp, code: KeyCode) -> bool {
    match &app.mode {
        AppMode::OnboardingStart => {
            match code {
                KeyCode::Esc | KeyCode::Backspace => app.mode = AppMode::Normal,
                KeyCode::Enter => {
                    if app.is_arch {
                        app.options_index = 0;
                        app.options_state.select(Some(0));
                        app.mode = AppMode::OnboardingChaoticAur;
                    } else {
                        app.mode = AppMode::OnboardingTheme;
                        app.theme_state.select(Some(app.current_theme));
                    }
                }
                _ => {}
            }
            true
        }

        AppMode::OnboardingChaoticAur => {
            match code {
                // Back → previous step (OnboardingStart)
                KeyCode::Esc | KeyCode::Backspace => {
                    app.mode = AppMode::OnboardingStart;
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    app.chaotic_aur_index = if app.chaotic_aur_index == 0 { 1 } else { 0 };
                    app.options_state.select(Some(app.chaotic_aur_index));
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    app.chaotic_aur_index = if app.chaotic_aur_index == 1 { 0 } else { 1 };
                    app.options_state.select(Some(app.chaotic_aur_index));
                }
                KeyCode::Enter => {
                    if app.chaotic_aur_index == 0 {
                        // Yes – enable Chaotic AUR
                        app.chaotic_aur_enabled = true;
                        app.mode = AppMode::OnboardingTheme;
                        app.theme_state.select(Some(app.current_theme));
                    } else {
                        // No – use AUR helper
                        app.chaotic_aur_enabled = false;
                        if app.aur_helper.is_some() {
                            app.mode = AppMode::OnboardingTheme;
                            app.theme_state.select(Some(app.current_theme));
                        } else {
                            app.options_index = 0;
                            app.options_state.select(Some(0));
                            app.mode = AppMode::OnboardingAurHelper;
                        }
                    }
                }
                _ => {}
            }
            true
        }

        AppMode::OnboardingAurHelper => {
            match code {
                // Back → previous step (ChaoticAur)
                KeyCode::Esc | KeyCode::Backspace => {
                    app.chaotic_aur_index = 0;
                    app.options_state.select(Some(0));
                    app.mode = AppMode::OnboardingChaoticAur;
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if app.aur_helper_index == 0 {
                        app.aur_helper_index = app.aur_helper_choices.len() - 1;
                    } else {
                        app.aur_helper_index -= 1;
                    }
                    app.options_state.select(Some(app.aur_helper_index));
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    app.aur_helper_index =
                        (app.aur_helper_index + 1) % app.aur_helper_choices.len();
                    app.options_state.select(Some(app.aur_helper_index));
                }
                KeyCode::Enter => {
                    let chosen = app.aur_helper_choices[app.aur_helper_index].clone();
                    // Install the helper if not already present
                    let already = ProcessCommand::new("sh")
                        .arg("-c")
                        .arg(format!("command -v {}", &chosen))
                        .output()
                        .map(|o| o.status.success())
                        .unwrap_or(false);
                    if !already {
                        let ok = ProcessCommand::new("sudo")
                            .args(&["pacman", "-S", "--needed", "--noconfirm", &chosen])
                            .output()
                            .map(|o| o.status.success())
                            .unwrap_or(false);
                        if !ok {
                            let _ = ProcessCommand::new("sh")
                                .arg("-c")
                                .arg(format!(
                                    "cd /tmp && git clone https://aur.archlinux.org/{}.git && cd {} && makepkg -si --noconfirm",
                                    &chosen, &chosen
                                ))
                                .output();
                        }
                    }
                    app.aur_helper = Some(chosen);
                    app.mode = AppMode::OnboardingTheme;
                    app.theme_state.select(Some(app.current_theme));
                }
                _ => {}
            }
            true
        }

        AppMode::OnboardingTheme => {
            match code {
                // Back → previous step (ChaoticAur if Arch, else Start)
                KeyCode::Esc | KeyCode::Backspace => {
                    if app.is_arch {
                        app.chaotic_aur_index = 0;
                        app.options_state.select(Some(0));
                        app.mode = AppMode::OnboardingChaoticAur;
                    } else {
                        app.mode = AppMode::OnboardingStart;
                    }
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    let i = app.theme_state.selected().unwrap_or(0);
                    app.theme_state
                        .select(Some(if i == 0 { app.themes.len() - 1 } else { i - 1 }));
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    let i = app.theme_state.selected().unwrap_or(0);
                    app.theme_state.select(Some((i + 1) % app.themes.len()));
                }
                KeyCode::Enter => {
                    // Apply theme (preview only)
                    if let Some(i) = app.theme_state.selected() {
                        app.current_theme = i;
                    }
                }
                KeyCode::Tab => {
                    // Apply + advance to next step
                    if let Some(i) = app.theme_state.selected() {
                        app.current_theme = i;
                    }
                    app.mode = AppMode::OnboardingLayout;
                    app.options_index = 0;
                    app.options_state.select(Some(0));
                }
                _ => {}
            }
            true
        }

        AppMode::OnboardingLayout => {
            match code {
                // Back → Theme
                KeyCode::Esc | KeyCode::Backspace => {
                    app.mode = AppMode::OnboardingTheme;
                    app.theme_state.select(Some(app.current_theme));
                }
                KeyCode::Up | KeyCode::Char('k') => app.prev_opt(app.layouts.len()),
                KeyCode::Down | KeyCode::Char('j') => app.next_opt(app.layouts.len()),
                KeyCode::Enter => {
                    // Apply layout (preview only)
                    app.current_layout = app.options_index;
                }
                KeyCode::Tab => {
                    // Apply + advance to next step
                    app.current_layout = app.options_index;
                    app.mode = AppMode::OnboardingApps;
                    app.options_index = 0;
                    app.options_state.select(Some(0));
                }
                _ => {}
            }
            true
        }

        AppMode::OnboardingApps => {
            match code {
                // Back → Layout
                KeyCode::Esc | KeyCode::Backspace => {
                    app.mode = AppMode::OnboardingLayout;
                    app.options_index = 0;
                    app.options_state.select(Some(0));
                }
                KeyCode::Up | KeyCode::Char('k') => app.prev_opt(app.suggested_apps.len()),
                KeyCode::Down | KeyCode::Char('j') => app.next_opt(app.suggested_apps.len()),
                KeyCode::Char(' ') => {
                    app.suggested_apps[app.options_index].selected =
                        !app.suggested_apps[app.options_index].selected;
                }
                KeyCode::Enter => {
                    install_selected_packages(app);
                    finalize_onboarding(app);
                }
                _ => {}
            }
            true
        }

        AppMode::OnboardingInstalling => {
            // Any key continues to complete
            app.mode = AppMode::OnboardingComplete;
            true
        }

        AppMode::OnboardingComplete => {
            app.mode = AppMode::Normal;
            app.state.select(Some(0));
            true
        }

        _ => false,
    }
}

// ── Private helpers ─────────────────────────────────────────────────────────

/// Installs the selected workspace packages using the appropriate package manager.
fn install_selected_packages(app: &mut MenuApp) {
    let selected_cmds: Vec<String> = app
        .suggested_apps
        .iter()
        .filter(|a| a.selected && !a.is_appbar)
        .map(|a| a.command.split_whitespace().next().unwrap_or("").to_string())
        .filter(|s| !s.is_empty() && *s != "curl")
        .collect();

    if selected_cmds.is_empty() {
        app.install_status = "No packages to install.".to_string();
        return;
    }

    let mut status_lines = Vec::new();
    let pkg_list = selected_cmds.join(" ");

    if app.is_arch {
        if app.chaotic_aur_enabled {
            setup_chaotic_aur();
            let result = ProcessCommand::new("sh")
                .arg("-c")
                .arg(format!("sudo pacman -S --needed --noconfirm {}", pkg_list))
                .output();
            match result {
                Ok(o) if o.status.success() => {
                    status_lines.push(format!("✓ Installed via Chaotic AUR: {}", pkg_list));
                }
                _ => {
                    status_lines.push(format!("✗ Some packages failed via Chaotic AUR: {}", pkg_list));
                }
            }
        } else if let Some(ref helper) = app.aur_helper {
            let result = ProcessCommand::new(helper)
                .args(&["-S", "--needed", "--noconfirm"])
                .args(&selected_cmds)
                .output();
            match result {
                Ok(o) if o.status.success() => {
                    status_lines.push(format!("✓ Installed via {}: {}", helper, pkg_list));
                }
                _ => {
                    status_lines.push(format!("✗ Some packages failed via {}: {}", helper, pkg_list));
                }
            }
        } else {
            let _ = ProcessCommand::new("sh")
                .arg("-c")
                .arg(format!("sudo pacman -S --needed --noconfirm {}", pkg_list))
                .output();
            status_lines.push(format!("Attempted install via pacman: {}", pkg_list));
        }
    } else if app.distro_id == "ubuntu" || app.distro_id == "debian" {
        let _ = ProcessCommand::new("sh")
            .arg("-c")
            .arg(format!("sudo apt install -y {}", pkg_list))
            .output();
        status_lines.push(format!("Attempted install via apt: {}", pkg_list));
    } else if app.distro_id == "fedora" {
        let _ = ProcessCommand::new("sh")
            .arg("-c")
            .arg(format!("sudo dnf install -y {}", pkg_list))
            .output();
        status_lines.push(format!("Attempted install via dnf: {}", pkg_list));
    } else if app.distro_id == "opensuse" {
        let _ = ProcessCommand::new("sh")
            .arg("-c")
            .arg(format!("sudo zypper install -y {}", pkg_list))
            .output();
        status_lines.push(format!("Attempted install via zypper: {}", pkg_list));
    } else if app.distro_id == "void" {
        let _ = ProcessCommand::new("sh")
            .arg("-c")
            .arg(format!("sudo xbps-install -y {}", pkg_list))
            .output();
        status_lines.push(format!("Attempted install via xbps: {}", pkg_list));
    } else if app.distro_id == "macos" {
        // Check if brew is available
        let brew_ok = ProcessCommand::new("sh")
            .arg("-c")
            .arg("command -v brew")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
        if brew_ok {
            let _ = ProcessCommand::new("sh")
                .arg("-c")
                .arg(format!("brew install {}", pkg_list))
                .output();
            status_lines.push(format!("Attempted install via brew: {}", pkg_list));
        } else {
            status_lines.push("✗ Homebrew not found. Install it from https://brew.sh".to_string());
        }
    } else {
        status_lines.push(format!("Unknown distro '{}' — skipping package install. Install manually.", app.distro_id));
    }

    app.install_status = status_lines.join("\n");
}

/// Sets up the Chaotic AUR repo (keys, mirrorlist, pacman.conf entry).
fn setup_chaotic_aur() {
    let _ = ProcessCommand::new("sh")
        .arg("-c")
        .arg("sudo pacman-key --recv-key 3056513887B78AEB --keyserver keyserver.ubuntu.com && sudo pacman-key --lsign-key 3056513887B78AEB && sudo pacman -U --noconfirm 'https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-keyring.pkg.tar.zst' 'https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-mirrorlist.pkg.tar.zst'")
        .output();

    let _ = ProcessCommand::new("sh")
        .arg("-c")
        .arg("grep -q chaotic-aur /etc/pacman.conf || echo -e '\\n[chaotic-aur]\\nInclude = /etc/pacman/chaotic-mirrorlist' | sudo tee -a /etc/pacman.conf")
        .output();

    let _ = ProcessCommand::new("sudo")
        .args(&["pacman", "-Sy"])
        .output();
}

/// Moves selected apps into the workspace/appbar, saves config, and transitions to Installing.
fn finalize_onboarding(app: &mut MenuApp) {
    for suggested in &app.suggested_apps {
        if suggested.selected {
            let entry = AppEntry {
                name: suggested.name.clone(),
                desc: suggested.description.clone(),
                cmd: suggested.command.clone(),
            };
            if suggested.is_appbar {
                app.app_bar_items.push(entry);
            } else {
                app.items.push(entry);
            }
        }
    }
    app.config.first_launch = false;
    app.config.current_theme = app.themes[app.current_theme].name.to_string();
    let _ = app.save_config();
    app.mode = AppMode::OnboardingInstalling;
}
