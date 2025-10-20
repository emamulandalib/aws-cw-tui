use crate::ui::themes::{blue_gold_colors, warm_sunset_colors};
use ratatui::style::Color;

/// UI configuration settings
#[derive(Debug, Clone)]
pub struct UiConfig {
    pub default_border_color: Color,
    pub focused_border_color: Color,
    pub error_color: Color,
    pub success_color: Color,
    pub warning_color: Color,
    pub info_color: Color,
    pub chart_line_color: Color,
    pub background_color: Color,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self::warm_sunset_theme()
    }
}

impl UiConfig {
    /// Create a new UI configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set default border color
    pub fn with_default_border_color(mut self, color: Color) -> Self {
        self.default_border_color = color;
        self
    }

    /// Set focused border color
    pub fn with_focused_border_color(mut self, color: Color) -> Self {
        self.focused_border_color = color;
        self
    }

    /// Set error color
    pub fn with_error_color(mut self, color: Color) -> Self {
        self.error_color = color;
        self
    }

    /// Create a warm sunset theme configuration
    pub fn warm_sunset_theme() -> Self {
        Self {
            default_border_color: warm_sunset_colors::WARM_CREAM,
            focused_border_color: warm_sunset_colors::GOLDEN_YELLOW,
            error_color: warm_sunset_colors::CORAL_RED,
            success_color: warm_sunset_colors::SUCCESS_GREEN,
            warning_color: warm_sunset_colors::VIBRANT_ORANGE,
            info_color: warm_sunset_colors::INFO_BLUE,
            chart_line_color: warm_sunset_colors::GOLDEN_YELLOW,
            background_color: warm_sunset_colors::DARK_TEAL,
        }
    }

    /// Create a blue and gold theme configuration
    pub fn blue_gold_theme() -> Self {
        Self {
            default_border_color: blue_gold_colors::MEDIUM_BLUE,
            focused_border_color: blue_gold_colors::BRIGHT_GOLD,
            error_color: blue_gold_colors::ERROR_RED,
            success_color: blue_gold_colors::SUCCESS_GREEN,
            warning_color: blue_gold_colors::WARNING_ORANGE,
            info_color: blue_gold_colors::INFO_CYAN,
            chart_line_color: blue_gold_colors::BRIGHT_GOLD,
            background_color: blue_gold_colors::DEEP_NAVY,
        }
    }

    /// Create a dark theme configuration
    pub fn dark_theme() -> Self {
        Self {
            default_border_color: Color::White,
            focused_border_color: Color::Yellow,
            error_color: Color::Red,
            success_color: Color::Green,
            warning_color: Color::Yellow,
            info_color: Color::Blue,
            chart_line_color: Color::Cyan,
            background_color: Color::Black,
        }
    }

    /// Create a light theme configuration
    pub fn light_theme() -> Self {
        Self {
            default_border_color: Color::Black,
            focused_border_color: Color::Blue,
            error_color: Color::Red,
            success_color: Color::Green,
            warning_color: Color::Yellow,
            info_color: Color::Blue,
            chart_line_color: Color::Blue,
            background_color: Color::White,
        }
    }
}
