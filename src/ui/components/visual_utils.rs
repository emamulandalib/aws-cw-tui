use super::display_utils::truncate_string;
use ratatui::{
    prelude::*,
    text::{Line, Span},
};

// Struct to group metric display parameters and reduce function argument count
pub struct MetricBlockParams {
    pub metric_name: String,
    pub sparkline: String,
    pub formatted_value: String,
    pub is_selected: bool,
    pub value_color: Color,
    pub sparkline_color: Color,
    pub name_width: usize,
    pub sparkline_width: usize,
}

/// Creates a distinct visual block for each metric item with proper spacing and styling
pub fn create_metric_block(params: MetricBlockParams) -> Vec<Line<'static>> {
    // Create content with proper spacing and calculate actual width
    let content = format!(
        " {:<name_width$}  {:<sparkline_width$}  {:>12} ",
        truncate_string(&params.metric_name, params.name_width),
        params.sparkline,
        params.formatted_value,
        name_width = params.name_width,
        sparkline_width = params.sparkline_width,
    );

    // Use actual content length for border width
    let total_width = content.chars().count();

    // Create the frame characters
    let top_border = format!("┌{}┐", "─".repeat(total_width));
    let bottom_border = format!("└{}┘", "─".repeat(total_width));

    if params.is_selected {
        // Selected metric with yellow background highlighting and yellow frame
        create_selected_metric_block(top_border, bottom_border, &params)
    } else {
        // Regular metric with yellow frame and colored content
        create_regular_metric_block(top_border, bottom_border, &params)
    }
}

/// Create the visual block for a selected metric
fn create_selected_metric_block(
    top_border: String,
    bottom_border: String,
    params: &MetricBlockParams,
) -> Vec<Line<'static>> {
    vec![
        Line::from(vec![Span::styled(
            top_border,
            Style::default().yellow(),
        )]),
        Line::from(vec![
            Span::styled("│", Style::default().yellow()),
            Span::styled(" ", Style::default().on_dark_gray()),
            Span::styled(
                format!(
                    "{:<width$}",
                    truncate_string(&params.metric_name, params.name_width),
                    width = params.name_width
                ),
                Style::default()
                    .cyan()
                    .on_dark_gray()
                    .bold(),
            ),
            Span::styled("  ", Style::default().on_dark_gray()),
            Span::styled(
                format!(
                    "{:<width$}",
                    params.sparkline,
                    width = params.sparkline_width
                ),
                Style::default()
                    .fg(params.sparkline_color)
                    .on_dark_gray()
                    .bold(),
            ),
            Span::styled("  ", Style::default().on_dark_gray()),
            Span::styled(
                format!("{:>12}", params.formatted_value),
                Style::default()
                    .fg(params.value_color)
                    .on_dark_gray()
                    .bold(),
            ),
            Span::styled(" ", Style::default().on_dark_gray()),
            Span::styled("│", Style::default().yellow()),
        ]),
        Line::from(vec![Span::styled(
            bottom_border,
            Style::default().yellow(),
        )]),
    ]
}

/// Create the visual block for a regular (non-selected) metric
fn create_regular_metric_block(
    top_border: String,
    bottom_border: String,
    params: &MetricBlockParams,
) -> Vec<Line<'static>> {
    vec![
        Line::from(vec![Span::styled(
            top_border,
            Style::default().yellow(),
        )]),
        Line::from(vec![
            Span::styled("│", Style::default().yellow()),
            Span::styled(" ", Style::default()),
            Span::styled(
                format!(
                    "{:<width$}",
                    truncate_string(&params.metric_name, params.name_width),
                    width = params.name_width
                ),
                Style::default().cyan(),
            ),
            Span::styled("  ", Style::default()),
            Span::styled(
                format!(
                    "{:<width$}",
                    params.sparkline,
                    width = params.sparkline_width
                ),
                Style::default().fg(params.sparkline_color),
            ),
            Span::styled("  ", Style::default()),
            Span::styled(
                format!("{:>12}", params.formatted_value),
                Style::default().fg(params.value_color).bold(),
            ),
            Span::styled(" ", Style::default()),
            Span::styled("│", Style::default().yellow()),
        ]),
        Line::from(vec![Span::styled(
            bottom_border,
            Style::default().yellow(),
        )]),
    ]
}


