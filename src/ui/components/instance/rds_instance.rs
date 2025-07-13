use crate::models::RdsInstance;
use crate::ui::components::list_styling::{
    ListItemBuilder, StatusIndicator, TypeIndicator, LayoutStyle,
    themes::instance_list_colors,
    utilities::create_k9s_instance_item,
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
) {
    let theme = UnifiedTheme::default();
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
    let status_color = match instance.status.as_str() {
        "available" => theme.success,
        "stopped" => theme.error,
        "starting" | "stopping" => theme.warning,
        _ => theme.muted,
    };
    let status_widget = Paragraph::new(format!("Status: {}", instance.status))
        .style(Style::default().fg(status_color))
        .alignment(Alignment::Left);
    f.render_widget(status_widget, chunks[2]);

    // Instance Class
    let class_widget = Paragraph::new(format!("Class: {}", instance.instance_class))
        .style(Style::default().fg(theme.accent))
        .alignment(Alignment::Left);
    f.render_widget(class_widget, chunks[3]);

    // Endpoint (if available)
    if let Some(endpoint) = &instance.endpoint {
        let endpoint_widget = Paragraph::new(format!("Endpoint: {}", endpoint))
            .style(Style::default().fg(theme.chart_accent))
            .alignment(Alignment::Left);
        f.render_widget(endpoint_widget, chunks[4]);
    }
}

/// Render RDS instance list item with k9s-style consistent formatting
pub fn render_rds_instance_list_item(instance: &RdsInstance, is_selected: bool) -> ListItem {
    let colors = instance_list_colors();

    // Use k9s-style consistent formatting
    create_k9s_instance_item(
        &instance.identifier,
        &instance.status,
        Some(&instance.instance_class),
        Some(&instance.engine),
        is_selected,
        false, // Not focused in list context
        &colors,
    )
}
