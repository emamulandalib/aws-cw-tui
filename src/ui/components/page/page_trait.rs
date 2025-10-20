use super::lifecycle::PageLifecycle;
/// Core page trait for standardized page structure
///
/// All pages in the application should implement this trait to ensure
/// consistent behavior and lifecycle management.
use crate::models::App;
use crate::ui::themes::UnifiedTheme;
use ratatui::{crossterm::event::Event, layout::Rect, Frame};

/// Result type for page operations
#[derive(Debug, Clone)]
pub enum PageResult {
    /// Page operation completed successfully
    Success,
    /// Page needs to be refreshed
    Refresh,
    /// Page encountered an error
    Error(String),
    /// Page requests navigation to another page
    Navigate(String),
}

/// Result type for event handling
#[derive(Debug, Clone)]
pub enum EventResult {
    /// Event was handled successfully
    Handled,
    /// Event was not handled by this page
    NotHandled,
    /// Event triggered a page action
    Action(PageResult),
}

/// Core page trait that all pages must implement
pub trait Page {
    /// Initialize the page with the given app state
    fn initialize(&mut self, app: &mut App) -> PageResult;

    /// Update the page state based on app changes
    fn update(&mut self, app: &mut App) -> PageResult;

    /// Render the page content
    fn render(&self, f: &mut Frame, area: Rect, app: &mut App, theme: &UnifiedTheme);

    /// Handle input events
    fn handle_event(&mut self, event: Event, app: &mut App) -> EventResult;

    /// Get the page lifecycle state
    fn lifecycle(&self) -> &PageLifecycle;

    /// Get the page lifecycle state mutably
    fn lifecycle_mut(&mut self) -> &mut PageLifecycle;

    /// Cleanup when page is being destroyed
    fn cleanup(&mut self, app: &mut App) -> PageResult;

    /// Get the page title for display
    fn title(&self) -> String;

    /// Check if the page can handle back navigation
    fn can_navigate_back(&self) -> bool {
        true
    }

    /// Check if the page is currently loading
    fn is_loading(&self) -> bool {
        false
    }

    /// Get current error message if any
    fn error_message(&self) -> Option<&String> {
        None
    }
}

/// Helper trait for pages that need custom content rendering
pub trait PageContent {
    /// Render the main content area of the page
    fn render_content(&self, f: &mut Frame, area: Rect, app: &mut App, theme: &UnifiedTheme);

    /// Get the content title
    fn content_title(&self) -> String;
}
