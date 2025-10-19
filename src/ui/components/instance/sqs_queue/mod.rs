//! SQS Queue UI components

use crate::ui::themes::UnifiedTheme;
use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Render SQS queue details
pub fn render_sqs_queue_details(
    f: &mut Frame,
    area: Rect,
    queue_name: &str,
    is_focused: bool,
    theme: &UnifiedTheme,
) {
    let block = if is_focused {
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.selected))
    } else {
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.secondary))
    };

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let paragraph = Paragraph::new(format!("SQS Queue: {}", queue_name))
        .style(Style::default().fg(theme.primary));

    f.render_widget(paragraph, inner_area);
}

/// Render SQS queue list item
pub fn render_sqs_queue_list_item(
    queue_name: &str,
    is_selected: bool,
    theme: &UnifiedTheme,
) -> ratatui::widgets::ListItem<'static> {
    let style = if is_selected {
        Style::default().fg(theme.selected_text).bg(theme.selected)
    } else {
        Style::default().fg(theme.primary)
    };

    ratatui::widgets::ListItem::new(queue_name.to_string()).style(style)
}

/// Render SQS queue list item with metrics
pub fn render_sqs_queue_list_item_with_metrics(
    queue_name: &str,
    message_count: u64,
    is_selected: bool,
    theme: &UnifiedTheme,
) -> ratatui::widgets::ListItem<'static> {
    let text = format!("{} ({} messages)", queue_name, message_count);
    let style = if is_selected {
        Style::default().fg(theme.selected_text).bg(theme.selected)
    } else {
        Style::default().fg(theme.primary)
    };

    ratatui::widgets::ListItem::new(text).style(style)
}
