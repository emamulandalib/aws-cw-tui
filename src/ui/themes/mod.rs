use ratatui::style::Color;
use serde::{Deserialize, Serialize};

/// Warm Sunset color constants for vibrant and warm terminal theming
pub mod warm_sunset_colors {
    use ratatui::style::Color;

    // Warm Sunset base colors from provided palette
    pub const DARK_TEAL: Color = Color::Rgb(0, 48, 73); // #003049 - Deep background
    pub const CORAL_RED: Color = Color::Rgb(214, 40, 40); // #D62828 - Error/critical states
    pub const VIBRANT_ORANGE: Color = Color::Rgb(247, 127, 0); // #F77F00 - Warning/highlights
    pub const GOLDEN_YELLOW: Color = Color::Rgb(252, 191, 73); // #FCBF49 - Primary accents
    pub const WARM_CREAM: Color = Color::Rgb(234, 226, 183); // #EAE2B7 - Light text/backgrounds

    // Derived colors for better UX and readability
    pub const LIGHT_TEAL: Color = Color::Rgb(25, 73, 98); // Lighter teal for borders
    pub const MUTED_TEAL: Color = Color::Rgb(13, 61, 86); // Muted teal for separators
    pub const SOFT_ORANGE: Color = Color::Rgb(255, 152, 51); // Softer orange for secondary elements
    pub const PALE_CREAM: Color = Color::Rgb(248, 243, 216); // Very light cream for highlights
    pub const WARM_WHITE: Color = Color::Rgb(255, 250, 240); // Warm white for primary text

    // Status colors maintaining the warm sunset theme
    pub const SUCCESS_GREEN: Color = Color::Rgb(76, 175, 80); // Success states (kept green for consistency)
    pub const INFO_BLUE: Color = Color::Rgb(52, 152, 219); // Information states (kept blue for clarity)
}

/// Blue and Gold color constants for vibrant terminal theming
pub mod blue_gold_colors {
    use ratatui::style::Color;

    // Blue and Gold base colors from provided palette
    pub const DEEP_NAVY: Color = Color::Rgb(0, 8, 20); // #000814 - Deep background
    pub const DARK_BLUE: Color = Color::Rgb(0, 29, 61); // #001D3D - Background highlights
    pub const MEDIUM_BLUE: Color = Color::Rgb(0, 53, 102); // #003566 - Text and borders
    pub const BRIGHT_GOLD: Color = Color::Rgb(255, 195, 0); // #FFC300 - Primary accents
    pub const LIGHT_GOLD: Color = Color::Rgb(255, 214, 10); // #FFD60A - Selection and focus

    // Derived colors for better UX
    pub const LIGHT_BLUE: Color = Color::Rgb(51, 133, 204); // Lighter blue for text
    pub const PALE_BLUE: Color = Color::Rgb(102, 163, 224); // Very light blue for secondary text
    pub const MUTED_BLUE: Color = Color::Rgb(25, 76, 127); // Muted blue for disabled elements
    pub const SEPARATOR_BLUE: Color = Color::Rgb(0, 41, 82); // Slightly lighter than dark blue

    // Status colors maintaining the blue-gold theme
    pub const SUCCESS_GREEN: Color = Color::Rgb(76, 175, 80); // Success states
    pub const WARNING_ORANGE: Color = Color::Rgb(255, 152, 0); // Warning states
    pub const ERROR_RED: Color = Color::Rgb(244, 67, 54); // Error states
    pub const INFO_CYAN: Color = Color::Rgb(0, 188, 212); // Information states
}

/// Solarized Dark color constants for consistent theming
pub mod solarized_colors {
    use ratatui::style::Color;

    // Solarized Dark base colors
    pub const BASE03: Color = Color::Rgb(0, 43, 54); // Background
    pub const BASE02: Color = Color::Rgb(7, 54, 66); // Background highlights
    pub const BASE01: Color = Color::Rgb(88, 110, 117); // Content tones
    pub const BASE00: Color = Color::Rgb(101, 123, 131); //
    pub const BASE0: Color = Color::Rgb(131, 148, 150); // Main text
    pub const BASE1: Color = Color::Rgb(147, 161, 161); // Secondary text

    // Solarized accent colors
    pub const YELLOW: Color = Color::Rgb(181, 137, 0); // Selection/focus
    pub const ORANGE: Color = Color::Rgb(203, 75, 22); // Warnings/highlights
    pub const RED: Color = Color::Rgb(220, 50, 47); // Errors/critical
    pub const MAGENTA: Color = Color::Rgb(211, 54, 130); // Status/highlights
    pub const VIOLET: Color = Color::Rgb(108, 113, 196); // Special elements
    pub const BLUE: Color = Color::Rgb(38, 139, 210); // Information
    pub const CYAN: Color = Color::Rgb(42, 161, 152); // Headers/borders
    pub const GREEN: Color = Color::Rgb(133, 153, 0); // Success/healthy
}

/// Terminal cyan color scheme - dark background with cyan/light blue highlights
pub mod terminal_colors {
    use ratatui::style::Color;

    // Terminal base colors
    pub const BACKGROUND: Color = Color::Rgb(24, 24, 24); // Dark background
    pub const TEXT_PRIMARY: Color = Color::Rgb(225, 225, 225); // Main text (light gray)
    pub const TEXT_SECONDARY: Color = Color::Rgb(180, 180, 180); // Secondary text
    pub const TEXT_MUTED: Color = Color::Rgb(120, 120, 120); // Muted text

    // Terminal signature colors
    pub const CYAN: Color = Color::Rgb(102, 217, 239); // Headers, borders, highlights
    pub const LIGHT_BLUE: Color = Color::Rgb(116, 199, 236); // Selection background
    pub const STEEL_BLUE: Color = Color::Rgb(70, 130, 180); // Focused borders
    pub const DODGER_BLUE: Color = Color::Rgb(30, 144, 255); // Information

    // Status colors
    pub const GREEN: Color = Color::Rgb(76, 175, 80); // Success/healthy
    pub const ORANGE: Color = Color::Rgb(255, 152, 0); // Warnings
    pub const RED: Color = Color::Rgb(244, 67, 54); // Errors/critical
    pub const YELLOW: Color = Color::Rgb(255, 235, 59); // Caution

    // UI elements
    pub const BORDER: Color = Color::Rgb(102, 217, 239); // Default borders (cyan)
    pub const SEPARATOR: Color = Color::Rgb(80, 80, 80); // Content separators
    pub const SELECTION_BG: Color = Color::Rgb(102, 217, 239); // Selected item background
    pub const SELECTION_TEXT: Color = Color::Rgb(24, 24, 24); // Selected item text (dark)
}

/// Sophisticated color palette for sleek terminal interface
/// Based on modern terminal colorschemes with excellent contrast and readability
#[derive(Clone, Debug)]
pub struct UnifiedTheme {
    /// Core interface colors
    pub primary: Color, // Main text and headers
    pub secondary: Color,  // Secondary text and descriptions
    pub tertiary: Color,   // Subtle text and metadata
    pub background: Color, // Background color

    /// Interactive elements
    pub accent: Color, // Highlights and active elements
    pub focused: Color,       // Focused panel borders and highlights
    pub selected: Color,      // Selected items background
    pub selected_text: Color, // Selected items text

    /// Status colors with subtle sophistication
    pub success: Color, // Success states and positive metrics
    pub warning: Color, // Warning states and caution
    pub error: Color,   // Error states and critical alerts
    pub info: Color,    // Information and neutral status

    /// Structural elements  
    pub border: Color, // Default borders and separators
    pub border_focused: Color, // Focused panel borders
    pub separator: Color,      // Content separators and dividers
    pub muted: Color,          // Muted text and disabled elements

    /// Data visualization
    pub chart_primary: Color, // Primary chart color
    pub chart_secondary: Color, // Secondary chart color
    pub chart_accent: Color,    // Chart accent color
    pub progress_bg: Color,     // Progress bar background
    pub progress_fg: Color,     // Progress bar foreground
}

impl UnifiedTheme {
    /// Warm Sunset theme - vibrant teal and warm orange/yellow palette
    pub fn warm_sunset() -> Self {
        Self {
            // Core interface - Warm sunset signature style
            primary: warm_sunset_colors::WARM_WHITE, // Main text (warm white)
            secondary: warm_sunset_colors::WARM_CREAM, // Secondary text (warm cream)
            tertiary: warm_sunset_colors::PALE_CREAM, // Metadata text (pale cream)
            background: warm_sunset_colors::DARK_TEAL, // Dark teal background

            // Interactive elements - Warm cream for consistent borders
            accent: warm_sunset_colors::WARM_CREAM, // Headers and borders (warm cream)
            focused: warm_sunset_colors::VIBRANT_ORANGE, // Focus indicators (vibrant orange)
            selected: warm_sunset_colors::GOLDEN_YELLOW, // Selection background (golden yellow)
            selected_text: warm_sunset_colors::DARK_TEAL, // Selected text (dark on gold)

            // Status colors - Warm semantic coloring
            success: warm_sunset_colors::SUCCESS_GREEN, // Success states
            warning: warm_sunset_colors::VIBRANT_ORANGE, // Warning states (vibrant orange)
            error: warm_sunset_colors::CORAL_RED,       // Error states (coral red)
            info: warm_sunset_colors::INFO_BLUE,        // Information status

            // Structural elements - Teal and cream tones
            border: warm_sunset_colors::WARM_CREAM, // Default borders (warm cream)
            border_focused: warm_sunset_colors::VIBRANT_ORANGE, // Focused borders (vibrant orange)
            separator: warm_sunset_colors::MUTED_TEAL, // Content separators
            muted: warm_sunset_colors::PALE_CREAM,  // Disabled/muted elements

            // Data visualization - Warm sunset chart colors
            chart_primary: warm_sunset_colors::GOLDEN_YELLOW, // Primary data lines (golden yellow)
            chart_secondary: warm_sunset_colors::VIBRANT_ORANGE, // Secondary data (vibrant orange)
            chart_accent: warm_sunset_colors::CORAL_RED,      // Accent data points (coral red)
            progress_bg: warm_sunset_colors::MUTED_TEAL,      // Progress backgrounds
            progress_fg: warm_sunset_colors::GOLDEN_YELLOW,   // Progress indicators (golden yellow)
        }
    }

    /// Blue and Gold theme - vibrant and professional
    pub fn blue_gold() -> Self {
        Self {
            // Core interface - Blue and Gold signature style
            primary: blue_gold_colors::PALE_BLUE, // Main text (pale blue)
            secondary: blue_gold_colors::LIGHT_BLUE, // Secondary text (light blue)
            tertiary: blue_gold_colors::MUTED_BLUE, // Metadata text (muted blue)
            background: blue_gold_colors::DEEP_NAVY, // Deep navy background

            // Interactive elements - Gold highlights
            accent: blue_gold_colors::BRIGHT_GOLD, // Headers and borders (bright gold)
            focused: blue_gold_colors::LIGHT_GOLD, // Focus indicators (light gold)
            selected: blue_gold_colors::LIGHT_GOLD, // Selection background (light gold)
            selected_text: blue_gold_colors::DEEP_NAVY, // Selected text (dark on gold)

            // Status colors - Semantic with blue-gold theme
            success: blue_gold_colors::SUCCESS_GREEN, // Success states
            warning: blue_gold_colors::WARNING_ORANGE, // Warning states
            error: blue_gold_colors::ERROR_RED,       // Error states
            info: blue_gold_colors::INFO_CYAN,        // Information status

            // Structural elements - Blue tones
            border: blue_gold_colors::MEDIUM_BLUE, // Default borders (medium blue)
            border_focused: blue_gold_colors::BRIGHT_GOLD, // Focused borders (bright gold)
            separator: blue_gold_colors::SEPARATOR_BLUE, // Content separators
            muted: blue_gold_colors::MUTED_BLUE,   // Disabled/muted elements

            // Data visualization - Blue-gold chart colors
            chart_primary: blue_gold_colors::BRIGHT_GOLD, // Primary data lines (gold)
            chart_secondary: blue_gold_colors::LIGHT_BLUE, // Secondary data (light blue)
            chart_accent: blue_gold_colors::INFO_CYAN,    // Accent data points (cyan)
            progress_bg: blue_gold_colors::SEPARATOR_BLUE, // Progress backgrounds
            progress_fg: blue_gold_colors::BRIGHT_GOLD,   // Progress indicators (gold)
        }
    }

    /// Terminal cyan theme for consistent terminal interface
    pub fn default() -> Self {
        Self::warm_sunset() // Use the new vibrant warm sunset theme as default
    }

    /// High contrast theme for better accessibility
    pub fn high_contrast() -> Self {
        Self {
            primary: Color::White,
            secondary: Color::Rgb(204, 204, 204),
            tertiary: Color::Rgb(153, 153, 153),
            background: Color::Black,

            accent: Color::Cyan,
            focused: Color::Cyan,
            selected: Color::Blue,
            selected_text: Color::White,

            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Cyan,

            border: Color::Gray,
            border_focused: Color::Cyan,
            separator: Color::DarkGray,
            muted: Color::DarkGray,

            chart_primary: Color::Cyan,
            chart_secondary: Color::Magenta,
            chart_accent: Color::Cyan,
            progress_bg: Color::DarkGray,
            progress_fg: Color::Green,
        }
    }

    /// Monochrome theme for ultra-minimal aesthetic
    pub fn monochrome() -> Self {
        Self {
            primary: Color::White,
            secondary: Color::Rgb(189, 189, 189),
            tertiary: Color::Rgb(117, 117, 117),
            background: Color::Black,

            accent: Color::White,
            focused: Color::Rgb(189, 189, 189),
            selected: Color::DarkGray,
            selected_text: Color::White,

            success: Color::White,
            warning: Color::Rgb(189, 189, 189),
            error: Color::Rgb(117, 117, 117),
            info: Color::White,

            border: Color::Gray,
            border_focused: Color::White,
            separator: Color::DarkGray,
            muted: Color::DarkGray,

            chart_primary: Color::White,
            chart_secondary: Color::Rgb(189, 189, 189),
            chart_accent: Color::Rgb(117, 117, 117),
            progress_bg: Color::DarkGray,
            progress_fg: Color::White,
        }
    }

    /// Solarized Dark theme for consistent terminal interface
    pub fn solarized_dark() -> Self {
        Self {
            // Core interface - Solarized Dark text hierarchy
            primary: solarized_colors::BASE0, // Main text (light gray)
            secondary: solarized_colors::BASE01, // Secondary text (medium gray)
            tertiary: solarized_colors::BASE00, // Metadata text (darker gray)
            background: solarized_colors::BASE03, // Dark background

            // Interactive elements - Solarized signature colors
            accent: solarized_colors::CYAN, // Headers and borders (cyan)
            focused: solarized_colors::YELLOW, // Focus indicators (yellow)
            selected: solarized_colors::YELLOW, // Selection background (yellow)
            selected_text: solarized_colors::BASE03, // Selected text (dark on yellow)

            // Status colors - Solarized semantic coloring
            success: solarized_colors::GREEN, // Healthy/running states
            warning: solarized_colors::ORANGE, // Warning states
            error: solarized_colors::RED,     // Error/critical states
            info: solarized_colors::BLUE,     // Information status

            // Structural elements - Solarized interface
            border: solarized_colors::BASE01, // Default borders
            border_focused: solarized_colors::YELLOW, // Focused borders (yellow)
            separator: solarized_colors::BASE01, // Content separators
            muted: solarized_colors::BASE00,  // Disabled/muted elements

            // Data visualization - Solarized chart colors
            chart_primary: solarized_colors::CYAN, // Primary data lines
            chart_secondary: solarized_colors::BLUE, // Secondary data
            chart_accent: solarized_colors::MAGENTA, // Accent data points
            progress_bg: solarized_colors::BASE02, // Progress backgrounds
            progress_fg: solarized_colors::CYAN,   // Progress indicators
        }
    }

    /// Terminal cyan theme - dark background with cyan highlights
    pub fn terminal_cyan() -> Self {
        Self {
            // Core interface - Terminal signature style
            primary: terminal_colors::TEXT_PRIMARY, // Main text (light gray)
            secondary: terminal_colors::TEXT_SECONDARY, // Secondary text (medium gray)
            tertiary: terminal_colors::TEXT_MUTED,  // Metadata text (dark gray)
            background: terminal_colors::BACKGROUND, // Dark background

            // Interactive elements - Terminal cyan highlights
            accent: terminal_colors::CYAN, // Headers and borders (cyan)
            focused: terminal_colors::STEEL_BLUE, // Focus indicators (steel blue)
            selected: terminal_colors::SELECTION_BG, // Selection background (cyan)
            selected_text: terminal_colors::SELECTION_TEXT, // Selected text (dark)

            // Status colors - Terminal semantic coloring
            success: terminal_colors::GREEN, // Healthy/running states
            warning: terminal_colors::ORANGE, // Warning states
            error: terminal_colors::RED,     // Error/critical states
            info: terminal_colors::DODGER_BLUE, // Information status

            // Structural elements - Terminal interface
            border: terminal_colors::BORDER, // Default borders (cyan)
            border_focused: terminal_colors::STEEL_BLUE, // Focused borders (steel blue)
            separator: terminal_colors::SEPARATOR, // Content separators
            muted: terminal_colors::TEXT_MUTED, // Disabled/muted elements

            // Data visualization - Terminal chart colors
            chart_primary: terminal_colors::CYAN, // Primary data lines
            chart_secondary: terminal_colors::LIGHT_BLUE, // Secondary data
            chart_accent: terminal_colors::STEEL_BLUE, // Accent data points
            progress_bg: terminal_colors::SEPARATOR, // Progress backgrounds
            progress_fg: terminal_colors::CYAN,   // Progress indicators
        }
    }
}

/// Theme variants for different contexts
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ThemeVariant {
    Default,
    WarmSunset,
    BlueGold,
    HighContrast,
    Monochrome,
    TerminalCyan,
}

impl ThemeVariant {
    pub fn to_theme(&self) -> UnifiedTheme {
        match self {
            ThemeVariant::Default => UnifiedTheme::default(),
            ThemeVariant::WarmSunset => UnifiedTheme::warm_sunset(),
            ThemeVariant::BlueGold => UnifiedTheme::blue_gold(),
            ThemeVariant::HighContrast => UnifiedTheme::high_contrast(),
            ThemeVariant::Monochrome => UnifiedTheme::monochrome(),
            ThemeVariant::TerminalCyan => UnifiedTheme::terminal_cyan(),
        }
    }
}

/// Component-specific theme configurations
#[derive(Clone, Debug)]
pub struct ComponentTheme {
    pub base: UnifiedTheme,
    pub component_accent: Color,
    pub component_highlight: Color,
}

impl ComponentTheme {
    /// Service selection theme - warm cream border for all panels
    pub fn service_list(base: UnifiedTheme) -> Self {
        Self {
            component_accent: base.border,      // Warm cream for service lists
            component_highlight: base.selected, // Golden yellow for selection
            base,
        }
    }

    /// Instance list theme - warm cream border for all panels
    pub fn instance_list(base: UnifiedTheme) -> Self {
        Self {
            component_accent: base.border,      // Warm cream for instances
            component_highlight: base.selected, // Golden yellow for selection
            base,
        }
    }

    /// Time range selection theme - warm cream border for all panels
    pub fn time_range(base: UnifiedTheme) -> Self {
        Self {
            component_accent: base.border,      // Warm cream for time controls
            component_highlight: base.selected, // Golden yellow for selection
            base,
        }
    }

    /// Metrics and charts theme - warm cream border for all panels
    pub fn metrics(base: UnifiedTheme) -> Self {
        Self {
            component_accent: base.border,      // Warm cream for metrics
            component_highlight: base.selected, // Golden yellow for selection
            base,
        }
    }
}
