// Self-contained sparkline utilities for inline display

/// Generates an elegant inline sparkline using Braille characters for compact visualization
/// This leverages the existing sparkline functionality for consistency
pub fn generate_inline_sparkline(history: &[f64], width: usize) -> String {
    if history.is_empty() || width == 0 {
        return "⠀".repeat(width.max(8));
    }

    if history.len() == 1 {
        // Single data point - show as centered indicator
        let half_width = width / 2;
        let mut sparkline = "⠀".repeat(half_width);
        sparkline.push('⣿');
        sparkline.push_str(&"⠀".repeat(width.saturating_sub(half_width + 1)));
        return sparkline;
    }

    // Calculate Y bounds for normalization
    let (y_min, y_max) = calculate_sparkline_y_bounds(history);
    let y_range = y_max - y_min;

    if y_range == 0.0 {
        // All values are the same - show as flat sparkline with indicators
        return generate_flat_sparkline(width);
    }

    // Sample data to fit the available width
    let sampled_data = sample_sparkline_data(history, width);

    // Normalize data to 0-1 range for Braille character selection
    let normalized_data: Vec<f64> = sampled_data
        .iter()
        .map(|&value| ((value - y_min) / y_range).clamp(0.0, 1.0))
        .collect();

    // Generate inline sparkline using Braille characters
    generate_braille_inline_sparkline(&normalized_data, width)
}

/// Generate a flat sparkline for constant values
fn generate_flat_sparkline(width: usize) -> String {
    if width <= 2 {
        return "⣿".repeat(width);
    }

    let mut sparkline = String::with_capacity(width);
    sparkline.push('⣿');
    sparkline.push_str(&"⣤".repeat(width.saturating_sub(2)));
    sparkline.push('⣿');
    sparkline
}

/// Generate inline sparkline using Braille characters for elegant visualization
fn generate_braille_inline_sparkline(normalized_data: &[f64], width: usize) -> String {
    if normalized_data.is_empty() {
        return "⠀".repeat(width);
    }

    let mut sparkline = String::with_capacity(width);

    for i in 0..width {
        let char_to_use = if i < normalized_data.len() {
            let current_value = normalized_data[i];

            // Map normalized value to Braille character representing vertical position
            match (current_value * 8.0) as usize {
                0 => '⣀',     // Bottom level
                1 => '⣄',     // Low level
                2 => '⣆',     // Low-medium level
                3 => '⣇',     // Medium-low level
                4 => '⣧',     // Medium level
                5 => '⣷',     // Medium-high level
                6 => '⣿',     // High level
                7..=8 => '⣿', // Maximum level
                _ => '⣤',     // Default medium
            }
        } else {
            '⠀' // Empty space
        };

        sparkline.push(char_to_use);
    }

    sparkline
}

/// Calculate Y bounds for sparkline normalization
fn calculate_sparkline_y_bounds(history: &[f64]) -> (f64, f64) {
    let min_val = history.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_val = history.iter().cloned().fold(-f64::INFINITY, f64::max);

    if min_val.is_finite() && max_val.is_finite() {
        if min_val == max_val {
            (min_val, max_val)
        } else {
            let range = max_val - min_val;
            let padding = range * 0.05; // Smaller padding for compact display
            (min_val - padding, max_val + padding)
        }
    } else {
        (0.0, 1.0)
    }
}

/// Sample data points to fit the target width for sparklines
fn sample_sparkline_data(history: &[f64], target_width: usize) -> Vec<f64> {
    if history.len() <= target_width {
        return history.to_owned();
    }

    let mut sampled = Vec::with_capacity(target_width);
    for i in 0..target_width {
        let idx = if target_width == 1 {
            0
        } else {
            (i * (history.len() - 1)) / (target_width - 1)
        };
        sampled.push(history[idx]);
    }
    sampled
}
