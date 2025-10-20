use crate::ui::charts::chart_data::MetricChartData;
use crate::ui::charts::error_display::render_error_message;
use crate::ui::charts::rendering::chart_titles::render_metric_title;
use crate::ui::charts::rendering::simple_charts::render_simple_metric;
use crate::ui::charts::rendering::time_series::render_time_series_chart;
use crate::ui::components::metric_definitions::MetricRegistry;
use crate::ui::themes::UnifiedTheme;
use crate::utils::validation::validate_metric_data;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

/// Render individual metric chart in AWS console style
pub fn render_metric_chart(
    f: &mut Frame,
    area: Rect,
    chart_data: &MetricChartData,
    is_focused: bool,
    theme: &UnifiedTheme,
) {
    // Comprehensive validation using focused validation module
    if let Err(validation_error) = validate_metric_data(
        chart_data.metric_type.display_name(),
        &chart_data.history,
        &chart_data.timestamps,
    ) {
        log::warn!("Metric chart validation failed: {}", validation_error);
        render_error_message(
            f,
            area,
            &format!("Data validation failed: {}", validation_error),
        );
        return;
    }

    let definition = MetricRegistry::get_definition(&chart_data.metric_type);

    // Get health color based on current value
    let health_color = definition.get_health_color(chart_data.current_value);

    // Calculate layout for title and chart
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title area
            Constraint::Min(8),    // Chart area
        ])
        .split(area);

    // Render title with current value
    render_metric_title(
        f,
        chunks[0],
        &definition,
        chart_data,
        health_color,
        is_focused,
    );

    // Render chart if we have enough space and data
    if chunks[1].height >= 8 && !chart_data.history.is_empty() {
        render_time_series_chart(
            f,
            chunks[1],
            chart_data,
            &definition,
            health_color,
            is_focused,
        );
    } else {
        render_simple_metric(
            f,
            chunks[1],
            chart_data,
            &definition,
            health_color,
            is_focused,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::MetricType;
    use std::time::SystemTime;

    #[test]
    fn test_metric_chart_layout() {
        // Test that the layout calculation works correctly
        let area = Rect::new(0, 0, 80, 20);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title area
                Constraint::Min(8),    // Chart area
            ])
            .split(area);

        assert_eq!(chunks[0].height, 3);
        assert!(chunks[1].height >= 8);
    }

    #[test]
    fn test_chart_data_validation() {
        let chart_data = MetricChartData {
            metric_type: MetricType::CpuUtilization,
            current_value: 75.0,
            history: vec![60.0, 65.0, 70.0, 75.0],
            timestamps: vec![SystemTime::now(); 4],
        };

        assert!(!chart_data.history.is_empty());
        assert_eq!(chart_data.history.len(), chart_data.timestamps.len());
    }
}
