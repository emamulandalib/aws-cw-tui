use crate::models::SqsQueue;
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

/// Render SQS queue details
pub fn render_sqs_queue_details(
    f: &mut Frame,
    area: Rect,
    queue: &SqsQueue,
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
    let type_widget = Paragraph::new(format!("Type: {}", queue.queue_type))
        .style(Style::default().fg(theme.info))
        .alignment(Alignment::Left);
    f.render_widget(type_widget, chunks[1]);

    // URL
    let url_widget = Paragraph::new(format!("URL: {}", queue.url))
        .style(Style::default().fg(theme.accent))
        .alignment(Alignment::Left);
    f.render_widget(url_widget, chunks[2]);

    // Attributes (if any)
    let attr_count = queue.attributes.len();
    let attr_widget = Paragraph::new(format!("Attributes: {} configured", attr_count))
        .style(Style::default().fg(theme.secondary))
        .alignment(Alignment::Left);
    f.render_widget(attr_widget, chunks[3]);
}

/// Render SQS queue as a list item
pub fn render_sqs_queue_list_item<'a>(queue: &'a SqsQueue, is_selected: bool, theme: &'a UnifiedTheme) -> ListItem<'a> {
    let colors = instance_list_colors_with_theme(theme);
    
    let status_indicator = StatusIndicator::Available; // SQS queues are always available when listed

    let item = ListItemBuilder::new()
        .with_layout_style(LayoutStyle::Enhanced)
        .add_status_indicator(status_indicator)
        .add_type_indicator(TypeIndicator::Queue)
        .add_primary_text(queue.name.clone())
        .add_visual_separator()
        .add_secondary_text(format!("Type: {}", queue.queue_type))
        .add_visual_separator()
        .add_secondary_text("Status: Available".to_string())
        .add_right_aligned_text(
            "SQS Queue".to_string(),
            if is_selected { colors.selected } else { colors.accent },
        )
        .build();

    item
}
