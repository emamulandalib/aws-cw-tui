use crate::models::App;
use crate::ui::components::list_styling::{
    border_factory::create_theme_border_style, themes::instance_list_colors_with_theme,
    utilities::create_highlight_style,
};
use crate::ui::components::{
    render_rds_instance_list_item, render_sqs_queue_list_item, UniversalBox,
};
use crate::ui::themes::UnifiedTheme;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::{
        Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
    },
    Frame,
};

pub fn render_rds_list(f: &mut Frame, app: &mut App, theme: &UnifiedTheme) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Content
            Constraint::Length(1), // Controls at bottom
        ])
        .split(f.area());

    // Check for errors first
    if let Some(error_msg) = &app.error_message {
        render_error_message(f, chunks[0], error_msg, theme);
    } else if app.loading {
        render_loading_message(f, chunks[0], app, theme);
    } else if app.get_current_instances().is_empty() {
        render_no_instances_message(f, chunks[0], app, theme);
    } else {
        render_instances_list(f, chunks[0], app, theme);
    }

    render_controls(f, chunks[1], theme);
}

fn render_loading_message(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    _app: &App,
    theme: &UnifiedTheme,
) {
    let loading_text = "Loading instances...\n\nThis may take a few moments.";

    UniversalBox::loading_box("", loading_text, theme.clone()).render(f, area);
}

fn render_error_message(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    error_msg: &str,
    theme: &UnifiedTheme,
) {
    UniversalBox::error_box("Error", error_msg, theme.clone()).render(f, area);
}

fn render_no_instances_message(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    app: &App,
    theme: &UnifiedTheme,
) {
    let service_name = app
        .selected_service
        .as_ref()
        .map(|s| s.short_name())
        .unwrap_or("Service");

    let no_instances_text = format!("No {} instances found.", service_name);
    let suggestion = "Check your AWS credentials and try again.";

    UniversalBox::new(theme.clone())
        .empty_with_suggestion(no_instances_text, suggestion)
        .render(f, area);
}

fn render_instances_list(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    app: &mut App,
    theme: &UnifiedTheme,
) {
    let colors = instance_list_colors_with_theme(theme);

    // Clone the instances to avoid borrowing issues
    let current_instances = app.get_current_instances().clone();

    // Create items from instances using pure service-specific components
    let selected_index = app.list_state.selected().unwrap_or(0);
    let items: Vec<ListItem> = current_instances
        .iter()
        .enumerate()
        .map(|(index, service_instance)| match service_instance {
            crate::models::ServiceInstance::Rds(instance) => {
                render_rds_instance_list_item(instance, index == selected_index, theme)
            }
            crate::models::ServiceInstance::Sqs(queue) => {
                render_sqs_queue_list_item(&queue.name, index == selected_index, theme)
            }
        })
        .collect();

    let items_list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(create_theme_border_style(theme, false)), // Always use unfocused border for instance list
        )
        .style(Style::default().fg(colors.primary))
        .highlight_style(create_highlight_style(&colors));

    f.render_stateful_widget(items_list, area, &mut app.list_state);

    // Add scrollbar if there are more instances than can fit on screen
    if current_instances.len() > area.height.saturating_sub(2) as usize {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));
        let mut scrollbar_state = ScrollbarState::new(current_instances.len())
            .position(app.list_state.selected().unwrap_or(0));
        f.render_stateful_widget(
            scrollbar,
            area.inner(ratatui::layout::Margin {
                horizontal: 0,
                vertical: 1,
            }),
            &mut scrollbar_state,
        );
    }
}

fn render_controls(f: &mut Frame, area: ratatui::layout::Rect, theme: &UnifiedTheme) {
    let controls = Paragraph::new(
        "Up/Down: Navigate • Enter: View Metrics • t: Change Theme • Esc: Back • q: Quit",
    )
    .style(Style::default().fg(theme.secondary));
    f.render_widget(controls, area);
}
