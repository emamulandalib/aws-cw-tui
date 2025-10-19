use crate::aws::dynamic_metric_discovery::DynamicMetricData;
use crate::ui::charts::chart_data::MetricChartData;
use crate::ui::charts::chart_labels::{
    create_dynamic_value_labels, create_time_labels, create_value_labels,
};
use crate::ui::charts::grid_layout::calculate_y_bounds;
use crate::ui::charts::rendering::simple_charts::render_error_chart;
use crate::ui::components::list_styling::border_factory;
use crate::ui::themes::UnifiedTheme;
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
    is_focused: bool,
) {
    if chart_data.history.is_empty() || chart_data.timestamps.is_empty() {
        render_error_chart(f, area, "No data points", is_focused);
        return;
    }

    // Validate data lengths match
    if chart_data.history.len() != chart_data.timestamps.len() {
        log::warn!(
            "Chart data length mismatch: history={}, timestamps={}",
            chart_data.history.len(),
            chart_data.timestamps.len()
        );
        render_error_chart(f, area, "Data length mismatch", is_focused);
        return;
    }

    // Validate data contains finite values
    let valid_data_count = chart_data
        .history
        .iter()
        .filter(|&&v| !v.is_nan() && !v.is_infinite())
        .count();
    if valid_data_count == 0 {
        render_error_chart(f, area, "No valid data points", is_focused);
        return;
    }

    // Convert timestamps to seconds for x-axis
    let start_time = chart_data.timestamps[0];
    let data_points: Vec<(f64, f64)> = chart_data
        .timestamps
        .iter()
        .zip(chart_data.history.iter())
        .filter_map(|(timestamp, value)| {
            if !value.is_nan() && !value.is_infinite() {
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
    let theme = UnifiedTheme::default();
    let x_labels = create_time_labels(&chart_data.timestamps, &theme);
    let y_labels = create_value_labels(y_bounds, definition, &theme);

    // Create dataset
    let dataset = Dataset::default()
        .name("")
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(health_color))
        .data(&data_points);
    let border_style = border_factory::create_theme_border_style(&theme, is_focused);

    let chart = Chart::new(vec![dataset])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style),
        )
        .x_axis(
            Axis::default()
                .style(Style::default().fg(theme.muted))
                .bounds(x_bounds)
                .labels(x_labels),
        )
        .y_axis(
            Axis::default()
                .style(Style::default().fg(theme.muted))
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
    is_focused: bool,
) {
    // Validate data before rendering
    if metric_data.history.is_empty() || metric_data.timestamps.is_empty() {
        render_error_chart(f, area, "No data points", is_focused);
        return;
    }

    // Validate data lengths match
    if metric_data.history.len() != metric_data.timestamps.len() {
        log::warn!(
            "Dynamic chart data length mismatch: history={}, timestamps={}",
            metric_data.history.len(),
            metric_data.timestamps.len()
        );
        render_error_chart(f, area, "Data length mismatch", is_focused);
        return;
    }

    // Validate data contains finite values
    let valid_data_count = metric_data
        .history
        .iter()
        .filter(|&&v| !v.is_nan() && !v.is_infinite())
        .count();
    if valid_data_count == 0 {
        render_error_chart(f, area, "No valid data points", is_focused);
        return;
    }

    // Convert timestamps to seconds for x-axis
    let start_time = metric_data.timestamps[0];
    let data_points: Vec<(f64, f64)> = metric_data
        .timestamps
        .iter()
        .zip(metric_data.history.iter())
        .filter_map(|(timestamp, value)| {
            if !value.is_nan() && !value.is_infinite() {
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
    let theme = crate::ui::themes::UnifiedTheme::default();
    let x_labels = create_time_labels(&metric_data.timestamps, &theme);
    let y_labels = create_dynamic_value_labels(y_bounds, metric_data, &theme);

    // Create dataset
    let dataset = Dataset::default()
        .name("")
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(
            Style::default().fg(crate::ui::charts::chart_utils::get_dynamic_metric_color(
                &metric_data.metric_name,
            )),
        )
        .data(&data_points);

    let border_style = border_factory::create_theme_border_style(&theme, is_focused);

    let chart = Chart::new(vec![dataset])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style),
        )
        .x_axis(
            Axis::default()
                .style(Style::default().fg(theme.muted))
                .bounds(x_bounds)
                .labels(x_labels),
        )
        .y_axis(
            Axis::default()
                .style(Style::default().fg(theme.muted))
                .bounds(y_bounds)
                .labels(y_labels),
        );

    f.render_widget(chart, area);
}

// Tests commented out due to compilation issues
// #[cfg(test)]
// mod tests {
//     // Test implementation would go here
// }
