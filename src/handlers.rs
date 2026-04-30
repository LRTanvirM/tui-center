use crossterm::event::KeyCode;
use crate::cheat;
use crate::types::*;

/// Handles key events for all normal-mode states (Normal, ThemePopup, Quitting,
/// HelpPopup, OptionsPopup, EditMain/EditApp, DeleteConfirm, AddStep).
///
/// Returns `Some(cmd)` when a child process should be spawned, or `None` otherwise.
/// Sets `should_quit` to `true` when the user confirms quitting.
pub fn handle_normal_key(
    app: &mut MenuApp,
    code: KeyCode,
    should_quit: &mut bool,
) -> Option<String> {
    match &app.mode {
        AppMode::Normal => {
            match code {
                KeyCode::Char('q') => {
                    app.mode = AppMode::Quitting;
                    app.quit_index = 0;
                }
                KeyCode::Esc => {
                    app.mode = AppMode::OptionsPopup;
                    app.options_index = 0;
                    app.options_state.select(Some(0));
                }
                KeyCode::F(1) | KeyCode::Char('?') => app.mode = AppMode::HelpPopup,
                KeyCode::Char('f') => app.show_sys_info = !app.show_sys_info,
                KeyCode::Char('t') => {
                    app.current_theme = (app.current_theme + 1) % app.themes.len();
                }
                KeyCode::Tab => {
                    app.focus = match app.focus {
                        FocusPane::StatusBar => FocusPane::Workspace,
                        FocusPane::Workspace => FocusPane::AppBar,
                        FocusPane::AppBar => FocusPane::StatusBar,
                    };
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if app.focus == FocusPane::Workspace { app.next_list() }
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if app.focus == FocusPane::Workspace { app.prev_list() }
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    if app.focus == FocusPane::StatusBar {
                        app.status_index = app.status_index.saturating_sub(1);
                    }
                    if app.focus == FocusPane::AppBar {
                        app.app_bar_index = app.app_bar_index.saturating_sub(1);
                    }
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    if app.focus == FocusPane::StatusBar && app.status_index < 4 {
                        app.status_index += 1;
                    }
                    if app.focus == FocusPane::AppBar
                        && app.app_bar_index < app.app_bar_items.len().saturating_sub(1)
                    {
                        app.app_bar_index += 1;
                    }
                }
                KeyCode::Enter => {
                    let mut cmd = String::new();
                    match app.focus {
                        FocusPane::StatusBar => match app.status_index {
                            0 => cmd = "tclock".to_string(),
                            1 => cmd = "btop".to_string(),
                            2 => cmd = "uptime".to_string(),
                            3 => {
                                app.mode = AppMode::ThemePopup;
                                app.theme_state.select(Some(app.current_theme));
                            }
                            4 => app.show_sys_info = !app.show_sys_info,
                            _ => {}
                        },
                        FocusPane::Workspace => {
                            if let Some(i) = app.state.selected() {
                                cmd = app.items[i].cmd.clone();
                            }
                        }
                        FocusPane::AppBar => {
                            if !app.app_bar_items.is_empty() {
                                cmd = app.app_bar_items[app.app_bar_index].cmd.clone();
                            }
                        }
                    }
                    if !cmd.is_empty() {
                        return Some(cmd);
                    }
                }
                _ => {}
            }
        }

        AppMode::ThemePopup => match code {
            KeyCode::Esc | KeyCode::Backspace => app.mode = AppMode::Normal,
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
                if let Some(i) = app.theme_state.selected() {
                    app.current_theme = i;
                }
                app.mode = AppMode::Normal;
            }
            _ => {}
        },

        AppMode::Quitting => match code {
            KeyCode::Left | KeyCode::Right | KeyCode::Char('h') | KeyCode::Char('l') => {
                app.quit_index = if app.quit_index == 0 { 1 } else { 0 };
            }
            KeyCode::Enter => {
                if app.quit_index == 1 {
                    *should_quit = true;
                } else {
                    app.mode = AppMode::Normal;
                }
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => *should_quit = true,
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc | KeyCode::Backspace => {
                app.mode = AppMode::Normal
            }
            _ => {}
        },

        AppMode::HelpPopup => match code {
            KeyCode::Esc | KeyCode::Enter | KeyCode::Backspace => app.mode = AppMode::Normal,
            _ => {}
        },

        AppMode::OptionsPopup => {
            let opts_len = 7;
            match code {
                KeyCode::Esc | KeyCode::Backspace => app.mode = AppMode::Normal,
                KeyCode::Up | KeyCode::Char('k') => app.prev_opt(opts_len),
                KeyCode::Down | KeyCode::Char('j') => app.next_opt(opts_len),
                KeyCode::Enter => match app.options_index {
                    0 => {
                        app.mode = AppMode::EditMain;
                        app.options_index = 0;
                        app.options_state.select(Some(0));
                    }
                    1 => {
                        app.mode = AppMode::EditApp;
                        app.options_index = 0;
                        app.options_state.select(Some(0));
                    }
                    2 => app.show_sys_info = !app.show_sys_info,
                    3 => {
                        app.default_show_sys_info = !app.default_show_sys_info;
                        app.show_sys_info = app.default_show_sys_info;
                    }
                    4 => {
                        app.mode = AppMode::ImportExportMenu;
                        app.import_export_index = 0;
                        app.options_index = 0;
                        app.options_state.select(Some(0));
                    }
                    5 => app.mode = AppMode::OnboardingStart,
                    6 => app.mode = AppMode::Normal,
                    _ => {}
                },
                _ => {}
            }
        }

        AppMode::EditMain | AppMode::EditApp => {
            let is_main = app.mode == AppMode::EditMain;
            let list_len = if is_main { app.items.len() } else { app.app_bar_items.len() };
            match code {
                KeyCode::Esc | KeyCode::Backspace => {
                    app.mode = AppMode::OptionsPopup;
                    app.options_index = 0;
                    app.options_state.select(Some(0));
                }
                KeyCode::Up | KeyCode::Char('k') => app.prev_opt(list_len),
                KeyCode::Down | KeyCode::Char('j') => app.next_opt(list_len),
                KeyCode::Char('a') => {
                    app.mode = if is_main {
                        AppMode::AddMainStep(AddField::Name)
                    } else {
                        AppMode::AddAppStep(AddField::Name)
                    };
                    app.input_buffer.clear();
                    app.add_name.clear();
                    app.add_desc.clear();
                    app.add_cmd.clear();
                }
                KeyCode::Char('d') => {
                    if list_len > 0 {
                        app.mode = if is_main {
                            AppMode::DeleteConfirmMain
                        } else {
                            AppMode::DeleteConfirmApp
                        };
                        app.quit_index = 0;
                    }
                }
                _ => {}
            }
        }

        AppMode::DeleteConfirmMain | AppMode::DeleteConfirmApp => {
            let is_main = app.mode == AppMode::DeleteConfirmMain;
            match code {
                KeyCode::Left | KeyCode::Right | KeyCode::Char('h') | KeyCode::Char('l') => {
                    app.quit_index = if app.quit_index == 0 { 1 } else { 0 };
                }
                KeyCode::Enter => {
                    if app.quit_index == 1 {
                        if is_main {
                            app.items.remove(app.options_index);
                            app.mode = AppMode::EditMain;
                        } else {
                            app.app_bar_items.remove(app.options_index);
                            app.mode = AppMode::EditApp;
                        }
                        app.options_index = 0;
                        app.options_state.select(Some(0));
                    } else {
                        app.mode = if is_main { AppMode::EditMain } else { AppMode::EditApp };
                    }
                }
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    if is_main {
                        app.items.remove(app.options_index);
                        app.mode = AppMode::EditMain;
                    } else {
                        app.app_bar_items.remove(app.options_index);
                        app.mode = AppMode::EditApp;
                    }
                    app.options_index = 0;
                    app.options_state.select(Some(0));
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc | KeyCode::Backspace => {
                    app.mode = if is_main { AppMode::EditMain } else { AppMode::EditApp };
                }
                _ => {}
            }
        }

        AppMode::AddMainStep(step) | AppMode::AddAppStep(step) => {
            let is_main = matches!(app.mode, AppMode::AddMainStep(_));
            let step = *step;
            match code {
                KeyCode::Esc => {
                    app.mode = if is_main { AppMode::EditMain } else { AppMode::EditApp };
                }
                KeyCode::Backspace => {
                    if app.input_buffer.is_empty() {
                        app.mode = if is_main { AppMode::EditMain } else { AppMode::EditApp };
                    } else {
                        app.input_buffer.pop();
                    }
                }
                KeyCode::Char(c) => app.input_buffer.push(c),
                KeyCode::Enter => match step {
                    AddField::Name => {
                        app.add_name = app.input_buffer.clone();
                        app.input_buffer.clear();
                        app.mode = if is_main {
                            AppMode::AddMainStep(AddField::Desc)
                        } else {
                            AppMode::AddAppStep(AddField::Desc)
                        };
                    }
                    AddField::Desc => {
                        app.add_desc = app.input_buffer.clone();
                        app.input_buffer.clear();
                        app.mode = if is_main {
                            AppMode::AddMainStep(AddField::Cmd)
                        } else {
                            AppMode::AddAppStep(AddField::Cmd)
                        };
                    }
                    AddField::Cmd => {
                        app.add_cmd = app.input_buffer.clone();
                        app.input_buffer.clear();
                        let new_entry = AppEntry {
                            name: app.add_name.clone(),
                            desc: app.add_desc.clone(),
                            cmd: app.add_cmd.clone(),
                        };
                        if is_main {
                            app.items.push(new_entry);
                            app.mode = AppMode::EditMain;
                        } else {
                            app.app_bar_items.push(new_entry);
                            app.mode = AppMode::EditApp;
                        }
                        app.options_index = 0;
                        app.options_state.select(Some(0));
                    }
                },
                _ => {}
            }
        }

        AppMode::ImportExportMenu => {
            let opts_len = 3; // Import, Export, Back
            match code {
                KeyCode::Esc | KeyCode::Backspace => {
                    app.mode = AppMode::OptionsPopup;
                    app.options_index = 4;
                    app.options_state.select(Some(4));
                }
                KeyCode::Up | KeyCode::Char('k') => app.prev_opt(opts_len),
                KeyCode::Down | KeyCode::Char('j') => app.next_opt(opts_len),
                KeyCode::Enter => match app.options_index {
                    0 => {
                        // Import — discover .cheat files from navi + tui-center dirs
                        let mut files = cheat::discover_cheat_files(&cheat::default_cheat_dir());
                        files.extend(cheat::discover_cheat_files(&cheat::tui_center_cheat_dir()));
                        app.cheat_files = files;
                        app.cheat_file_index = 0;
                        app.options_index = 0;
                        app.options_state.select(Some(0));
                        app.cheat_status.clear();
                        app.mode = AppMode::CheatBrowser;
                    }
                    1 => {
                        // Export — confirm dialog
                        app.quit_index = 0;
                        app.mode = AppMode::CheatExportConfirm;
                    }
                    2 => {
                        // Back
                        app.mode = AppMode::OptionsPopup;
                        app.options_index = 4;
                        app.options_state.select(Some(4));
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        AppMode::CheatBrowser => {
            match code {
                KeyCode::Esc | KeyCode::Backspace => {
                    app.mode = AppMode::ImportExportMenu;
                    app.options_index = 0;
                    app.options_state.select(Some(0));
                }
                KeyCode::Up | KeyCode::Char('k') => app.prev_opt(app.cheat_files.len()),
                KeyCode::Down | KeyCode::Char('j') => app.next_opt(app.cheat_files.len()),
                KeyCode::Enter => {
                    if !app.cheat_files.is_empty() {
                        let path = app.cheat_files[app.options_index].clone();
                        match cheat::import_cheat_file(&path) {
                            Ok(entries) => {
                                let count = entries.len();
                                for entry in &entries {
                                    app.items.push(cheat::cheat_to_app_entry(entry));
                                }
                                app.cheat_status = format!(
                                    "✓ Imported {} commands from {}",
                                    count,
                                    path.file_name().unwrap_or_default().to_string_lossy()
                                );
                            }
                            Err(e) => {
                                app.cheat_status = format!(
                                    "✗ Failed to import {}: {}",
                                    path.file_name().unwrap_or_default().to_string_lossy(),
                                    e
                                );
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        AppMode::CheatExportConfirm => {
            match code {
                KeyCode::Esc | KeyCode::Backspace | KeyCode::Char('n') | KeyCode::Char('N') => {
                    app.mode = AppMode::ImportExportMenu;
                    app.options_index = 1;
                    app.options_state.select(Some(1));
                }
                KeyCode::Enter | KeyCode::Char('y') | KeyCode::Char('Y') => {
                    let dir = cheat::tui_center_cheat_dir();
                    let _ = std::fs::create_dir_all(&dir);
                    let path = dir.join("workspace.cheat");
                    match cheat::export_cheat_file(&path, &app.items, "tui-center, workspace") {
                        Ok(()) => {
                            app.cheat_status = format!(
                                "✓ Exported {} commands to {}",
                                app.items.len(),
                                path.display()
                            );
                        }
                        Err(e) => {
                            app.cheat_status = format!("✗ Export failed: {}", e);
                        }
                    }
                    app.mode = AppMode::ImportExportMenu;
                    app.options_index = 1;
                    app.options_state.select(Some(1));
                }
                KeyCode::Left | KeyCode::Right | KeyCode::Char('h') | KeyCode::Char('l') => {
                    app.quit_index = if app.quit_index == 0 { 1 } else { 0 };
                }
                _ => {}
            }
        }

        // Onboarding modes are handled in onboarding.rs
        _ => {}
    }
    None
}
