use crate::aws::dynamic_metric_discovery::DynamicMetricData;
use crate::ui::components::metric_definitions::MetricDefinition;
use crate::ui::themes::UnifiedTheme;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};
use std::time::SystemTime;

/// Create time labels for X axis
pub fn create_time_labels<'a>(timestamps: &[SystemTime], theme: &'a UnifiedTheme) -> Vec<Line<'a>> {
    use chrono::{DateTime, Utc};

    if timestamps.is_empty() {
        return vec![Line::from("No data")];
    }

    let num_labels = 4.min(timestamps.len());

    (0..num_labels)
        .map(|i| {
            let idx = if num_labels == 1 {
                0
            } else {
                (i * (timestamps.len() - 1)) / (num_labels - 1)
            };

            let timestamp = timestamps[idx];
            let dt: DateTime<Utc> = timestamp.into();
            let local_time: chrono::DateTime<chrono::Local> = dt.into();

            Line::from(Span::styled(
                format!("{}", local_time.format("%H:%M")),
                Style::default().fg(theme.muted),
            ))
        })
        .collect()
}

/// Create value labels for Y axis
pub fn create_value_labels<'a>(
    y_bounds: [f64; 2],
    definition: &MetricDefinition,
    theme: &'a UnifiedTheme,
) -> Vec<Line<'a>> {
    let num_labels = 5;
    let range = y_bounds[1] - y_bounds[0];

    (0..num_labels)
        .map(|i| {
            let ratio = i as f64 / (num_labels - 1) as f64;
            let value = y_bounds[0] + ratio * range;

            Line::from(Span::styled(
                definition.format_value(value),
                Style::default().fg(theme.muted),
            ))
        })
        .collect()
}

/// Create dynamic value labels for Y axis
pub fn create_dynamic_value_labels<'a>(
    y_bounds: [f64; 2],
    _metric_data: &DynamicMetricData,
    theme: &'a UnifiedTheme,
) -> Vec<Line<'a>> {
    let num_labels = 5;
    let range = y_bounds[1] - y_bounds[0];

    (0..num_labels)
        .map(|i| {
            let ratio = i as f64 / (num_labels - 1) as f64;
            let value = y_bounds[0] + ratio * range;

            Line::from(Span::styled(
                format!("{:.2}", value),
                Style::default().fg(theme.muted),
            ))
        })
        .collect()
}
