/// Truncate string to fit within specified length with ellipsis
pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

/// Calculate responsive time panel width based on terminal width
pub fn calculate_time_panel_width(total_width: u16) -> u16 {
    // Responsive breakpoints: min 20, max 25 chars based on available space
    if total_width < 60 {
        20 // Minimum usable width for small terminals
    } else if total_width < 100 {
        22 // Medium terminals
    } else {
        25 // Large terminals - maximum width to preserve space for trend chart
    }
}

/// Get abbreviated display name for time range selection
pub fn get_selected_time_range_display(selected_time_period: &str) -> String {
    match selected_time_period {
        "5 minutes" => "5m".to_string(),
        "1 hour" => "1h".to_string(),
        "3 hours" => "3h".to_string(),
        "6 hours" => "6h".to_string(),
        "12 hours" => "12h".to_string(),
        "1 day" => "1d".to_string(),
        "3 days" => "3d".to_string(),
        "1 week" => "1w".to_string(),
        "2 weeks" => "2w".to_string(),
        "1 month" => "1m".to_string(),
        _ => selected_time_period.to_string(), // Fallback to original if no match
    }
}
