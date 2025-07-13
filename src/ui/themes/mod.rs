use ratatui::style::Color;

/// K9s/Solarized Dark color constants for consistent theming
pub mod k9s_colors {
    use ratatui::style::Color;
    
    // Solarized Dark base colors (K9s default)
    pub const BASE03: Color = Color::Rgb(0, 43, 54);      // Background
    pub const BASE02: Color = Color::Rgb(7, 54, 66);      // Background highlights  
    pub const BASE01: Color = Color::Rgb(88, 110, 117);   // Content tones
    pub const BASE00: Color = Color::Rgb(101, 123, 131);  // 
    pub const BASE0: Color = Color::Rgb(131, 148, 150);   // Main text
    pub const BASE1: Color = Color::Rgb(147, 161, 161);   // Secondary text
    
    // Solarized accent colors (K9s highlights)
    pub const YELLOW: Color = Color::Rgb(181, 137, 0);    // Selection/focus
    pub const ORANGE: Color = Color::Rgb(203, 75, 22);    // Warnings/highlights
    pub const RED: Color = Color::Rgb(220, 50, 47);       // Errors/critical
    pub const MAGENTA: Color = Color::Rgb(211, 54, 130);  // Status/highlights
    pub const VIOLET: Color = Color::Rgb(108, 113, 196);  // Special elements
    pub const BLUE: Color = Color::Rgb(38, 139, 210);     // Information
    pub const CYAN: Color = Color::Rgb(42, 161, 152);     // Headers/borders
    pub const GREEN: Color = Color::Rgb(133, 153, 0);     // Success/healthy
}

/// Sophisticated color palette for sleek terminal interface
/// Based on modern terminal colorschemes with excellent contrast and readability
#[derive(Clone, Debug)]
pub struct UnifiedTheme {
    /// Core interface colors
    pub primary: Color,           // Main text and headers
    pub secondary: Color,         // Secondary text and descriptions  
    pub tertiary: Color,          // Subtle text and metadata
    pub background: Color,        // Background color
    
    /// Interactive elements
    pub accent: Color,            // Highlights and active elements
    pub focused: Color,           // Focused panel borders and highlights
    pub selected: Color,          // Selected items background
    pub selected_text: Color,     // Selected items text
    
    /// Status colors with subtle sophistication
    pub success: Color,           // Success states and positive metrics
    pub warning: Color,           // Warning states and caution
    pub error: Color,             // Error states and critical alerts
    pub info: Color,              // Information and neutral status
    
    /// Structural elements  
    pub border: Color,            // Default borders and separators
    pub border_focused: Color,    // Focused panel borders
    pub separator: Color,         // Content separators and dividers
    pub muted: Color,             // Muted text and disabled elements
    
    /// Data visualization
    pub chart_primary: Color,     // Primary chart color
    pub chart_secondary: Color,   // Secondary chart color
    pub chart_accent: Color,      // Chart accent color
    pub progress_bg: Color,       // Progress bar background
    pub progress_fg: Color,       // Progress bar foreground
}

impl UnifiedTheme {
    /// K9s-inspired theme using exact Solarized Dark colors for authentic K9s experience
    pub fn default() -> Self {
        Self {
            // Core interface - K9s/Solarized Dark text hierarchy
            primary: k9s_colors::BASE0,                    // Main text (light gray)
            secondary: k9s_colors::BASE01,                 // Secondary text (medium gray)
            tertiary: k9s_colors::BASE00,                  // Metadata text (darker gray)
            background: k9s_colors::BASE03,                // Dark background
            
            // Interactive elements - K9s signature colors
            accent: k9s_colors::CYAN,                      // Headers and borders (cyan)
            focused: k9s_colors::YELLOW,                   // Focus indicators (yellow)
            selected: k9s_colors::YELLOW,                  // Selection background (yellow)
            selected_text: k9s_colors::BASE03,             // Selected text (dark on yellow)
            
            // Status colors - K9s semantic coloring
            success: k9s_colors::GREEN,                    // Healthy/running states
            warning: k9s_colors::ORANGE,                   // Warning states
            error: k9s_colors::RED,                        // Error/critical states
            info: k9s_colors::BLUE,                        // Information status
            
            // Structural elements - K9s interface
            border: k9s_colors::BASE01,                    // Default borders
            border_focused: k9s_colors::YELLOW,            // Focused borders (yellow)
            separator: k9s_colors::BASE01,                 // Content separators
            muted: k9s_colors::BASE00,                     // Disabled/muted elements
            
            // Data visualization - K9s chart colors
            chart_primary: k9s_colors::CYAN,               // Primary data lines
            chart_secondary: k9s_colors::BLUE,             // Secondary data
            chart_accent: k9s_colors::MAGENTA,             // Accent data points
            progress_bg: k9s_colors::BASE02,               // Progress backgrounds
            progress_fg: k9s_colors::CYAN,                 // Progress indicators
        }
    }
    
    /// High contrast theme for better accessibility
    pub fn high_contrast() -> Self {
        Self {
            primary: Color::White,
            secondary: Color::Rgb(204, 204, 204),
            tertiary: Color::Rgb(153, 153, 153),
            background: Color::Black,
            
            accent: Color::Cyan,
            focused: Color::Yellow,
            selected: Color::Blue,
            selected_text: Color::White,
            
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Cyan,
            
            border: Color::Gray,
            border_focused: Color::Yellow,
            separator: Color::DarkGray,
            muted: Color::DarkGray,
            
            chart_primary: Color::Cyan,
            chart_secondary: Color::Magenta,
            chart_accent: Color::Yellow,
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
}

/// Theme variants for different contexts
#[derive(Clone, Debug)]
pub enum ThemeVariant {
    Default,
    HighContrast,
    Monochrome,
}

impl ThemeVariant {
    pub fn get_theme(&self) -> UnifiedTheme {
        match self {
            ThemeVariant::Default => UnifiedTheme::default(),
            ThemeVariant::HighContrast => UnifiedTheme::high_contrast(),
            ThemeVariant::Monochrome => UnifiedTheme::monochrome(),
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
    /// Service selection theme - k9s style cyan accent
    pub fn service_list(base: UnifiedTheme) -> Self {
        Self {
            component_accent: base.accent,      // Cyan for service lists
            component_highlight: base.focused,  // Yellow for selection
            base,
        }
    }
    
    /// Instance list theme - k9s style green for healthy instances
    pub fn instance_list(base: UnifiedTheme) -> Self {
        Self {
            component_accent: base.success,     // Green for instances
            component_highlight: base.focused,  // Yellow for selection
            base,
        }
    }
    
    /// Time range selection theme - k9s style yellow accent
    pub fn time_range(base: UnifiedTheme) -> Self {
        Self {
            component_accent: base.warning,     // Yellow for time controls
            component_highlight: base.focused,  // Yellow for selection
            base,
        }
    }
    
    /// Metrics and charts theme - k9s style cyan for data
    pub fn metrics(base: UnifiedTheme) -> Self {
        Self {
            component_accent: base.info,        // Cyan for metrics
            component_highlight: base.focused,  // Yellow for selection
            base,
        }
    }
} 