use super::display_format::DisplayFormat;
use super::health_thresholds::HealthThresholds;
use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct MetricDefinition {
    pub name: &'static str,
    pub display_format: DisplayFormat,
    pub thresholds: Option<HealthThresholds>,
    pub color: Color,
    pub description: &'static str,
}

impl MetricDefinition {
    /// Create a new metric definition
    pub fn new(
        name: &'static str,
        display_format: DisplayFormat,
        thresholds: Option<HealthThresholds>,
        color: Color,
        description: &'static str,
    ) -> Self {
        Self {
            name,
            display_format,
            thresholds,
            color,
            description,
        }
    }

    /// Get health color based on current value and thresholds
    pub fn get_health_color(&self, value: f64) -> Color {
        match &self.thresholds {
            Some(thresholds) => thresholds.get_health_color(value),
            None => self.color, // Use default color if no thresholds
        }
    }

    /// Format value according to display format
    pub fn format_value(&self, value: f64) -> String {
        self.display_format.format_value(value)
    }

    /// Check if metric has health thresholds
    pub fn has_thresholds(&self) -> bool {
        self.thresholds.is_some()
    }

    /// Get metric status based on thresholds
    pub fn get_status(&self, value: f64) -> MetricStatus {
        match &self.thresholds {
            Some(thresholds) => {
                if thresholds.is_critical(value) {
                    MetricStatus::Critical
                } else if thresholds.is_warning(value) {
                    MetricStatus::Warning
                } else {
                    MetricStatus::Healthy
                }
            }
            None => MetricStatus::Unknown,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MetricStatus {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

impl MetricStatus {
    pub fn color(&self) -> Color {
        let theme = crate::ui::themes::UnifiedTheme::default();
        match self {
            MetricStatus::Healthy => theme.success,
            MetricStatus::Warning => theme.warning,
            MetricStatus::Critical => theme.error,
            MetricStatus::Unknown => theme.muted,
        }
    }
}
