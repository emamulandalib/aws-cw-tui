use crate::aws::cloudwatch_service::load_dynamic_metrics;
use crate::models::{App, AwsService};
use anyhow::Result;
use log::{error, info};

impl App {
    /// Load metrics dynamically using CloudWatch list_metrics API
    pub async fn load_metrics_dynamic(&mut self, instance_id: &str) -> Result<()> {
        self.metrics_loading = true;

        let service = self.selected_service.as_ref().unwrap_or(&AwsService::Rds);

        info!(
            "Loading dynamic metrics for {:?} service, instance: {}",
            service, instance_id
        );

        match load_dynamic_metrics(service, instance_id, self.time_range, self.selected_period)
            .await
        {
            Ok(dynamic_metrics) => {
                info!(
                    "Successfully loaded {} dynamic metrics",
                    dynamic_metrics.len()
                );
                self.dynamic_metrics = Some(dynamic_metrics);
                self.metrics_loading = false;
                self.clear_error();
                self.initialize_sparkline_grid_dynamic();
                self.mark_refreshed();
                Ok(())
            }
            Err(e) => {
                error!("Failed to load dynamic metrics: {}", e);
                self.metrics_loading = false;
                self.set_error(format!("CloudWatch Dynamic Error: {e}"));
                self.dynamic_metrics = None;
                self.selected_metric_name = None;
                self.sparkline_grid_selected_index = 0;
                Ok(())
            }
        }
    }

    /// Load metrics using dynamic discovery system (main entry point)
    pub async fn load_metrics(&mut self, instance_id: &str) -> Result<()> {
        self.metrics_loading = true;

        let service = self.selected_service.as_ref().unwrap_or(&AwsService::Rds);

        info!(
            "Loading dynamic metrics for {:?} service, instance: {}",
            service, instance_id
        );

        match load_dynamic_metrics(service, instance_id, self.time_range, self.selected_period)
            .await
        {
            Ok(dynamic_metrics) => {
                info!(
                    "Successfully loaded {} dynamic metrics for instance: {}",
                    dynamic_metrics.len(),
                    instance_id
                );

                self.dynamic_metrics = Some(dynamic_metrics);
                self.initialize_sparkline_grid_dynamic();
                self.metrics_loading = false;
                self.clear_error();
                self.mark_refreshed();
                Ok(())
            }
            Err(e) => {
                error!("Failed to load dynamic metrics: {}", e);
                self.dynamic_metrics = None;
                self.selected_metric_name = None;
                self.metrics_loading = false;
                self.set_error(format!("Failed to load metrics: {}", e));
                Ok(())
            }
        }
    }

    /// Get list of available metric names
    pub fn get_available_metrics(&self) -> Vec<String> {
        if let Some(ref dynamic_metrics) = self.dynamic_metrics {
            dynamic_metrics.get_available_metric_names()
        } else {
            Vec::new()
        }
    }

    /// Update the selected metric based on current selection index
    pub fn update_selected_metric(&mut self) {
        if let Some(ref dynamic_metrics) = self.dynamic_metrics {
            let available_metrics = dynamic_metrics.get_available_metric_names();
            // Use the same source of truth as navigation methods
            let selected_index = self.sparkline_grid_list_state.selected().unwrap_or(0);
            if let Some(metric_name) = available_metrics.get(selected_index) {
                self.selected_metric_name = Some(metric_name.clone());
                // Keep sparkline_grid_selected_index synchronized with list_state
                self.sparkline_grid_selected_index = selected_index;
            }
        }
    }

    /// Initialize sparkline grid using dynamic metrics
    pub fn initialize_sparkline_grid_dynamic(&mut self) {
        if let Some(ref dynamic_metrics) = self.dynamic_metrics {
            let available_metrics = dynamic_metrics.get_available_metric_names();
            if !available_metrics.is_empty() {
                // Initialize both list_state and selected_index to the same value
                self.sparkline_grid_list_state.select(Some(0));
                self.sparkline_grid_selected_index = 0;
                self.selected_metric_name = Some(available_metrics[0].clone());
            } else {
                self.sparkline_grid_list_state.select(None);
                self.sparkline_grid_selected_index = 0;
                self.selected_metric_name = None;
            }
        } else {
            self.sparkline_grid_list_state.select(None);
            self.sparkline_grid_selected_index = 0;
            self.selected_metric_name = None;
        }

        // Initialize saved state to match current state
        self.saved_sparkline_grid_selected_index = self.sparkline_grid_selected_index;

        // Ensure all metric state is consistent
        self.update_selected_metric();
    }

    /// Legacy method - redirects to dynamic method
    pub fn initialize_sparkline_grid(&mut self) {
        self.initialize_sparkline_grid_dynamic();
    }

    /// Check if metrics are currently loading
    pub fn is_metrics_loading(&self) -> bool {
        self.metrics_loading
    }

    /// Check if metrics are available
    pub fn has_metrics(&self) -> bool {
        self.dynamic_metrics.is_some() && self.dynamic_metrics.as_ref().unwrap().len() > 0
    }

    /// Clear all metric data
    pub fn clear_metrics(&mut self) {
        self.dynamic_metrics = None;
        self.selected_metric_name = None;
        self.sparkline_grid_selected_index = 0;
        self.sparkline_grid_list_state.select(None);
        self.metrics_loading = false;
    }
}
