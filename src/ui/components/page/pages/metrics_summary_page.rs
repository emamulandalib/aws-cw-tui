/// Metrics summary page implementation using the standardized page system
///
/// This page displays the metrics summary for the selected instance.
use crate::models::App;
use crate::ui::components::page::{EventResult, Page, PageContent, PageLifecycle, PageResult};
use crate::ui::themes::UnifiedTheme;
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent},
    layout::Rect,
    Frame,
};

/// Metrics summary page content (placeholder)
#[derive(Debug)]
pub struct MetricsSummaryContent;

impl PageContent for MetricsSummaryContent {
    fn render_content(&self, f: &mut Frame, area: Rect, app: &mut App, theme: &UnifiedTheme) {
        // For now, use the existing metrics_summary render function
        // This will be refactored in a future task
        crate::ui::components::render_metrics_summary(f, app, theme);
    }

    fn content_title(&self) -> String {
        "Metrics Summary".to_string()
    }
}

/// Metrics summary page implementation
#[derive(Debug)]
pub struct MetricsSummaryPage {
    lifecycle: PageLifecycle,
    content: MetricsSummaryContent,
}

impl MetricsSummaryPage {
    /// Create a new metrics summary page
    pub fn new() -> Self {
        Self {
            lifecycle: PageLifecycle::new(),
            content: MetricsSummaryContent,
        }
    }
}

impl Page for MetricsSummaryPage {
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
                } => EventResult::Action(PageResult::Navigate("instance_list".to_string())),
                KeyEvent {
                    code: KeyCode::Enter,
                    ..
                } => EventResult::Action(PageResult::Navigate("instance_details".to_string())),
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
        "Metrics Summary".to_string()
    }
}

impl Default for MetricsSummaryPage {
    fn default() -> Self {
        Self::new()
    }
}
