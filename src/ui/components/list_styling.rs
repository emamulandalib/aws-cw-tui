use crate::ui::themes::{ComponentTheme, UnifiedTheme};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::ListItem,
};

/// Legacy compatibility layer - will be phased out
#[derive(Clone)]
pub struct ListColors {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,
    pub muted: Color,
    pub background: Color,
    pub selected: Color,
    pub focused: Color,
    pub separator: Color,
    pub progress_bg: Color,
    pub progress_fg: Color,
    pub highlight: Color,
    pub dim: Color,
    pub border: Color,
}

impl ListColors {
    pub fn new() -> Self {
        let theme = UnifiedTheme::default();
        Self::from_theme(&theme)
    }

    /// Convert from unified theme for backward compatibility
    pub fn from_theme(theme: &UnifiedTheme) -> Self {
        Self {
            primary: theme.primary,
            secondary: theme.secondary,
            accent: theme.accent,
            success: theme.success,
            warning: theme.warning,
            error: theme.error,
            info: theme.info,
            muted: theme.muted,
            background: theme.background,
            selected: theme.selected_text, // Black text on yellow background
            focused: theme.focused,        // Yellow for focus
            separator: theme.separator,
            progress_bg: theme.progress_bg,
            progress_fg: theme.progress_fg,
            highlight: theme.selected, // Yellow background for selection
            dim: theme.tertiary,
            border: theme.border, // Add border color from theme
        }
    }
}

/// Status indicators using descriptive text instead of symbols
pub enum StatusIndicator {
    Available,
    Stopped,
    Starting,
    Stopping,
    Error,
    Warning,
    Info,
    Unknown,
    None,
}

impl StatusIndicator {
    pub fn to_text(&self) -> &'static str {
        match self {
            StatusIndicator::Available => "ONLINE",
            StatusIndicator::Stopped => "STOPPED",
            StatusIndicator::Starting => "STARTING",
            StatusIndicator::Stopping => "STOPPING",
            StatusIndicator::Error => "ERROR",
            StatusIndicator::Warning => "WARNING",
            StatusIndicator::Info => "INFO",
            StatusIndicator::Unknown => "UNKNOWN",
            StatusIndicator::None => "",
        }
    }

    pub fn color(&self, colors: &ListColors) -> Color {
        match self {
            StatusIndicator::Available => colors.success,
            StatusIndicator::Stopped => colors.error,
            StatusIndicator::Starting | StatusIndicator::Stopping => colors.warning,
            StatusIndicator::Error => colors.error,
            StatusIndicator::Warning => colors.warning,
            StatusIndicator::Info => colors.info,
            StatusIndicator::Unknown => colors.muted,
            StatusIndicator::None => colors.primary,
        }
    }
}

/// Type indicators for different service types
pub enum TypeIndicator {
    Fifo,
    Standard,
    Database,
    Queue,
    Service,
    Other(String),
}

impl TypeIndicator {
    pub fn to_text(&self) -> String {
        match self {
            TypeIndicator::Fifo => "FIFO".to_string(),
            TypeIndicator::Standard => "STANDARD".to_string(),
            TypeIndicator::Database => "DATABASE".to_string(),
            TypeIndicator::Queue => "QUEUE".to_string(),
            TypeIndicator::Service => "SERVICE".to_string(),
            TypeIndicator::Other(text) => text.to_uppercase(),
        }
    }

    pub fn color(&self, colors: &ListColors) -> Color {
        match self {
            TypeIndicator::Fifo => colors.info,
            TypeIndicator::Standard => colors.accent,
            TypeIndicator::Database => colors.success,
            TypeIndicator::Queue => colors.warning,
            TypeIndicator::Service => colors.primary,
            TypeIndicator::Other(_) => colors.muted,
        }
    }
}

/// Enhanced list item builder for consistent styling
pub struct ListItemBuilder {
    colors: ListColors,
    content_parts: Vec<ContentPart>,
    is_selected: bool,
    is_focused: bool,
    progress_info: Option<ProgressInfo>,
    layout_style: LayoutStyle,
    width: Option<usize>,
    show_separator: bool,
}

struct ContentPart {
    text: String,
    color: Color,
    modifier: Option<Modifier>,
    alignment: Alignment,
}

#[derive(Clone)]
struct ProgressInfo {
    current: usize,
    total: usize,
    show_percentage: bool,
    bar_width: usize,
}

#[derive(Clone)]
pub enum LayoutStyle {
    /// Standard single-line layout
    Standard,
    /// Multi-line layout with enhanced spacing
    Enhanced,
    /// Compact layout for dense information
    Compact,
    /// Card-like layout with visual grouping
    Card,
}

#[derive(Clone)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

#[derive(Clone)]
pub enum BadgeType {
    Success,
    Warning,
    Error,
    Info,
    Primary,
}

impl ListItemBuilder {
    pub fn new() -> Self {
        Self {
            colors: ListColors::new(),
            content_parts: Vec::new(),
            is_selected: false,
            is_focused: false,
            progress_info: None,
            layout_style: LayoutStyle::Standard,
            width: None,
            show_separator: false,
        }
    }

    pub fn with_colors(mut self, colors: ListColors) -> Self {
        self.colors = colors;
        self
    }

    pub fn selected(mut self, is_selected: bool) -> Self {
        self.is_selected = is_selected;
        self
    }

    pub fn focused(mut self, is_focused: bool) -> Self {
        self.is_focused = is_focused;
        self
    }

    pub fn add_status_indicator(mut self, status: StatusIndicator) -> Self {
        if !matches!(status, StatusIndicator::None) {
            self.content_parts.push(ContentPart {
                text: format!("[{}]", status.to_text()),
                color: status.color(&self.colors),
                modifier: Some(Modifier::BOLD),
                alignment: Alignment::Left,
            });
        }
        self
    }

    pub fn add_type_indicator(mut self, type_indicator: TypeIndicator) -> Self {
        self.content_parts.push(ContentPart {
            text: format!("({})", type_indicator.to_text()),
            color: type_indicator.color(&self.colors),
            modifier: Some(Modifier::BOLD),
            alignment: Alignment::Left,
        });
        self
    }

    pub fn add_primary_text(mut self, text: String) -> Self {
        self.content_parts.push(ContentPart {
            text,
            color: self.colors.primary,
            modifier: Some(Modifier::BOLD),
            alignment: Alignment::Left,
        });
        self
    }

    pub fn add_secondary_text(mut self, text: String) -> Self {
        self.content_parts.push(ContentPart {
            text,
            color: self.colors.secondary,
            modifier: None,
            alignment: Alignment::Left,
        });
        self
    }

    pub fn add_accent_text(mut self, text: String) -> Self {
        self.content_parts.push(ContentPart {
            text,
            color: self.colors.accent,
            modifier: None,
            alignment: Alignment::Left,
        });
        self
    }

    pub fn add_colored_text(mut self, text: String, color: Color) -> Self {
        self.content_parts.push(ContentPart {
            text,
            color,
            modifier: None,
            alignment: Alignment::Left,
        });
        self
    }

    pub fn add_separator(mut self) -> Self {
        self.content_parts.push(ContentPart {
            text: " • ".to_string(),
            color: self.colors.muted,
            modifier: None,
            alignment: Alignment::Left,
        });
        self
    }

    pub fn add_spacer(mut self) -> Self {
        self.content_parts.push(ContentPart {
            text: " ".to_string(),
            color: self.colors.primary,
            modifier: None,
            alignment: Alignment::Left,
        });
        self
    }

    pub fn with_layout_style(mut self, style: LayoutStyle) -> Self {
        self.layout_style = style;
        self
    }

    pub fn with_width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_progress(mut self, current: usize, total: usize) -> Self {
        self.progress_info = Some(ProgressInfo {
            current,
            total,
            show_percentage: true,
            bar_width: 20,
        });
        self
    }

    pub fn with_simple_progress(mut self, current: usize, total: usize, bar_width: usize) -> Self {
        self.progress_info = Some(ProgressInfo {
            current,
            total,
            show_percentage: false,
            bar_width,
        });
        self
    }

    pub fn with_separator(mut self, show: bool) -> Self {
        self.show_separator = show;
        self
    }

    pub fn add_right_aligned_text(mut self, text: String, color: Color) -> Self {
        self.content_parts.push(ContentPart {
            text,
            color,
            modifier: None,
            alignment: Alignment::Right,
        });
        self
    }

    pub fn add_centered_text(mut self, text: String, color: Color) -> Self {
        self.content_parts.push(ContentPart {
            text,
            color,
            modifier: None,
            alignment: Alignment::Center,
        });
        self
    }

    pub fn add_badge(mut self, text: String, badge_type: BadgeType) -> Self {
        let (_bg_color, fg_color) = match badge_type {
            BadgeType::Success => (self.colors.success, Color::Black),
            BadgeType::Warning => (self.colors.warning, Color::Black),
            BadgeType::Error => (self.colors.error, Color::White),
            BadgeType::Info => (self.colors.info, Color::White),
            BadgeType::Primary => (self.colors.primary, Color::Black),
        };

        self.content_parts.push(ContentPart {
            text: format!(" {} ", text),
            color: fg_color,
            modifier: Some(Modifier::BOLD),
            alignment: Alignment::Left,
        });
        self
    }

    pub fn add_visual_separator(mut self) -> Self {
        self.content_parts.push(ContentPart {
            text: " | ".to_string(),
            color: self.colors.separator,
            modifier: None,
            alignment: Alignment::Left,
        });
        self
    }

    pub fn add_subtle_separator(mut self) -> Self {
        self.content_parts.push(ContentPart {
            text: " - ".to_string(),
            color: self.colors.dim,
            modifier: None,
            alignment: Alignment::Left,
        });
        self
    }

    pub fn build(self) -> ListItem<'static> {
        let mut lines = Vec::new();

        // Build the main content line
        let main_line = self.build_main_line();
        lines.push(main_line);

        // Add progress bar if specified
        if let Some(progress) = &self.progress_info {
            let progress_line = self.build_progress_line(progress);
            lines.push(progress_line);
        }

        // Add separator line for card layout
        if matches!(self.layout_style, LayoutStyle::Card) && self.show_separator {
            let separator_line = self.build_separator_line();
            lines.push(separator_line);
        }

        // Apply layout-specific styling
        let styled_lines = self.apply_layout_styling(lines);

        // Create the final ListItem
        let item = if styled_lines.len() == 1 {
            ListItem::new(styled_lines.into_iter().next().unwrap())
        } else {
            ListItem::new(styled_lines)
        };

        // Apply overall styling based on state
        if self.is_selected {
            item.style(
                Style::default()
                    .fg(self.colors.selected)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            )
        } else if self.is_focused {
            item.style(
                Style::default()
                    .fg(self.colors.focused)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            item
        }
    }

    fn build_main_line(&self) -> Line<'static> {
        let available_width = self.width.unwrap_or(80);
        let mut left_spans = Vec::new();
        let mut right_spans = Vec::new();
        let mut center_spans = Vec::new();

        // Separate content by alignment
        for part in &self.content_parts {
            let mut style = Style::default().fg(part.color);
            if let Some(modifier) = part.modifier {
                style = style.add_modifier(modifier);
            }
            let span = Span::styled(part.text.clone(), style);

            match part.alignment {
                Alignment::Left => left_spans.push(span),
                Alignment::Right => right_spans.push(span),
                Alignment::Center => center_spans.push(span),
            }
        }

        // Calculate text lengths
        let left_len: usize = left_spans.iter().map(|s| s.content.len()).sum();
        let right_len: usize = right_spans.iter().map(|s| s.content.len()).sum();
        let center_len: usize = center_spans.iter().map(|s| s.content.len()).sum();

        // Build the final line with proper spacing
        let mut final_spans = Vec::new();

        // Add left-aligned content
        final_spans.extend(left_spans);

        // Add center content with proper spacing
        let center_empty = center_spans.is_empty();
        if !center_empty {
            let remaining_width = available_width.saturating_sub(left_len + right_len);
            let center_padding = remaining_width.saturating_sub(center_len) / 2;

            if center_padding > 0 {
                final_spans.push(Span::raw(" ".repeat(center_padding)));
            }
            final_spans.extend(center_spans);
        }

        // Add right-aligned content
        if !right_spans.is_empty() {
            let used_width = left_len + center_len + if center_empty { 0 } else { center_len };
            let padding = available_width.saturating_sub(used_width + right_len);

            if padding > 0 {
                final_spans.push(Span::raw(" ".repeat(padding)));
            }
            final_spans.extend(right_spans);
        }

        Line::from(final_spans)
    }

    fn build_progress_line(&self, progress: &ProgressInfo) -> Line<'static> {
        let progress_ratio = if progress.total > 0 {
            progress.current as f64 / progress.total as f64
        } else {
            0.0
        };

        let filled_width = (progress.bar_width as f64 * progress_ratio) as usize;
        let empty_width = progress.bar_width.saturating_sub(filled_width);

        let mut spans = Vec::new();

        // Add progress bar
        spans.push(Span::styled(" ".to_string(), Style::default()));
        if filled_width > 0 {
            spans.push(Span::styled(
                "█".repeat(filled_width),
                Style::default().fg(self.colors.progress_fg),
            ));
        }
        if empty_width > 0 {
            spans.push(Span::styled(
                "░".repeat(empty_width),
                Style::default().fg(self.colors.progress_bg),
            ));
        }

        // Add progress text
        if progress.show_percentage {
            let percentage = (progress_ratio * 100.0) as usize;
            spans.push(Span::styled(
                format!(" {}%", percentage),
                Style::default().fg(self.colors.muted),
            ));
        } else {
            spans.push(Span::styled(
                format!(" {}/{}", progress.current, progress.total),
                Style::default().fg(self.colors.muted),
            ));
        }

        Line::from(spans)
    }

    fn build_separator_line(&self) -> Line<'static> {
        let width = self.width.unwrap_or(80);
        let separator_char = "-";

        Line::from(vec![
            Span::styled(" ".to_string(), Style::default()),
            Span::styled(
                separator_char.repeat(width.saturating_sub(2)),
                Style::default().fg(self.colors.separator),
            ),
        ])
    }

    fn apply_layout_styling(&self, lines: Vec<Line<'static>>) -> Vec<Line<'static>> {
        match self.layout_style {
            LayoutStyle::Standard => lines,
            LayoutStyle::Enhanced => {
                // Add spacing between elements
                let mut enhanced_lines = Vec::new();
                for (i, line) in lines.into_iter().enumerate() {
                    if i > 0 {
                        enhanced_lines.push(Line::from(vec![Span::raw(" ")]));
                    }
                    enhanced_lines.push(line);
                }
                enhanced_lines
            }
            LayoutStyle::Compact => {
                // Compact everything into single line if possible
                if lines.len() <= 1 {
                    lines
                } else {
                    // Combine all spans into single line
                    let mut all_spans = Vec::new();
                    for (i, line) in lines.into_iter().enumerate() {
                        if i > 0 {
                            all_spans.push(Span::raw(" "));
                        }
                        all_spans.extend(line.spans);
                    }
                    vec![Line::from(all_spans)]
                }
            }
            LayoutStyle::Card => {
                // Add visual borders/grouping using text-only characters
                let mut card_lines = Vec::new();
                let width = self.width.unwrap_or(80);

                // Top border using dashes
                card_lines.push(Line::from(vec![
                    Span::styled(" ".to_string(), Style::default()),
                    Span::styled(
                        "+".to_string() + &"-".repeat(width.saturating_sub(4)) + "+",
                        Style::default().fg(self.colors.separator),
                    ),
                ]));

                // Content with side borders using pipes
                for line in lines {
                    let mut bordered_spans = vec![Span::styled(
                        " | ".to_string(),
                        Style::default().fg(self.colors.separator),
                    )];
                    bordered_spans.extend(line.spans);
                    bordered_spans.push(Span::styled(
                        " |".to_string(),
                        Style::default().fg(self.colors.separator),
                    ));
                    card_lines.push(Line::from(bordered_spans));
                }

                // Bottom border using dashes
                card_lines.push(Line::from(vec![
                    Span::styled(" ".to_string(), Style::default()),
                    Span::styled(
                        "+".to_string() + &"-".repeat(width.saturating_sub(4)) + "+",
                        Style::default().fg(self.colors.separator),
                    ),
                ]));

                card_lines
            }
        }
    }
}

/// Enhanced border style factory utilities for theme-consistent styling
pub mod border_factory {
    use super::*;
    use crate::ui::themes::UnifiedTheme;

    /// Create theme-consistent border style based on focus state
    /// This is the primary function for creating borders across all components
    pub fn create_theme_border_style(theme: &UnifiedTheme, is_focused: bool) -> Style {
        Style::default().fg(if is_focused {
            theme.border_focused // Use theme's focused border color
        } else {
            theme.border // Use theme's default border color
        })
    }

    /// Create theme-consistent border style with custom focus color override
    /// Useful for components that need special focus highlighting
    pub fn create_theme_border_style_with_focus_override(
        theme: &UnifiedTheme,
        is_focused: bool,
        focus_color: Color,
    ) -> Style {
        Style::default().fg(if is_focused {
            focus_color // Use custom focus color
        } else {
            theme.border // Use theme's default border color
        })
    }

    /// Validate border consistency during development
    /// Returns true if the provided style uses theme-consistent colors
    pub fn validate_border_consistency(
        style: &Style,
        theme: &UnifiedTheme,
        is_focused: bool,
    ) -> bool {
        let expected_color = if is_focused {
            theme.border_focused
        } else {
            theme.border
        };

        // Check if the style's foreground color matches expected theme color
        match style.fg {
            Some(color) => color == expected_color,
            None => false, // No color set means not using theme colors
        }
    }

    /// Get the appropriate border color from theme based on focus state
    /// Utility function for components that need direct color access
    pub fn get_theme_border_color(theme: &UnifiedTheme, is_focused: bool) -> Color {
        if is_focused {
            theme.border_focused
        } else {
            theme.border
        }
    }

    /// Create theme-consistent border style for status-specific borders
    /// Used for error displays, warnings, etc.
    pub fn create_theme_status_border_style(theme: &UnifiedTheme, status: BorderStatus) -> Style {
        let color = match status {
            BorderStatus::Normal => theme.border,
            BorderStatus::Focused => theme.border_focused,
            BorderStatus::Success => theme.success,
            BorderStatus::Warning => theme.warning,
            BorderStatus::Error => theme.error,
            BorderStatus::Info => theme.info,
        };

        Style::default().fg(color)
    }

    /// Border status types for status-specific styling
    #[derive(Clone, Copy, Debug)]
    pub enum BorderStatus {
        Normal,
        Focused,
        Success,
        Warning,
        Error,
        Info,
    }
}

/// Utility functions for consistent list styling
pub mod utilities {
    use super::*;

    /// Create a consistent header style for list panels
    pub fn create_header_style(is_focused: bool, colors: &ListColors) -> Style {
        let base_color = if is_focused {
            colors.focused
        } else {
            colors.accent
        };

        Style::default().fg(base_color).add_modifier(Modifier::BOLD)
    }

    /// Create highlight for list selections (dark text on highlight, yellow background)
    pub fn create_highlight_style(colors: &ListColors) -> Style {
        Style::default()
            .fg(colors.selected) // Dark text on highlight
            .bg(colors.highlight) // Highlight background
            .add_modifier(Modifier::BOLD)
    }

    /// Create border (focus color for focused panels, border color for normal panels)
    /// DEPRECATED: Use border_factory::create_theme_border_style instead
    pub fn create_border_style(is_focused: bool, colors: &ListColors) -> Style {
        Style::default().fg(if is_focused {
            colors.focused // Focus color for focused panels
        } else {
            colors.border // Use border color for normal panels
        })
    }

    /// Format text with consistent padding and alignment
    pub fn format_list_text(text: &str, width: Option<usize>) -> String {
        match width {
            Some(w) => {
                if text.len() > w {
                    format!("{}...", &text[..w.saturating_sub(3)])
                } else {
                    format!("{:<width$}", text, width = w)
                }
            }
            None => format!("  {}", text),
        }
    }

    /// Create consistent loading/empty state messages
    pub fn create_status_message(
        message: &str,
        message_type: StatusIndicator,
        _colors: &ListColors,
    ) -> String {
        format!("[{}] {}", message_type.to_text(), message)
    }

    /// Create a list item with consistent formatting
    pub fn create_list_item(
        name: &str,
        status: StatusIndicator,
        description: Option<&str>,
        right_info: Option<&str>,
        is_selected: bool,
        is_focused: bool,
        colors: &ListColors,
    ) -> ratatui::widgets::ListItem<'static> {
        let mut builder = ListItemBuilder::new()
            .with_colors(colors.clone())
            .selected(is_selected)
            .focused(is_focused)
            .with_layout_style(LayoutStyle::Standard);

        // Add status indicator and separator only if there's a meaningful status indicator
        let has_meaningful_status = !matches!(status, StatusIndicator::None);
        builder = builder.add_status_indicator(status);

        if has_meaningful_status {
            builder = builder.add_visual_separator();
        }

        // Add main name (bold white)
        builder = builder.add_primary_text(name.to_string());

        // Add description if provided
        if let Some(desc) = description {
            builder = builder.add_visual_separator();
            builder = builder.add_secondary_text(desc.to_string());
        }

        // Add right-aligned info if provided
        if let Some(info) = right_info {
            builder = builder.add_right_aligned_text(info.to_string(), colors.secondary);
        }

        builder.build()
    }

    /// Create a simple list item without status indicators or separators (for time selections)
    pub fn create_simple_list_item(
        name: &str,
        right_info: Option<&str>,
        is_selected: bool,
        is_focused: bool,
        colors: &ListColors,
    ) -> ratatui::widgets::ListItem<'static> {
        let mut builder = ListItemBuilder::new()
            .with_colors(colors.clone())
            .selected(is_selected)
            .focused(is_focused)
            .with_layout_style(LayoutStyle::Standard);

        // Add main name (bold white)
        builder = builder.add_primary_text(name.to_string());

        // Add right-aligned info if provided
        if let Some(info) = right_info {
            builder = builder.add_right_aligned_text(info.to_string(), colors.secondary);
        }

        builder.build()
    }

    /// Create a service list item
    pub fn create_service_item(
        service_name: &str,
        service_type: &str,
        description: &str,
        is_selected: bool,
        is_focused: bool,
        colors: &ListColors,
    ) -> ratatui::widgets::ListItem<'static> {
        create_list_item(
            service_name,
            StatusIndicator::Available,
            Some(description),
            Some(service_type),
            is_selected,
            is_focused,
            colors,
        )
    }

    /// Create an instance list item
    pub fn create_instance_item(
        instance_name: &str,
        status: &str,
        instance_type: Option<&str>,
        engine_or_type: Option<&str>,
        is_selected: bool,
        is_focused: bool,
        colors: &ListColors,
    ) -> ratatui::widgets::ListItem<'static> {
        // Map string status to StatusIndicator
        let status_indicator = match status.to_lowercase().as_str() {
            "available" | "running" | "online" => StatusIndicator::Available,
            "stopped" | "offline" => StatusIndicator::Stopped,
            "starting" => StatusIndicator::Starting,
            "stopping" => StatusIndicator::Stopping,
            "error" | "failed" => StatusIndicator::Error,
            _ => StatusIndicator::Unknown,
        };

        let description = match (instance_type, engine_or_type) {
            (Some(inst_type), Some(engine)) => format!("{} - {}", inst_type, engine),
            (Some(inst_type), None) => inst_type.to_string(),
            (None, Some(engine)) => engine.to_string(),
            (None, None) => "Instance".to_string(),
        };

        create_list_item(
            instance_name,
            status_indicator,
            Some(&description),
            Some(status),
            is_selected,
            is_focused,
            colors,
        )
    }
}

/// Predefined styling themes for different list types using unified theme system
pub mod themes {
    use super::*;

    pub fn service_list_colors() -> ListColors {
        let base_theme = UnifiedTheme::default();
        let component_theme = ComponentTheme::service_list(base_theme);

        let mut colors = ListColors::from_theme(&component_theme.base);
        colors.accent = component_theme.component_accent;
        colors.highlight = component_theme.component_highlight;
        colors
    }

    pub fn service_list_colors_with_theme(theme: &UnifiedTheme) -> ListColors {
        let component_theme = ComponentTheme::service_list(theme.clone());

        let mut colors = ListColors::from_theme(&component_theme.base);
        colors.accent = component_theme.component_accent;
        colors.highlight = component_theme.component_highlight;

        colors
    }

    pub fn instance_list_colors() -> ListColors {
        let base_theme = UnifiedTheme::default();
        let component_theme = ComponentTheme::instance_list(base_theme);

        let mut colors = ListColors::from_theme(&component_theme.base);
        colors.accent = component_theme.component_accent;
        colors.highlight = component_theme.component_highlight;
        colors
    }

    pub fn instance_list_colors_with_theme(theme: &UnifiedTheme) -> ListColors {
        let component_theme = ComponentTheme::instance_list(theme.clone());

        let mut colors = ListColors::from_theme(&component_theme.base);
        colors.accent = component_theme.component_accent;
        colors.highlight = component_theme.component_highlight;

        colors
    }

    pub fn time_range_colors() -> ListColors {
        let base_theme = UnifiedTheme::default();
        let component_theme = ComponentTheme::time_range(base_theme);

        let mut colors = ListColors::from_theme(&component_theme.base);
        colors.accent = component_theme.component_accent;
        colors.highlight = component_theme.component_highlight;
        colors
    }

    pub fn time_range_colors_with_theme(theme: &UnifiedTheme) -> ListColors {
        let component_theme = ComponentTheme::time_range(theme.clone());

        let mut colors = ListColors::from_theme(&component_theme.base);
        colors.accent = component_theme.component_accent;
        colors.highlight = component_theme.component_highlight;
        colors
    }

    pub fn metrics_colors() -> ListColors {
        let base_theme = UnifiedTheme::default();
        let component_theme = ComponentTheme::metrics(base_theme);

        let mut colors = ListColors::from_theme(&component_theme.base);
        colors.accent = component_theme.component_accent;
        colors.highlight = component_theme.component_highlight;
        colors
    }

    pub fn metrics_colors_with_theme(theme: &UnifiedTheme) -> ListColors {
        let component_theme = ComponentTheme::metrics(theme.clone());

        let mut colors = ListColors::from_theme(&component_theme.base);
        colors.accent = component_theme.component_accent;
        colors.highlight = component_theme.component_highlight;
        colors
    }
}
