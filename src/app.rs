use crate::aws::{load_rds_instances, rds::RdsInstanceManager, cloudwatch_service::load_metrics};
use crate::aws::time_range::{TimeRange, TimeUnit};
use crate::models::{App, AppState, AwsService, FocusedPanel, MetricType, ServiceInstance};
use anyhow::Result;
use std::time::{Duration, Instant};

impl App {
    // ================================
    // 1. INITIALIZATION
    // ================================

    pub fn new() -> App {
            let mut app = App {
                // Service selection initialization (RDS-focused for now)
                available_services: vec![AwsService::Rds],  // Focus on RDS only
                service_list_state: ratatui::widgets::ListState::default(),
                selected_service: None,  // No service selected initially
    
                // Instance list initialization
                instances: Vec::new(),
                rds_instances: Vec::new(),
                list_state: ratatui::widgets::ListState::default(),
                loading: false,
                state: AppState::ServiceList,  // Start with service selection
                selected_instance: None,
                metrics: crate::models::MetricData::default(),
                metrics_loading: false,
                last_refresh: None,
                auto_refresh_enabled: true,
                scroll_offset: 0,
                metrics_per_screen: 1,
                metrics_summary_scroll: 0,
                time_range_scroll: 2,
                focused_panel: FocusedPanel::TimeRanges,
                saved_focused_panel: FocusedPanel::TimeRanges,
                time_range: TimeRange::new(3, TimeUnit::Hours, 1).unwrap(),
    
                // Initialize sparkline grid state
                selected_metric: None,
                sparkline_grid_scroll: 0,
                sparkline_grid_selected_index: 0,
                saved_sparkline_grid_selected_index: 0,
    
                // Initialize error handling
                error_message: None,
                
                // Initialize loading timeout
                loading_start_time: None,
            };
            app.service_list_state.select(Some(0));
            app
        }



    // ================================
    // 2. STATE MANAGEMENT
    // ================================

    pub fn needs_refresh(&self) -> bool {
        if !self.auto_refresh_enabled {
            return false;
        }
        match self.last_refresh {
            None => true,
            Some(last) => last.elapsed() > Duration::from_secs(30), // Reduced from 60 to 30 seconds
        }
    }


    pub fn mark_refreshed(&mut self) {
        self.last_refresh = Some(Instant::now());
    }

    pub fn clear_error(&mut self) {
        self.error_message = None;
    }
    
    pub fn check_loading_timeout(&mut self) -> bool {
        if let Some(start_time) = self.loading_start_time {
            if start_time.elapsed() > Duration::from_secs(30) { // 30 second timeout
                self.loading = false;
                self.loading_start_time = None;
                self.error_message = Some("Loading timeout - operation took too long. Press 'r' to retry.".to_string());
                return true;
            }
        }
        false
    }    
        // ================================
        // RDS-FOCUSED METHODS  
        // ================================
        

        

    // ================================
    // 3. NAVIGATION METHODS
    // ================================

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

    pub fn next(&mut self) {
        if self.instances.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.instances.len() - 1 {
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
        if self.instances.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.instances.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    // ================================
    // 4. SERVICE MANAGEMENT
    // ================================

    pub fn select_service(&mut self) -> Option<&AwsService> {
        if let Some(index) = self.service_list_state.selected() {
            if let Some(service) = self.available_services.get(index) {
                self.selected_service = Some(service.clone());
                self.state = AppState::InstanceList;
                self.list_state.select(Some(0));
                return Some(service);
            }
        }
        None
    }

    pub fn back_to_service_list(&mut self) {
        self.state = AppState::ServiceList;
        self.selected_service = None;
        self.instances.clear();
        self.rds_instances.clear();
        self.loading = true;
    }

    pub async fn load_service_instances(&mut self, service: &AwsService) -> Result<()> {
        match service {
            AwsService::Rds => match load_rds_instances().await {
                Ok(rds_instances) => {
                    self.rds_instances = rds_instances.clone();
                    self.instances = rds_instances
                        .into_iter()
                        .map(ServiceInstance::Rds)
                        .collect();
                    self.clear_error();
                    self.loading = false;
                    self.mark_refreshed();

                    if !self.instances.is_empty() {
                        let current_selection = self.list_state.selected().unwrap_or(0);
                        let new_selection = if current_selection < self.instances.len() {
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
                Err(e) => {
                    self.error_message = Some(format!("AWS Error: {}", e));
                    self.loading = false;
                    self.instances = Vec::new();
                    self.rds_instances = Vec::new();
                    self.list_state.select(None);
                    Ok(())
                }
            },

        }
    }

    pub async fn load_rds_instances(&mut self) -> Result<()> {
        self.loading = true;
        self.loading_start_time = Some(Instant::now());
        self.error_message = None;

        match RdsInstanceManager::load_instances().await {
            Ok(instances) => {
                // Store in both places for compatibility
                self.rds_instances = instances.clone();
                self.instances = instances
                    .into_iter()
                    .map(ServiceInstance::Rds)
                    .collect();

                self.loading = false;
                self.loading_start_time = None;
                self.list_state = ratatui::widgets::ListState::default();
                if !self.instances.is_empty() {
                    self.list_state.select(Some(0));
                }
            }
            Err(e) => {
                self.loading = false;
                self.loading_start_time = None;
                self.error_message = Some(e.to_string());
            }
        }

        Ok(())
    }

// ================================
    // 5. INSTANCE ACCESS HELPERS
    // ================================

    pub fn get_current_instances(&self) -> &Vec<ServiceInstance> {
        &self.instances
    }

    pub fn get_selected_instance(&self) -> Option<&ServiceInstance> {
        if let Some(index) = self.list_state.selected() {
            self.instances.get(index)
        } else {
            None
        }
    }

    pub fn get_selected_instance_id(&self) -> Option<String> {
        self.get_selected_instance()
            .map(|instance| instance.as_aws_instance().id().to_string())
    }

    // ================================
    // 6. METRICS MANAGEMENT
    // ================================

    pub async fn load_metrics(&mut self, instance_id: &str) -> Result<()> {
        self.metrics_loading = true;

        let _service = self.selected_service.as_ref().unwrap_or(&AwsService::Rds);

        match load_metrics(instance_id, self.time_range).await {
            Ok(metrics) => {
                self.metrics = metrics;
                self.metrics_loading = false;
                self.clear_error();
                self.initialize_sparkline_grid();
                Ok(())
            }
            Err(e) => {
                self.metrics_loading = false;
                self.error_message = Some(format!("CloudWatch Error: {}", e));
                self.metrics = crate::models::MetricData::default();
                self.selected_metric = None;
                self.sparkline_grid_selected_index = 0;
                Ok(())
            }
        }
    }

    pub fn get_available_metrics(&self) -> Vec<MetricType> {
        self.metrics.get_available_metrics()
    }

    pub fn get_sparkline_grid_selected_index(&self) -> usize {
        self.sparkline_grid_selected_index
    }

    pub fn update_selected_metric(&mut self) {
        let available_metrics = self.metrics.get_available_metrics();
        if let Some(metric) = available_metrics.get(self.sparkline_grid_selected_index) {
            self.selected_metric = Some(metric.clone());
        }
    }

    pub fn initialize_sparkline_grid(&mut self) {
        let available_metrics = self.metrics.get_available_metrics();
        if !available_metrics.is_empty() {
            if self.selected_metric.is_none() {
                self.selected_metric = Some(available_metrics[0].clone());
                self.sparkline_grid_selected_index = 0;
            } else if let Some(ref current_metric) = self.selected_metric {
                if let Some(index) = available_metrics.iter().position(|m| m == current_metric) {
                    self.sparkline_grid_selected_index = index;
                } else {
                    self.selected_metric = Some(available_metrics[0].clone());
                    self.sparkline_grid_selected_index = 0;
                }
            }
        } else {
            self.selected_metric = None;
            self.sparkline_grid_selected_index = 0;
        }
    }

    // ================================
    // 7. TIME RANGE MANAGEMENT
    // ================================

    pub fn update_time_range(
        &mut self,
        value: u32,
        unit: TimeUnit,
        period_days: u32,
    ) -> Result<()> {
        self.time_range = TimeRange::new(value, unit, period_days)?;
        Ok(())
    }

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

    // ================================
    // 8. SCREEN NAVIGATION & STATE TRANSITIONS
    // ================================

    pub fn enter_metrics_summary(&mut self) {
        if let Some(i) = self.list_state.selected() {
            self.selected_instance = Some(i);
            self.state = AppState::MetricsSummary;
            self.metrics_summary_scroll = 0;
            self.scroll_offset = 0;
            self.focused_panel = FocusedPanel::TimeRanges;
            self.sparkline_grid_selected_index = 0;
            self.initialize_sparkline_grid();
        }
    }

    pub fn back_to_metrics_summary(&mut self) {
        self.state = AppState::MetricsSummary;
        self.scroll_offset = self.metrics_summary_scroll;
        self.focused_panel = self.saved_focused_panel.clone();
        self.sparkline_grid_selected_index = self.saved_sparkline_grid_selected_index;
        self.update_selected_metric();
    }

    pub fn enter_instance_details(&mut self) {
        if let Some(i) = self.list_state.selected() {
            self.selected_instance = Some(i);
            self.state = AppState::InstanceDetails;
            self.metrics_summary_scroll = self.scroll_offset;
            self.saved_focused_panel = self.focused_panel.clone();
            self.saved_sparkline_grid_selected_index = self.sparkline_grid_selected_index;

            let available_metrics_count = self.metrics.count_available_metrics();
            self.scroll_offset = self
                .sparkline_grid_selected_index
                .min(available_metrics_count.saturating_sub(1));
        }
    }

    pub fn back_to_list(&mut self) {
        self.state = AppState::InstanceList;
        self.selected_instance = None;
        self.scroll_offset = 0;
        self.metrics_summary_scroll = 0;
    }

    // ================================
    // 9. SCROLLING & PANEL MANAGEMENT
    // ================================

    pub fn scroll_up(&mut self) {
        match self.state {
            AppState::MetricsSummary => match self.focused_panel {
                FocusedPanel::TimeRanges => {
                    self.time_range_scroll_up();
                }
                FocusedPanel::SparklineGrid => {
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
                FocusedPanel::TimeRanges => {
                    self.time_range_scroll_down();
                }
                FocusedPanel::SparklineGrid => {
                    self.sparkline_grid_scroll_down();
                }
            },
            AppState::InstanceDetails => {
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
                self.focused_panel = FocusedPanel::TimeRanges;
                self.saved_focused_panel = FocusedPanel::TimeRanges;
                self.sparkline_grid_scroll = 0;
                self.sparkline_grid_selected_index = 0;
                self.saved_sparkline_grid_selected_index = 0;
                self.initialize_sparkline_grid();
            }
            _ => {
                self.scroll_offset = 0;
                self.sparkline_grid_selected_index = 0;
                self.metrics_summary_scroll = 0;
            }
        }
    }

    pub fn switch_panel(&mut self) {
        self.focused_panel = match self.focused_panel {
            FocusedPanel::TimeRanges => FocusedPanel::SparklineGrid,
            FocusedPanel::SparklineGrid => FocusedPanel::TimeRanges,
        };
    }

    pub fn get_focused_panel(&self) -> &FocusedPanel {
        &self.focused_panel
    }

    // ================================
    // 10. SPARKLINE GRID NAVIGATION
    // ================================

    pub fn sparkline_grid_scroll_up(&mut self) {
        if self.sparkline_grid_selected_index > 0 {
            self.sparkline_grid_selected_index -= 1;
            self.update_selected_metric();

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

            let max_visible_index = self.scroll_offset + self.metrics_per_screen.saturating_sub(1);
            if self.sparkline_grid_selected_index > max_visible_index {
                self.scroll_offset = self
                    .sparkline_grid_selected_index
                    .saturating_sub(self.metrics_per_screen.saturating_sub(1));
                self.metrics_summary_scroll = self.scroll_offset;
            }
        }
    }
}
