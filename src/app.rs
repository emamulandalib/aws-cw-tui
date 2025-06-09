use std::time::{Duration, Instant};
use ratatui::widgets::ListState;
use anyhow::Result;
use crate::models::{App, AppState, MetricData};
use crate::aws::{load_rds_instances, load_metrics};
use crate::aws::cloudwatch_service::{TimeRange, TimeUnit};

impl App {
    pub fn update_time_range(&mut self, value: u32, unit: TimeUnit, period_days: u32) -> Result<()> {
        self.time_range = TimeRange::new(value, unit, period_days)?;
        Ok(())
    }

    pub fn new() -> App {
        let mut app = App {
            rds_instances: Vec::new(),
            list_state: ListState::default(),
            loading: true,
            state: AppState::RdsList,
            selected_instance: None,
            metrics: MetricData::default(),
            metrics_loading: false,
            last_refresh: None,
            auto_refresh_enabled: true,
            scroll_offset: 0,
            metrics_per_screen: 1, // Show 1 metric per screen for maximum chart size
            metrics_summary_scroll: 0, // Initialize metrics summary scroll position
            time_range_scroll: 2, // Initialize to "3 hours" (index 2 in the list)
            focused_panel: crate::models::FocusedPanel::TimeRanges, // Start with time ranges panel focused
            time_range: TimeRange::new(3, TimeUnit::Hours, 1).unwrap(), // Default: 3 hours with 1-day period
            
            // Initialize sparkline grid state
            selected_metric: None,
            sparkline_grid_scroll: 0,
            sparkline_grid_selected_index: 0,
        };
        app.list_state.select(Some(0));
        app
    }

    pub fn needs_refresh(&self) -> bool {
        if !self.auto_refresh_enabled {
            return false;
        }
        
        match self.last_refresh {
            None => true,
            Some(last) => last.elapsed() > Duration::from_secs(60), // Refresh every minute
        }
    }

    pub fn mark_refreshed(&mut self) {
        self.last_refresh = Some(Instant::now());
    }

    pub fn next(&mut self) {
        if self.rds_instances.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.rds_instances.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.rds_instances.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.rds_instances.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub async fn load_rds_instances(&mut self) -> Result<()> {
        self.rds_instances = load_rds_instances().await?;
        self.loading = false;
        self.mark_refreshed();
        
        // Ensure list state is properly set
        if !self.rds_instances.is_empty() {
            // If we had a selection before, try to maintain it
            let current_selection = self.list_state.selected().unwrap_or(0);
            let new_selection = if current_selection < self.rds_instances.len() {
                current_selection
            } else {
                0
            };
            self.list_state.select(Some(new_selection));
        } else {
            self.list_state.select(None);
        }
        Ok(())
    }

    pub async fn load_metrics(&mut self, instance_id: &str) -> Result<()> {
        self.metrics_loading = true;
        
        match load_metrics(instance_id, self.time_range).await {
            Ok(metrics) => {
                self.metrics = metrics;
                self.metrics_loading = false;
                // Initialize sparkline grid with new metrics data
                self.initialize_sparkline_grid();
                Ok(())
            }
            Err(e) => {
                self.metrics_loading = false;
                // Reset to default metrics on error
                self.metrics = MetricData::default();
                // Reset sparkline grid state
                self.selected_metric = None;
                self.sparkline_grid_selected_index = 0;
                Err(e)
            }
        }
    }

    pub fn enter_metrics_summary(&mut self) {
        if let Some(i) = self.list_state.selected() {
            self.selected_instance = Some(i);
            self.state = AppState::MetricsSummary;
            // Initialize scroll positions for metrics summary
            self.metrics_summary_scroll = 0;
            self.scroll_offset = 0;
            self.focused_panel = crate::models::FocusedPanel::TimeRanges; // Start with time ranges focused
            // Initialize sparkline grid state and synchronize with scroll offset
            self.sparkline_grid_selected_index = 0;
            self.initialize_sparkline_grid();
        }
    }

    pub fn back_to_metrics_summary(&mut self) {
        self.state = AppState::MetricsSummary;
        // Restore the metrics summary scroll position
        self.scroll_offset = self.metrics_summary_scroll;
        // Synchronize sparkline grid state with scroll position
        self.sparkline_grid_selected_index = self.scroll_offset;
        self.update_selected_metric();
    }

    pub fn enter_instance_details(&mut self) {
        if let Some(i) = self.list_state.selected() {
            self.selected_instance = Some(i);
            self.state = AppState::InstanceDetails;
            // Save current metrics summary scroll position before transitioning
            self.metrics_summary_scroll = self.scroll_offset;
            // Set scroll_offset to match the sparkline grid selected index for consistency
            self.scroll_offset = self.sparkline_grid_selected_index;
            // Update metrics summary scroll to match
            self.metrics_summary_scroll = self.scroll_offset;
        }
    }

    pub fn back_to_list(&mut self) {
        self.state = AppState::RdsList;
        self.selected_instance = None;
        // Reset all scroll positions when going back to the main list
        self.scroll_offset = 0;
        self.metrics_summary_scroll = 0;
    }

    pub fn scroll_up(&mut self) {
        match self.state {
            AppState::MetricsSummary => {
                match self.focused_panel {
                    crate::models::FocusedPanel::TimeRanges => {
                        self.time_range_scroll_up();
                    },
                    crate::models::FocusedPanel::SparklineGrid => {
                        self.sparkline_grid_scroll_up();
                    }
                }
            }
            _ => {
                if self.scroll_offset > 0 {
                    self.scroll_offset -= 1;
                }
            }
        }
    }

    pub fn scroll_down(&mut self) {
        match self.state {
            AppState::MetricsSummary => {
                match self.focused_panel {
                    crate::models::FocusedPanel::TimeRanges => {
                        self.time_range_scroll_down();
                    },
                    crate::models::FocusedPanel::SparklineGrid => {
                        self.sparkline_grid_scroll_down();
                    }
                }
            }
            AppState::InstanceDetails => {
                // For instance details, use the original logic
                let total_individual_metrics = self.metrics.count_available_metrics();
                let max_offset = total_individual_metrics.saturating_sub(1);
                if self.scroll_offset < max_offset {
                    self.scroll_offset += 1;
                }
            }
            _ => {}
        }
    }

    fn get_available_metrics_count(&self) -> usize {
        let mut count = 0;
        
        // Core metrics
        if !self.metrics.cpu_history.is_empty() { count += 1; }
        if !self.metrics.connections_history.is_empty() { count += 1; }
        if !self.metrics.read_iops_history.is_empty() { count += 1; }
        if !self.metrics.write_iops_history.is_empty() { count += 1; }
        if !self.metrics.read_latency_history.is_empty() { count += 1; }
        if !self.metrics.write_latency_history.is_empty() { count += 1; }
        if !self.metrics.free_storage_space_history.is_empty() { count += 1; }
        if !self.metrics.read_throughput_history.is_empty() { count += 1; }
        if !self.metrics.write_throughput_history.is_empty() { count += 1; }
        if !self.metrics.network_receive_history.is_empty() { count += 1; }
        if !self.metrics.network_transmit_history.is_empty() { count += 1; }
        if !self.metrics.freeable_memory_history.is_empty() { count += 1; }
        if !self.metrics.swap_usage_history.is_empty() { count += 1; }
        if !self.metrics.queue_depth_history.is_empty() { count += 1; }

        // Advanced metrics
        if !self.metrics.burst_balance_history.is_empty() { count += 1; }
        if !self.metrics.cpu_credit_usage_history.is_empty() { count += 1; }
        if !self.metrics.cpu_credit_balance_history.is_empty() { count += 1; }
        if !self.metrics.replica_lag_history.is_empty() { count += 1; }
        
        count
    }

    pub fn get_current_scroll_position(&self) -> usize {
        match self.state {
            AppState::MetricsSummary => self.metrics_summary_scroll,
            _ => self.scroll_offset,
        }
    }

    pub fn reset_scroll(&mut self) {
        match self.state {
            AppState::MetricsSummary => {
                self.metrics_summary_scroll = 0;
                self.scroll_offset = 0;
                self.focused_panel = crate::models::FocusedPanel::TimeRanges; // Reset to time ranges panel
                // Also reset sparkline grid state and synchronize
                self.sparkline_grid_scroll = 0;
                self.sparkline_grid_selected_index = 0;
                self.initialize_sparkline_grid();
            }
            _ => {
                self.scroll_offset = 0;
                // Also synchronize sparkline grid state for consistency
                self.sparkline_grid_selected_index = 0;
                self.metrics_summary_scroll = 0;
            }
        }
    }

    // Time range options similar to AWS CloudWatch web interface
    pub fn get_time_range_options() -> Vec<(&'static str, u32, TimeUnit, u32)> {
        vec![
            ("5 minutes", 5, TimeUnit::Minutes, 1),
            ("1 hour", 1, TimeUnit::Hours, 1),
            ("3 hours", 3, TimeUnit::Hours, 1),
            ("6 hours", 6, TimeUnit::Hours, 1),
            ("12 hours", 12, TimeUnit::Hours, 1),
            ("1 day", 1, TimeUnit::Days, 1),
            ("3 days", 3, TimeUnit::Days, 1),
            ("1 week", 1, TimeUnit::Weeks, 7),
            ("2 weeks", 2, TimeUnit::Weeks, 14),
            ("1 month", 1, TimeUnit::Months, 30),
        ]
    }

    pub fn get_current_time_range_index(&self) -> usize {
        self.time_range_scroll
    }

    pub fn select_time_range(&mut self, index: usize) -> Result<()> {
        let options = Self::get_time_range_options();
        if let Some(&(_, value, unit, period_days)) = options.get(index) {
            self.time_range_scroll = index;
            self.time_range = TimeRange::new(value, unit, period_days)?;
            Ok(())
        } else {
            Ok(())
        }
    }

    pub fn time_range_scroll_up(&mut self) {
        if self.time_range_scroll > 0 {
            self.time_range_scroll -= 1;
        }
    }

    pub fn time_range_scroll_down(&mut self) {
        let options = Self::get_time_range_options();
        if self.time_range_scroll < options.len() - 1 {
            self.time_range_scroll += 1;
        }
    }

    pub fn switch_panel(&mut self) {
        self.focused_panel = match self.focused_panel {
            crate::models::FocusedPanel::TimeRanges => crate::models::FocusedPanel::SparklineGrid,
            crate::models::FocusedPanel::SparklineGrid => crate::models::FocusedPanel::TimeRanges,
        };
    }

    pub fn get_focused_panel(&self) -> &crate::models::FocusedPanel {
        &self.focused_panel
    }

    // Sparkline grid navigation methods
    pub fn sparkline_grid_scroll_up(&mut self) {
        if self.sparkline_grid_selected_index > 0 {
            self.sparkline_grid_selected_index -= 1;
            self.update_selected_metric();
            // Synchronize with main scroll offset
            self.scroll_offset = self.sparkline_grid_selected_index;
            self.metrics_summary_scroll = self.scroll_offset;
        }
    }

    pub fn sparkline_grid_scroll_down(&mut self) {
        let available_metrics = self.metrics.get_available_metrics();
        if self.sparkline_grid_selected_index < available_metrics.len().saturating_sub(1) {
            self.sparkline_grid_selected_index += 1;
            self.update_selected_metric();
            // Synchronize with main scroll offset
            self.scroll_offset = self.sparkline_grid_selected_index;
            self.metrics_summary_scroll = self.scroll_offset;
        }
    }

    pub fn get_available_metrics(&self) -> Vec<crate::models::MetricType> {
        self.metrics.get_available_metrics()
    }

    pub fn get_selected_metric(&self) -> Option<&crate::models::MetricType> {
        self.selected_metric.as_ref()
    }

    pub fn set_selected_metric(&mut self, metric: crate::models::MetricType) {
        // Update the selected index to match the new metric
        let available_metrics = self.metrics.get_available_metrics();
        if let Some(index) = available_metrics.iter().position(|m| *m == metric) {
            self.sparkline_grid_selected_index = index;
        }
        self.selected_metric = Some(metric);
    }

    pub fn get_sparkline_grid_selected_index(&self) -> usize {
        self.sparkline_grid_selected_index
    }

    fn update_selected_metric(&mut self) {
        let available_metrics = self.metrics.get_available_metrics();
        if let Some(metric) = available_metrics.get(self.sparkline_grid_selected_index) {
            self.selected_metric = Some(metric.clone());
        }
    }

    pub fn initialize_sparkline_grid(&mut self) {
        let available_metrics = self.metrics.get_available_metrics();
        if !available_metrics.is_empty() {
            // Initialize with the first available metric if none selected
            if self.selected_metric.is_none() {
                self.selected_metric = Some(available_metrics[0].clone());
                self.sparkline_grid_selected_index = 0;
            } else {
                // Ensure the selected metric is still available and update index
                if let Some(ref current_metric) = self.selected_metric {
                    if let Some(index) = available_metrics.iter().position(|m| m == current_metric) {
                        self.sparkline_grid_selected_index = index;
                    } else {
                        // Selected metric is no longer available, select the first one
                        self.selected_metric = Some(available_metrics[0].clone());
                        self.sparkline_grid_selected_index = 0;
                    }
                }
            }
        } else {
            // No metrics available
            self.selected_metric = None;
            self.sparkline_grid_selected_index = 0;
        }
    }

    pub fn get_metric_history(&self, metric_type: &crate::models::MetricType) -> &Vec<f64> {
        self.metrics.get_metric_history(metric_type)
    }

    pub fn has_sufficient_data_for_sparklines(&self) -> bool {
        let available_metrics = self.metrics.get_available_metrics();
        available_metrics.len() >= 4  // Require at least 4 metrics for meaningful sparkline grid
    }
}
