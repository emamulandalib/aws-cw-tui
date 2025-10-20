/// Grid layout configuration
#[derive(Debug, Clone)]
pub struct GridLayout {
    pub rows: usize,
    pub cols: usize,
}

/// Calculate scrollable grid layout (always 2 columns for readability)
pub fn calculate_scrollable_grid_layout(
    visible_metric_count: usize,
    metrics_per_row: usize,
) -> GridLayout {
    if visible_metric_count == 0 {
        return GridLayout { rows: 1, cols: 1 };
    }

    let cols = metrics_per_row; // Always 2 columns for better readability
    let rows = visible_metric_count.div_ceil(cols); // Ceiling division

    GridLayout { rows, cols }
}

/// Calculate Y axis bounds for chart
pub fn calculate_y_bounds(history: &[f64]) -> [f64; 2] {
    if history.is_empty() {
        return [0.0, 1.0];
    }

    if history.len() == 1 {
        let val = history[0];
        let margin = if val.abs() > 1.0 {
            val.abs() * 0.1
        } else {
            1.0
        };
        [val - margin, val + margin]
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
            [y_min, max_val + padding]
        } else {
            [0.0, 1.0]
        }
    }
}
