use crate::aws::cloudwatch_service::{TimeRange, TimeUnit};
use crate::aws::{load_metrics, load_rds_instances};
use crate::models::{App, AppState, AwsService, MetricData};
use anyhow::Result;
use ratatui::widgets::ListState;
use std::time::{Duration, Instant};

impl App {
    pub fn update_time_range(
        &mut self,
        value: u32,
        unit: TimeUnit,
        period_days: u32,
    ) -> Result<()> {
        self.time_range = TimeRange::new(value, unit, period_days)?;
        Ok(())
    }

    pub fn new() -> App {
            let mut app = App {
                // Service selection initialization (NEW)
                available_services: vec![AwsService::Rds, AwsService::Sqs],
                service_list_state: ListState::default(),
                selected_service: None,
    
                // Instance list initialization
                rds_instances: Vec::new(),
                list_state: ListState::default(),
                loading: true,
                state: AppState::ServiceList, // Start with service selection
                selected_instance: None,
                metrics: MetricData::default(),
                metrics_loading: false,
                last_refresh: None,
                auto_refresh_enabled: true,
                scroll_offset: 0,
                metrics_per_screen: 1, // Show 1 metric per screen for maximum chart size
                metrics_summary_scroll: 0, // Initialize metrics summary scroll position
                time_range_scroll: 2,  // Initialize to "3 hours" (index 2 in the list)
                focused_panel: crate::models::FocusedPanel::TimeRanges, // Start with time ranges panel focused
                saved_focused_panel: crate::models::FocusedPanel::TimeRanges, // Initialize saved focused panel
                time_range: TimeRange::new(3, TimeUnit::Hours, 1).unwrap(), // Default: 3 hours with 1-day period
    
                // Initialize sparkline grid state
                selected_metric: None,
                sparkline_grid_scroll: 0,
                sparkline_grid_selected_index: 0,
                saved_sparkline_grid_selected_index: 0,
    
                // Initialize error handling
                error_message: None,
            };
            app.service_list_state.select(Some(0)); // Select first service by default
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

    // Service navigation methods
    pub fn service_next(&mut self) {
        let i = match self.service_list_state.selected() {
            Some(i) => {
                if i >= self.available_services.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.service_list_state.select(Some(i));
    }

    pub fn service_previous(&mut self) {
        let i = match self.service_list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.available_services.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.service_list_state.select(Some(i));
    }

    pub fn select_service(&mut self) -> Option<&AwsService> {
        if let Some(index) = self.service_list_state.selected() {
            if let Some(service) = self.available_services.get(index) {
                self.selected_service = Some(service.clone());
                self.state = AppState::InstanceList;
                self.list_state.select(Some(0)); // Reset instance selection
                return Some(service);
            }
        }
        None
    }

    pub fn back_to_service_list(&mut self) {
        self.state = AppState::ServiceList;
        self.selected_service = None;
        self.rds_instances.clear(); // Clear instances when going back
        self.loading = true;
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
            match load_rds_instances().await {
                Ok(instances) => {
                    self.rds_instances = instances;
                    self.clear_error(); // Clear any previous errors
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
                },
                Err(e) => {
                    // Store user-friendly error message instead of panicking
                    self.error_message = Some(format!("AWS Error: {}", e));
                    self.loading = false;
                    self.rds_instances = Vec::new(); // Clear any partial data
                    self.list_state.select(None);
                    
                    // Don't propagate the error - let the UI show the error message
                    Ok(())
                }
            }
        }


    pub async fn load_metrics(&mut self, instance_id: &str) -> Result<()> {
            self.metrics_loading = true;
    
            match load_metrics(instance_id, self.time_range).await {
                Ok(metrics) => {
                    self.metrics = metrics;
                    self.metrics_loading = false;
                    self.clear_error(); // Clear any previous errors
                    // Initialize sparkline grid with new metrics data
                    self.initialize_sparkline_grid();
                    Ok(())
                }
                Err(e) => {
                    self.metrics_loading = false;
                    // Store user-friendly error message instead of panicking
                    self.error_message = Some(format!("CloudWatch Error: {}", e));
                    // Reset to default metrics on error
                    self.metrics = MetricData::default();
                    // Reset sparkline grid state
                    self.selected_metric = None;
                    self.sparkline_grid_selected_index = 0;
                    // Don't propagate the error - let the UI show the error message
                    Ok(())
                }
            }
        }
    
        // Clear error message
        pub fn clear_error(&mut self) {
            self.error_message = None;
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
        // Restore the focused panel state
        self.focused_panel = self.saved_focused_panel.clone();
        // Restore the sparkline grid selected index
        self.sparkline_grid_selected_index = self.saved_sparkline_grid_selected_index;
        self.update_selected_metric();
    }

    pub fn enter_instance_details(&mut self) {
        if let Some(i) = self.list_state.selected() {
            self.selected_instance = Some(i);
            self.state = AppState::InstanceDetails;
            // Save current metrics summary scroll position before transitioning
            self.metrics_summary_scroll = self.scroll_offset;
            // Save current focused panel state before transitioning
            self.saved_focused_panel = self.focused_panel.clone();
            // Save current sparkline grid selected index
            self.saved_sparkline_grid_selected_index = self.sparkline_grid_selected_index;

            // For instance details (chart view), set scroll offset to the currently selected metric
            // but ensure it doesn't exceed the available metrics count for charts
            let available_metrics_count = self.metrics.count_available_metrics();
            self.scroll_offset = self
                .sparkline_grid_selected_index
                .min(available_metrics_count.saturating_sub(1));
        }
    }

    pub fn back_to_list(&mut self) {
        self.state = AppState::InstanceList;
        self.selected_instance = None;
        // Reset all scroll positions when going back to the instance list
        self.scroll_offset = 0;
        self.metrics_summary_scroll = 0;
    }

    pub fn scroll_up(&mut self) {
        match self.state {
            AppState::MetricsSummary => match self.focused_panel {
                crate::models::FocusedPanel::TimeRanges => {
                    self.time_range_scroll_up();
                }
                crate::models::FocusedPanel::SparklineGrid => {
                    self.sparkline_grid_scroll_up();
                }
            },
            _ => {
                if self.scroll_offset > 0 {
                    self.scroll_offset -= 1;
                }
            }
        }
    }

    pub fn scroll_down(&mut self) {
        match self.state {
            AppState::MetricsSummary => match self.focused_panel {
                crate::models::FocusedPanel::TimeRanges => {
                    self.time_range_scroll_down();
                }
                crate::models::FocusedPanel::SparklineGrid => {
                    self.sparkline_grid_scroll_down();
                }
            },
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

    pub fn reset_scroll(&mut self) {
        match self.state {
            AppState::MetricsSummary => {
                self.metrics_summary_scroll = 0;
                self.scroll_offset = 0;
                self.focused_panel = crate::models::FocusedPanel::TimeRanges; // Reset to time ranges panel
                self.saved_focused_panel = crate::models::FocusedPanel::TimeRanges; // Reset saved state
                                                                                    // Also reset sparkline grid state and synchronize
                self.sparkline_grid_scroll = 0;
                self.sparkline_grid_selected_index = 0;
                self.saved_sparkline_grid_selected_index = 0; // Reset saved index
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

            // Update scroll offset only if selected item goes above visible area
            if self.sparkline_grid_selected_index < self.scroll_offset {
                self.scroll_offset = self.sparkline_grid_selected_index;
                self.metrics_summary_scroll = self.scroll_offset;
            }
        }
    }

    pub fn sparkline_grid_scroll_down(&mut self) {
        let available_metrics = self.metrics.get_available_metrics();
        if self.sparkline_grid_selected_index < available_metrics.len().saturating_sub(1) {
            self.sparkline_grid_selected_index += 1;
            self.update_selected_metric();

            // Update scroll offset only if selected item goes below visible area
            let max_visible_index = self.scroll_offset + self.metrics_per_screen.saturating_sub(1);
            if self.sparkline_grid_selected_index > max_visible_index {
                self.scroll_offset = self
                    .sparkline_grid_selected_index
                    .saturating_sub(self.metrics_per_screen.saturating_sub(1));
                self.metrics_summary_scroll = self.scroll_offset;
            }
        }
    }

    pub fn get_available_metrics(&self) -> Vec<crate::models::MetricType> {
        self.metrics.get_available_metrics()
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
                    if let Some(index) = available_metrics.iter().position(|m| m == current_metric)
                    {
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
}
