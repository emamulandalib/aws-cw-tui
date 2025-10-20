use super::{
    pages::{InstanceDetailsPage, InstanceListPage, MetricsSummaryPage, ServiceListPage},
    transitions::TransitionManager,
    EventResult, Page, PageResult,
};
/// Page manager for coordinating page lifecycle and navigation
///
/// This module provides centralized management of all pages in the application,
/// handling navigation, lifecycle management, and page transitions.
use crate::models::{App, AppState};
use crate::ui::themes::UnifiedTheme;
use ratatui::{crossterm::event::Event, layout::Rect, Frame};
use std::collections::HashMap;

/// Page manager that coordinates all pages
pub struct PageManager {
    /// Currently active page
    current_page: String,
    /// All registered pages
    pages: HashMap<String, Box<dyn Page>>,
    /// Transition manager
    transition_manager: TransitionManager,
    /// Whether the page system is enabled
    enabled: bool,
}

impl PageManager {
    /// Create a new page manager
    pub fn new() -> Self {
        let mut manager = Self {
            current_page: "service_list".to_string(),
            pages: HashMap::new(),
            transition_manager: TransitionManager::new(),
            enabled: true,
        };

        // Register all pages
        manager.register_default_pages();

        manager
    }

    /// Register all default pages
    fn register_default_pages(&mut self) {
        self.register_page("service_list", Box::new(ServiceListPage::new()));
        self.register_page("instance_list", Box::new(InstanceListPage::new()));
        self.register_page("metrics_summary", Box::new(MetricsSummaryPage::new()));
        self.register_page("instance_details", Box::new(InstanceDetailsPage::new()));
    }

    /// Register a page with the manager
    pub fn register_page(&mut self, name: impl Into<String>, page: Box<dyn Page>) {
        self.pages.insert(name.into(), page);
    }

    /// Navigate to a specific page
    pub fn navigate_to(&mut self, page_name: impl Into<String>, app: &mut App) -> PageResult {
        let page_name = page_name.into();

        if !self.pages.contains_key(&page_name) {
            return PageResult::Error(format!("Page '{}' not found", page_name));
        }

        // Cleanup current page
        if let Some(current_page) = self.pages.get_mut(&self.current_page) {
            let _ = current_page.cleanup(app);
        }

        // Switch to new page
        self.current_page = page_name.clone();

        // Initialize new page
        if let Some(new_page) = self.pages.get_mut(&page_name) {
            let result = new_page.initialize(app);

            // Update app state to match page
            self.update_app_state_for_page(&page_name, app);

            result
        } else {
            PageResult::Error(format!("Failed to initialize page '{}'", page_name))
        }
    }

    /// Update app state to match the current page
    fn update_app_state_for_page(&self, page_name: &str, app: &mut App) {
        app.state = match page_name {
            "service_list" => AppState::ServiceList,
            "instance_list" => AppState::InstanceList,
            "metrics_summary" => AppState::MetricsSummary,
            "instance_details" => AppState::InstanceDetails,
            _ => app.state.clone(), // Keep current state if unknown page
        };
    }

    /// Get the current page name
    pub fn current_page(&self) -> &str {
        &self.current_page
    }

    /// Update the current page
    pub fn update(&mut self, app: &mut App) -> PageResult {
        if !self.enabled {
            return PageResult::Success;
        }

        // Update transition manager
        self.transition_manager.update();

        // Update current page
        if let Some(page) = self.pages.get_mut(&self.current_page) {
            page.update(app)
        } else {
            PageResult::Error(format!("Current page '{}' not found", self.current_page))
        }
    }

    /// Render the current page
    pub fn render(&self, f: &mut Frame, area: Rect, app: &mut App, theme: &UnifiedTheme) {
        if !self.enabled {
            return;
        }

        if let Some(page) = self.pages.get(&self.current_page) {
            page.render(f, area, app, theme);
        }
    }

    /// Handle events for the current page
    pub fn handle_event(&mut self, event: Event, app: &mut App) -> EventResult {
        if !self.enabled {
            return EventResult::NotHandled;
        }

        if let Some(page) = self.pages.get_mut(&self.current_page) {
            let result = page.handle_event(event, app);

            // Handle navigation requests
            if let EventResult::Action(PageResult::Navigate(target_page)) = &result {
                let _ = self.navigate_to(target_page.clone(), app);
            }

            result
        } else {
            EventResult::NotHandled
        }
    }

    /// Enable or disable the page system
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Check if the page system is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Get the current page title
    pub fn current_page_title(&self) -> String {
        if let Some(page) = self.pages.get(&self.current_page) {
            page.title()
        } else {
            "Unknown Page".to_string()
        }
    }

    /// Check if the current page can navigate back
    pub fn can_navigate_back(&self) -> bool {
        if let Some(page) = self.pages.get(&self.current_page) {
            page.can_navigate_back()
        } else {
            false
        }
    }

    /// Navigate back to the previous page (simplified implementation)
    pub fn navigate_back(&mut self, app: &mut App) -> PageResult {
        let previous_page = match self.current_page.as_str() {
            "instance_list" => "service_list",
            "metrics_summary" => "instance_list",
            "instance_details" => "metrics_summary",
            _ => return PageResult::Error("Cannot navigate back from this page".to_string()),
        };

        self.navigate_to(previous_page, app)
    }

    /// Sync the page manager with the current app state
    pub fn sync_with_app_state(&mut self, app: &mut App) -> PageResult {
        let target_page = match app.state {
            AppState::ServiceList => "service_list",
            AppState::InstanceList => "instance_list",
            AppState::MetricsSummary => "metrics_summary",
            AppState::InstanceDetails => "instance_details",
        };

        if target_page != self.current_page {
            self.navigate_to(target_page, app)
        } else {
            PageResult::Success
        }
    }
}

impl Default for PageManager {
    fn default() -> Self {
        Self::new()
    }
}
