//! Navi `.cheat` file parser, importer, and exporter.
//!
//! ## .cheat Format Reference
//!
//! ```text
//! % tag1, tag2            ← category tags (becomes AppEntry.desc prefix)
//! ; comment               ← ignored
//! # description           ← human-readable description
//! command <arg>           ← the shell command
//! $ arg: some_cmd         ← variable generator (preserved as-is)
//! ```

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::types::AppEntry;

// ── Parsed intermediate representation ──────────────────────────────────────

/// One command block parsed from a `.cheat` file.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CheatEntry {
    /// Category tags from the `%` line (e.g. "git, code").
    pub tags: String,
    /// Description from the `#` line.
    pub description: String,
    /// The shell command (everything that isn't a %, #, ;, $ or @ line).
    pub command: String,
    /// Variable definitions (`$` lines) associated with this command.
    pub variables: Vec<String>,
}

// ── Parser ──────────────────────────────────────────────────────────────────

/// Parses the contents of a `.cheat` file into a list of `CheatEntry`.
pub fn parse_cheat(content: &str) -> Vec<CheatEntry> {
    let mut entries: Vec<CheatEntry> = Vec::new();
    let mut current_tags = String::new();
    let mut current_desc = String::new();
    let mut current_cmd_lines: Vec<String> = Vec::new();
    let mut current_vars: Vec<String> = Vec::new();
    let mut in_code_block = false;

    for line in content.lines() {
        let trimmed = line.trim();

        // Handle markdown-style code fences (```sh ... ```)
        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }

        // Inside a code block, everything is a command line
        if in_code_block {
            current_cmd_lines.push(line.to_string());
            continue;
        }

        if trimmed.is_empty() {
            // Empty lines between entries — flush if we have a command
            flush_entry(
                &mut entries,
                &current_tags,
                &current_desc,
                &current_cmd_lines,
                &current_vars,
            );
            current_desc.clear();
            current_cmd_lines.clear();
            current_vars.clear();
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix('%') {
            // New tag section — flush any pending entry first
            flush_entry(
                &mut entries,
                &current_tags,
                &current_desc,
                &current_cmd_lines,
                &current_vars,
            );
            current_tags = rest.trim().to_string();
            current_desc.clear();
            current_cmd_lines.clear();
            current_vars.clear();
        } else if trimmed.starts_with(';') {
            // Comment — skip
        } else if let Some(rest) = trimmed.strip_prefix('#') {
            // Description
            current_desc = rest.trim().to_string();
        } else if trimmed.starts_with('$') {
            // Variable definition
            current_vars.push(trimmed.to_string());
        } else if trimmed.starts_with('@') {
            // Tag reference — skip (not useful for TUI Center import)
        } else {
            // Command line
            current_cmd_lines.push(line.to_string());
        }
    }

    // Flush the last entry
    flush_entry(
        &mut entries,
        &current_tags,
        &current_desc,
        &current_cmd_lines,
        &current_vars,
    );

    entries
}

/// Pushes a completed entry if there's at least a command.
fn flush_entry(
    entries: &mut Vec<CheatEntry>,
    tags: &str,
    desc: &str,
    cmd_lines: &[String],
    vars: &[String],
) {
    if cmd_lines.is_empty() {
        return;
    }
    let command = cmd_lines
        .iter()
        .map(|l| l.trim())
        .collect::<Vec<_>>()
        .join(" && ");

    entries.push(CheatEntry {
        tags: tags.to_string(),
        description: if desc.is_empty() {
            // Derive a description from the command if none provided
            command.chars().take(60).collect()
        } else {
            desc.to_string()
        },
        command,
        variables: vars.to_vec(),
    });
}

// ── Conversion to/from AppEntry ─────────────────────────────────────────────

/// Converts a `CheatEntry` to an `AppEntry` for the workspace.
pub fn cheat_to_app_entry(entry: &CheatEntry) -> AppEntry {
    let name = if entry.description.len() > 30 {
        format!("{}…", &entry.description[..29])
    } else {
        entry.description.clone()
    };

    let desc = if entry.tags.is_empty() {
        entry.description.clone()
    } else {
        format!("[{}] {}", entry.tags, entry.description)
    };

    AppEntry {
        name,
        desc,
        cmd: entry.command.clone(),
    }
}

/// Converts a list of `AppEntry` into `.cheat` file content.
pub fn app_entries_to_cheat(entries: &[AppEntry], tag: &str) -> String {
    let mut output = String::new();
    output.push_str(&format!("% {}\n\n", tag));

    for entry in entries {
        output.push_str(&format!("# {}\n", entry.desc));
        output.push_str(&format!("{}\n\n", entry.cmd));
    }

    output
}

// ── File I/O ────────────────────────────────────────────────────────────────

/// Reads and parses a `.cheat` file from disk.
pub fn import_cheat_file(path: &Path) -> io::Result<Vec<CheatEntry>> {
    let content = fs::read_to_string(path)?;
    Ok(parse_cheat(&content))
}

/// Exports the given app entries to a `.cheat` file.
pub fn export_cheat_file(path: &Path, entries: &[AppEntry], tag: &str) -> io::Result<()> {
    let content = app_entries_to_cheat(entries, tag);
    fs::write(path, content)?;
    Ok(())
}

/// Returns the default navi cheats directory (`~/.local/share/navi/cheats/`).
pub fn default_cheat_dir() -> PathBuf {
    let data_dir = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    data_dir.join("navi").join("cheats")
}

/// Returns the TUI Center cheats directory (`~/.config/tui-center/cheats/`).
pub fn tui_center_cheat_dir() -> PathBuf {
    let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    config_dir.join("tui-center").join("cheats")
}

/// Discovers all `.cheat` files in a directory (non-recursive).
pub fn discover_cheat_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(read_dir) = fs::read_dir(dir) {
        for entry in read_dir.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "cheat" {
                        files.push(path);
                    }
                }
            }
        }
    }
    files.sort();
    files
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_cheat() {
        let content = r#"
% git, code

# Change branch
git checkout <branch>

$ branch: git branch | awk '{print $NF}'

# Stash changes
git stash
"#;
        let entries = parse_cheat(content);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].tags, "git, code");
        assert_eq!(entries[0].description, "Change branch");
        assert_eq!(entries[0].command, "git checkout <branch>");
        assert_eq!(entries[1].description, "Stash changes");
        assert_eq!(entries[1].command, "git stash");
    }

    #[test]
    fn test_export_round_trip() {
        let entries = vec![
            AppEntry { name: "test".into(), desc: "Test command".into(), cmd: "echo hello".into() },
        ];
        let output = app_entries_to_cheat(&entries, "tui-center");
        assert!(output.contains("% tui-center"));
        assert!(output.contains("# Test command"));
        assert!(output.contains("echo hello"));
    }
}
