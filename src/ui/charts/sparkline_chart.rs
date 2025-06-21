use ratatui::{
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::time::SystemTime;

/// Renders a compact sparkline chart using Braille Unicode characters
/// Optimized for small dimensions (20-30 characters wide, 3-4 lines tall)
#[allow(dead_code)]
pub fn render_sparkline(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    timestamps: &[SystemTime],
    history: &[f64],
    color: Color,
    metric_name: &str,
) {
    // Handle empty or insufficient data
    if history.is_empty() || timestamps.is_empty() {
        render_no_data_sparkline(f, area, metric_name, color);
        return;
    }

    // Check if we have enough space for sparkline rendering
    if area.width < 10 || area.height < 3 {
        render_minimal_sparkline(f, area, history, color, metric_name);
        return;
    }

    // Calculate the available width for the sparkline (accounting for borders)
    let chart_width = (area.width.saturating_sub(2)) as usize;
    let chart_height = (area.height.saturating_sub(2)) as usize;

    // Generate sparkline using Braille characters
    let sparkline_content = generate_braille_sparkline(history, chart_width, chart_height);

    // Apply health-based color coding
    let sparkline_color = get_health_color(history, metric_name, color);

    let sparkline_widget = Paragraph::new(sparkline_content)
        .style(Style::default().fg(sparkline_color))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(metric_name)
                .border_style(Style::default().fg(Color::White)),
        );

    f.render_widget(sparkline_widget, area);
}

/// Generates a sparkline using Braille Unicode characters for high-resolution rendering
#[allow(dead_code)]
fn generate_braille_sparkline(history: &[f64], width: usize, height: usize) -> Vec<Line<'static>> {
    if history.is_empty() || width == 0 || height == 0 {
        return vec![Line::from("No data")];
    }

    // Calculate Y bounds using the same logic as main charts
    let (y_min, y_max) = calculate_y_bounds(history);
    if y_max <= y_min {
        return vec![Line::from("─".repeat(width))];
    }

    // Sample data points to fit the available width
    // Each Braille character represents 2x4 pixels, so we can fit more data
    let points_per_char = 2;
    let total_points = width * points_per_char;
    let sampled_data = sample_data(history, total_points);

    // Convert to Braille characters and return directly
    create_braille_lines_owned(sampled_data, y_min, y_max, width, height)
}

/// Samples data to fit the available width while preserving trend
#[allow(dead_code)]
fn sample_data(history: &[f64], target_points: usize) -> Vec<f64> {
    if history.len() <= target_points {
        return history.to_vec();
    }

    let mut sampled = Vec::with_capacity(target_points);
    for i in 0..target_points {
        let idx = (i * (history.len() - 1)) / (target_points - 1);
        sampled.push(history[idx]);
    }
    sampled
}

/// Creates lines of text using Braille characters to represent the sparkline
#[allow(dead_code)]
fn create_braille_lines_owned(
    data: Vec<f64>,
    y_min: f64,
    y_max: f64,
    width: usize,
    height: usize,
) -> Vec<Line<'static>> {
    let mut lines = Vec::with_capacity(height);
    let y_range = y_max - y_min;

    // Braille characters are used directly in the match expressions below
    // Each character represents a 2x4 grid of dots for high-resolution rendering

    // For simplicity, create a basic line chart using selected Braille patterns
    for row in 0..height {
        let mut line_chars = Vec::with_capacity(width);

        for col in 0..width {
            let data_idx = (col * 2).min(data.len().saturating_sub(1));
            if data_idx < data.len() {
                let normalized_value = if y_range > 0.0 {
                    ((data[data_idx] - y_min) / y_range).clamp(0.0, 1.0)
                } else {
                    0.5
                };

                // Map the normalized value to the row position
                let target_row = ((1.0 - normalized_value) * (height - 1) as f64) as usize;

                // Choose Braille character based on position
                let char_to_use = if target_row == row {
                    match (normalized_value * 8.0) as usize {
                        0..=1 => '⣀', // Bottom dots
                        2..=3 => '⣤', // Middle dots
                        4..=5 => '⣶', // Upper-middle dots
                        _ => '⣿',     // Full dots
                    }
                } else if target_row.abs_diff(row) <= 1 {
                    match (normalized_value * 4.0) as usize {
                        0 => '⣀',
                        1 => '⣄',
                        2 => '⣆',
                        _ => '⣇',
                    }
                } else {
                    '⠀' // Empty
                };

                line_chars.push(char_to_use);
            } else {
                line_chars.push('⠀');
            }
        }

        let line_string: String = line_chars.into_iter().collect();
        lines.push(Line::from(line_string));
    }

    lines
}

/// Calculates Y bounds using the same logic as the main charts for consistency
#[allow(dead_code)]
fn calculate_y_bounds(history: &[f64]) -> (f64, f64) {
    if history.len() == 1 {
        let val = history[0];
        let margin = if val.abs() > 1.0 {
            val.abs() * 0.1
        } else {
            1.0
        };
        (val - margin, val + margin)
    } else {
        let min_val = history.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_val = history.iter().cloned().fold(-f64::INFINITY, f64::max);
        if min_val.is_finite() && max_val.is_finite() && min_val != max_val {
            let range = max_val - min_val;
            let padding = range * 0.1;
            let y_min = if min_val >= 0.0 {
                (min_val - padding).max(0.0)
            } else {
                min_val - padding
            };
            (y_min, max_val + padding)
        } else {
            (0.0, 1.0)
        }
    }
}

/// Determines health-based color coding based on metric values
#[allow(dead_code)]
fn get_health_color(history: &[f64], metric_name: &str, default_color: Color) -> Color {
    if history.is_empty() {
        return Color::DarkGray;
    }

    let latest_value = history[history.len() - 1];
    let avg_value = history.iter().sum::<f64>() / history.len() as f64;

    // Health-based color logic based on metric type
    match metric_name {
        name if name.contains("CPU") => {
            if latest_value > 80.0 || avg_value > 70.0 {
                Color::Red // High CPU usage
            } else if latest_value > 60.0 || avg_value > 50.0 {
                Color::Yellow // Medium CPU usage
            } else {
                Color::Green // Normal CPU usage
            }
        }
        name if name.contains("Latency") => {
            // Latency values are in seconds, convert to ms for threshold
            let latest_ms = latest_value * 1000.0;
            let avg_ms = avg_value * 1000.0;
            if latest_ms > 100.0 || avg_ms > 50.0 {
                Color::Red // High latency
            } else if latest_ms > 50.0 || avg_ms > 25.0 {
                Color::Yellow // Medium latency
            } else {
                Color::Green // Low latency
            }
        }
        name if name.contains("Memory") || name.contains("Storage") => {
            // For memory/storage, lower values (less free space) are worse
            let gb_value = latest_value / (1024.0 * 1024.0 * 1024.0);
            if gb_value < 1.0 {
                Color::Red // Very low free space
            } else if gb_value < 5.0 {
                Color::Yellow // Low free space
            } else {
                Color::Green // Adequate free space
            }
        }
        name if name.contains("Connection") => {
            if latest_value > 150.0 || avg_value > 100.0 {
                Color::Red // High connection count
            } else if latest_value > 75.0 || avg_value > 50.0 {
                Color::Yellow // Medium connection count
            } else {
                Color::Green // Normal connection count
            }
        }
        name if name.contains("Failed") || name.contains("Error") => {
            if latest_value > 0.0 || avg_value > 0.0 {
                Color::Red // Any failures are bad
            } else {
                Color::Green // No failures
            }
        }
        _ => {
            // For other metrics, use trend-based coloring
            if history.len() > 1 {
                let trend = latest_value - history[0];
                let relative_change = if history[0] != 0.0 {
                    (trend / history[0]).abs()
                } else {
                    0.0
                };

                if relative_change > 0.5 {
                    Color::Yellow // Significant change
                } else {
                    default_color // Use provided color
                }
            } else {
                default_color
            }
        }
    }
}

/// Renders a minimal sparkline when space is very limited
#[allow(dead_code)]
fn render_minimal_sparkline(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    history: &[f64],
    color: Color,
    metric_name: &str,
) {
    let trend_symbol = if history.len() > 1 {
        let start = history[0];
        let end = history[history.len() - 1];
        if end > start * 1.1 {
            "↗" // Trending up
        } else if end < start * 0.9 {
            "↘" // Trending down
        } else {
            "→" // Stable
        }
    } else {
        "─" // No trend data
    };

    let health_color = get_health_color(history, metric_name, color);
    let minimal_widget = Paragraph::new(format!("{} {}", metric_name, trend_symbol))
        .style(Style::default().fg(health_color));

    f.render_widget(minimal_widget, area);
}

/// Renders a message when no data is available
#[allow(dead_code)]
fn render_no_data_sparkline(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    metric_name: &str,
    _color: Color,
) {
    let no_data_widget = Paragraph::new("No data")
        .style(Style::default().fg(Color::DarkGray))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(metric_name)
                .border_style(Style::default().fg(Color::DarkGray)),
        );

    f.render_widget(no_data_widget, area);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_y_bounds_single_value() {
        let history = vec![10.0];
        let (min, max) = calculate_y_bounds(&history);
        assert!(min < 10.0);
        assert!(max > 10.0);
    }

    #[test]
    fn test_calculate_y_bounds_multiple_values() {
        let history = vec![1.0, 5.0, 3.0, 7.0];
        let (min, max) = calculate_y_bounds(&history);
        assert!(min <= 1.0);
        assert!(max >= 7.0);
    }

    #[test]
    fn test_sample_data() {
        let history = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let sampled = sample_data(&history, 3);
        assert_eq!(sampled.len(), 3);
        assert_eq!(sampled[0], 1.0);
        assert_eq!(sampled[2], 5.0);
    }

    #[test]
    fn test_health_color_cpu() {
        let high_cpu = vec![85.0, 90.0, 88.0];
        let color = get_health_color(&high_cpu, "CPU Utilization", Color::Blue);
        assert_eq!(color, Color::Red);

        let normal_cpu = vec![20.0, 30.0, 25.0];
        let color = get_health_color(&normal_cpu, "CPU Utilization", Color::Blue);
        assert_eq!(color, Color::Green);
    }

    #[test]
    fn test_health_color_latency() {
        let high_latency = vec![0.15, 0.12, 0.18]; // 150ms, 120ms, 180ms
        let color = get_health_color(&high_latency, "Read Latency", Color::Blue);
        assert_eq!(color, Color::Red);

        let low_latency = vec![0.01, 0.015, 0.012]; // 10ms, 15ms, 12ms
        let color = get_health_color(&low_latency, "Read Latency", Color::Blue);
        assert_eq!(color, Color::Green);
    }
}
