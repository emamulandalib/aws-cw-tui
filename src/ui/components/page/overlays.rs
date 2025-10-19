/// Overlay components for loading and error states
///
/// This module provides consistent overlay components that can be displayed
/// on top of page content for loading states, errors, and other notifications.
use crate::ui::components::UniversalBox;
use crate::ui::themes::UnifiedTheme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Clear, Paragraph},
    Frame,
};

/// Types of overlays that can be displayed
#[derive(Debug, Clone)]
pub enum OverlayType {
    /// Loading overlay with optional message
    Loading(Option<String>),
    /// Error overlay with error message
    Error(String),
    /// Information overlay with message
    Info(String),
    /// Confirmation overlay with message and options
    Confirmation(String, Vec<String>),
}

/// Loading overlay component
#[derive(Debug, Clone)]
pub struct LoadingOverlay {
    message: String,
    show_spinner: bool,
}

impl LoadingOverlay {
    /// Create a new loading overlay
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            show_spinner: true,
        }
    }

    /// Create a loading overlay with default message
    pub fn default() -> Self {
        Self::new("Loading...")
    }

    /// Set whether to show spinner animation
    pub fn with_spinner(mut self, show_spinner: bool) -> Self {
        self.show_spinner = show_spinner;
        self
    }

    /// Render the loading overlay
    pub fn render(&self, f: &mut Frame, area: Rect, theme: &UnifiedTheme) {
        // Create a centered overlay area
        let overlay_area = self.center_area(area, 40, 8);

        // Clear the background
        f.render_widget(Clear, overlay_area);

        // Create loading message with spinner if enabled
        let message = if self.show_spinner {
            format!("⠋ {}", self.message)
        } else {
            self.message.clone()
        };

        // Use UniversalBox for consistent styling
        UniversalBox::loading_box("Loading", &message, theme.clone()).render(f, overlay_area);
    }

    /// Calculate centered area for overlay
    fn center_area(&self, area: Rect, width: u16, height: u16) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length((area.height.saturating_sub(height)) / 2),
                Constraint::Length(height),
                Constraint::Length((area.height.saturating_sub(height)) / 2),
            ])
            .split(area);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length((area.width.saturating_sub(width)) / 2),
                Constraint::Length(width),
                Constraint::Length((area.width.saturating_sub(width)) / 2),
            ])
            .split(popup_layout[1])[1]
    }
}

/// Error overlay component
#[derive(Debug, Clone)]
pub struct ErrorOverlay {
    title: String,
    message: String,
    show_details: bool,
}

impl ErrorOverlay {
    /// Create a new error overlay
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            show_details: false,
        }
    }

    /// Create an error overlay with default title
    pub fn with_message(message: impl Into<String>) -> Self {
        Self::new("Error", message)
    }

    /// Set whether to show detailed error information
    pub fn with_details(mut self, show_details: bool) -> Self {
        self.show_details = show_details;
        self
    }

    /// Render the error overlay
    pub fn render(&self, f: &mut Frame, area: Rect, theme: &UnifiedTheme) {
        // Create a centered overlay area
        let overlay_area = self.center_area(area, 60, 10);

        // Clear the background
        f.render_widget(Clear, overlay_area);

        // Use UniversalBox for consistent error styling
        UniversalBox::error_box(&self.title, &self.message, theme.clone()).render(f, overlay_area);

        // Add controls hint at the bottom
        if overlay_area.height > 8 {
            let controls_area = Rect {
                x: overlay_area.x,
                y: overlay_area.y + overlay_area.height - 2,
                width: overlay_area.width,
                height: 1,
            };

            let controls = Paragraph::new("Press any key to continue")
                .style(Style::default().fg(theme.secondary))
                .alignment(Alignment::Center);

            f.render_widget(controls, controls_area);
        }
    }

    /// Calculate centered area for overlay
    fn center_area(&self, area: Rect, width: u16, height: u16) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length((area.height.saturating_sub(height)) / 2),
                Constraint::Length(height),
                Constraint::Length((area.height.saturating_sub(height)) / 2),
            ])
            .split(area);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length((area.width.saturating_sub(width)) / 2),
                Constraint::Length(width),
                Constraint::Length((area.width.saturating_sub(width)) / 2),
            ])
            .split(popup_layout[1])[1]
    }
}

/// Generic overlay renderer for different overlay types
pub struct OverlayRenderer;

impl OverlayRenderer {
    /// Render an overlay based on its type
    pub fn render(overlay_type: &OverlayType, f: &mut Frame, area: Rect, theme: &UnifiedTheme) {
        match overlay_type {
            OverlayType::Loading(message) => {
                let overlay = if let Some(msg) = message {
                    LoadingOverlay::new(msg.clone())
                } else {
                    LoadingOverlay::default()
                };
                overlay.render(f, area, theme);
            }
            OverlayType::Error(message) => {
                let overlay = ErrorOverlay::with_message(message.clone());
                overlay.render(f, area, theme);
            }
            OverlayType::Info(message) => {
                // For now, render info as a simple overlay
                // Could be extended with a dedicated InfoOverlay component
                let overlay = Self::render_info_overlay(message, f, area, theme);
            }
            OverlayType::Confirmation(message, _options) => {
                // For now, render confirmation as an info overlay
                // Could be extended with a dedicated ConfirmationOverlay component
                let overlay = Self::render_info_overlay(message, f, area, theme);
            }
        }
    }

    /// Render a simple info overlay
    fn render_info_overlay(message: &str, f: &mut Frame, area: Rect, theme: &UnifiedTheme) {
        let overlay_area = Self::center_area(area, 50, 6);
        f.render_widget(Clear, overlay_area);

        UniversalBox::new(theme.clone())
            .title("Information")
            .text(message)
            .render(f, overlay_area);
    }

    /// Calculate centered area for overlay
    fn center_area(area: Rect, width: u16, height: u16) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length((area.height.saturating_sub(height)) / 2),
                Constraint::Length(height),
                Constraint::Length((area.height.saturating_sub(height)) / 2),
            ])
            .split(area);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length((area.width.saturating_sub(width)) / 2),
                Constraint::Length(width),
                Constraint::Length((area.width.saturating_sub(width)) / 2),
            ])
            .split(popup_layout[1])[1]
    }
}
