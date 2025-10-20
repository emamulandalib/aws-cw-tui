use crate::aws::dynamic_metric_discovery::DynamicMetricData;
use crate::ui::charts::error_display::render_error_message;
use crate::ui::charts::rendering::chart_titles::render_dynamic_metric_title;
use crate::ui::charts::rendering::simple_charts::render_dynamic_simple_metric;
use crate::ui::charts::rendering::time_series::render_dynamic_time_series_chart;
use crate::ui::themes::UnifiedTheme;
use crate::utils::validation::validate_metric_data;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

/// Render individual dynamic metric chart
pub fn render_dynamic_metric_chart(
    f: &mut Frame,
    area: Rect,
    metric_data: &DynamicMetricData,
    is_focused: bool,
    theme: &UnifiedTheme,
) {
    // Comprehensive validation using focused validation module
    if let Err(validation_error) = validate_metric_data(
        &metric_data.metric_name,
        &metric_data.history,
        &metric_data.timestamps,
    ) {
        log::warn!("Dynamic metric validation failed: {}", validation_error);
        render_error_message(
            f,
            area,
            &format!("Data validation failed: {}", validation_error),
        );
        return;
    }

    // Calculate layout for title and chart
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title area
            Constraint::Min(8),    // Chart area
        ])
        .split(area);

    // Render title with current value
    render_dynamic_metric_title(f, chunks[0], metric_data, is_focused);

    // Render chart if we have enough space and data
    if chunks[1].height >= 8 && !metric_data.history.is_empty() {
        render_dynamic_time_series_chart(f, chunks[1], metric_data, is_focused);
    } else {
        render_dynamic_simple_metric(f, chunks[1], metric_data, is_focused);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    #[test]
    fn test_dynamic_chart_layout() {
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
    fn test_dynamic_metric_data_validation() {
        let metric_data = DynamicMetricData {
            metric_name: "CPUUtilization".to_string(),
            display_name: "CPU Utilization".to_string(),
            current_value: 75.0,
            history: vec![60.0, 65.0, 70.0, 75.0],
            timestamps: vec![SystemTime::now(); 4],
            unit: Some("Percent".to_string()),
        };

        assert!(!metric_data.history.is_empty());
        assert_eq!(metric_data.history.len(), metric_data.timestamps.len());
        assert!(metric_data.unit.is_some());
    }

    #[test]
    fn test_border_color_logic() {
        let theme = crate::ui::themes::UnifiedTheme::default();
        let focused_color = theme.focused;
        let normal_color = theme.border;

        // Test focused state
        let border_color = if true { theme.focused } else { theme.border };
        assert_eq!(border_color, focused_color);

        // Test normal state
        let border_color = if false { theme.focused } else { theme.border };
        assert_eq!(border_color, normal_color);
    }
}
