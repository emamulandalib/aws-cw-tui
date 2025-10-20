use crate::aws::dynamic_metric_discovery::DynamicMetricData;
use crate::models::App;
use crate::models::MetricType;
use crate::ui::charts::chart_data::MetricChartData;
use crate::ui::charts::grid_layout::calculate_scrollable_grid_layout;
use crate::ui::charts::rendering::dynamic_charts::render_dynamic_metric_chart;
use crate::ui::charts::rendering::metric_charts::render_metric_chart;
use crate::ui::components::metric::display_format::DisplayFormat;
use crate::ui::components::metric::{MetricDefinition, MetricRegistry};
use crate::ui::components::universal_box::UniversalBox;
use crate::ui::themes::UnifiedTheme;
use log::{debug, info, warn};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

// Include debug logging macros
use crate::log_metric_operation;

/// Unified data structure for both legacy and dynamic metrics
#[derive(Clone)]
pub enum UnifiedMetricData {
    Legacy(MetricChartData),
    Dynamic(DynamicMetricData),
}

impl UnifiedMetricData {
    /// Get metric definition for this metric
    pub fn get_definition(&self) -> MetricDefinition {
        match self {
            UnifiedMetricData::Legacy(data) => MetricRegistry::get_definition(&data.metric_type),
            UnifiedMetricData::Dynamic(data) => {
                // Create a dynamic metric definition based on the metric name
                Self::create_dynamic_definition(data)
            }
        }
    }

    /// Get current value for this metric
    pub fn get_current_value(&self) -> f64 {
        match self {
            UnifiedMetricData::Legacy(data) => data.current_value,
            UnifiedMetricData::Dynamic(data) => data.current_value,
        }
    }

    /// Get display name for this metric
    pub fn get_display_name(&self) -> String {
        match self {
            UnifiedMetricData::Legacy(data) => MetricRegistry::get_definition(&data.metric_type)
                .name
                .to_string(),
            UnifiedMetricData::Dynamic(data) => data.display_name.clone(),
        }
    }

    /// Create metric definition for dynamic metrics
    fn create_dynamic_definition(data: &DynamicMetricData) -> MetricDefinition {
        // Map common metric names to proper definitions
        let metric_type = Self::map_dynamic_to_metric_type(&data.metric_name);
        match metric_type {
            Some(mt) => MetricRegistry::get_definition(&mt),
            None => {
                // Default definition for unknown metrics
                MetricDefinition::new(
                    // Convert to static str using Box::leak for simplicity
                    Box::leak(data.display_name.clone().into_boxed_str()),
                    DisplayFormat::Decimal(2),
                    None,
                    Color::Gray,
                    "Dynamic metric from CloudWatch",
                )
            }
        }
    }

    /// Map dynamic metric names to MetricType
    fn map_dynamic_to_metric_type(metric_name: &str) -> Option<MetricType> {
        match metric_name {
            "CPUUtilization" => Some(MetricType::CpuUtilization),
            "DatabaseConnections" => Some(MetricType::DatabaseConnections),
            "FreeStorageSpace" => Some(MetricType::FreeStorageSpace),
            "ReadLatency" => Some(MetricType::ReadLatency),
            "WriteLatency" => Some(MetricType::WriteLatency),
            "ReadIOPS" => Some(MetricType::ReadIops),
            "WriteIOPS" => Some(MetricType::WriteIops),
            "ApproximateNumberOfMessagesVisible" => {
                Some(MetricType::ApproximateNumberOfMessagesVisible)
            }
            "ApproximateNumberOfMessagesNotVisible" => {
                Some(MetricType::ApproximateNumberOfMessagesNotVisible)
            }
            "ApproximateAgeOfOldestMessage" => Some(MetricType::ApproximateAgeOfOldestMessage),
            _ => None,
        }
    }
}

/// AWS Console-style metrics grid renderer with unified architecture
pub struct AwsMetricsGrid;

impl AwsMetricsGrid {
    /// Render metrics in AWS console-style grid layout with unified approach
    pub fn render(f: &mut Frame, app: &App, area: Rect, theme: &UnifiedTheme) {
        debug!("METRICS_GRID: Starting unified render for area: {:?}", area);

        let service = match app.selected_service.as_ref() {
            Some(service) => {
                debug!("METRICS_GRID: Rendering for service: {:?}", service);
                service
            }
            None => {
                warn!("METRICS_GRID: No service selected, cannot render metrics");
                return;
            }
        };

        let instance_id = app
            .get_selected_instance_id()
            .unwrap_or_else(|| "unknown".to_string());
        debug!("METRICS_GRID: Selected instance ID: {}", instance_id);

        // Collect all available metrics using unified approach
        let unified_metrics = Self::collect_unified_metrics(app);

        if unified_metrics.is_empty() {
            info!("METRICS_GRID: No metrics available, showing no-metrics message");
            Self::render_no_metrics(f, area, service, theme);
            return;
        }

        info!(
            "METRICS_GRID: Found {} unified metrics",
            unified_metrics.len()
        );

        // Force 2x2 grid layout (4 metrics per screen)
        let metrics_per_row = 2;
        let metrics_per_screen = 4;

        let total_metrics = unified_metrics.len();
        let selected_index = app.sparkline_grid_list_state.selected().unwrap_or(0);
        info!(
            "METRICS_GRID: Grid state - Total: {}, Selected Index: {}, Metrics per screen: {}",
            total_metrics, selected_index, metrics_per_screen
        );

        // CRITICAL: Bounds check to prevent crashes
        let safe_selected_index = selected_index.min(total_metrics.saturating_sub(1));
        if safe_selected_index != selected_index {
            warn!(
                "METRICS_GRID: Clamped selected index from {} to {} to prevent bounds error",
                selected_index, safe_selected_index
            );
        }

        // Calculate which metrics to show in the current 2x2 grid
        let current_page = safe_selected_index / metrics_per_screen;
        let start_index = current_page * metrics_per_screen;
        let end_index = (start_index + metrics_per_screen).min(total_metrics);

        info!(
            "METRICS_GRID: Page calculation - Current page: {}, Start: {}, End: {}",
            current_page, start_index, end_index
        );

        // Get the metrics to display in current 2x2 grid
        let visible_metrics: Vec<UnifiedMetricData> =
            unified_metrics[start_index..end_index].to_vec();

        // Log which specific metrics are being displayed
        let visible_metric_names: Vec<String> = visible_metrics
            .iter()
            .map(|m| m.get_display_name())
            .collect();
        info!(
            "METRICS_GRID: Displaying {} metrics on current page: {:?}",
            visible_metrics.len(),
            visible_metric_names
        );

        // Identify the currently selected metric
        let relative_selected_index = safe_selected_index - start_index;
        if let Some(selected_metric) = visible_metrics.get(relative_selected_index) {
            log_metric_operation!(
                "Display unified metric in grid",
                &selected_metric.get_display_name(),
                format!(
                    "Grid position: {}/{}, Page: {}",
                    relative_selected_index,
                    visible_metrics.len(),
                    current_page
                )
            );
            info!(
                "METRICS_GRID: Currently selected metric: '{}' at relative position: {}",
                selected_metric.get_display_name(),
                relative_selected_index
            );
        }

        // Calculate grid layout
        let grid_layout = calculate_scrollable_grid_layout(visible_metrics.len(), metrics_per_row);
        debug!(
            "METRICS_GRID: Calculated grid layout for {} metrics",
            visible_metrics.len()
        );

        // Render the unified grid
        Self::render_unified_grid(
            f,
            area,
            &visible_metrics,
            grid_layout,
            relative_selected_index,
            theme,
        );

        info!("METRICS_GRID: Completed rendering unified metrics grid");
    }

    /// Collect all available metrics using unified approach
    fn collect_unified_metrics(app: &App) -> Vec<UnifiedMetricData> {
        let mut unified_metrics = Vec::new();

        // Check if we have dynamic metrics available (preferred)
        if let Some(ref dynamic_metrics) = app.dynamic_metrics {
            let available_metrics = dynamic_metrics.get_available_metric_names();

            if !available_metrics.is_empty() {
                debug!("METRICS_GRID: Using dynamic metrics system");
                // Sort dynamic metrics by display name alphabetically
                let mut sorted_dynamic_metrics = dynamic_metrics.metrics.clone();
                sorted_dynamic_metrics.sort_by(|a, b| a.display_name.cmp(&b.display_name));

                for metric in sorted_dynamic_metrics {
                    unified_metrics.push(UnifiedMetricData::Dynamic(metric));
                }
                return unified_metrics;
            }
        }

        // Fallback to legacy system
        if let Some(service) = app.selected_service.as_ref() {
            match service {
                crate::models::AwsService::Rds => {
                    debug!("METRICS_GRID: Using legacy RDS metrics system");
                    let available_metrics = app.metrics.get_available_metrics_with_data();

                    // Create metric chart data for all available metrics
                    let mut chart_data_list: Vec<MetricChartData> = available_metrics
                        .into_iter()
                        .filter_map(|metric_type| MetricChartData::from_app(app, metric_type))
                        .collect();

                    // Sort metrics alphabetically by metric name
                    chart_data_list.sort_by(|a, b| {
                        let a_name = MetricRegistry::get_definition(&a.metric_type).name;
                        let b_name = MetricRegistry::get_definition(&b.metric_type).name;
                        a_name.cmp(b_name)
                    });

                    for chart_data in chart_data_list {
                        unified_metrics.push(UnifiedMetricData::Legacy(chart_data));
                    }
                }
                crate::models::AwsService::Sqs => {
                    debug!("METRICS_GRID: SQS service with no dynamic metrics - showing error");
                    // For SQS, we expect dynamic metrics to be available
                    // Return empty vec to show error message
                    if let Some(ref dynamic_metrics) = app.dynamic_metrics {
                        if dynamic_metrics.is_empty() {
                            debug!("METRICS_GRID: SQS dynamic metrics container is empty");
                        }
                    } else {
                        debug!("METRICS_GRID: No SQS dynamic metrics container");
                    }
                }
            }
        }

        unified_metrics
    }

    /// Render unified grid using the new approach
    fn render_unified_grid(
        f: &mut Frame,
        area: Rect,
        metrics: &[UnifiedMetricData],
        grid_layout: crate::ui::charts::grid_layout::GridLayout,
        selected_metric_index: usize,
        theme: &UnifiedTheme,
    ) {
        if metrics.is_empty() {
            return;
        }

        // Create row constraints
        let row_constraints: Vec<Constraint> = (0..grid_layout.rows)
            .map(|_| Constraint::Percentage(100 / grid_layout.rows as u16))
            .collect();

        let row_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(row_constraints)
            .split(area);

        // Render each row
        for (row_idx, &row_area) in row_chunks.iter().enumerate() {
            // Create column constraints for this row
            let col_constraints: Vec<Constraint> = (0..grid_layout.cols)
                .map(|_| Constraint::Percentage(100 / grid_layout.cols as u16))
                .collect();

            let col_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(col_constraints)
                .split(row_area);

            // Render each column in this row
            for (col_idx, &col_area) in col_chunks.iter().enumerate() {
                let metric_index = row_idx * grid_layout.cols + col_idx;

                if metric_index < metrics.len() {
                    let is_focused = metric_index == selected_metric_index;
                    Self::render_unified_metric(
                        f,
                        col_area,
                        &metrics[metric_index],
                        is_focused,
                        theme,
                    );
                }
            }
        }
    }

    /// Render individual metric using unified approach
    fn render_unified_metric(
        f: &mut Frame,
        area: Rect,
        metric: &UnifiedMetricData,
        is_focused: bool,
        theme: &UnifiedTheme,
    ) {
        // Use appropriate rendering based on metric type
        match metric {
            UnifiedMetricData::Legacy(chart_data) => {
                render_metric_chart(f, area, chart_data, is_focused, theme);
            }
            UnifiedMetricData::Dynamic(dynamic_data) => {
                render_dynamic_metric_chart(f, area, dynamic_data, is_focused, theme);
            }
        }
    }

    /// Render no metrics available state
    fn render_no_metrics(
        f: &mut Frame,
        area: Rect,
        service: &crate::models::AwsService,
        theme: &UnifiedTheme,
    ) {
        let service_name = service.short_name();
        let (message, color) = if service_name == "RDS" {
            ("No RDS instances found.\nEnsure you have RDS instances running in your AWS account.", theme.info)
        } else {
            ("No metrics available for this service.", theme.warning)
        };

        UniversalBox::info(format!("{} Metrics", service_name), message, theme.clone())
            .render(f, area);
    }
}
