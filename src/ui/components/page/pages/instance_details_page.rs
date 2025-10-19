/// Instance details page implementation using the standardized page system
///
/// This page displays detailed metrics for the selected instance.
use crate::models::App;
use crate::ui::components::page::{EventResult, Page, PageContent, PageLifecycle, PageResult};
use crate::ui::themes::UnifiedTheme;
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent},
    layout::Rect,
    Frame,
};

/// Instance details page content (placeholder)
#[derive(Debug)]
pub struct InstanceDetailsContent;

impl PageContent for InstanceDetailsContent {
    fn render_content(&self, f: &mut Frame, area: Rect, app: &mut App, theme: &UnifiedTheme) {
        // For now, use the existing instance_details render function
        // This will be refactored in a future task
        crate::ui::components::render_instance_details(f, app, theme);
    }

    fn content_title(&self) -> String {
        "Instance Details".to_string()
    }
}

/// Instance details page implementation
#[derive(Debug)]
pub struct InstanceDetailsPage {
    lifecycle: PageLifecycle,
    content: InstanceDetailsContent,
}

impl InstanceDetailsPage {
    /// Create a new instance details page
    pub fn new() -> Self {
        Self {
            lifecycle: PageLifecycle::new(),
            content: InstanceDetailsContent,
        }
    }
}

impl Page for InstanceDetailsPage {
    fn initialize(&mut self, _app: &mut App) -> PageResult {
        self.lifecycle.mark_initialized();
        PageResult::Success
    }

    fn update(&mut self, _app: &mut App) -> PageResult {
        self.lifecycle.mark_updated();
        PageResult::Success
    }

    fn render(&self, f: &mut Frame, area: Rect, app: &mut App, theme: &UnifiedTheme) {
        // For now, delegate to existing render function
        // This maintains compatibility while we transition to the page system
        self.content.render_content(f, area, app, theme);
    }

    fn handle_event(&mut self, event: Event, _app: &mut App) -> EventResult {
        if let Event::Key(key) = event {
            match key {
                KeyEvent {
                    code: KeyCode::Esc, ..
                } => EventResult::Action(PageResult::Navigate("metrics_summary".to_string())),
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
        "Instance Details".to_string()
    }
}

impl Default for InstanceDetailsPage {
    fn default() -> Self {
        Self::new()
    }
}
