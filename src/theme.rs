use ratatui::style::Color;
use crate::types::Theme;

/// Returns all built-in themes.
pub fn default_themes() -> Vec<Theme> {
    vec![
        Theme {
            name: "Nord",
            focus_border: Color::Cyan,
            unfocus_border: Color::DarkGray,
            highlight_bg: Color::Cyan,
            highlight_fg: Color::Black,
            text_normal: Color::White,
            text_accent: Color::LightCyan,
        },
        Theme {
            name: "Dracula",
            focus_border: Color::Magenta,
            unfocus_border: Color::DarkGray,
            highlight_bg: Color::Magenta,
            highlight_fg: Color::Black,
            text_normal: Color::White,
            text_accent: Color::LightMagenta,
        },
        Theme {
            name: "Gruvbox",
            focus_border: Color::Yellow,
            unfocus_border: Color::DarkGray,
            highlight_bg: Color::Yellow,
            highlight_fg: Color::Black,
            text_normal: Color::White,
            text_accent: Color::LightYellow,
        },
    ]
}
