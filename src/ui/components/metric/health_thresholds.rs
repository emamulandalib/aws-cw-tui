use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct HealthThresholds {
    pub critical: f64,
    pub warning: f64,
    pub reverse_logic: bool, // true for metrics where lower values are bad (like free memory)
}

impl HealthThresholds {
    /// Create new health thresholds
    pub fn new(critical: f64, warning: f64, reverse_logic: bool) -> Self {
        Self {
            critical,
            warning,
            reverse_logic,
        }
    }

    /// Get health color based on current value and thresholds
    pub fn get_health_color(&self, value: f64) -> Color {
        let theme = crate::ui::themes::UnifiedTheme::default();
        if self.reverse_logic {
            // For metrics like free memory where lower is worse
            if value < self.critical {
                theme.error
            } else if value < self.warning {
                theme.warning
            } else {
                theme.success
            }
        } else {
            // For metrics like CPU where higher is worse
            if value > self.critical {
                theme.error
            } else if value > self.warning {
                theme.warning
            } else {
                theme.success
            }
        }
    }

    /// Check if value is in critical range
    pub fn is_critical(&self, value: f64) -> bool {
        if self.reverse_logic {
            value < self.critical
        } else {
            value > self.critical
        }
    }

    /// Check if value is in warning range
    pub fn is_warning(&self, value: f64) -> bool {
        if self.reverse_logic {
            value < self.warning && value >= self.critical
        } else {
            value > self.warning && value <= self.critical
        }
    }

    /// Check if value is in healthy range
    pub fn is_healthy(&self, value: f64) -> bool {
        !self.is_critical(value) && !self.is_warning(value)
    }
}
