use crate::aws::time_range::{TimeRange, TimeUnit};
use crate::aws::{cloudwatch_service::{load_metrics, load_metrics_with_instance}, load_rds_instances, rds::RdsInstanceManager};
use crate::models::{App, AppState, AwsService, FocusedPanel, MetricType, ServiceInstance, TimeRangeMode, Timezone};
use anyhow::Result;
use std::time::{Duration, Instant};
use log::info;

use crate::models::{RdsInstance, SqsQueue};
impl App {
    // ================================
    // 1. INITIALIZATION
    // ================================

    pub fn new() -> App {
        let mut app = App {
            // Service selection initialization (RDS-focused for now)
            available_services: vec![AwsService::Rds, AwsService::Sqs], // Support both RDS and SQS
            service_list_state: ratatui::widgets::ListState::default(),
            selected_service: None, // No service selected initially

            // Instance list initialization
            instances: Vec::new(),
            rds_instances: Vec::new(),
            sqs_queues: Vec::new(),
            list_state: ratatui::widgets::ListState::default(),
            loading: false,
            state: AppState::ServiceList, // Start with service selection
            selected_instance: None,
            metrics: crate::models::MetricData::default(),
            sqs_metrics: crate::models::SqsMetricData::default(),
            metrics_loading: false,
            last_refresh: None,
            auto_refresh_enabled: true,
            scroll_offset: 0,
            metrics_per_screen: 1,
            metrics_summary_scroll: 0,
            time_range_scroll: 8, // Default to "3 hours" in the new extended options
            focused_panel: FocusedPanel::Timezone,
            saved_focused_panel: FocusedPanel::Timezone,
            time_range: TimeRange::new(3, TimeUnit::Hours, 1).unwrap(),

            // Initialize sparkline grid state
            selected_metric: None,
            sparkline_grid_scroll: 0,
            sparkline_grid_selected_index: 0,
            saved_sparkline_grid_selected_index: 0,
            sparkline_grid_list_state: ratatui::widgets::ListState::default(),

            // Initialize error handling
            error_message: None,

            // Initialize loading timeout
            loading_start_time: None,
            
            // Initialize time range mode
            time_range_mode: TimeRangeMode::Relative,
            
            // Initialize period selection
            period_scroll: 2, // Default to a reasonable period option
            
            // Initialize timezone selection
            timezone: Timezone::Utc, // Default to UTC timezone
            timezone_scroll: 1, // Default to UTC (index 1 in the options)
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
            if start_time.elapsed() > Duration::from_secs(30) {
                // 30 second timeout
                self.loading = false;
                self.loading_start_time = None;
                self.error_message = Some(
                    "Loading timeout - operation took too long. Press 'r' to retry.".to_string(),
                );
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
        if self.available_services.is_empty() {
            return;
        }
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
        if self.available_services.is_empty() {
            return;
        }
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
        info!("Loading service instances for: {:?}", service);
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
                    self.error_message = Some(format!("AWS Error: {e}"));
                    self.loading = false;
                    self.instances = Vec::new();
                    self.rds_instances = Vec::new();
                    self.list_state.select(None);
                    Ok(())
                }
            },
            AwsService::Sqs => {
                // Load SQS queues
                match crate::aws::sqs_service::load_sqs_queues().await {
                    Ok(sqs_queues) => {
                        self.sqs_queues = sqs_queues.clone();
                        self.instances = sqs_queues
                            .into_iter()
                            .map(ServiceInstance::Sqs)
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
                        self.error_message = Some(format!("AWS SQS Error: {e}"));
                        self.loading = false;
                        self.instances = Vec::new();
                        self.sqs_queues = Vec::new();
                        self.list_state.select(None);
                        Ok(())
                    }
                }
            }
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
                self.instances = instances.into_iter().map(ServiceInstance::Rds).collect();

                self.loading = false;
                self.loading_start_time = None;
                self.list_state = ratatui::widgets::ListState::default();
                if !self.instances.is_empty() {
                    self.list_state.select(Some(0));
                }

                // Mark as refreshed to prevent continuous refresh loops
                self.mark_refreshed();
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
        log::info!("Getting selected instance, list_state.selected(): {:?}, instances.len(): {}", 
            self.list_state.selected(), self.instances.len());
        if let Some(index) = self.list_state.selected() {
            log::info!("List state has selected index: {}", index);
            let instance = self.instances.get(index);
            if let Some(inst) = instance {
                log::info!("Found instance at index {}: {:?}", index, inst.as_aws_instance().id());
            } else {
                log::warn!("No instance found at index {}", index);
            }
            instance
        } else {
            log::info!("No index selected in list state");
            None
        }
    }

    pub fn get_selected_instance_id(&self) -> Option<String> {
        self.get_selected_instance()
            .map(|instance| instance.as_aws_instance().id().to_string())
    }

    /// Safely get the selected RDS instance with bounds checking
    pub fn get_selected_rds_instance(&self) -> Option<&RdsInstance> {
        log::info!("Getting selected RDS instance, selected_instance: {:?}", self.selected_instance);
        if let Some(instance) = self.get_selected_instance() {
            log::info!("Found selected instance, checking type...");
            match instance {
                ServiceInstance::Rds(rds) => {
                    log::info!("Selected instance is RDS: {} ({})", rds.identifier, rds.engine);
                    Some(rds)
                },
                _ => {
                    log::info!("Selected instance is not RDS type");
                    None
                }
            }
        } else {
            log::info!("No instance selected");
            None
        }
    }

    /// Safely get the selected RDS instance ID with bounds checking
    pub fn get_selected_rds_instance_id(&self) -> Option<String> {
        self.get_selected_rds_instance()
            .map(|instance| instance.identifier.clone())
    }

    /// Safely get the selected SQS queue with bounds checking
    pub fn get_selected_sqs_queue(&self) -> Option<&SqsQueue> {
        if let Some(instance) = self.get_selected_instance() {
            match instance {
                ServiceInstance::Sqs(queue) => Some(queue),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Count available metrics based on the current service
    pub fn count_available_metrics(&self) -> usize {
        match self.selected_service.as_ref().unwrap_or(&AwsService::Rds) {
            AwsService::Rds => self.metrics.get_available_metrics_with_data().len(),
            AwsService::Sqs => self.sqs_metrics.get_available_metrics().len(),
        }
    }

    // ================================
    // 6. METRICS MANAGEMENT
    // ================================

    pub async fn load_metrics(&mut self, instance_id: &str) -> Result<()> {
    self.metrics_loading = true;

    let service = self.selected_service.as_ref().unwrap_or(&AwsService::Rds);

    match service {
        AwsService::Rds => {
            // Load RDS metrics with intelligent filtering
            if let Some(rds_instance) = self.get_selected_rds_instance() {
                log::info!("Loading metrics with intelligent filtering for RDS instance: {} ({})", 
                    rds_instance.identifier, rds_instance.engine);
                match load_metrics_with_instance(rds_instance, self.time_range).await {
                    Ok(metrics) => {
                        log::info!("Successfully loaded metrics with characteristics");
                        self.metrics = metrics;
                        self.metrics_loading = false;
                        self.clear_error();
                        self.initialize_sparkline_grid();
                        self.mark_refreshed();
                        Ok(())
                    }
                    Err(e) => {
                        log::error!("Failed to load metrics with characteristics: {}", e);
                        self.metrics_loading = false;
                        self.error_message = Some(format!("CloudWatch Error: {e}"));
                        self.metrics = crate::models::MetricData::default();
                        self.selected_metric = None;
                        self.sparkline_grid_selected_index = 0;
                        Ok(())
                    }
                }
            } else {
                log::warn!("No RDS instance selected, falling back to old method");
                // Fallback to old method if no RDS instance is selected
                match load_metrics(instance_id, self.time_range).await {
                    Ok(metrics) => {
                        self.metrics = metrics;
                        self.metrics_loading = false;
                        self.clear_error();
                        self.initialize_sparkline_grid();
                        self.mark_refreshed();
                        Ok(())
                    }
                    Err(e) => {
                        self.metrics_loading = false;
                        self.error_message = Some(format!("CloudWatch Error: {e}"));
                        self.metrics = crate::models::MetricData::default();
                        self.selected_metric = None;
                        self.sparkline_grid_selected_index = 0;
                        Ok(())
                    }
                }
            }
        }
        AwsService::Sqs => {
            // Load SQS metrics
            if let Some(queue) = self.get_selected_sqs_queue() {
                match crate::aws::sqs_metrics::fetch_sqs_metrics(queue, &self.time_range).await {
                    Ok(sqs_metrics) => {
                        self.sqs_metrics = sqs_metrics;
                        self.metrics_loading = false;
                        self.clear_error();
                        self.initialize_sparkline_grid_for_sqs();
                        self.mark_refreshed();
                        Ok(())
                    }
                    Err(e) => {
                        self.metrics_loading = false;
                        self.error_message = Some(format!("CloudWatch SQS Error: {e}"));
                        self.sqs_metrics = crate::models::SqsMetricData::default();
                        self.selected_metric = None;
                        self.sparkline_grid_selected_index = 0;
                        Ok(())
                    }
                }
            } else {
                self.metrics_loading = false;
                self.error_message = Some("No SQS queue selected".to_string());
                Ok(())
            }
        }
    }
}

    pub fn get_available_metrics(&self) -> Vec<MetricType> {
    match self.selected_service.as_ref().unwrap_or(&AwsService::Rds) {
        AwsService::Rds => self.metrics.get_available_metrics_with_data(),
        AwsService::Sqs => self.sqs_metrics.get_available_metrics(),
    }
}

    pub fn get_sparkline_grid_selected_index(&self) -> usize {
        self.sparkline_grid_selected_index
    }

    pub fn update_selected_metric(&mut self) {
    match self.selected_service.as_ref().unwrap_or(&AwsService::Rds) {
        AwsService::Rds => {
            let available_metrics = self.metrics.get_available_metrics_with_data();
            if let Some(metric) = available_metrics.get(self.sparkline_grid_selected_index) {
                self.selected_metric = Some(metric.clone());
            }
        }
        AwsService::Sqs => {
            let available_metrics = self.sqs_metrics.get_available_metrics();
            if let Some(metric) = available_metrics.get(self.sparkline_grid_selected_index) {
                self.selected_metric = Some(metric.clone());
            }
        }
    }
}

        pub fn initialize_sparkline_grid(&mut self) {
        match self.selected_service.as_ref().unwrap_or(&AwsService::Rds) {
        AwsService::Rds => {
            let available_metrics = self.metrics.get_available_metrics_with_data();
            if !available_metrics.is_empty() {
                if self.selected_metric.is_none() {
                    self.selected_metric = Some(available_metrics[0].clone());
                    self.sparkline_grid_selected_index = 0;
                    self.sparkline_grid_list_state.select(Some(0));
                } else if let Some(ref current_metric) = self.selected_metric {
                    if let Some(index) = available_metrics.iter().position(|m| m == current_metric) {
                        self.sparkline_grid_selected_index = index;
                        self.sparkline_grid_list_state.select(Some(index));
                    } else {
                        self.selected_metric = Some(available_metrics[0].clone());
                        self.sparkline_grid_selected_index = 0;
                        self.sparkline_grid_list_state.select(Some(0));
                    }
                }
            } else {
                self.selected_metric = None;
                self.sparkline_grid_selected_index = 0;
                self.sparkline_grid_list_state.select(None);
            }
        }
        AwsService::Sqs => {
            let available_metrics = self.sqs_metrics.get_available_metrics();
            if !available_metrics.is_empty() {
                if self.selected_metric.is_none() {
                    self.selected_metric = Some(available_metrics[0].clone());
                    self.sparkline_grid_selected_index = 0;
                    self.sparkline_grid_list_state.select(Some(0));
                } else if let Some(ref current_metric) = self.selected_metric {
                    if let Some(index) = available_metrics.iter().position(|m| m == current_metric) {
                        self.sparkline_grid_selected_index = index;
                        self.sparkline_grid_list_state.select(Some(index));
                    } else {
                        self.selected_metric = Some(available_metrics[0].clone());
                        self.sparkline_grid_selected_index = 0;
                        self.sparkline_grid_list_state.select(Some(0));
                    }
                }
            } else {
                self.selected_metric = None;
                self.sparkline_grid_selected_index = 0;
                self.sparkline_grid_list_state.select(None);
            }
        }
    }
}

    pub fn initialize_sparkline_grid_for_sqs(&mut self) {
        let available_metrics = self.sqs_metrics.get_available_metrics();
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
            // Minutes
            ("1 minute", 1, TimeUnit::Minutes, 1),
            ("3 minutes", 3, TimeUnit::Minutes, 1),
            ("5 minutes", 5, TimeUnit::Minutes, 1),
            ("15 minutes", 15, TimeUnit::Minutes, 1),
            ("30 minutes", 30, TimeUnit::Minutes, 1),
            ("45 minutes", 45, TimeUnit::Minutes, 1),
            
            // Hours
            ("1 hour", 1, TimeUnit::Hours, 1),
            ("2 hours", 2, TimeUnit::Hours, 1),
            ("3 hours", 3, TimeUnit::Hours, 1),
            ("6 hours", 6, TimeUnit::Hours, 1),
            ("8 hours", 8, TimeUnit::Hours, 1),
            ("12 hours", 12, TimeUnit::Hours, 1),
            
            // Days
            ("1 day", 1, TimeUnit::Days, 1),
            ("2 days", 2, TimeUnit::Days, 1),
            ("3 days", 3, TimeUnit::Days, 1),
            ("4 days", 4, TimeUnit::Days, 1),
            ("5 days", 5, TimeUnit::Days, 1),
            ("6 days", 6, TimeUnit::Days, 1),
            
            // Weeks
            ("1 week", 1, TimeUnit::Weeks, 7),
            ("2 weeks", 2, TimeUnit::Weeks, 14),
            ("4 weeks", 4, TimeUnit::Weeks, 28),
            ("6 weeks", 6, TimeUnit::Weeks, 42),
            
            // Months
            ("3 months", 3, TimeUnit::Months, 90),
            ("6 months", 6, TimeUnit::Months, 180),
            ("12 months", 12, TimeUnit::Months, 365),
            ("15 months", 15, TimeUnit::Months, 455),
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
    
    pub fn time_range_scroll_left(&mut self) {
        // In simple vertical list, left arrow acts like up arrow (previous item)
        self.time_range_scroll_up();
    }
    
    pub fn time_range_scroll_right(&mut self) {
        // In simple vertical list, right arrow acts like down arrow (next item)  
        self.time_range_scroll_down();
    }
    
    pub fn toggle_time_range_mode(&mut self) {
        self.time_range_mode = match self.time_range_mode {
            TimeRangeMode::Absolute => TimeRangeMode::Relative,
            TimeRangeMode::Relative => TimeRangeMode::Absolute,
        };
    }
    
    pub fn get_time_range_mode(&self) -> &TimeRangeMode {
        &self.time_range_mode
    }

    // Period selection methods
    pub fn get_period_options() -> Vec<(&'static str, i32)> {
        vec![
            ("5 seconds", 5),
            ("10 seconds", 10),
            ("20 seconds", 20),
            ("30 seconds", 30),
            ("1 minute", 60),
            ("5 minutes", 300),
            ("15 minutes", 900),
            ("1 hour", 3600),
            ("6 hours", 21600),
            ("1 day", 86400),
            ("7 days", 604800),
            ("30 days", 2592000),
        ]
    }

    pub fn get_current_period_index(&self) -> usize {
        self.period_scroll
    }

    pub fn period_scroll_up(&mut self) {
        if self.period_scroll > 0 {
            self.period_scroll -= 1;
        }
    }

    pub fn period_scroll_down(&mut self) {
        let options = Self::get_period_options();
        if self.period_scroll < options.len() - 1 {
            self.period_scroll += 1;
        }
    }
    
    // Timezone selection methods
    pub fn get_timezone_options() -> Vec<Timezone> {
        Timezone::get_timezone_options()
    }
    
    pub fn get_current_timezone(&self) -> &Timezone {
        &self.timezone
    }
    
    pub fn get_current_timezone_index(&self) -> usize {
        self.timezone_scroll
    }
    
    pub fn timezone_scroll_up(&mut self) {
        if self.timezone_scroll > 0 {
            self.timezone_scroll -= 1;
            let options = Self::get_timezone_options();
            if let Some(timezone) = options.get(self.timezone_scroll) {
                self.timezone = timezone.clone();
            }
        }
    }
    
    pub fn timezone_scroll_down(&mut self) {
        let options = Self::get_timezone_options();
        if self.timezone_scroll < options.len() - 1 {
            self.timezone_scroll += 1;
            if let Some(timezone) = options.get(self.timezone_scroll) {
                self.timezone = timezone.clone();
            }
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
            self.focused_panel = FocusedPanel::SparklineGrid; // Focus on metrics list instead of timezone
            self.sparkline_grid_selected_index = 0;
            self.initialize_sparkline_grid();
        }
    }

    pub fn back_to_metrics_summary(&mut self) {
        self.state = AppState::MetricsSummary;
        self.focused_panel = self.saved_focused_panel.clone();
        self.sparkline_grid_selected_index = self.saved_sparkline_grid_selected_index;
        self.update_selected_metric();
        // ListState maintains its own position, no need for manual scroll_offset restoration
    }

    pub fn enter_instance_details(&mut self) {
        if let Some(i) = self.list_state.selected() {
            self.selected_instance = Some(i);
            self.state = AppState::InstanceDetails;
            self.saved_focused_panel = self.focused_panel.clone();
            self.saved_sparkline_grid_selected_index = self.sparkline_grid_selected_index;

            // The ListState will handle the selection automatically
            // No need for manual scroll_offset manipulation since we're using the same ListState
        }
    }

    pub fn back_to_list(&mut self) {
        self.state = AppState::InstanceList;
        self.selected_instance = None;
        // Reset metrics navigation state
        self.reset_scroll();
    }

    // ================================
    // 9. SCROLLING & PANEL MANAGEMENT
    // ================================

    pub fn scroll_up(&mut self) {
        match self.state {
            AppState::MetricsSummary => match self.focused_panel {
                FocusedPanel::Timezone => {
                    self.timezone_scroll_up();
                }
                FocusedPanel::Period => {
                    self.period_scroll_up();
                }
                FocusedPanel::TimeRanges => {
                    self.time_range_scroll_up();
                }
                FocusedPanel::SparklineGrid => {
                    self.sparkline_grid_scroll_up();
                }
            },
            AppState::InstanceDetails => {
                // Use the same built-in ListState navigation as MetricsSummary
                self.sparkline_grid_scroll_up();
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
            AppState::MetricsSummary => match self.focused_panel {
                FocusedPanel::Timezone => {
                    self.timezone_scroll_down();
                }
                FocusedPanel::Period => {
                    self.period_scroll_down();
                }
                FocusedPanel::TimeRanges => {
                    self.time_range_scroll_down();
                }
                FocusedPanel::SparklineGrid => {
                    self.sparkline_grid_scroll_down();
                }
            },
            AppState::InstanceDetails => {
                // Use the same built-in ListState navigation as MetricsSummary
                self.sparkline_grid_scroll_down();
            }
            _ => {}
        }
    }

    pub fn reset_scroll(&mut self) {
        // Use unified reset for all states since we're using ListState everywhere
        self.focused_panel = FocusedPanel::Timezone;
        self.saved_focused_panel = FocusedPanel::Timezone;
        self.sparkline_grid_selected_index = 0;
        self.saved_sparkline_grid_selected_index = 0;
        self.sparkline_grid_list_state = ratatui::widgets::ListState::default();
        self.initialize_sparkline_grid();
        
        // Legacy fields can be set to 0 for compatibility
        self.scroll_offset = 0;
        self.metrics_summary_scroll = 0;
        self.sparkline_grid_scroll = 0;
    }

    pub fn switch_panel(&mut self) {
        self.focused_panel = match self.focused_panel {
            FocusedPanel::Timezone => FocusedPanel::Period,
            FocusedPanel::Period => FocusedPanel::TimeRanges,
            FocusedPanel::TimeRanges => FocusedPanel::SparklineGrid,
            FocusedPanel::SparklineGrid => FocusedPanel::Timezone,
        };
    }

    pub fn get_focused_panel(&self) -> &FocusedPanel {
        &self.focused_panel
    }

    /// Update metrics_per_screen based on available area
    /// This should be called before rendering to ensure navigation functions work correctly
    pub fn update_metrics_per_screen(&mut self, area_height: u16) {
        // For the new metrics grid system (2 metrics per row)
        let metrics_per_row = 2;
        let min_height_per_row = 12; // Minimum height needed for each row
        let available_height = area_height.saturating_sub(2) as usize; // Account for borders
        
        // Ensure we have at least enough space for one row
        let max_rows = if available_height >= min_height_per_row {
            (available_height / min_height_per_row).max(1)
        } else {
            1 // Fallback to show at least one row even if space is tight
        };
        
        let calculated_metrics_per_screen = max_rows * metrics_per_row;
        
        // Ensure metrics_per_screen is never zero and always a multiple of metrics_per_row
        if calculated_metrics_per_screen == 0 {
            self.metrics_per_screen = metrics_per_row; // Default to 2 metrics (1 row)
        } else {
            // Ensure it's always a multiple of metrics_per_row for grid alignment
            self.metrics_per_screen = (calculated_metrics_per_screen / metrics_per_row) * metrics_per_row;
            if self.metrics_per_screen == 0 {
                self.metrics_per_screen = metrics_per_row; // Fallback to at least one row
            }
        }
    }

    // ================================
    // 10. SPARKLINE GRID NAVIGATION
    // ================================

    pub fn sparkline_grid_scroll_up(&mut self) {
        // Get total metrics count for bounds checking
        let available_metrics = match self.selected_service.as_ref().unwrap_or(&AwsService::Rds) {
            AwsService::Rds => self.metrics.get_available_metrics_with_data(),
            AwsService::Sqs => self.sqs_metrics.get_available_metrics(),
        };
        
        let total_metrics = available_metrics.len();
        if total_metrics == 0 {
            return;
        }
        
        // Use ratatui's built-in ListState for navigation
        let current_selection = self.sparkline_grid_list_state.selected().unwrap_or(0);
        
        if current_selection > 0 {
            let new_selection = current_selection.saturating_sub(1);
            self.sparkline_grid_list_state.select(Some(new_selection));
            self.sparkline_grid_selected_index = new_selection;
            self.update_selected_metric();
        }
    }

    pub fn sparkline_grid_scroll_down(&mut self) {
        let available_metrics = match self.selected_service.as_ref().unwrap_or(&AwsService::Rds) {
            AwsService::Rds => self.metrics.get_available_metrics_with_data(),
            AwsService::Sqs => self.sqs_metrics.get_available_metrics(),
        };
        
        let total_metrics = available_metrics.len();
        if total_metrics == 0 {
            return;
        }
        
        // Use ratatui's built-in ListState for navigation
        let current_selection = self.sparkline_grid_list_state.selected().unwrap_or(0);
        
        if current_selection < total_metrics.saturating_sub(1) {
            let new_selection = (current_selection + 1).min(total_metrics.saturating_sub(1));
            self.sparkline_grid_list_state.select(Some(new_selection));
            self.sparkline_grid_selected_index = new_selection;
            self.update_selected_metric();
        }
    }

    pub fn sparkline_grid_scroll_left(&mut self) {
        let available_metrics = match self.selected_service.as_ref().unwrap_or(&AwsService::Rds) {
            AwsService::Rds => self.metrics.get_available_metrics_with_data(),
            AwsService::Sqs => self.sqs_metrics.get_available_metrics(),
        };
        
        let total_metrics = available_metrics.len();
        if total_metrics == 0 {
            return;
        }
        
        // Use ratatui's built-in ListState for navigation
        let current_selection = self.sparkline_grid_list_state.selected().unwrap_or(0);
        
        if current_selection > 0 {
            let new_selection = current_selection.saturating_sub(1);
            self.sparkline_grid_list_state.select(Some(new_selection));
            self.sparkline_grid_selected_index = new_selection;
            self.update_selected_metric();
        }
    }

    pub fn sparkline_grid_scroll_right(&mut self) {
        let available_metrics = match self.selected_service.as_ref().unwrap_or(&AwsService::Rds) {
            AwsService::Rds => self.metrics.get_available_metrics_with_data(),
            AwsService::Sqs => self.sqs_metrics.get_available_metrics(),
        };
        
        let total_metrics = available_metrics.len();
        if total_metrics == 0 {
            return;
        }
        
        // Use ratatui's built-in ListState for navigation
        let current_selection = self.sparkline_grid_list_state.selected().unwrap_or(0);
        
        if current_selection < total_metrics.saturating_sub(1) {
            let new_selection = (current_selection + 1).min(total_metrics.saturating_sub(1));
            self.sparkline_grid_list_state.select(Some(new_selection));
            self.sparkline_grid_selected_index = new_selection;
            self.update_selected_metric();
        }
    }

    // This function is no longer needed as ratatui's ListState handles scrolling automatically
}
