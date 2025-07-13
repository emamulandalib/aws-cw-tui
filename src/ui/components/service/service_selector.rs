use crate::models::AwsService;
use crate::ui::components::list_styling::{
    ListItemBuilder, StatusIndicator, TypeIndicator, LayoutStyle,
    themes::service_list_colors_with_theme,
    utilities::{create_border_style, create_highlight_style},
};
use crate::ui::themes::UnifiedTheme;
use ratatui::{
    layout::{Rect},
    style::{Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

/// Render service selection list with enhanced styling
pub fn render_service_selection_list(
    f: &mut Frame,
    area: Rect,
    services: &[AwsService],
    list_state: &mut ListState,
    is_focused: bool,
    theme: &UnifiedTheme,
) {
    let colors = service_list_colors_with_theme(theme);
    let selected_index = list_state.selected().unwrap_or(0);

    let items: Vec<ListItem> = services
        .iter()
        .enumerate()
        .map(|(index, service)| {
            let is_selected = index == selected_index;
            
            let text = format!("{}", service.display_name());
            
            ListItem::new(text)
                .style(if is_selected {
                    create_highlight_style(&colors)
                } else {
                    Style::default().fg(colors.primary)
                })
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

pub fn render_service_details(f: &mut Frame, area: Rect, service: &AwsService, is_focused: bool, theme: &UnifiedTheme) {
    let colors = service_list_colors_with_theme(theme);
    
    let type_indicator = match service {
        AwsService::Rds => TypeIndicator::Database,
        AwsService::Sqs => TypeIndicator::Queue,
    };
    
    let items = vec![
        ListItemBuilder::new()
            .add_type_indicator(type_indicator)
            .add_primary_text(service.display_name().to_string())
            .add_secondary_text(format!(" - {}", service.short_name()))
            .build(),
    ];

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Service Details")
                .border_style(create_border_style(is_focused, &colors)),
        )
        .style(Style::default().fg(colors.primary));

    f.render_widget(list, area);
}
