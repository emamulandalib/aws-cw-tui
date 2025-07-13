use crate::models::RdsInstance;
use crate::ui::components::list_styling::{
    ListItemBuilder, StatusIndicator, TypeIndicator, LayoutStyle,
    themes::instance_list_colors_with_theme,
    utilities::create_instance_item,
};
use crate::ui::themes::UnifiedTheme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, ListItem, Paragraph},
    Frame,
};

/// Render RDS instance details
pub fn render_rds_instance_details(
    f: &mut Frame,
    area: Rect,
    instance: &RdsInstance,
    is_focused: bool,
    theme: &UnifiedTheme,
) {
    let border_color = if is_focused {
        theme.border_focused
    } else {
        theme.border
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title("RDS Instance Details")
        .border_style(Style::default().fg(border_color));

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(inner_area);

    // Instance ID
    let id_widget = Paragraph::new(format!("ID: {}", instance.identifier))
        .style(
            Style::default()
                .fg(theme.success)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Left);
    f.render_widget(id_widget, chunks[0]);

    // Engine
    let engine_widget = Paragraph::new(format!("Engine: {}", instance.engine))
        .style(Style::default().fg(theme.info))
        .alignment(Alignment::Left);
    f.render_widget(engine_widget, chunks[1]);

    // Status
    let status_widget = Paragraph::new(format!("Status: {}", instance.status))
        .style(Style::default().fg(theme.secondary))
        .alignment(Alignment::Left);
    f.render_widget(status_widget, chunks[2]);

    // Endpoint
    let endpoint_widget = Paragraph::new(format!("Endpoint: {}", instance.endpoint.as_ref().unwrap_or(&"N/A".to_string())))
        .style(Style::default().fg(theme.accent))
        .alignment(Alignment::Left);
    f.render_widget(endpoint_widget, chunks[3]);
}

/// Render RDS instance as a list item
pub fn render_rds_instance_list_item<'a>(instance: &'a RdsInstance, is_selected: bool, theme: &'a UnifiedTheme) -> ListItem<'a> {
    let colors = instance_list_colors_with_theme(theme);
    
    let status_indicator = match instance.status.as_str() {
        "available" => StatusIndicator::Available,
        "stopped" => StatusIndicator::Stopped,
        "starting" => StatusIndicator::Starting,
        "stopping" => StatusIndicator::Stopping,
        _ => StatusIndicator::Unknown,
    };

    let item = ListItemBuilder::new()
        .with_layout_style(LayoutStyle::Enhanced)
        .add_status_indicator(status_indicator)
        .add_type_indicator(TypeIndicator::Database)
        .add_primary_text(instance.identifier.clone())
        .add_visual_separator()
        .add_secondary_text(format!("Engine: {}", instance.engine))
        .add_visual_separator()
        .add_secondary_text(format!("Status: {}", instance.status))
        .add_right_aligned_text(
            format!("Class: {}", instance.instance_class),
            if is_selected { colors.selected } else { colors.accent },
        )
        .build();

    item
}
