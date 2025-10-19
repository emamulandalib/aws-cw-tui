use crate::ui::components::list_styling::border_factory::{
    create_theme_status_border_style, BorderStatus,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

/// Render a general error message in the chart area
pub fn render_error_message(f: &mut Frame, area: ratatui::layout::Rect, message: &str) {
    let theme = crate::ui::themes::UnifiedTheme::default();
    let error_paragraph = Paragraph::new(message)
        .style(Style::default().fg(theme.error))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Error")
                .border_style(create_theme_status_border_style(
                    &theme,
                    BorderStatus::Error,
                )),
        );

    f.render_widget(error_paragraph, area);
}

/// Render an error chart with specific error message
pub fn render_error_chart(f: &mut Frame, area: ratatui::layout::Rect, error_msg: &str) {
    let theme = crate::ui::themes::UnifiedTheme::default();
    let error_block = Block::default()
        .borders(Borders::ALL)
        .title("Chart Error")
        .border_style(create_theme_status_border_style(
            &theme,
            BorderStatus::Error,
        ));

    let error_paragraph = Paragraph::new(error_msg)
        .style(Style::default().fg(theme.error))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(error_block);

    f.render_widget(error_paragraph, area);
}

/// Render a data validation error with detailed information
pub fn render_data_validation_error(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    metric_name: &str,
    error_details: &str,
) {
    let theme = crate::ui::themes::UnifiedTheme::default();
    let title = format!("Data Validation Error: {}", metric_name);
    let message = format!(
        "Failed to validate metric data:\n\n{}\n\nPlease check your data source and try refreshing.",
        error_details
    );

    let error_block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_style(create_theme_status_border_style(
            &theme,
            BorderStatus::Error,
        ));

    let error_paragraph = Paragraph::new(message)
        .style(Style::default().fg(theme.error))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
        .block(error_block);

    f.render_widget(error_paragraph, area);
}

/// Render a "no data" message when metrics are empty
pub fn render_no_data_message(f: &mut Frame, area: ratatui::layout::Rect, metric_name: &str) {
    let message = format!("No data available for: {}", metric_name);

    let theme = crate::ui::themes::UnifiedTheme::default();
    let no_data_block = Block::default()
        .borders(Borders::ALL)
        .title("No Data")
        .border_style(create_theme_status_border_style(
            &theme,
            BorderStatus::Warning,
        ));

    let no_data_paragraph = Paragraph::new(message)
        .style(Style::default().fg(theme.warning))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(no_data_block);

    f.render_widget(no_data_paragraph, area);
}

/// Render a loading message while data is being fetched
pub fn render_loading_message(f: &mut Frame, area: ratatui::layout::Rect) {
    let theme = crate::ui::themes::UnifiedTheme::default();
    let loading_block = Block::default()
        .borders(Borders::ALL)
        .title("Loading")
        .border_style(create_theme_status_border_style(&theme, BorderStatus::Info));

    let loading_paragraph = Paragraph::new("Loading metric data...")
        .style(Style::default().fg(theme.info))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(loading_block);

    f.render_widget(loading_paragraph, area);
}

/// Render a timeout error message
pub fn render_timeout_error(f: &mut Frame, area: ratatui::layout::Rect, service: &str) {
    let theme = crate::ui::themes::UnifiedTheme::default();
    let message = format!(
        "Timeout loading {} metrics.\n\nThis may be due to:\n• Network connectivity issues\n• AWS service availability\n• Large time range selected\n\nTry reducing the time range or checking your connection.",
        service
    );

    let timeout_block = Block::default()
        .borders(Borders::ALL)
        .title("Timeout Error")
        .border_style(create_theme_status_border_style(
            &theme,
            BorderStatus::Error,
        ));

    let timeout_paragraph = Paragraph::new(message)
        .style(Style::default().fg(theme.error))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
        .block(timeout_block);

    f.render_widget(timeout_paragraph, area);
}

/// Render an AWS authentication error
pub fn render_auth_error(f: &mut Frame, area: ratatui::layout::Rect) {
    let theme = crate::ui::themes::UnifiedTheme::default();
    let message = "AWS Authentication Error\n\nPlease check:\n• AWS credentials are configured\n• AWS profile is valid\n• Required permissions are granted\n\nRun 'aws configure' or set AWS_PROFILE environment variable.";

    let auth_block = Block::default()
        .borders(Borders::ALL)
        .title("Authentication Error")
        .border_style(create_theme_status_border_style(
            &theme,
            BorderStatus::Error,
        ));

    let auth_paragraph = Paragraph::new(message)
        .style(Style::default().fg(theme.error))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
        .block(auth_block);

    f.render_widget(auth_paragraph, area);
}

/// Render a popup overlay error message
pub fn render_error_popup(f: &mut Frame, area: ratatui::layout::Rect, title: &str, message: &str) {
    let theme = crate::ui::themes::UnifiedTheme::default();
    // Calculate popup size (60% of the area)
    let popup_area = centered_rect(60, 40, area);

    // Clear the background
    f.render_widget(Clear, popup_area);

    // Render the error popup
    let error_block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_style(
            create_theme_status_border_style(&theme, BorderStatus::Error)
                .add_modifier(Modifier::BOLD),
        );

    let error_paragraph = Paragraph::new(message)
        .style(Style::default().fg(theme.error))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(error_block);

    f.render_widget(error_paragraph, popup_area);
}

/// Helper function to create a centered rectangle
fn centered_rect(
    percent_x: u16,
    percent_y: u16,
    r: ratatui::layout::Rect,
) -> ratatui::layout::Rect {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_centered_rect_calculation() {
        let area = ratatui::layout::Rect::new(0, 0, 100, 50);
        let centered = centered_rect(60, 40, area);

        // Should be centered within the area
        assert!(centered.x >= area.x);
        assert!(centered.y >= area.y);
        assert!(centered.width <= area.width);
        assert!(centered.height <= area.height);
    }
}
