use std::time::{Duration, Instant};
use ratatui::widgets::ListState;
use anyhow::Result;
use crate::models::{App, AppState, MetricData};
use crate::aws::{load_rds_instances, load_metrics};

impl App {
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
        
        match load_metrics(instance_id).await {
            Ok(metrics) => {
                self.metrics = metrics;
                self.metrics_loading = false;
                Ok(())
            }
            Err(e) => {
                self.metrics_loading = false;
                // Reset to default metrics on error
                self.metrics = MetricData::default();
                Err(e)
            }
        }
    }

    pub fn enter_instance_details(&mut self) {
        if let Some(i) = self.list_state.selected() {
            self.selected_instance = Some(i);
            self.state = AppState::InstanceDetails;
        }
    }

    pub fn back_to_list(&mut self) {
        self.state = AppState::RdsList;
        self.selected_instance = None;
    }

    pub fn scroll_up(&mut self) {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
        }
    }

    pub fn scroll_down(&mut self) {
        let total_individual_metrics = self.metrics.count_available_metrics();
        // Always allow scrolling through all metrics, one at a time
        let max_offset = total_individual_metrics.saturating_sub(1);
        if self.scroll_offset < max_offset {
            self.scroll_offset += 1;
        }
    }

    pub fn reset_scroll(&mut self) {
        self.scroll_offset = 0;
    }
}
