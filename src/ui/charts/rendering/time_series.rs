use crate::aws::dynamic_metric_discovery::DynamicMetricData;
use crate::ui::charts::chart_data::MetricChartData;
use crate::ui::charts::chart_labels::{
    create_dynamic_value_labels, create_time_labels, create_value_labels,
};
use crate::ui::charts::grid_layout::calculate_y_bounds;
use crate::ui::charts::rendering::simple_charts::render_error_chart;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    symbols,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
    Frame,
};

/// Render time series chart for standard metrics
pub fn render_time_series_chart(
    f: &mut Frame,
    area: Rect,
    chart_data: &MetricChartData,
    definition: &crate::ui::components::metric_definitions::MetricDefinition,
    health_color: Color,
    border_color: Color,
) {
    if chart_data.history.is_empty() || chart_data.timestamps.is_empty() {
        render_error_chart(f, area, "No data points", border_color);
        return;
    }

    // Validate data lengths match
    if chart_data.history.len() != chart_data.timestamps.len() {
        log::warn!(
            "Chart data length mismatch: history={}, timestamps={}",
            chart_data.history.len(),
            chart_data.timestamps.len()
        );
        render_error_chart(f, area, "Data length mismatch", border_color);
        return;
    }

    // Validate data contains finite values
    let valid_data_count = chart_data
        .history
        .iter()
        .filter(|&&v| v.is_finite())
        .count();
    if valid_data_count == 0 {
        render_error_chart(f, area, "No valid data points", border_color);
        return;
    }

    // Convert timestamps to seconds for x-axis
    let start_time = chart_data.timestamps[0];
    let data_points: Vec<(f64, f64)> = chart_data
        .timestamps
        .iter()
        .zip(chart_data.history.iter())
        .filter_map(|(timestamp, value)| {
            if value.is_finite() {
                let seconds = timestamp
                    .duration_since(start_time)
                    .unwrap_or_default()
                    .as_secs() as f64;
                Some((seconds, *value))
            } else {
                None
            }
        })
        .collect();

    // Calculate bounds
    let x_bounds = [0.0, data_points.last().map(|p| p.0).unwrap_or(1.0)];
    let y_bounds = calculate_y_bounds(&chart_data.history);

    // Create labels
    let x_labels = create_time_labels(&chart_data.timestamps);
    let y_labels = create_value_labels(y_bounds, definition);

    // Create dataset
    let dataset = Dataset::default()
        .name("")
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(health_color))
        .data(&data_points);

    let chart = Chart::new(vec![dataset])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color)),
        )
        .x_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .bounds(x_bounds)
                .labels(x_labels),
        )
        .y_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .bounds(y_bounds)
                .labels(y_labels),
        );

    f.render_widget(chart, area);
}

/// Render dynamic time series chart
pub fn render_dynamic_time_series_chart(
    f: &mut Frame,
    area: Rect,
    metric_data: &DynamicMetricData,
    border_color: Color,
) {
    // Validate data before rendering
    if metric_data.history.is_empty() || metric_data.timestamps.is_empty() {
        render_error_chart(f, area, "No data points", border_color);
        return;
    }

    // Validate data lengths match
    if metric_data.history.len() != metric_data.timestamps.len() {
        log::warn!(
            "Dynamic chart data length mismatch: history={}, timestamps={}",
            metric_data.history.len(),
            metric_data.timestamps.len()
        );
        render_error_chart(f, area, "Data length mismatch", border_color);
        return;
    }

    // Validate data contains finite values
    let valid_data_count = metric_data
        .history
        .iter()
        .filter(|&&v| v.is_finite())
        .count();
    if valid_data_count == 0 {
        render_error_chart(f, area, "No valid data points", border_color);
        return;
    }

    // Convert timestamps to seconds for x-axis
    let start_time = metric_data.timestamps[0];
    let data_points: Vec<(f64, f64)> = metric_data
        .timestamps
        .iter()
        .zip(metric_data.history.iter())
        .filter_map(|(timestamp, value)| {
            if value.is_finite() {
                let seconds = timestamp
                    .duration_since(start_time)
                    .unwrap_or_default()
                    .as_secs() as f64;
                Some((seconds, *value))
            } else {
                None
            }
        })
        .collect();

    // Calculate bounds
    let x_bounds = [0.0, data_points.last().map(|p| p.0).unwrap_or(1.0)];
    let y_bounds = calculate_y_bounds(&metric_data.history);

    // Create labels
    let x_labels = create_time_labels(&metric_data.timestamps);
    let y_labels = create_dynamic_value_labels(y_bounds, metric_data);

    // Create dataset
    let dataset = Dataset::default()
        .name("")
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Green))
        .data(&data_points);

    let chart = Chart::new(vec![dataset])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color)),
        )
        .x_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .bounds(x_bounds)
                .labels(x_labels),
        )
        .y_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .bounds(y_bounds)
                .labels(y_labels),
        );

    f.render_widget(chart, area);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::MetricType;
    use std::time::{Duration, UNIX_EPOCH};

    #[test]
    fn test_data_point_conversion() {
        let start_time = UNIX_EPOCH + Duration::from_secs(1000);
        let timestamps = vec![
            start_time,
            start_time + Duration::from_secs(60),
            start_time + Duration::from_secs(120),
        ];
        let history = vec![10.0, 20.0, 30.0];

        let data_points: Vec<(f64, f64)> = timestamps
            .iter()
            .zip(history.iter())
            .filter_map(|(timestamp, value)| {
                if value.is_finite() {
                    let seconds = timestamp
                        .duration_since(start_time)
                        .unwrap_or_default()
                        .as_secs() as f64;
                    Some((seconds, *value))
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(data_points.len(), 3);
        assert_eq!(data_points[0], (0.0, 10.0));
        assert_eq!(data_points[1], (60.0, 20.0));
        assert_eq!(data_points[2], (120.0, 30.0));
    }

    #[test]
    fn test_invalid_data_filtering() {
        let history = vec![10.0, f64::NAN, 30.0, f64::INFINITY, 50.0];
        let valid_count = history.iter().filter(|&&v| v.is_finite()).count();
        assert_eq!(valid_count, 3);
    }

    #[test]
    fn test_x_bounds_calculation() {
        let data_points = vec![(0.0, 10.0), (60.0, 20.0), (120.0, 30.0)];
        let x_bounds = [0.0, data_points.last().map(|p| p.0).unwrap_or(1.0)];
        assert_eq!(x_bounds, [0.0, 120.0]);
    }
}
