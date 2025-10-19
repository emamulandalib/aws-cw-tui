use crate::aws::dynamic_metric_discovery::DynamicMetricData;
use crate::ui::charts::chart_data::MetricChartData;
use crate::ui::components::list_styling::border_factory;
use crate::ui::themes::UnifiedTheme;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Render simple metric display when chart space is limited
pub fn render_simple_metric(
    f: &mut Frame,
    area: Rect,
    chart_data: &MetricChartData,
    definition: &crate::ui::components::metric_definitions::MetricDefinition,
    health_color: Color,
    is_focused: bool,
) {
    let theme = UnifiedTheme::default();
    let border_style = border_factory::create_theme_border_style(&theme, is_focused);

    let trend_indicator = calculate_trend_indicator(&chart_data.history, chart_data.current_value);
    let content = format!("{}{}", definition.description, trend_indicator);

    let simple_widget = Paragraph::new(content)
        .style(Style::default().fg(health_color))
        .alignment(Alignment::Center)
        .wrap(ratatui::widgets::Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style),
        );

    f.render_widget(simple_widget, area);
}

/// Render simple dynamic metric (when space is limited)
pub fn render_dynamic_simple_metric(
    f: &mut Frame,
    area: Rect,
    metric_data: &DynamicMetricData,
    is_focused: bool,
) {
    let theme = UnifiedTheme::default();
    let border_style = border_factory::create_theme_border_style(&theme, is_focused);

    let formatted_value = format_simple_metric_value(
        metric_data.current_value,
        metric_data.unit.as_deref().unwrap_or(""),
    );

    let simple_widget = Paragraph::new(formatted_value)
        .style(
            Style::default()
                .fg(theme.success)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style),
        );

    f.render_widget(simple_widget, area);
}

/// Render error chart
pub fn render_error_chart(f: &mut Frame, area: Rect, error_msg: &str, is_focused: bool) {
    let theme = UnifiedTheme::default();
    let border_style = border_factory::create_theme_status_border_style(
        &theme,
        border_factory::BorderStatus::Error,
    );

    let error_widget = Paragraph::new(format!("Error: {}", error_msg))
        .style(Style::default().fg(theme.error))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Chart Error")
                .border_style(border_style),
        );

    f.render_widget(error_widget, area);
}

/// Calculate trend indicator based on historical data
fn calculate_trend_indicator(history: &[f64], current_value: f64) -> &'static str {
    if history.len() > 1 {
        let first = history[0];
        if current_value > first * 1.1 {
            " Trending Up"
        } else if current_value < first * 0.9 {
            " Trending Down"
        } else {
            " Stable"
        }
    } else {
        ""
    }
}

/// Format metric value for simple display
fn format_simple_metric_value(value: f64, unit: &str) -> String {
    match unit {
        unit_str if unit_str.contains("Percent") => {
            format!("{:.1}%", value)
        }
        unit_str if unit_str.contains("Bytes") => {
            if value > 1_000_000.0 {
                format!("{:.1} MB", value / 1_000_000.0)
            } else if value > 1_000.0 {
                format!("{:.1} KB", value / 1_000.0)
            } else {
                format!("{:.0} B", value)
            }
        }
        unit_str if unit_str.contains("Count") => {
            if value > 1_000.0 {
                format!("{:.1}K", value / 1_000.0)
            } else {
                format!("{:.0}", value)
            }
        }
        _ => {
            if value > 1000.0 {
                format!("{:.0}", value)
            } else {
                format!("{:.2}", value)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_trend_indicator() {
        // Test trending up
        let history = vec![10.0, 12.0, 14.0];
        let current_value = 16.0;
        assert_eq!(
            calculate_trend_indicator(&history, current_value),
            " Trending Up"
        );

        // Test trending down
        let history = vec![20.0, 18.0, 16.0];
        let current_value = 14.0;
        assert_eq!(
            calculate_trend_indicator(&history, current_value),
            " Trending Down"
        );

        // Test stable
        let history = vec![15.0, 15.5, 14.5];
        let current_value = 15.2;
        assert_eq!(
            calculate_trend_indicator(&history, current_value),
            " Stable"
        );

        // Test empty history
        let history = vec![];
        let current_value = 10.0;
        assert_eq!(calculate_trend_indicator(&history, current_value), "");

        // Test single value history
        let history = vec![10.0];
        let current_value = 15.0;
        assert_eq!(calculate_trend_indicator(&history, current_value), "");
    }

    #[test]
    fn test_format_simple_metric_value() {
        // Test percentage
        assert_eq!(format_simple_metric_value(75.5, "Percent"), "75.5%");

        // Test bytes
        assert_eq!(format_simple_metric_value(1_500_000.0, "Bytes"), "1.5 MB");
        assert_eq!(format_simple_metric_value(1_500.0, "Bytes"), "1.5 KB");
        assert_eq!(format_simple_metric_value(500.0, "Bytes"), "500 B");

        // Test count
        assert_eq!(format_simple_metric_value(1_500.0, "Count"), "1.5K");
        assert_eq!(format_simple_metric_value(500.0, "Count"), "500");

        // Test default
        assert_eq!(format_simple_metric_value(1500.0, "Unknown"), "1500");
        assert_eq!(format_simple_metric_value(15.5, "Unknown"), "15.50");
    }

    #[test]
    fn test_trend_thresholds() {
        let history = vec![100.0];

        // Test 10% increase threshold
        assert_eq!(calculate_trend_indicator(&history, 111.0), " Trending Up");
        assert_eq!(calculate_trend_indicator(&history, 110.0), " Stable");

        // Test 10% decrease threshold
        assert_eq!(calculate_trend_indicator(&history, 89.0), " Trending Down");
        assert_eq!(calculate_trend_indicator(&history, 90.0), " Stable");
    }
}
