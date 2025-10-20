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

/// Render metric title with current value
pub fn render_metric_title(
    f: &mut Frame,
    area: Rect,
    definition: &crate::ui::components::metric_definitions::MetricDefinition,
    chart_data: &MetricChartData,
    health_color: Color,
    is_focused: bool,
) {
    let theme = UnifiedTheme::default();
    let border_style = border_factory::create_theme_border_style(&theme, is_focused);

    let title_text = format!(
        "{}: {}",
        definition.name,
        definition.format_value(chart_data.current_value)
    );

    let title_widget = Paragraph::new(title_text)
        .style(
            Style::default()
                .fg(health_color)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style),
        );

    f.render_widget(title_widget, area);
}

/// Render dynamic metric title with current value
pub fn render_dynamic_metric_title(
    f: &mut Frame,
    area: Rect,
    metric_data: &DynamicMetricData,
    is_focused: bool,
) {
    let theme = UnifiedTheme::default();
    let border_style = border_factory::create_theme_border_style(&theme, is_focused);

    let formatted_value = format_dynamic_metric_value(
        metric_data.current_value,
        metric_data.unit.as_deref().unwrap_or(""),
    );

    let title_text = format!("{}: {}", metric_data.display_name, formatted_value);

    let title_widget = Paragraph::new(title_text)
        .style(
            Style::default()
                .fg(theme.success) // Use theme success color instead of hardcoded green
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style),
        );

    f.render_widget(title_widget, area);
}

/// Format dynamic metric value with appropriate units and precision
fn format_dynamic_metric_value(value: f64, unit: &str) -> String {
    match unit {
        unit_str if unit_str.contains("Percent") => {
            format!("{:.1}%", value)
        }
        unit_str if unit_str.contains("Bytes") => {
            if value > 1_000_000_000.0 {
                format!("{:.1} GB", value / 1_000_000_000.0)
            } else if value > 1_000_000.0 {
                format!("{:.1} MB", value / 1_000_000.0)
            } else if value > 1_000.0 {
                format!("{:.1} KB", value / 1_000.0)
            } else {
                format!("{:.0} B", value)
            }
        }
        unit_str if unit_str.contains("Seconds") => {
            if value > 60.0 {
                format!("{:.1} min", value / 60.0)
            } else {
                format!("{:.2} s", value)
            }
        }
        unit_str if unit_str.contains("Count") => {
            if value > 1_000_000.0 {
                format!("{:.1}M", value / 1_000_000.0)
            } else if value > 1_000.0 {
                format!("{:.1}K", value / 1_000.0)
            } else {
                format!("{:.0}", value)
            }
        }
        _ => {
            // Default formatting for unknown units
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
    fn test_format_dynamic_metric_value() {
        // Test percentage formatting
        assert_eq!(format_dynamic_metric_value(75.5, "Percent"), "75.5%");

        // Test bytes formatting
        assert_eq!(
            format_dynamic_metric_value(1_500_000_000.0, "Bytes"),
            "1.5 GB"
        );
        assert_eq!(format_dynamic_metric_value(1_500_000.0, "Bytes"), "1.5 MB");
        assert_eq!(format_dynamic_metric_value(1_500.0, "Bytes"), "1.5 KB");
        assert_eq!(format_dynamic_metric_value(500.0, "Bytes"), "500 B");

        // Test seconds formatting
        assert_eq!(format_dynamic_metric_value(120.0, "Seconds"), "2.0 min");
        assert_eq!(format_dynamic_metric_value(30.5, "Seconds"), "30.50 s");

        // Test count formatting
        assert_eq!(format_dynamic_metric_value(1_500_000.0, "Count"), "1.5M");
        assert_eq!(format_dynamic_metric_value(1_500.0, "Count"), "1.5K");
        assert_eq!(format_dynamic_metric_value(150.0, "Count"), "150");

        // Test default formatting
        assert_eq!(format_dynamic_metric_value(1500.0, "Unknown"), "1500");
        assert_eq!(format_dynamic_metric_value(15.5, "Unknown"), "15.50");
    }

    #[test]
    fn test_title_text_format() {
        let display_name = "CPU Utilization";
        let formatted_value = "75.5%";
        let expected = "CPU Utilization: 75.5%";
        let actual = format!("{}: {}", display_name, formatted_value);
        assert_eq!(actual, expected);
    }
}
