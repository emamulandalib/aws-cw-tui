use crate::models::App;
use crate::ui::themes::UnifiedTheme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};
use std::time::{Duration, Instant};

/// Error feedback system for user-friendly error display and recovery
#[derive(Debug, Clone)]
pub struct ErrorFeedback {
    pub error_message: Option<String>,
    pub error_type: ErrorType,
    pub error_timestamp: Option<Instant>,
    pub recovery_suggestions: Vec<String>,
    pub auto_dismiss_after: Option<Duration>,
    pub dismissible: bool,
}

/// Types of errors with different visual treatments
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorType {
    Network,        // AWS connection issues
    Authentication, // AWS credential issues
    Configuration,  // Config file issues
    DataFormat,     // Data parsing issues
    UserInput,      // Invalid user input
    System,         // System-level errors
    Warning,        // Non-critical warnings
    Info,           // Informational messages
}

impl Default for ErrorFeedback {
    fn default() -> Self {
        Self {
            error_message: None,
            error_type: ErrorType::System,
            error_timestamp: None,
            recovery_suggestions: Vec::new(),
            auto_dismiss_after: Some(Duration::from_secs(10)),
            dismissible: true,
        }
    }
}

impl ErrorFeedback {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set an error with automatic recovery suggestions
    pub fn set_error(&mut self, message: String, error_type: ErrorType) {
        self.error_message = Some(message.clone());
        self.error_type = error_type.clone();
        self.error_timestamp = Some(Instant::now());
        self.recovery_suggestions = self.generate_recovery_suggestions(&message, &error_type);

        // Set auto-dismiss based on error type
        self.auto_dismiss_after = match error_type {
            ErrorType::Info => Some(Duration::from_secs(5)),
            ErrorType::Warning => Some(Duration::from_secs(8)),
            ErrorType::UserInput => Some(Duration::from_secs(10)),
            ErrorType::Network | ErrorType::Authentication => Some(Duration::from_secs(15)),
            ErrorType::Configuration | ErrorType::DataFormat | ErrorType::System => None, // Manual dismiss
        };
    }

    /// Set a network error with specific recovery suggestions
    pub fn set_network_error(&mut self, message: String) {
        self.set_error(message, ErrorType::Network);
        self.recovery_suggestions = vec![
            "Check your internet connection".to_string(),
            "Verify AWS region settings".to_string(),
            "Press 'r' to retry the operation".to_string(),
            "Check AWS service status".to_string(),
        ];
    }

    /// Set an authentication error with specific recovery suggestions
    pub fn set_auth_error(&mut self, message: String) {
        self.set_error(message, ErrorType::Authentication);
        self.recovery_suggestions = vec![
            "Check AWS credentials (aws configure)".to_string(),
            "Verify AWS profile settings".to_string(),
            "Ensure proper IAM permissions".to_string(),
            "Check AWS_PROFILE environment variable".to_string(),
        ];
    }

    /// Set a configuration error with specific recovery suggestions
    pub fn set_config_error(&mut self, message: String) {
        self.set_error(message, ErrorType::Configuration);
        self.recovery_suggestions = vec![
            "Check configuration file syntax".to_string(),
            "Verify file permissions".to_string(),
            "Reset to default configuration".to_string(),
            "Check configuration file location".to_string(),
        ];
    }

    /// Set a user input error with specific recovery suggestions
    pub fn set_user_input_error(&mut self, message: String) {
        self.set_error(message, ErrorType::UserInput);
        self.recovery_suggestions = vec![
            "Check your input format".to_string(),
            "Try a different selection".to_string(),
            "Press '?' for help".to_string(),
        ];
    }

    /// Set an informational message
    pub fn set_info(&mut self, message: String) {
        self.set_error(message, ErrorType::Info);
        self.recovery_suggestions = vec![];
    }

    /// Set a warning message
    pub fn set_warning(&mut self, message: String) {
        self.set_error(message, ErrorType::Warning);
    }

    /// Clear the current error
    pub fn clear(&mut self) {
        self.error_message = None;
        self.error_timestamp = None;
        self.recovery_suggestions.clear();
    }

    /// Check if error should be auto-dismissed
    pub fn should_auto_dismiss(&self) -> bool {
        if let (Some(timestamp), Some(duration)) = (self.error_timestamp, self.auto_dismiss_after) {
            timestamp.elapsed() >= duration
        } else {
            false
        }
    }

    /// Check if there's an active error
    pub fn has_error(&self) -> bool {
        self.error_message.is_some()
    }

    /// Get the current error message
    pub fn get_error(&self) -> Option<&String> {
        self.error_message.as_ref()
    }

    /// Render the error feedback overlay
    pub fn render(&self, f: &mut Frame, theme: &UnifiedTheme) {
        if !self.has_error() {
            return;
        }

        // Auto-dismiss if needed
        if self.should_auto_dismiss() {
            return;
        }

        let error_message = self.error_message.as_ref().unwrap();

        // Create error area (60% width, positioned at top)
        let area = self.get_error_area(f.area());

        // Clear the background
        f.render_widget(Clear, area);

        // Get colors based on error type
        let (bg_color, border_color, text_color) = self.get_error_colors(theme);

        // Create error block
        let error_block = Block::default()
            .title(format!(" {} ", self.get_error_title()))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .style(Style::default().bg(bg_color));

        // Split into message and suggestions
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),                                             // Error message
                Constraint::Length(self.recovery_suggestions.len() as u16 + 2), // Suggestions
                Constraint::Length(1),                                          // Dismiss info
            ])
            .split(area);

        // Render main block
        f.render_widget(error_block, area);

        // Render error message
        let message_paragraph = Paragraph::new(error_message.as_str())
            .style(Style::default().fg(text_color).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        f.render_widget(message_paragraph, chunks[0]);

        // Render recovery suggestions if any
        if !self.recovery_suggestions.is_empty() {
            let suggestions_text: Vec<Line> = self
                .recovery_suggestions
                .iter()
                .enumerate()
                .map(|(i, suggestion)| {
                    Line::from(vec![
                        Span::styled(format!("{}. ", i + 1), Style::default().fg(theme.accent)),
                        Span::styled(suggestion, Style::default().fg(text_color)),
                    ])
                })
                .collect();

            let suggestions_paragraph = Paragraph::new(suggestions_text)
                .style(Style::default().fg(text_color))
                .wrap(Wrap { trim: true });

            f.render_widget(suggestions_paragraph, chunks[1]);
        }

        // Render dismiss information
        if self.dismissible {
            let dismiss_text = if self.auto_dismiss_after.is_some() {
                "Press any key to dismiss (auto-dismiss in a few seconds)"
            } else {
                "Press any key to dismiss"
            };

            let dismiss_paragraph = Paragraph::new(dismiss_text)
                .style(
                    Style::default()
                        .fg(theme.muted)
                        .add_modifier(Modifier::ITALIC),
                )
                .alignment(Alignment::Center);

            f.render_widget(dismiss_paragraph, chunks[2]);
        }
    }

    /// Get the error display area
    fn get_error_area(&self, full_area: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),                                          // Top margin
                Constraint::Length(8 + self.recovery_suggestions.len() as u16), // Error content
                Constraint::Min(0),                                             // Bottom space
            ])
            .split(full_area);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20), // Left margin
                Constraint::Percentage(60), // Error content
                Constraint::Percentage(20), // Right margin
            ])
            .split(popup_layout[1])[1]
    }

    /// Get colors based on error type
    fn get_error_colors(&self, theme: &UnifiedTheme) -> (Color, Color, Color) {
        match self.error_type {
            ErrorType::Network => (theme.background, theme.warning, theme.primary),
            ErrorType::Authentication => (theme.background, theme.error, theme.primary),
            ErrorType::Configuration => (theme.background, theme.error, theme.primary),
            ErrorType::DataFormat => (theme.background, theme.warning, theme.primary),
            ErrorType::UserInput => (theme.background, theme.info, theme.primary),
            ErrorType::System => (theme.background, theme.error, theme.primary),
            ErrorType::Warning => (theme.background, theme.warning, theme.primary),
            ErrorType::Info => (theme.background, theme.info, theme.primary),
        }
    }

    /// Get error title based on type
    fn get_error_title(&self) -> &'static str {
        match self.error_type {
            ErrorType::Network => "Network Error",
            ErrorType::Authentication => "Authentication Error",
            ErrorType::Configuration => "Configuration Error",
            ErrorType::DataFormat => "Data Format Error",
            ErrorType::UserInput => "Input Error",
            ErrorType::System => "System Error",
            ErrorType::Warning => "Warning",
            ErrorType::Info => "Information",
        }
    }

    /// Generate recovery suggestions based on error message and type
    fn generate_recovery_suggestions(&self, message: &str, error_type: &ErrorType) -> Vec<String> {
        let mut suggestions = Vec::new();

        // Add type-specific suggestions
        match error_type {
            ErrorType::Network => {
                suggestions.extend(vec![
                    "Check your internet connection".to_string(),
                    "Verify AWS region settings".to_string(),
                    "Press 'r' to retry".to_string(),
                ]);
            }
            ErrorType::Authentication => {
                suggestions.extend(vec![
                    "Run 'aws configure' to set credentials".to_string(),
                    "Check AWS profile settings".to_string(),
                    "Verify IAM permissions".to_string(),
                ]);
            }
            ErrorType::Configuration => {
                suggestions.extend(vec![
                    "Check configuration file syntax".to_string(),
                    "Verify file permissions".to_string(),
                    "Reset to defaults if needed".to_string(),
                ]);
            }
            ErrorType::UserInput => {
                suggestions.extend(vec![
                    "Check your input format".to_string(),
                    "Press '?' for help".to_string(),
                ]);
            }
            _ => {
                suggestions.push("Press 'r' to retry".to_string());
            }
        }

        // Add message-specific suggestions
        if message.to_lowercase().contains("timeout") {
            suggestions.push("Try increasing timeout settings".to_string());
        }
        if message.to_lowercase().contains("permission") {
            suggestions.push("Check file/directory permissions".to_string());
        }
        if message.to_lowercase().contains("not found") {
            suggestions.push("Verify the resource exists".to_string());
        }

        suggestions
    }
}

/// Extension trait for App to add error feedback functionality
pub trait ErrorFeedbackExt {
    fn get_error_feedback(&self) -> &ErrorFeedback;
    fn get_error_feedback_mut(&mut self) -> &mut ErrorFeedback;
    fn set_error(&mut self, message: String, error_type: ErrorType);
    fn set_network_error(&mut self, message: String);
    fn set_auth_error(&mut self, message: String);
    fn set_config_error(&mut self, message: String);
    fn set_user_input_error(&mut self, message: String);
    fn set_info(&mut self, message: String);
    fn set_warning(&mut self, message: String);
    fn clear_error_feedback(&mut self);
    fn has_error_feedback(&self) -> bool;
}

impl ErrorFeedbackExt for App {
    fn get_error_feedback(&self) -> &ErrorFeedback {
        &self.error_feedback
    }

    fn get_error_feedback_mut(&mut self) -> &mut ErrorFeedback {
        &mut self.error_feedback
    }

    fn set_error(&mut self, message: String, error_type: ErrorType) {
        self.error_feedback.set_error(message, error_type);
    }

    fn set_network_error(&mut self, message: String) {
        self.error_feedback.set_network_error(message);
    }

    fn set_auth_error(&mut self, message: String) {
        self.error_feedback.set_auth_error(message);
    }

    fn set_config_error(&mut self, message: String) {
        self.error_feedback.set_config_error(message);
    }

    fn set_user_input_error(&mut self, message: String) {
        self.error_feedback.set_user_input_error(message);
    }

    fn set_info(&mut self, message: String) {
        self.error_feedback.set_info(message);
    }

    fn set_warning(&mut self, message: String) {
        self.error_feedback.set_warning(message);
    }

    fn clear_error_feedback(&mut self) {
        self.error_feedback.clear();
    }

    fn has_error_feedback(&self) -> bool {
        self.error_feedback.has_error()
    }
}
