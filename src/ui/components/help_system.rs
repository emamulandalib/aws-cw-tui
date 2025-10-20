use crate::models::{App, AppState, FocusedPanel};
use crate::ui::themes::UnifiedTheme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

/// Help system state
#[derive(Debug, Clone)]
pub struct HelpSystem {
    pub visible: bool,
    pub current_context: HelpContext,
}

/// Different help contexts based on current application state
#[derive(Debug, Clone, PartialEq)]
pub enum HelpContext {
    ServiceList,
    InstanceList,
    MetricsSummary,
    InstanceDetails,
    Global,
}

impl Default for HelpSystem {
    fn default() -> Self {
        Self {
            visible: false,
            current_context: HelpContext::Global,
        }
    }
}

impl HelpSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn update_context(&mut self, app_state: &AppState) {
        self.current_context = match app_state {
            AppState::ServiceList => HelpContext::ServiceList,
            AppState::InstanceList => HelpContext::InstanceList,
            AppState::MetricsSummary => HelpContext::MetricsSummary,
            AppState::InstanceDetails => HelpContext::InstanceDetails,
        };
    }

    pub fn render(&self, f: &mut Frame, app: &App, theme: &UnifiedTheme) {
        if !self.visible {
            return;
        }

        // Create overlay area (80% of screen)
        let area = centered_rect(80, 80, f.area());

        // Clear the background
        f.render_widget(Clear, area);

        // Create help content based on current context
        let help_content = self.get_help_content(app, theme);

        // Create main help block
        let border_style = Style::default().fg(theme.border);
        let background_style = Style::default().bg(theme.background).fg(theme.primary);

        let help_block = Block::default()
            .title(" Help & Keyboard Shortcuts ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(border_style)
            .style(background_style);

        // Split into header and content
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Content
            ])
            .split(area);

        // Render main block
        f.render_widget(help_block, area);

        // Render header with context info
        let header_text = format!(
            "Context: {} | Press '?' or 'h' to close",
            self.get_context_name()
        );
        let header_style = Style::default()
            .fg(theme.accent)
            .add_modifier(Modifier::BOLD);
        let header = Paragraph::new(header_text)
            .style(header_style)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        f.render_widget(header, chunks[0]);

        // Render help content
        f.render_widget(help_content, chunks[1]);
    }

    fn get_context_name(&self) -> &'static str {
        match self.current_context {
            HelpContext::ServiceList => "Service Selection",
            HelpContext::InstanceList => "Instance List",
            HelpContext::MetricsSummary => "Metrics Summary",
            HelpContext::InstanceDetails => "Instance Details",
            HelpContext::Global => "Global",
        }
    }

    fn get_help_content(&self, app: &App, theme: &UnifiedTheme) -> List {
        let shortcuts = self.get_keyboard_shortcuts(app);

        let key_style = Style::default()
            .fg(theme.selected)
            .add_modifier(Modifier::BOLD);
        let desc_style = Style::default().fg(theme.primary);
        let category_style = Style::default()
            .fg(theme.secondary)
            .add_modifier(Modifier::ITALIC);

        let items: Vec<ListItem> = shortcuts
            .into_iter()
            .map(|(key, description, category)| {
                let content = vec![
                    Line::from(vec![
                        Span::styled(format!("{:12}", key), key_style),
                        Span::raw(" "),
                        Span::styled(description, desc_style),
                    ]),
                    Line::from(vec![
                        Span::raw("            "),
                        Span::styled(format!("({})", category), category_style),
                    ]),
                ];

                ListItem::new(content)
            })
            .collect();

        let list_style = Style::default().bg(theme.background).fg(theme.primary);
        let highlight_style = Style::default().bg(theme.focused).fg(theme.background);

        List::new(items)
            .style(list_style)
            .highlight_style(highlight_style)
    }

    fn get_keyboard_shortcuts(&self, app: &App) -> Vec<(String, String, String)> {
        let mut shortcuts = Vec::new();

        // Global shortcuts (always available)
        shortcuts.extend(vec![
            (
                "q".to_string(),
                "Quit application".to_string(),
                "Global".to_string(),
            ),
            (
                "t".to_string(),
                "Switch theme".to_string(),
                "Global".to_string(),
            ),
            (
                "? / h".to_string(),
                "Toggle help".to_string(),
                "Global".to_string(),
            ),
            (
                "r".to_string(),
                "Refresh data".to_string(),
                "Global".to_string(),
            ),
        ]);

        // Context-specific shortcuts
        match self.current_context {
            HelpContext::ServiceList => {
                shortcuts.extend(vec![
                    (
                        "↑ / k".to_string(),
                        "Move up".to_string(),
                        "Navigation".to_string(),
                    ),
                    (
                        "↓ / j".to_string(),
                        "Move down".to_string(),
                        "Navigation".to_string(),
                    ),
                    (
                        "Enter".to_string(),
                        "Select service".to_string(),
                        "Action".to_string(),
                    ),
                ]);
            }
            HelpContext::InstanceList => {
                shortcuts.extend(vec![
                    (
                        "↑ / k".to_string(),
                        "Move up".to_string(),
                        "Navigation".to_string(),
                    ),
                    (
                        "↓ / j".to_string(),
                        "Move down".to_string(),
                        "Navigation".to_string(),
                    ),
                    (
                        "Enter".to_string(),
                        "View metrics".to_string(),
                        "Action".to_string(),
                    ),
                    (
                        "Esc".to_string(),
                        "Back to services".to_string(),
                        "Navigation".to_string(),
                    ),
                ]);
            }
            HelpContext::MetricsSummary => {
                shortcuts.extend(vec![
                    (
                        "↑↓ / jk".to_string(),
                        "Navigate vertically".to_string(),
                        "Navigation".to_string(),
                    ),
                    (
                        "←→ / hl".to_string(),
                        "Navigate horizontally".to_string(),
                        "Navigation".to_string(),
                    ),
                    (
                        "Tab".to_string(),
                        "Switch panel".to_string(),
                        "Navigation".to_string(),
                    ),
                    (
                        "Enter".to_string(),
                        "Apply selection / View details".to_string(),
                        "Action".to_string(),
                    ),
                    (
                        "Esc".to_string(),
                        "Back to instances".to_string(),
                        "Navigation".to_string(),
                    ),
                ]);

                // Add panel-specific help
                match app.focused_panel {
                    FocusedPanel::Timezone => {
                        shortcuts.push((
                            "↑↓".to_string(),
                            "Select timezone".to_string(),
                            "Timezone".to_string(),
                        ));
                    }
                    FocusedPanel::Period => {
                        shortcuts.push((
                            "↑↓".to_string(),
                            "Select period".to_string(),
                            "Period".to_string(),
                        ));
                    }
                    FocusedPanel::TimeRanges => {
                        shortcuts.push((
                            "↑↓←→".to_string(),
                            "Select time range".to_string(),
                            "Time Range".to_string(),
                        ));
                    }
                    FocusedPanel::SparklineGrid => {
                        shortcuts.push((
                            "↑↓←→".to_string(),
                            "Navigate metrics grid".to_string(),
                            "Metrics".to_string(),
                        ));
                    }
                }
            }
            HelpContext::InstanceDetails => {
                shortcuts.extend(vec![
                    (
                        "↑ / k".to_string(),
                        "Previous metric".to_string(),
                        "Navigation".to_string(),
                    ),
                    (
                        "↓ / j".to_string(),
                        "Next metric".to_string(),
                        "Navigation".to_string(),
                    ),
                    (
                        "Esc".to_string(),
                        "Back to summary".to_string(),
                        "Navigation".to_string(),
                    ),
                ]);
            }
            HelpContext::Global => {
                // Already added global shortcuts above
            }
        }

        shortcuts
    }
}

/// Helper function to create a centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// Extension trait for App to add help system functionality
pub trait HelpSystemExt {
    fn get_help_system(&self) -> &HelpSystem;
    fn get_help_system_mut(&mut self) -> &mut HelpSystem;
    fn toggle_help(&mut self);
    fn update_help_context(&mut self);
}

impl HelpSystemExt for App {
    fn get_help_system(&self) -> &HelpSystem {
        &self.help_system
    }

    fn get_help_system_mut(&mut self) -> &mut HelpSystem {
        &mut self.help_system
    }

    fn toggle_help(&mut self) {
        self.help_system.toggle();
        self.update_help_context();
    }

    fn update_help_context(&mut self) {
        self.help_system.update_context(&self.state);
    }
}
