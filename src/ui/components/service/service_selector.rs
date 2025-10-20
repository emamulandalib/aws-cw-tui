use crate::models::aws_services::AwsService;
use crate::ui::themes::UnifiedTheme;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

/// Render service selection list
pub fn render_service_selection_list(
    f: &mut Frame,
    area: Rect,
    services: &[AwsService],
    list_state: &mut ListState,
    is_focused: bool,
    theme: &UnifiedTheme,
) {
    let items: Vec<ListItem> = services
        .iter()
        .enumerate()
        .map(|(i, service)| {
            let style = if i == list_state.selected().unwrap_or(0) && is_focused {
                Style::default()
                    .fg(theme.primary)
                    .add_modifier(Modifier::BOLD)
            } else if i == list_state.selected().unwrap_or(0) {
                Style::default().fg(theme.primary)
            } else {
                Style::default().fg(theme.secondary)
            };

            ListItem::new(service.display_name()).style(style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(if is_focused {
                Style::default().fg(theme.primary)
            } else {
                Style::default().fg(theme.secondary)
            }),
    );

    f.render_stateful_widget(list, area, list_state);
}
