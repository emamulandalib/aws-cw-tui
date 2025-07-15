use crate::models::AwsService;
use crate::ui::components::list_styling::{
    ListItemBuilder, StatusIndicator, TypeIndicator, LayoutStyle,
    themes::service_list_colors_with_theme,
    utilities::create_highlight_style,
    border_factory::create_theme_border_style,
};
use crate::ui::themes::UnifiedTheme;
use ratatui::{
    layout::{Rect},
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
            let builder = ListItemBuilder::new()
                .with_colors(colors.clone())
                .selected(is_selected)
                .focused(is_focused)
                .add_status_indicator(StatusIndicator::Available)
                .add_type_indicator(TypeIndicator::Service)
                .add_primary_text(service.display_name().to_string())
                .with_layout_style(LayoutStyle::Standard);

            match service {
                AwsService::Rds => builder.add_secondary_text("Relational Database Service".to_string()),
                AwsService::Sqs => builder.add_secondary_text("Simple Queue Service".to_string()),
            }
            .build()
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("AWS Services")
                .border_style(create_theme_border_style(theme, false)), // Always use unfocused border for service list
        )
        .highlight_style(create_highlight_style(&colors));

    f.render_stateful_widget(list, area, list_state);
}

pub fn render_service_details(f: &mut Frame, area: Rect, service: &AwsService, is_focused: bool, theme: &UnifiedTheme) {
    let colors = service_list_colors_with_theme(theme);
    
    let items = vec![
        ListItemBuilder::new()
            .with_colors(colors.clone())
            .add_primary_text(format!("Service: {}", service.display_name()))
            .build(),
        ListItemBuilder::new()
            .with_colors(colors.clone())
            .add_primary_text(format!("Type: {}", service.short_name()))
            .build(),
        ListItemBuilder::new()
            .with_colors(colors.clone())
            .add_primary_text("Description: AWS Cloud Service".to_string())
            .build(),
    ];

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Service Details")
                .border_style(create_theme_border_style(theme, false)), // Always use unfocused border for service details
        );

    f.render_widget(list, area);
}
