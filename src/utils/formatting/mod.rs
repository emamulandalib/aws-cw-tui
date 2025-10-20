// Removed unused imports to eliminate compiler warnings

/// Format bytes with appropriate units (B, KB, MB, GB)
pub fn format_bytes(bytes: f64) -> String {
    if bytes < 1024.0 {
        format!("{:.0}B", bytes)
    } else if bytes < 1024.0 * 1024.0 {
        format!("{:.1}KB", bytes / 1024.0)
    } else if bytes < 1024.0 * 1024.0 * 1024.0 {
        format!("{:.1}MB", bytes / (1024.0 * 1024.0))
    } else {
        format!("{:.1}GB", bytes / (1024.0 * 1024.0 * 1024.0))
    }
}

/// Format duration in seconds with appropriate units
pub fn format_duration(seconds: f64) -> String {
    if seconds < 1.0 {
        format!("{:.2}ms", seconds * 1000.0)
    } else if seconds < 60.0 {
        format!("{:.1}s", seconds)
    } else if seconds < 3600.0 {
        format!("{:.1}m", seconds / 60.0)
    } else {
        format!("{:.1}h", seconds / 3600.0)
    }
}

/// Format a value as a percentage
pub fn format_percentage(value: f64) -> String {
    format!("{:.1}%", value * 100.0)
}
