use crate::models::AwsService;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

/// Render service selection list
pub fn render_service_selection_list(
    f: &mut Frame,
    area: Rect,
    services: &[AwsService],
    list_state: &mut ListState,
    is_focused: bool,
) {
    let border_color = if is_focused {
        Color::Yellow
    } else {
        Color::White
    };

    let items: Vec<ListItem> = services
        .iter()
        .map(|service| {
            let content = format!("{} - {}", service.short_name(), service.display_name());
            ListItem::new(content)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("AWS Services")
                .border_style(Style::default().fg(border_color)),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(list, area, list_state);
}

/// Render service details
pub fn render_service_details(f: &mut Frame, area: Rect, service: &AwsService, is_focused: bool) {
    let border_color = if is_focused {
        Color::Yellow
    } else {
        Color::White
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Service Details")
        .border_style(Style::default().fg(border_color));

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(inner_area);

    // Service name
    let name_widget = Paragraph::new(format!("Service: {}", service.short_name()))
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Left);
    f.render_widget(name_widget, chunks[0]);

    // Full description
    let desc_widget = Paragraph::new(service.display_name())
        .style(Style::default().fg(Color::Blue))
        .alignment(Alignment::Left);
    f.render_widget(desc_widget, chunks[1]);

    // Additional information based on service type
    let info_text = match service {
        AwsService::Rds => "Monitor database instances, connections, and performance metrics",
        AwsService::Sqs => "Monitor queue depth, message throughput, and processing metrics",
    };

    let info_widget = Paragraph::new(info_text)
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Left);
    f.render_widget(info_widget, chunks[2]);
}
