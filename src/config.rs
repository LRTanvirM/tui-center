use crate::types::Config;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command as ProcessCommand;

/// Returns the path to the app's config file, creating the directory if needed.
pub fn get_config_path() -> PathBuf {
    let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    let app_config_dir = config_dir.join("tui-center");
    if !app_config_dir.exists() {
        let _ = fs::create_dir_all(&app_config_dir);
    }
    app_config_dir.join("config.json")
}

/// Loads a Config from the given path, returning None if the file is missing or malformed.
pub fn load_config(path: &PathBuf) -> Option<Config> {
    fs::read_to_string(path)
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
}

/// Saves the given Config to the given path.
pub fn save_config(config: &Config, path: &PathBuf) -> io::Result<()> {
    let config_json = serde_json::to_string_pretty(config)?;
    fs::write(path, config_json)?;
    Ok(())
}

/// Detects the Linux distribution by parsing `/etc/os-release`.
pub fn detect_distro() -> String {
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        if content.contains("ID=arch") || content.contains("ID_LIKE=arch") {
            return "arch".to_string();
        } else if content.contains("ID=ubuntu") || content.contains("ID_LIKE=ubuntu") {
            return "ubuntu".to_string();
        } else if content.contains("ID=debian") || content.contains("ID_LIKE=debian") {
            return "debian".to_string();
        } else if content.contains("ID=fedora") || content.contains("ID_LIKE=fedora") {
            return "fedora".to_string();
        }
    }
    "unknown".to_string()
}

/// Fetches system info by running fastfetch/cachyos-fetch/neofetch and stripping ANSI escapes.
pub fn fetch_sys_info() -> String {
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
