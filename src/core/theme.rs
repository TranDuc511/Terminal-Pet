// theme.rs — Color theme definitions for all 5 user-selectable themes.
//
// Each theme provides a full palette: pet ASCII color, UI borders, title,
// status bar, gauges, accent highlights, and dim text. Colors are expressed
// as crossterm `Color::Rgb` values for maximum visual quality.

use ratatui::style::Color;
use serde::{Deserialize, Serialize};

/// The five user-selectable color themes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeColor {
    Red,
    Blue,
    Green,
    Pink,
    Yellow,
}

impl ThemeColor {
    /// Cycle to the next theme in the list.
    pub fn next(self) -> ThemeColor {
        match self {
            ThemeColor::Red    => ThemeColor::Blue,
            ThemeColor::Blue   => ThemeColor::Green,
            ThemeColor::Green  => ThemeColor::Pink,
            ThemeColor::Pink   => ThemeColor::Yellow,
            ThemeColor::Yellow => ThemeColor::Red,
        }
    }

    /// Human-readable name for display.
    pub fn name(self) -> &'static str {
        match self {
            ThemeColor::Red    => "Red",
            ThemeColor::Blue   => "Blue",
            ThemeColor::Green  => "Green",
            ThemeColor::Pink   => "Pink",
            ThemeColor::Yellow => "Yellow",
        }
    }

    /// Emoji icon for the theme.
    pub fn icon(self) -> &'static str {
        match self {
            ThemeColor::Red    => "🔴",
            ThemeColor::Blue   => "🔵",
            ThemeColor::Green  => "🟢",
            ThemeColor::Pink   => "🩷",
            ThemeColor::Yellow => "🟡",
        }
    }
}

/// A resolved color palette for a given theme.
pub struct Theme {
    /// The theme variant.
    pub variant: ThemeColor,

    /// Primary foreground color — used for the ASCII art pet and key UI text.
    pub primary: Color,

    /// Slightly lighter accent color — used for borders, highlights.
    pub accent: Color,

    /// Dim muted tone — used for secondary text.
    pub muted: Color,

    /// Background color for the status bar block.
    pub status_bg: Color,

    /// Color used for the bond gauge fill.
    pub gauge_bond: Color,

    /// Color used for the hunger gauge fill.
    pub gauge_hunger: Color,

    /// Color used for the happiness gauge fill.
    pub gauge_happiness: Color,

    /// Color for the title text.
    pub title: Color,

    /// Color for border lines.
    pub border: Color,

    /// Color for help/keybind text.
    pub help_text: Color,

    /// Color for received messages in the log.
    pub message: Color,
}

impl Theme {
    /// Build the full theme palette from a `ThemeColor` variant.
    pub fn from_color(color: ThemeColor) -> Self {
        match color {
            ThemeColor::Red => Theme {
                variant:          ThemeColor::Red,
                primary:          Color::Rgb(255, 107, 107),
                accent:           Color::Rgb(255, 153, 153),
                muted:            Color::Rgb(160, 80, 80),
                status_bg:        Color::Rgb(40, 10, 10),
                gauge_bond:       Color::Rgb(255, 85, 85),
                gauge_hunger:     Color::Rgb(200, 60, 60),
                gauge_happiness:  Color::Rgb(255, 130, 130),
                title:            Color::Rgb(255, 200, 200),
                border:           Color::Rgb(180, 60, 60),
                help_text:        Color::Rgb(200, 120, 120),
                message:          Color::Rgb(230, 160, 160),
            },
            ThemeColor::Blue => Theme {
                variant:          ThemeColor::Blue,
                primary:          Color::Rgb(107, 197, 255),
                accent:           Color::Rgb(153, 220, 255),
                muted:            Color::Rgb(70, 130, 170),
                status_bg:        Color::Rgb(10, 20, 40),
                gauge_bond:       Color::Rgb(70, 160, 230),
                gauge_hunger:     Color::Rgb(50, 120, 200),
                gauge_happiness:  Color::Rgb(120, 200, 255),
                title:            Color::Rgb(200, 230, 255),
                border:           Color::Rgb(60, 130, 200),
                help_text:        Color::Rgb(120, 180, 220),
                message:          Color::Rgb(160, 210, 240),
            },
            ThemeColor::Green => Theme {
                variant:          ThemeColor::Green,
                primary:          Color::Rgb(107, 255, 138),
                accent:           Color::Rgb(153, 255, 178),
                muted:            Color::Rgb(60, 150, 80),
                status_bg:        Color::Rgb(8, 30, 15),
                gauge_bond:       Color::Rgb(70, 210, 100),
                gauge_hunger:     Color::Rgb(50, 170, 75),
                gauge_happiness:  Color::Rgb(130, 245, 155),
                title:            Color::Rgb(190, 255, 205),
                border:           Color::Rgb(55, 180, 85),
                help_text:        Color::Rgb(110, 200, 130),
                message:          Color::Rgb(160, 240, 175),
            },
            ThemeColor::Pink => Theme {
                variant:          ThemeColor::Pink,
                primary:          Color::Rgb(255, 107, 202),
                accent:           Color::Rgb(255, 153, 225),
                muted:            Color::Rgb(170, 70, 140),
                status_bg:        Color::Rgb(40, 8, 35),
                gauge_bond:       Color::Rgb(235, 80, 185),
                gauge_hunger:     Color::Rgb(200, 55, 155),
                gauge_happiness:  Color::Rgb(255, 140, 215),
                title:            Color::Rgb(255, 210, 240),
                border:           Color::Rgb(195, 65, 165),
                help_text:        Color::Rgb(220, 130, 195),
                message:          Color::Rgb(245, 175, 225),
            },
            ThemeColor::Yellow => Theme {
                variant:          ThemeColor::Yellow,
                primary:          Color::Rgb(255, 217, 107),
                accent:           Color::Rgb(255, 235, 153),
                muted:            Color::Rgb(170, 140, 60),
                status_bg:        Color::Rgb(38, 30, 8),
                gauge_bond:       Color::Rgb(240, 195, 60),
                gauge_hunger:     Color::Rgb(205, 165, 40),
                gauge_happiness:  Color::Rgb(255, 230, 130),
                title:            Color::Rgb(255, 245, 200),
                border:           Color::Rgb(195, 160, 50),
                help_text:        Color::Rgb(220, 195, 100),
                message:          Color::Rgb(245, 225, 155),
            },
        }
    }
}
