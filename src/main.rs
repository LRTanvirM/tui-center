mod app;
mod cheat;
mod config;
mod handlers;
mod onboarding;
mod theme;
mod types;
mod ui;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, stdout};
use std::process::Command as ProcessCommand;
use sysinfo::System;

use types::{AppMode, MenuApp};

fn main() -> io::Result<()> {
    // Ignore OS-level Ctrl+C so the TUI stays in control
    let _ = ctrlc::set_handler(|| {});

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut sys = System::new_all();
    let mut app = MenuApp::new();
    let mut should_quit = false;

    while !should_quit {
        sys.refresh_all();
        terminal.draw(|f| ui::ui(f, &sys, &mut app))?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                // ── Global hotkeys (digit → app bar launch) ─────────
                if app.mode == AppMode::Normal {
                    if let KeyCode::Char(c) = key.code {
                        if c.is_ascii_digit() {
                            let digit = c.to_digit(10).unwrap() as usize;
                            if let Some(cmd) = app.run_app_bar(digit.saturating_sub(1)) {
                                spawn_child(&mut terminal, &cmd)?;
                                continue;
                            }
                        }
                    }
                }

                // ── Onboarding key events ───────────────────────────
                if onboarding::handle_onboarding_key(&mut app, key.code) {
                    continue;
                }

                // ── Normal key events ───────────────────────────────
                if let Some(cmd) = handlers::handle_normal_key(&mut app, key.code, &mut should_quit) {
                    spawn_child(&mut terminal, &cmd)?;
                }
            }
        }
    }

    let _ = app.save_config();
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

/// Suspends the TUI, runs a child command, and resumes the TUI.
fn spawn_child(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    cmd: &str,
) -> io::Result<()> {
    let _ = disable_raw_mode();
    let _ = stdout().execute(LeaveAlternateScreen);

    let mut child = ProcessCommand::new("sh").arg("-c").arg(cmd).spawn()?;
    let _ = child.wait();

    let _ = enable_raw_mode();
    let _ = stdout().execute(EnterAlternateScreen);
    terminal.clear()?;
    Ok(())
}
