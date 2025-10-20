use super::{
    overlays::{ErrorOverlay, LoadingOverlay},
    page_trait::PageContent,
};
/// Standard page layout component with header, content, and footer sections
///
/// This component provides a consistent layout structure that all pages
/// should use for uniform appearance and behavior.
use crate::models::App;
use crate::ui::components::UniversalBox;
use crate::ui::themes::UnifiedTheme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Borders, Paragraph},
    Frame,
};

/// Standard page layout with header, content, and footer
pub struct StandardPageLayout {
    /// Page title for the header
    title: String,
    /// Optional subtitle for the header
    subtitle: Option<String>,
    /// Content component
    content: Box<dyn PageContent>,
    /// Footer text (controls, status, etc.)
    footer_text: String,
    /// Loading overlay if page is loading
    loading_overlay: Option<LoadingOverlay>,
    /// Error overlay if page has error
    error_overlay: Option<ErrorOverlay>,
    /// Whether to show borders around sections
    show_borders: bool,
}

impl StandardPageLayout {
    /// Create a new standard page layout
    pub fn new(
        title: impl Into<String>,
        content: Box<dyn PageContent>,
        footer_text: impl Into<String>,
    ) -> Self {
        Self {
            title: title.into(),
            subtitle: None,
            content,
            footer_text: footer_text.into(),
            loading_overlay: None,
            error_overlay: None,
            show_borders: true,
        }
    }

    /// Set the subtitle
    pub fn with_subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Set whether to show borders
    pub fn with_borders(mut self, show_borders: bool) -> Self {
        self.show_borders = show_borders;
        self
    }

    /// Set loading overlay
    pub fn with_loading(mut self, message: impl Into<String>) -> Self {
        self.loading_overlay = Some(LoadingOverlay::new(message));
        self
    }

    /// Set error overlay
    pub fn with_error(mut self, title: impl Into<String>, message: impl Into<String>) -> Self {
        self.error_overlay = Some(ErrorOverlay::new(title, message));
        self
    }

    /// Update the title
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    /// Update the subtitle
    pub fn set_subtitle(&mut self, subtitle: Option<String>) {
        self.subtitle = subtitle;
    }

    /// Update the footer text
    pub fn set_footer_text(&mut self, footer_text: impl Into<String>) {
        self.footer_text = footer_text.into();
    }

    /// Show loading overlay
    pub fn show_loading(&mut self, message: impl Into<String>) {
        self.loading_overlay = Some(LoadingOverlay::new(message));
    }

    /// Hide loading overlay
    pub fn hide_loading(&mut self) {
        self.loading_overlay = None;
    }

    /// Show error overlay
    pub fn show_error(&mut self, title: impl Into<String>, message: impl Into<String>) {
        self.error_overlay = Some(ErrorOverlay::new(title, message));
    }

    /// Hide error overlay
    pub fn hide_error(&mut self) {
        self.error_overlay = None;
    }

    /// Render the complete page layout
    pub fn render(&self, f: &mut Frame, area: Rect, app: &mut App, theme: &UnifiedTheme) {
        // Create the main layout with content and footer only
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(area);

        // Render content
        self.content.render_content(f, chunks[0], app, theme);

        // Render footer
        self.render_footer(f, chunks[1], theme);

        // Render overlays if present
        if let Some(loading) = &self.loading_overlay {
            loading.render(f, area, theme);
        }

        if let Some(error) = &self.error_overlay {
            error.render(f, area, theme);
        }
    }

    /// Render the footer section
    fn render_footer(&self, f: &mut Frame, area: Rect, theme: &UnifiedTheme) {
        if self.show_borders {
            UniversalBox::new(theme.clone())
                .text_styled(&self.footer_text, Style::default().fg(theme.secondary))
                .borders(Borders::NONE)
                .render(f, area);
        } else {
            let footer = Paragraph::new(self.footer_text.clone())
                .style(Style::default().fg(theme.secondary));
            f.render_widget(footer, area);
        }
    }
}

/// Simple content wrapper for basic text content
#[derive(Debug)]
pub struct SimplePageContent {
    title: String,
    text: String,
}

impl SimplePageContent {
    /// Create new simple page content
    pub fn new(title: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            text: text.into(),
        }
    }
}

impl PageContent for SimplePageContent {
    fn render_content(&self, f: &mut Frame, area: Rect, _app: &mut App, theme: &UnifiedTheme) {
        UniversalBox::new(theme.clone())
            .title(&self.title)
            .text(&self.text)
            .render(f, area);
    }

    fn content_title(&self) -> String {
        self.title.clone()
    }
}

/// Empty content for pages that handle their own content rendering
#[derive(Debug)]
pub struct EmptyPageContent;

impl PageContent for EmptyPageContent {
    fn render_content(&self, _f: &mut Frame, _area: Rect, _app: &mut App, _theme: &UnifiedTheme) {
        // No-op - page handles its own content rendering
    }

    fn content_title(&self) -> String {
        String::new()
    }
}
