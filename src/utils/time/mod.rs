use std::time::{SystemTime, UNIX_EPOCH};

/// Calculate relative time from now
pub fn time_ago(timestamp: SystemTime) -> String {
    match SystemTime::now().duration_since(timestamp) {
        Ok(duration) => {
            let seconds = duration.as_secs();
            if seconds < 60 {
                "now".to_string()
            } else if seconds < 3600 {
                let minutes = seconds / 60;
                format!("-{}m", minutes)
            } else if seconds < 86400 {
                let hours = seconds / 3600;
                format!("-{}h", hours)
            } else {
                let days = seconds / 86400;
                format!("-{}d", days)
            }
        }
        Err(_) => "future".to_string(),
    }
}

/// Format timestamp as relative time with appropriate precision
pub fn format_relative_time(timestamp: SystemTime) -> String {
    match SystemTime::now().duration_since(timestamp) {
        Ok(duration) => {
            let total_seconds = duration.as_secs();

            if total_seconds < 60 {
                if total_seconds == 0 {
                    "now".to_string()
                } else {
                    format!("{}s ago", total_seconds)
                }
            } else if total_seconds < 3600 {
                let minutes = total_seconds / 60;
                format!("{}m ago", minutes)
            } else if total_seconds < 86400 {
                let hours = total_seconds / 3600;
                let remaining_minutes = (total_seconds % 3600) / 60;
                if remaining_minutes > 0 {
                    format!("{}h {}m ago", hours, remaining_minutes)
                } else {
                    format!("{}h ago", hours)
                }
            } else {
                let days = total_seconds / 86400;
                let remaining_hours = (total_seconds % 86400) / 3600;
                if remaining_hours > 0 {
                    format!("{}d {}h ago", days, remaining_hours)
                } else {
                    format!("{}d ago", days)
                }
            }
        }
        Err(_) => "in future".to_string(),
    }
}

/// Convert SystemTime to Unix timestamp
pub fn to_unix_timestamp(time: SystemTime) -> u64 {
    time.duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Convert Unix timestamp to SystemTime
pub fn from_unix_timestamp(timestamp: u64) -> SystemTime {
    UNIX_EPOCH + std::time::Duration::from_secs(timestamp)
}

/// Check if two timestamps are within a certain duration of each other
pub fn within_duration(
    time1: SystemTime,
    time2: SystemTime,
    max_diff: std::time::Duration,
) -> bool {
    match time1
        .duration_since(time2)
        .or_else(|_| time2.duration_since(time1))
    {
        Ok(diff) => diff <= max_diff,
        Err(_) => false,
    }
}

/// Round timestamp down to the nearest interval
pub fn round_down_to_interval(timestamp: SystemTime, interval_seconds: u64) -> SystemTime {
    let unix_time = to_unix_timestamp(timestamp);
    let rounded = (unix_time / interval_seconds) * interval_seconds;
    from_unix_timestamp(rounded)
}

/// Generate evenly spaced timestamps between start and end
pub fn generate_timestamp_range(
    start: SystemTime,
    end: SystemTime,
    count: usize,
) -> Vec<SystemTime> {
    if count <= 1 {
        return vec![start];
    }

    let start_unix = to_unix_timestamp(start);
    let end_unix = to_unix_timestamp(end);

    if end_unix <= start_unix {
        return vec![start; count];
    }

    let step = (end_unix - start_unix) / (count - 1) as u64;

    (0..count)
        .map(|i| from_unix_timestamp(start_unix + step * i as u64))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_time_ago() {
        let now = SystemTime::now();
        let one_hour_ago = now - Duration::from_secs(3600);

        assert_eq!(time_ago(now), "now");
        assert_eq!(time_ago(one_hour_ago), "-1h");
    }

    #[test]
    fn test_format_relative_time() {
        let now = SystemTime::now();
        let thirty_seconds_ago = now - Duration::from_secs(30);
        let one_hour_ago = now - Duration::from_secs(3600);

        assert_eq!(format_relative_time(now), "now");
        assert_eq!(format_relative_time(thirty_seconds_ago), "30s ago");
        assert_eq!(format_relative_time(one_hour_ago), "1h ago");
    }

    #[test]
    fn test_unix_timestamp_conversion() {
        let now = SystemTime::now();
        let unix_time = to_unix_timestamp(now);
        let converted_back = from_unix_timestamp(unix_time);

        // Should be within 1 second due to precision
        assert!(within_duration(now, converted_back, Duration::from_secs(1)));
    }

    #[test]
    fn test_within_duration() {
        let time1 = SystemTime::now();
        let time2 = time1 + Duration::from_secs(5);

        assert!(within_duration(time1, time2, Duration::from_secs(10)));
        assert!(!within_duration(time1, time2, Duration::from_secs(1)));
    }

    #[test]
    fn test_generate_timestamp_range() {
        let start = SystemTime::now();
        let end = start + Duration::from_secs(100);

        let range = generate_timestamp_range(start, end, 5);
        assert_eq!(range.len(), 5);
        assert_eq!(range[0], start);
        // Last timestamp should be close to end (within rounding)
        assert!(within_duration(range[4], end, Duration::from_secs(1)));
    }
}
