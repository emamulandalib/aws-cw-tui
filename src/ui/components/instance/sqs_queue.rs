use crate::models::SqsQueue;
use crate::ui::components::list_styling::{
    ListItemBuilder, TypeIndicator, LayoutStyle, BadgeType,
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

/// Render SQS queue details
pub fn render_sqs_queue_details(f: &mut Frame, area: Rect, queue: &SqsQueue, is_focused: bool) {
    let theme = UnifiedTheme::default();
    let border_color = if is_focused {
        theme.border_focused
    } else {
        theme.border
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title("SQS Queue Details")
        .border_style(Style::default().fg(border_color));

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(inner_area);

    // Queue Name
    let name_widget = Paragraph::new(format!("Name: {}", queue.name))
        .style(
            Style::default()
                .fg(theme.success)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Left);
    f.render_widget(name_widget, chunks[0]);

    // Queue Type
    let type_color = match queue.queue_type.as_str() {
        "FIFO" => theme.info,
        "Standard" => theme.accent,
        _ => theme.muted,
    };
    let type_widget = Paragraph::new(format!("Type: {}", queue.queue_type))
        .style(Style::default().fg(type_color))
        .alignment(Alignment::Left);
    f.render_widget(type_widget, chunks[1]);

    // URL
    let url_widget = Paragraph::new(format!("URL: {}", queue.url))
        .style(Style::default().fg(theme.chart_secondary))
        .alignment(Alignment::Left);
    f.render_widget(url_widget, chunks[2]);

    // Attributes (if any)
    if !queue.attributes.is_empty() {
        let attr_count = queue.attributes.len();
        let attr_widget = Paragraph::new(format!("Attributes: {} configured", attr_count))
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Left);
        f.render_widget(attr_widget, chunks[3]);
    }
}

/// Render SQS queue list item with k9s-style consistent formatting
pub fn render_sqs_queue_list_item(queue: &SqsQueue, is_selected: bool) -> ListItem {
    let colors = instance_list_colors();

    // Use k9s-style consistent formatting
    // For SQS queues, we'll show them as "available" since they're active resources
    create_k9s_instance_item(
        &queue.name,
        "available", // SQS queues are always available when listed
        Some(&queue.queue_type),
        Some("AWS SQS"),
        is_selected,
        false, // Not focused in list context
        &colors,
    )
}
