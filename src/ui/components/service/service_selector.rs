use crate::models::AwsService;
use crate::ui::components::list_styling::{
    ListItemBuilder, TypeIndicator, LayoutStyle, BadgeType,
    themes::service_list_colors,
    utilities::{create_border_style, create_highlight_style, create_k9s_service_item},
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

/// Render service selection list with enhanced styling
pub fn render_service_selection_list(
    f: &mut Frame,
    area: Rect,
    services: &[AwsService],
    list_state: &mut ListState,
    is_focused: bool,
) {
    let colors = service_list_colors();
    let selected_index = list_state.selected().unwrap_or(0);

    let items: Vec<ListItem> = services
        .iter()
        .enumerate()
        .map(|(index, service)| {
            let is_selected = index == selected_index;
            
            // Use k9s-style consistent formatting
            create_k9s_service_item(
                service.short_name(),
                match service {
                    crate::models::AwsService::Rds => "DATABASE",
                    crate::models::AwsService::Sqs => "QUEUE",
                },
                service.display_name(),
                is_selected,
                is_focused,
                &colors,
            )
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("AWS Services")
                .border_style(create_border_style(is_focused, &colors)),
        )
        .style(Style::default().fg(colors.primary))
        .highlight_style(create_highlight_style(&colors));

    f.render_stateful_widget(list, area, list_state);
}

/// Render service details
pub fn render_service_details(f: &mut Frame, area: Rect, service: &AwsService, is_focused: bool) {
    let colors = service_list_colors();
    
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Service Details")
        .border_style(create_border_style(is_focused, &colors));

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
