use crate::ui::components::list_styling::{
    border_factory, BadgeType, LayoutStyle, ListItemBuilder, StatusIndicator, TypeIndicator,
};
use crate::ui::themes::{ThemeVariant, UnifiedTheme};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

/// Render style guide for theme demonstration and testing
pub fn render_style_guide(f: &mut Frame, area: Rect, theme_variant: &ThemeVariant) {
    let theme = theme_variant.to_theme();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Content
        ])
        .split(area);

    render_style_guide_header(f, chunks[0], &theme);
    render_style_guide_content(f, chunks[1], &theme);
}

fn render_style_guide_header(f: &mut Frame, area: Rect, theme: &UnifiedTheme) {
    let header = Paragraph::new("AWS CloudWatch TUI - Warm Sunset Theme")
        .style(
            Style::default()
                .fg(theme.primary)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Warm Sunset Vibrant Interface")
                .border_style(border_factory::create_theme_border_style(theme, false)),
        );
    f.render_widget(header, area);
}

fn render_style_guide_content(f: &mut Frame, area: Rect, theme: &UnifiedTheme) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33), // Color Palette
            Constraint::Percentage(33), // Component Demos
            Constraint::Percentage(34), // Layout Examples
        ])
        .split(area);

    render_color_palette(f, chunks[0], theme);
    render_component_demos(f, chunks[1], theme);
    render_layout_examples(f, chunks[2], theme);
}

fn render_color_palette(f: &mut Frame, area: Rect, theme: &UnifiedTheme) {
    let color_items = vec![
        ListItemBuilder::new()
            .add_colored_text("Warm White".to_string(), theme.primary)
            .add_colored_text(
                " - Main text (warm and readable)".to_string(),
                theme.secondary,
            )
            .build(),
        ListItemBuilder::new()
            .add_colored_text("Warm Cream".to_string(), theme.secondary)
            .add_colored_text(" - Secondary information".to_string(), theme.tertiary)
            .build(),
        ListItemBuilder::new()
            .add_colored_text("Golden Yellow".to_string(), theme.accent)
            .add_colored_text(" - Headers and accents".to_string(), theme.secondary)
            .build(),
        ListItemBuilder::new()
            .add_colored_text("Vibrant Orange".to_string(), theme.focused)
            .add_colored_text(" - Focus and highlights".to_string(), theme.secondary)
            .build(),
        ListItemBuilder::new()
            .add_colored_text("Success Green".to_string(), theme.success)
            .add_colored_text(" - Running/healthy status".to_string(), theme.secondary)
            .build(),
        ListItemBuilder::new()
            .add_colored_text("Vibrant Orange".to_string(), theme.warning)
            .add_colored_text(" - Warnings and caution".to_string(), theme.secondary)
            .build(),
        ListItemBuilder::new()
            .add_colored_text("Coral Red".to_string(), theme.error)
            .add_colored_text(" - Error states".to_string(), theme.secondary)
            .build(),
        ListItemBuilder::new()
            .add_colored_text("Info Blue".to_string(), theme.info)
            .add_colored_text(
                " - Information and neutral status".to_string(),
                theme.secondary,
            )
            .build(),
        ListItemBuilder::new()
            .add_colored_text("Chart Primary".to_string(), theme.chart_primary)
            .add_colored_text(
                " - Primary data visualization (golden)".to_string(),
                theme.secondary,
            )
            .build(),
        ListItemBuilder::new()
            .add_colored_text("Chart Secondary".to_string(), theme.chart_secondary)
            .add_colored_text(
                " - Secondary data lines (orange)".to_string(),
                theme.secondary,
            )
            .build(),
        ListItemBuilder::new()
            .add_colored_text("Chart Accent".to_string(), theme.chart_accent)
            .add_colored_text(" - Accent data points (coral)".to_string(), theme.secondary)
            .build(),
    ];

    let list = List::new(color_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Warm Sunset Color Palette")
            .border_style(border_factory::create_theme_border_style(theme, false)),
    );

    f.render_widget(list, area);
}

fn render_component_demos(f: &mut Frame, area: Rect, theme: &UnifiedTheme) {
    let component_items = vec![
        // Status indicators demo
        ListItemBuilder::new()
            .add_status_indicator(StatusIndicator::Available)
            .add_primary_text(" Service Online".to_string())
            .build(),
        ListItemBuilder::new()
            .add_status_indicator(StatusIndicator::Warning)
            .add_primary_text(" High CPU Usage".to_string())
            .build(),
        ListItemBuilder::new()
            .add_status_indicator(StatusIndicator::Error)
            .add_primary_text(" Connection Failed".to_string())
            .build(),
        // Type indicators demo
        ListItemBuilder::new()
            .add_type_indicator(TypeIndicator::Database)
            .add_primary_text(" RDS Instance".to_string())
            .build(),
        ListItemBuilder::new()
            .add_type_indicator(TypeIndicator::Queue)
            .add_primary_text(" SQS Queue".to_string())
            .build(),
        // Badge demo
        ListItemBuilder::new()
            .add_badge("PRODUCTION".to_string(), BadgeType::Error)
            .add_primary_text(" Critical Environment".to_string())
            .build(),
        ListItemBuilder::new()
            .add_badge("STAGING".to_string(), BadgeType::Warning)
            .add_primary_text(" Test Environment".to_string())
            .build(),
        // Mixed content demo
        ListItemBuilder::new()
            .add_primary_text("Instance".to_string())
            .add_visual_separator()
            .add_secondary_text("my-database-01".to_string())
            .add_right_aligned_text("Running".to_string(), theme.success)
            .build(),
    ];

    let list = List::new(component_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Component Examples")
            .border_style(border_factory::create_theme_border_style(theme, false)),
    );

    f.render_widget(list, area);
}

fn render_layout_examples(f: &mut Frame, area: Rect, theme: &UnifiedTheme) {
    let layout_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(25), // Standard
            Constraint::Percentage(25), // Enhanced
            Constraint::Percentage(25), // Compact
            Constraint::Percentage(25), // Card
        ])
        .split(area);

    // Standard layout
    let standard_item = vec![ListItemBuilder::new()
        .with_layout_style(LayoutStyle::Standard)
        .add_primary_text("Standard Layout".to_string())
        .add_secondary_text(" - Single line".to_string())
        .build()];

    let standard_list = List::new(standard_item).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Standard")
            .border_style(border_factory::create_theme_border_style(theme, false)),
    );
    f.render_widget(standard_list, layout_chunks[0]);

    // Enhanced layout
    let enhanced_item = vec![ListItemBuilder::new()
        .with_layout_style(LayoutStyle::Enhanced)
        .add_primary_text("Enhanced Layout".to_string())
        .add_secondary_text(" - With spacing".to_string())
        .build()];

    let enhanced_list = List::new(enhanced_item).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Enhanced")
            .border_style(border_factory::create_theme_border_style(theme, false)),
    );
    f.render_widget(enhanced_list, layout_chunks[1]);

    // Compact layout
    let compact_item = vec![ListItemBuilder::new()
        .with_layout_style(LayoutStyle::Compact)
        .add_primary_text("Compact".to_string())
        .add_secondary_text(" Dense".to_string())
        .build()];

    let compact_list = List::new(compact_item).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Compact")
            .border_style(border_factory::create_theme_border_style(theme, false)),
    );
    f.render_widget(compact_list, layout_chunks[2]);

    // Card layout
    let card_item = vec![ListItemBuilder::new()
        .with_layout_style(LayoutStyle::Card)
        .with_width(30)
        .add_primary_text("Card Layout".to_string())
        .add_secondary_text(" - Bordered".to_string())
        .build()];

    let card_list = List::new(card_item).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Card")
            .border_style(border_factory::create_theme_border_style(theme, false)),
    );
    f.render_widget(card_list, layout_chunks[3]);
}

/// Demo function to showcase all three theme variants
pub fn render_theme_comparison(f: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(area);

    render_style_guide(f, chunks[0], &ThemeVariant::Default);
    render_style_guide(f, chunks[1], &ThemeVariant::HighContrast);
    render_style_guide(f, chunks[2], &ThemeVariant::Monochrome);
    render_style_guide(f, chunks[3], &ThemeVariant::TerminalCyan);
}
