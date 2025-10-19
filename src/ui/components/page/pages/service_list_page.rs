/// Service list page implementation using the standardized page system
///
/// This page displays the list of available AWS services and allows
/// the user to select one to view its instances.
use crate::models::App;
use crate::ui::components::page::{EventResult, Page, PageContent, PageLifecycle, PageResult};
use crate::ui::components::{render_service_selection_list, UniversalBox};
use crate::ui::themes::UnifiedTheme;
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent},
    layout::Rect,
    Frame,
};

/// Service list page content
#[derive(Debug)]
pub struct ServiceListContent;

impl PageContent for ServiceListContent {
    fn render_content(&self, f: &mut Frame, area: Rect, app: &mut App, theme: &UnifiedTheme) {
        // Use the existing service selection component
        render_service_selection_list(
            f,
            area,
            &app.available_services,
            &mut app.service_list_state,
            true, // Always focused in service list view
            theme,
        );
    }

    fn content_title(&self) -> String {
        "Available Services".to_string()
    }
}

/// Service list page implementation
#[derive(Debug)]
pub struct ServiceListPage {
    lifecycle: PageLifecycle,
    content: ServiceListContent,
}

impl ServiceListPage {
    /// Create a new service list page
    pub fn new() -> Self {
        Self {
            lifecycle: PageLifecycle::new(),
            content: ServiceListContent,
        }
    }
}

impl Page for ServiceListPage {
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
                "Up/Down: Navigate • Enter: Select Service • t: Change Theme • q: Quit",
                ratatui::style::Style::default().fg(theme.secondary),
            )
            .borders(ratatui::widgets::Borders::NONE)
            .render(f, chunks[1]);
    }

    fn handle_event(&mut self, event: Event, app: &mut App) -> EventResult {
        if let Event::Key(key) = event {
            match key {
                KeyEvent {
                    code: KeyCode::Up, ..
                } => {
                    // Navigate up in service list
                    let selected = app.service_list_state.selected().unwrap_or(0);
                    if selected > 0 {
                        app.service_list_state.select(Some(selected - 1));
                    } else if !app.available_services.is_empty() {
                        app.service_list_state
                            .select(Some(app.available_services.len() - 1));
                    }
                    EventResult::Handled
                }
                KeyEvent {
                    code: KeyCode::Down,
                    ..
                } => {
                    // Navigate down in service list
                    let selected = app.service_list_state.selected().unwrap_or(0);
                    if selected < app.available_services.len().saturating_sub(1) {
                        app.service_list_state.select(Some(selected + 1));
                    } else {
                        app.service_list_state.select(Some(0));
                    }
                    EventResult::Handled
                }
                KeyEvent {
                    code: KeyCode::Enter,
                    ..
                } => {
                    // Select service and navigate to instance list
                    if let Some(selected_index) = app.service_list_state.selected() {
                        if selected_index < app.available_services.len() {
                            app.selected_service =
                                Some(app.available_services[selected_index].clone());
                            EventResult::Action(PageResult::Navigate("instance_list".to_string()))
                        } else {
                            EventResult::NotHandled
                        }
                    } else {
                        EventResult::NotHandled
                    }
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
        "Service Selection".to_string()
    }

    fn can_navigate_back(&self) -> bool {
        false // Service list is the root page
    }
}

impl Default for ServiceListPage {
    fn default() -> Self {
        Self::new()
    }
}
