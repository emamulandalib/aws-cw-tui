use crate::models::{AppState, AwsService};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Workflow memory system to track and optimize common user patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowMemory {
    /// Recently accessed services with frequency
    pub recent_services: HashMap<AwsService, ServiceUsage>,

    /// Recently accessed instances with frequency
    pub recent_instances: HashMap<String, InstanceUsage>,

    /// Common navigation patterns
    pub navigation_patterns: Vec<NavigationPattern>,

    /// Preferred time ranges for different services
    pub preferred_time_ranges: HashMap<AwsService, String>,

    /// User preferences for metrics display
    pub metric_preferences: MetricPreferences,

    /// Session statistics
    pub session_stats: SessionStats,
}

/// Service usage tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceUsage {
    pub access_count: u32,
    pub last_accessed: Option<std::time::SystemTime>,
    pub average_session_duration: Duration,
    pub preferred_instances: Vec<String>,
}

/// Instance usage tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceUsage {
    pub service_type: AwsService,
    pub access_count: u32,
    pub last_accessed: Option<std::time::SystemTime>,
    pub preferred_metrics: Vec<String>,
    pub average_view_duration: Duration,
}

/// Navigation pattern tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationPattern {
    pub sequence: Vec<AppState>,
    pub frequency: u32,
    pub average_duration: Duration,
    pub success_rate: f32, // Percentage of times user completed the workflow
}

/// User preferences for metrics display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricPreferences {
    pub preferred_chart_type: ChartType,
    pub auto_refresh_enabled: bool,
    pub refresh_interval: Duration,
    pub show_sparklines: bool,
    pub metrics_per_page: usize,
    pub preferred_time_format: TimeFormat,
}

/// Chart display preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChartType {
    Line,
    Bar,
    Sparkline,
    Combined,
}

/// Time format preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeFormat {
    Relative,
    Absolute,
    Both,
}

/// Session statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStats {
    pub total_sessions: u32,
    pub average_session_duration: Duration,
    pub most_used_service: Option<AwsService>,
    pub most_viewed_instance: Option<String>,
    pub common_errors: HashMap<String, u32>,
    pub feature_usage: HashMap<String, u32>,
}

impl Default for WorkflowMemory {
    fn default() -> Self {
        Self {
            recent_services: HashMap::new(),
            recent_instances: HashMap::new(),
            navigation_patterns: Vec::new(),
            preferred_time_ranges: HashMap::new(),
            metric_preferences: MetricPreferences::default(),
            session_stats: SessionStats::default(),
        }
    }
}

impl Default for MetricPreferences {
    fn default() -> Self {
        Self {
            preferred_chart_type: ChartType::Line,
            auto_refresh_enabled: true,
            refresh_interval: Duration::from_secs(30),
            show_sparklines: true,
            metrics_per_page: 4,
            preferred_time_format: TimeFormat::Relative,
        }
    }
}

impl Default for SessionStats {
    fn default() -> Self {
        Self {
            total_sessions: 0,
            average_session_duration: Duration::from_secs(0),
            most_used_service: None,
            most_viewed_instance: None,
            common_errors: HashMap::new(),
            feature_usage: HashMap::new(),
        }
    }
}

impl WorkflowMemory {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record service access
    pub fn record_service_access(&mut self, service: AwsService) {
        let usage = self
            .recent_services
            .entry(service)
            .or_insert_with(|| ServiceUsage {
                access_count: 0,
                last_accessed: None,
                average_session_duration: Duration::from_secs(0),
                preferred_instances: Vec::new(),
            });

        usage.access_count += 1;
        usage.last_accessed = Some(std::time::SystemTime::now());
    }

    /// Record instance access
    pub fn record_instance_access(&mut self, instance_id: String, service: AwsService) {
        let usage = self
            .recent_instances
            .entry(instance_id.clone())
            .or_insert_with(|| InstanceUsage {
                service_type: service,
                access_count: 0,
                last_accessed: None,
                preferred_metrics: Vec::new(),
                average_view_duration: Duration::from_secs(0),
            });

        usage.access_count += 1;
        usage.last_accessed = Some(std::time::SystemTime::now());

        // Also update service's preferred instances
        if let Some(service_usage) = self.recent_services.get_mut(&service) {
            if !service_usage.preferred_instances.contains(&instance_id) {
                service_usage.preferred_instances.push(instance_id);
                // Keep only top 5 most recent instances
                if service_usage.preferred_instances.len() > 5 {
                    service_usage.preferred_instances.remove(0);
                }
            }
        }
    }

    /// Record navigation pattern
    pub fn record_navigation(&mut self, from_state: AppState, to_state: AppState) {
        // Find existing pattern or create new one
        let pattern_sequence = vec![from_state, to_state];

        if let Some(pattern) = self
            .navigation_patterns
            .iter_mut()
            .find(|p| p.sequence == pattern_sequence)
        {
            pattern.frequency += 1;
        } else {
            self.navigation_patterns.push(NavigationPattern {
                sequence: pattern_sequence,
                frequency: 1,
                average_duration: Duration::from_secs(0),
                success_rate: 1.0,
            });
        }
    }

    /// Record error occurrence
    pub fn record_error(&mut self, error_message: &str) {
        let error_key = error_message.chars().take(50).collect::<String>(); // Truncate for storage
        *self
            .session_stats
            .common_errors
            .entry(error_key)
            .or_insert(0) += 1;
    }

    /// Record feature usage
    pub fn record_feature_usage(&mut self, feature: &str) {
        *self
            .session_stats
            .feature_usage
            .entry(feature.to_string())
            .or_insert(0) += 1;
    }

    /// Get most frequently used service
    pub fn get_most_used_service(&self) -> Option<AwsService> {
        self.recent_services
            .iter()
            .max_by_key(|(_, usage)| usage.access_count)
            .map(|(service, _)| *service)
    }

    /// Get recently accessed services (sorted by recency and frequency)
    pub fn get_recent_services(&self) -> Vec<AwsService> {
        let mut services: Vec<_> = self.recent_services.iter().collect();

        // Sort by a combination of recency and frequency
        services.sort_by(|(_, a), (_, b)| {
            let a_score = a.access_count as f64
                + if let Some(last) = a.last_accessed {
                    // More recent = higher score
                    match last.elapsed() {
                        Ok(elapsed) => 1000.0 / (elapsed.as_secs() as f64 + 1.0),
                        Err(_) => 0.0,
                    }
                } else {
                    0.0
                };

            let b_score = b.access_count as f64
                + if let Some(last) = b.last_accessed {
                    match last.elapsed() {
                        Ok(elapsed) => 1000.0 / (elapsed.as_secs() as f64 + 1.0),
                        Err(_) => 0.0,
                    }
                } else {
                    0.0
                };

            b_score
                .partial_cmp(&a_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        services.into_iter().map(|(service, _)| *service).collect()
    }

    /// Get suggested instances for a service
    pub fn get_suggested_instances(&self, service: &AwsService) -> Vec<String> {
        if let Some(usage) = self.recent_services.get(service) {
            usage.preferred_instances.clone()
        } else {
            Vec::new()
        }
    }

    /// Get preferred time range for a service
    pub fn get_preferred_time_range(&self, service: &AwsService) -> Option<&String> {
        self.preferred_time_ranges.get(service)
    }

    /// Set preferred time range for a service
    pub fn set_preferred_time_range(&mut self, service: AwsService, time_range: String) {
        self.preferred_time_ranges.insert(service, time_range);
    }

    /// Get navigation suggestions based on current state
    pub fn get_navigation_suggestions(&self, current_state: &AppState) -> Vec<AppState> {
        let mut suggestions = Vec::new();

        // Find patterns that start with current state
        for pattern in &self.navigation_patterns {
            if pattern.sequence.len() >= 2 && pattern.sequence[0] == *current_state {
                suggestions.push(pattern.sequence[1]);
            }
        }

        // Sort by frequency
        suggestions.sort_by(|a, b| {
            let a_freq = self
                .navigation_patterns
                .iter()
                .find(|p| {
                    p.sequence.len() >= 2 && p.sequence[0] == *current_state && p.sequence[1] == *a
                })
                .map(|p| p.frequency)
                .unwrap_or(0);

            let b_freq = self
                .navigation_patterns
                .iter()
                .find(|p| {
                    p.sequence.len() >= 2 && p.sequence[0] == *current_state && p.sequence[1] == *b
                })
                .map(|p| p.frequency)
                .unwrap_or(0);

            b_freq.cmp(&a_freq)
        });

        // Remove duplicates and return top 3
        suggestions.dedup();
        suggestions.truncate(3);
        suggestions
    }

    /// Update session statistics
    pub fn update_session_stats(&mut self, session_duration: Duration) {
        self.session_stats.total_sessions += 1;

        // Update average session duration
        let total_duration = self.session_stats.average_session_duration.as_secs()
            * (self.session_stats.total_sessions - 1) as u64
            + session_duration.as_secs();
        self.session_stats.average_session_duration =
            Duration::from_secs(total_duration / self.session_stats.total_sessions as u64);

        // Update most used service
        self.session_stats.most_used_service = self.get_most_used_service();

        // Update most viewed instance
        self.session_stats.most_viewed_instance = self
            .recent_instances
            .iter()
            .max_by_key(|(_, usage)| usage.access_count)
            .map(|(instance, _)| instance.clone());
    }

    /// Clean up old entries to prevent memory bloat
    pub fn cleanup_old_entries(&mut self, max_age: Duration) {
        let cutoff_time = std::time::SystemTime::now() - max_age;

        // Clean up old service entries
        self.recent_services.retain(|_, usage| {
            if let Some(last_accessed) = usage.last_accessed {
                last_accessed > cutoff_time
            } else {
                false
            }
        });

        // Clean up old instance entries
        self.recent_instances.retain(|_, usage| {
            if let Some(last_accessed) = usage.last_accessed {
                last_accessed > cutoff_time
            } else {
                false
            }
        });

        // Clean up low-frequency navigation patterns
        self.navigation_patterns
            .retain(|pattern| pattern.frequency > 1);
    }

    /// Get workflow efficiency metrics
    pub fn get_efficiency_metrics(&self) -> WorkflowEfficiencyMetrics {
        let total_navigations: u32 = self.navigation_patterns.iter().map(|p| p.frequency).sum();

        let successful_navigations: f32 = self
            .navigation_patterns
            .iter()
            .map(|p| p.frequency as f32 * p.success_rate)
            .sum();

        let overall_success_rate = if total_navigations > 0 {
            successful_navigations / total_navigations as f32
        } else {
            0.0
        };

        WorkflowEfficiencyMetrics {
            overall_success_rate,
            most_common_pattern: self
                .navigation_patterns
                .iter()
                .max_by_key(|p| p.frequency)
                .cloned(),
            average_session_duration: self.session_stats.average_session_duration,
            total_sessions: self.session_stats.total_sessions,
        }
    }
}

/// Workflow efficiency metrics for analysis
#[derive(Debug, Clone)]
pub struct WorkflowEfficiencyMetrics {
    pub overall_success_rate: f32,
    pub most_common_pattern: Option<NavigationPattern>,
    pub average_session_duration: Duration,
    pub total_sessions: u32,
}
