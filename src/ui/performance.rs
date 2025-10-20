use crate::models::App;
use ratatui::{layout::Rect, Frame};
use std::time::{Duration, Instant};

/// Performance optimization system for UI responsiveness
#[derive(Debug, Clone)]
pub struct PerformanceOptimizer {
    /// Frame rate tracking
    pub frame_times: Vec<Duration>,
    pub target_fps: u32,
    pub max_frame_time: Duration,

    /// Rendering optimizations
    pub enable_lazy_rendering: bool,
    pub enable_viewport_culling: bool,
    pub enable_data_throttling: bool,

    /// Memory management
    pub max_history_points: usize,
    pub cleanup_interval: Duration,
    pub last_cleanup: Option<Instant>,

    /// Adaptive rendering
    pub adaptive_quality: bool,
    pub current_quality_level: QualityLevel,
    pub performance_threshold: Duration,
}

/// Quality levels for adaptive rendering
#[derive(Debug, Clone, PartialEq)]
pub enum QualityLevel {
    High,    // Full detail, all animations
    Medium,  // Reduced animations, simplified charts
    Low,     // Minimal animations, basic charts
    Minimal, // Text-only, no graphics
}

impl Default for PerformanceOptimizer {
    fn default() -> Self {
        Self {
            frame_times: Vec::with_capacity(60), // Track last 60 frames
            target_fps: 30,
            max_frame_time: Duration::from_millis(33), // ~30 FPS

            enable_lazy_rendering: true,
            enable_viewport_culling: true,
            enable_data_throttling: true,

            max_history_points: 100,
            cleanup_interval: Duration::from_secs(30),
            last_cleanup: None,

            adaptive_quality: true,
            current_quality_level: QualityLevel::High,
            performance_threshold: Duration::from_millis(50),
        }
    }
}

impl PerformanceOptimizer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record frame rendering time
    pub fn record_frame_time(&mut self, frame_time: Duration) {
        self.frame_times.push(frame_time);

        // Keep only recent frame times
        if self.frame_times.len() > 60 {
            self.frame_times.remove(0);
        }

        // Adjust quality based on performance
        if self.adaptive_quality {
            self.adjust_quality_level();
        }
    }

    /// Get average frame time
    pub fn get_average_frame_time(&self) -> Duration {
        if self.frame_times.is_empty() {
            return Duration::from_millis(16); // Default to 60 FPS equivalent
        }

        let total: Duration = self.frame_times.iter().sum();
        total / self.frame_times.len() as u32
    }

    /// Get current FPS
    pub fn get_current_fps(&self) -> f32 {
        let avg_frame_time = self.get_average_frame_time();
        if avg_frame_time.as_millis() > 0 {
            1000.0 / avg_frame_time.as_millis() as f32
        } else {
            60.0
        }
    }

    /// Check if performance is acceptable
    pub fn is_performance_acceptable(&self) -> bool {
        self.get_average_frame_time() <= self.max_frame_time
    }

    /// Adjust quality level based on performance
    fn adjust_quality_level(&mut self) {
        let avg_frame_time = self.get_average_frame_time();

        match self.current_quality_level {
            QualityLevel::High => {
                if avg_frame_time > self.performance_threshold {
                    self.current_quality_level = QualityLevel::Medium;
                }
            }
            QualityLevel::Medium => {
                if avg_frame_time > self.performance_threshold * 2 {
                    self.current_quality_level = QualityLevel::Low;
                } else if avg_frame_time < self.performance_threshold / 2 {
                    self.current_quality_level = QualityLevel::High;
                }
            }
            QualityLevel::Low => {
                if avg_frame_time > self.performance_threshold * 3 {
                    self.current_quality_level = QualityLevel::Minimal;
                } else if avg_frame_time < self.performance_threshold {
                    self.current_quality_level = QualityLevel::Medium;
                }
            }
            QualityLevel::Minimal => {
                if avg_frame_time < self.performance_threshold {
                    self.current_quality_level = QualityLevel::Low;
                }
            }
        }
    }

    /// Check if area is visible in viewport (for culling)
    pub fn is_area_visible(&self, area: Rect, viewport: Rect) -> bool {
        if !self.enable_viewport_culling {
            return true;
        }

        // Simple rectangle intersection test
        area.x < viewport.x + viewport.width
            && area.x + area.width > viewport.x
            && area.y < viewport.y + viewport.height
            && area.y + area.height > viewport.y
    }

    /// Should render with reduced quality
    pub fn should_use_reduced_quality(&self) -> bool {
        matches!(
            self.current_quality_level,
            QualityLevel::Low | QualityLevel::Minimal
        )
    }

    /// Should skip animations
    pub fn should_skip_animations(&self) -> bool {
        matches!(self.current_quality_level, QualityLevel::Minimal)
    }

    /// Should throttle data updates
    pub fn should_throttle_data(&self) -> bool {
        self.enable_data_throttling && !self.is_performance_acceptable()
    }

    /// Get recommended update interval based on performance
    pub fn get_recommended_update_interval(&self) -> Duration {
        match self.current_quality_level {
            QualityLevel::High => Duration::from_millis(100),
            QualityLevel::Medium => Duration::from_millis(200),
            QualityLevel::Low => Duration::from_millis(500),
            QualityLevel::Minimal => Duration::from_secs(1),
        }
    }

    /// Cleanup old data to prevent memory bloat
    pub fn cleanup_if_needed(&mut self, app: &mut App) {
        let now = Instant::now();

        if let Some(last_cleanup) = self.last_cleanup {
            if now.duration_since(last_cleanup) < self.cleanup_interval {
                return;
            }
        }

        // Cleanup workflow memory
        app.workflow_memory
            .cleanup_old_entries(Duration::from_secs(3600)); // 1 hour

        // Cleanup metrics history if too large
        if let Some(ref mut metrics) = app.dynamic_metrics {
            for metric in &mut metrics.metrics {
                if metric.history.len() > self.max_history_points {
                    let excess = metric.history.len() - self.max_history_points;
                    metric.history.drain(0..excess);
                }
            }
        }

        self.last_cleanup = Some(now);
    }

    /// Get performance statistics
    pub fn get_performance_stats(&self) -> PerformanceStats {
        PerformanceStats {
            current_fps: self.get_current_fps(),
            average_frame_time: self.get_average_frame_time(),
            quality_level: self.current_quality_level.clone(),
            is_performance_acceptable: self.is_performance_acceptable(),
            frame_count: self.frame_times.len(),
        }
    }
}

/// Performance statistics for monitoring
#[derive(Debug, Clone)]
pub struct PerformanceStats {
    pub current_fps: f32,
    pub average_frame_time: Duration,
    pub quality_level: QualityLevel,
    pub is_performance_acceptable: bool,
    pub frame_count: usize,
}

/// Responsive rendering utilities
pub struct ResponsiveRenderer;

impl ResponsiveRenderer {
    /// Render with performance optimizations
    pub fn render_optimized<F>(
        f: &mut Frame,
        area: Rect,
        optimizer: &PerformanceOptimizer,
        render_fn: F,
    ) where
        F: FnOnce(&mut Frame, Rect, &QualityLevel),
    {
        let start_time = Instant::now();

        // Only render if area is visible
        if optimizer.is_area_visible(area, f.area()) {
            render_fn(f, area, &optimizer.current_quality_level);
        }

        // Could record timing here if needed
    }

    /// Get responsive chart configuration
    pub fn get_chart_config(quality_level: &QualityLevel) -> ChartConfig {
        match quality_level {
            QualityLevel::High => ChartConfig {
                show_grid: true,
                show_labels: true,
                show_legend: true,
                animation_enabled: true,
                data_points: 100,
                line_style: LineStyle::Smooth,
            },
            QualityLevel::Medium => ChartConfig {
                show_grid: true,
                show_labels: true,
                show_legend: false,
                animation_enabled: false,
                data_points: 50,
                line_style: LineStyle::Linear,
            },
            QualityLevel::Low => ChartConfig {
                show_grid: false,
                show_labels: true,
                show_legend: false,
                animation_enabled: false,
                data_points: 25,
                line_style: LineStyle::Linear,
            },
            QualityLevel::Minimal => ChartConfig {
                show_grid: false,
                show_labels: false,
                show_legend: false,
                animation_enabled: false,
                data_points: 10,
                line_style: LineStyle::Basic,
            },
        }
    }

    /// Get responsive text configuration
    pub fn get_text_config(quality_level: &QualityLevel) -> TextConfig {
        match quality_level {
            QualityLevel::High => TextConfig {
                show_icons: true,
                show_colors: true,
                show_formatting: true,
                truncate_long_text: false,
            },
            QualityLevel::Medium => TextConfig {
                show_icons: true,
                show_colors: true,
                show_formatting: true,
                truncate_long_text: true,
            },
            QualityLevel::Low => TextConfig {
                show_icons: false,
                show_colors: true,
                show_formatting: false,
                truncate_long_text: true,
            },
            QualityLevel::Minimal => TextConfig {
                show_icons: false,
                show_colors: false,
                show_formatting: false,
                truncate_long_text: true,
            },
        }
    }
}

/// Chart configuration for responsive rendering
#[derive(Debug, Clone)]
pub struct ChartConfig {
    pub show_grid: bool,
    pub show_labels: bool,
    pub show_legend: bool,
    pub animation_enabled: bool,
    pub data_points: usize,
    pub line_style: LineStyle,
}

/// Line style options
#[derive(Debug, Clone)]
pub enum LineStyle {
    Smooth,
    Linear,
    Basic,
}

/// Text configuration for responsive rendering
#[derive(Debug, Clone)]
pub struct TextConfig {
    pub show_icons: bool,
    pub show_colors: bool,
    pub show_formatting: bool,
    pub truncate_long_text: bool,
}

/// Extension trait for App to add performance optimization
pub trait PerformanceExt {
    fn get_performance_optimizer(&self) -> &PerformanceOptimizer;
    fn get_performance_optimizer_mut(&mut self) -> &mut PerformanceOptimizer;
    fn record_frame_time(&mut self, frame_time: Duration);
    fn cleanup_performance_data(&mut self);
    fn get_performance_stats(&self) -> PerformanceStats;
}

impl PerformanceExt for App {
    fn get_performance_optimizer(&self) -> &PerformanceOptimizer {
        &self.performance_optimizer
    }

    fn get_performance_optimizer_mut(&mut self) -> &mut PerformanceOptimizer {
        &mut self.performance_optimizer
    }

    fn record_frame_time(&mut self, frame_time: Duration) {
        self.performance_optimizer.record_frame_time(frame_time);
    }

    fn cleanup_performance_data(&mut self) {
        // Split the borrow to avoid borrowing self twice
        let max_history_points = self.performance_optimizer.max_history_points;
        let cleanup_interval = self.performance_optimizer.cleanup_interval;
        let last_cleanup = self.performance_optimizer.last_cleanup;

        let now = std::time::Instant::now();

        if let Some(last_cleanup) = last_cleanup {
            if now.duration_since(last_cleanup) < cleanup_interval {
                return;
            }
        }

        // Cleanup workflow memory
        self.workflow_memory
            .cleanup_old_entries(Duration::from_secs(3600)); // 1 hour

        // Cleanup metrics history if too large
        if let Some(ref mut metrics) = self.dynamic_metrics {
            for metric in &mut metrics.metrics {
                if metric.history.len() > max_history_points {
                    let excess = metric.history.len() - max_history_points;
                    metric.history.drain(0..excess);
                }
            }
        }

        self.performance_optimizer.last_cleanup = Some(now);
    }

    fn get_performance_stats(&self) -> PerformanceStats {
        self.performance_optimizer.get_performance_stats()
    }
}
