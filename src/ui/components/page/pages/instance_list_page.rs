/// Instance list page implementation using the standardized page system
///
/// This page displays the list of instances for the selected AWS service
/// and allows the user to select one to view its metrics.
use crate::models::{App, ServiceInstance};
use crate::ui::components::page::{EventResult, Page, PageContent, PageLifecycle, PageResult};
use crate::ui::components::{
    list_styling::{
        border_factory::create_theme_border_style, themes::instance_list_colors_with_theme,
        utilities::create_highlight_style,
    },
    render_rds_instance_list_item, render_sqs_queue_list_item, UniversalBox,
};
use crate::ui::themes::UnifiedTheme;
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent},
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, List, ListItem, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

/// Instance list page content
#[derive(Debug)]
pub struct InstanceListContent;

impl PageContent for InstanceListContent {
    fn render_content(&self, f: &mut Frame, area: Rect, app: &mut App, theme: &UnifiedTheme) {
        // Check for errors first
        if let Some(error_msg) = &app.error_message {
            self.render_error_message(f, area, error_msg, theme);
        } else if app.loading {
            self.render_loading_message(f, area, app, theme);
        } else if app.get_current_instances().is_empty() {
            self.render_no_instances_message(f, area, app, theme);
        } else {
            self.render_instances_list(f, area, app, theme);
        }
    }

    fn content_title(&self) -> String {
        "Instances".to_string()
    }
}

impl InstanceListContent {
    fn render_loading_message(&self, f: &mut Frame, area: Rect, app: &App, theme: &UnifiedTheme) {
        let loading_text = "Loading instances...\n\nThis may take a few moments.";

        UniversalBox::loading_box(
            "",
            loading_text,
            theme.clone(),
        )
        .render(f, area);
    }

    fn render_error_message(
        &self,
        f: &mut Frame,
        area: Rect,
        error_msg: &str,
        theme: &UnifiedTheme,
    ) {
        UniversalBox::error_box("Error", error_msg, theme.clone()).render(f, area);
    }

    fn render_no_instances_message(
        &self,
        f: &mut Frame,
        area: Rect,
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
        &self,
        f: &mut Frame,
        area: Rect,
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
                ServiceInstance::Rds(instance) => {
                    render_rds_instance_list_item(instance, index == selected_index, theme)
                }
                ServiceInstance::Sqs(queue) => {
                    render_sqs_queue_list_item(&queue.name, index == selected_index, theme)
                }
            })
            .collect();

        let items_list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(create_theme_border_style(theme, false)),
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
}

/// Instance list page implementation
#[derive(Debug)]
pub struct InstanceListPage {
    lifecycle: PageLifecycle,
    content: InstanceListContent,
}

impl InstanceListPage {
    /// Create a new instance list page
    pub fn new() -> Self {
        Self {
            lifecycle: PageLifecycle::new(),
            content: InstanceListContent,
        }
    }
}

impl Page for InstanceListPage {
    fn initialize(&mut self, _app: &mut App) -> PageResult {
        self.lifecycle.mark_initialized();
        PageResult::Success
    }

    fn update(&mut self, _app: &mut App) -> PageResult {
        self.lifecycle.mark_updated();
        PageResult::Success
    }

    fn render(&self, f: &mut Frame, area: Rect, app: &mut App, theme: &UnifiedTheme) {
        // Render content and footer only
        let chunks = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Min(0),    // Content
                ratatui::layout::Constraint::Length(1), // Footer
            ])
            .split(area);

        // Render content
        self.content.render_content(f, chunks[0], app, theme);

        // Render footer
        UniversalBox::new(theme.clone())
            .text_styled(
                "Up/Down: Navigate • Enter: View Metrics • t: Change Theme • Esc: Back • q: Quit",
                Style::default().fg(theme.secondary),
            )
            .borders(Borders::NONE)
            .render(f, chunks[1]);
    }

    fn handle_event(&mut self, event: Event, app: &mut App) -> EventResult {
        if let Event::Key(key) = event {
            match key {
                KeyEvent {
                    code: KeyCode::Up, ..
                } => {
                    let instances = app.get_current_instances();
                    if !instances.is_empty() {
                        let selected = app.list_state.selected().unwrap_or(0);
                        if selected > 0 {
                            app.list_state.select(Some(selected - 1));
                        } else {
                            app.list_state.select(Some(instances.len() - 1));
                        }
                    }
                    EventResult::Handled
                }
                KeyEvent {
                    code: KeyCode::Down,
                    ..
                } => {
                    let instances = app.get_current_instances();
                    if !instances.is_empty() {
                        let selected = app.list_state.selected().unwrap_or(0);
                        if selected < instances.len().saturating_sub(1) {
                            app.list_state.select(Some(selected + 1));
                        } else {
                            app.list_state.select(Some(0));
                        }
                    }
                    EventResult::Handled
                }
                KeyEvent {
                    code: KeyCode::Enter,
                    ..
                } => {
                    // Select instance and navigate to metrics
                    if let Some(selected_index) = app.list_state.selected() {
                        let instances = app.get_current_instances();
                        if selected_index < instances.len() {
                            app.selected_instance = Some(selected_index);
                            EventResult::Action(PageResult::Navigate("metrics_summary".to_string()))
                        } else {
                            EventResult::NotHandled
                        }
                    } else {
                        EventResult::NotHandled
                    }
                }
                KeyEvent {
                    code: KeyCode::Esc, ..
                } => {
                    // Navigate back to service list
                    EventResult::Action(PageResult::Navigate("service_list".to_string()))
                }
                _ => EventResult::NotHandled,
            }
        } else {
            EventResult::NotHandled
        }
    }

    fn lifecycle(&self) -> &PageLifecycle {
        &self.lifecycle
    }

    fn lifecycle_mut(&mut self) -> &mut PageLifecycle {
        &mut self.lifecycle
    }

    fn cleanup(&mut self, _app: &mut App) -> PageResult {
        self.lifecycle.mark_destroying();
        PageResult::Success
    }

    fn title(&self) -> String {
        "Instance List".to_string()
    }

    fn is_loading(&self) -> bool {
        // Could check app loading state here
        false
    }

    fn error_message(&self) -> Option<&String> {
        // Could return app error message here
        None
    }
}

impl Default for InstanceListPage {
    fn default() -> Self {
        Self::new()
    }
}
