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
        Theme {
            name: "Catppuccin Mocha",
            focus_border: Color::Rgb(203, 166, 247), // Mauve
            unfocus_border: Color::Rgb(88, 91, 112), // Surface2
            highlight_bg: Color::Rgb(203, 166, 247), // Mauve
            highlight_fg: Color::Rgb(30, 30, 46),    // Base
            text_normal: Color::Rgb(205, 214, 244),  // Text
            text_accent: Color::Rgb(180, 190, 254),  // Lavender
        },
        Theme {
            name: "Catppuccin Macchiato",
            focus_border: Color::Rgb(138, 173, 244), // Blue
            unfocus_border: Color::Rgb(91, 96, 120), // Surface2
            highlight_bg: Color::Rgb(138, 173, 244), // Blue
            highlight_fg: Color::Rgb(36, 39, 58),    // Base
            text_normal: Color::Rgb(202, 211, 245),  // Text
            text_accent: Color::Rgb(125, 196, 228),  // Sapphire
        },
        Theme {
            name: "Catppuccin Frappé",
            focus_border: Color::Rgb(244, 184, 228), // Pink
            unfocus_border: Color::Rgb(98, 104, 128), // Surface2
            highlight_bg: Color::Rgb(244, 184, 228), // Pink
            highlight_fg: Color::Rgb(48, 52, 70),    // Base
            text_normal: Color::Rgb(198, 208, 245),  // Text
            text_accent: Color::Rgb(242, 206, 239),  // Flamingo
        },
        Theme {
            name: "Catppuccin Latte",
            focus_border: Color::Rgb(23, 146, 153),  // Teal
            unfocus_border: Color::Rgb(172, 176, 190),// Surface2
            highlight_bg: Color::Rgb(23, 146, 153),  // Teal
            highlight_fg: Color::Rgb(239, 241, 245), // Base
            text_normal: Color::Rgb(76, 79, 105),    // Text
            text_accent: Color::Rgb(4, 165, 229),    // Sky
        },
        Theme {
            name: "Tokyo Night",
            focus_border: Color::Rgb(122, 162, 247), // Blue
            unfocus_border: Color::Rgb(86, 95, 137), // Grey
            highlight_bg: Color::Rgb(122, 162, 247), // Blue
            highlight_fg: Color::Rgb(26, 27, 38),    // Background
            text_normal: Color::Rgb(192, 202, 245),  // Foreground
            text_accent: Color::Rgb(187, 154, 247),  // Purple
        },
        Theme {
            name: "Solarized Dark",
            focus_border: Color::Rgb(38, 139, 210),  // Blue
            unfocus_border: Color::Rgb(88, 110, 117),// Base01
            highlight_bg: Color::Rgb(38, 139, 210),  // Blue
            highlight_fg: Color::Rgb(0, 43, 54),     // Base03 (Background)
            text_normal: Color::Rgb(131, 148, 150),  // Base0 (Foreground)
            text_accent: Color::Rgb(42, 161, 152),   // Cyan
        },
        Theme {
            name: "Solarized Light",
            focus_border: Color::Rgb(38, 139, 210),  // Blue
            unfocus_border: Color::Rgb(147, 161, 161),// Base1
            highlight_bg: Color::Rgb(38, 139, 210),  // Blue
            highlight_fg: Color::Rgb(253, 246, 227), // Base3 (Background)
            text_normal: Color::Rgb(101, 123, 131),  // Base00 (Foreground)
            text_accent: Color::Rgb(42, 161, 152),   // Cyan
        },
        Theme {
            name: "Monokai",
            focus_border: Color::Rgb(249, 38, 114),  // Pink
            unfocus_border: Color::Rgb(117, 113, 94),// Grey
            highlight_bg: Color::Rgb(249, 38, 114),  // Pink
            highlight_fg: Color::Rgb(39, 40, 34),    // Background
            text_normal: Color::Rgb(248, 248, 242),  // Foreground
            text_accent: Color::Rgb(166, 226, 46),   // Green
        },
        Theme {
            name: "One Dark",
            focus_border: Color::Rgb(97, 175, 239),  // Blue
            unfocus_border: Color::Rgb(92, 99, 112), // Grey
            highlight_bg: Color::Rgb(97, 175, 239),  // Blue
            highlight_fg: Color::Rgb(40, 44, 52),    // Background
            text_normal: Color::Rgb(171, 178, 191),  // Foreground
            text_accent: Color::Rgb(198, 120, 221),  // Purple
        },
    ]
}
