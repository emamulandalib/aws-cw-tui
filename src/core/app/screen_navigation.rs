use crate::models::{App, AppState, FocusedPanel};
use log::{debug, info};

// Include debug logging macros
use crate::{log_focus_change, log_navigation, log_state_transition};

impl App {
    /// Enter the metrics summary screen from instance list
    pub fn enter_metrics_summary(&mut self) {
        debug!("SCREEN_NAVIGATION: enter_metrics_summary called");

        if let Some(i) = self.list_state.selected() {
            let instance_id = self.get_selected_instance_id();
            info!(
                "SCREEN_NAVIGATION: Entering MetricsSummary for instance index: {}, ID: {:?}",
                i, instance_id
            );

            self.selected_instance = Some(i);
            let old_state = format!("{:?}", self.state);
            self.state = AppState::MetricsSummary;
            log_state_transition!(
                &old_state,
                "MetricsSummary",
                format!("Instance selected: {:?}", instance_id)
            );

            let old_panel = self.focused_panel.clone();
            self.focused_panel = FocusedPanel::SparklineGrid;
            log_focus_change!(old_panel, self.focused_panel);

            // Let initialize_sparkline_grid handle proper state synchronization
            self.initialize_sparkline_grid();
            debug!(
                "SCREEN_NAVIGATION: Initialized sparkline grid with proper state synchronization"
            );
        } else {
            debug!("SCREEN_NAVIGATION: Cannot enter MetricsSummary - no instance selected");
        }
    }

    /// Go back to metrics summary from instance details
    pub fn back_to_metrics_summary(&mut self) {
        debug!("SCREEN_NAVIGATION: back_to_metrics_summary called");

        let old_state = format!("{:?}", self.state);
        self.state = AppState::MetricsSummary;
        log_state_transition!(
            &old_state,
            "MetricsSummary",
            "Returning from InstanceDetails"
        );

        let old_panel = self.focused_panel.clone();
        self.focused_panel = self.saved_focused_panel.clone();
        log_focus_change!(old_panel, self.focused_panel);

        // Persist the current scrolled index instead of restoring saved
        self.saved_sparkline_grid_selected_index = self.sparkline_grid_selected_index;
        debug!(
            "SCREEN_NAVIGATION: Persisted current sparkline grid index: {}",
            self.sparkline_grid_selected_index
        );

        self.update_selected_metric();
        debug!("SCREEN_NAVIGATION: Updated selected metric");

        info!("SCREEN_NAVIGATION: Successfully returned to MetricsSummary");
    }

    /// Enter the instance details screen
    pub fn enter_instance_details(&mut self) {
        debug!("SCREEN_NAVIGATION: enter_instance_details called");

        // Use the already saved selected_instance from metrics summary
        // Don't override it with current list_state as this causes sync issues
        let old_state = format!("{:?}", self.state);
        self.state = AppState::InstanceDetails;
        log_state_transition!(&old_state, "InstanceDetails", "Viewing detailed metric");

        self.saved_focused_panel = self.focused_panel.clone();
        // Save the current index from the source of truth used by navigation methods
        let current_selected_index = self.sparkline_grid_list_state.selected().unwrap_or(0);
        self.saved_sparkline_grid_selected_index = current_selected_index;
        debug!(
            "SCREEN_NAVIGATION: Saved panel state - Panel: {:?}, Grid Index: {}",
            self.saved_focused_panel, self.saved_sparkline_grid_selected_index
        );

        if let Some(ref metrics) = self.dynamic_metrics {
            if let Some(metric_name) = metrics
                .get_available_metric_names()
                .get(current_selected_index)
            {
                info!(
                    "SCREEN_NAVIGATION: Entered InstanceDetails for metric: {}",
                    metric_name
                );
            }
        }

        info!("SCREEN_NAVIGATION: Successfully entered InstanceDetails");
    }

    /// Go back to the instance list from metrics screens
    pub fn back_to_list(&mut self) {
        debug!("SCREEN_NAVIGATION: back_to_list called");

        let old_state = format!("{:?}", self.state);
        self.state = AppState::InstanceList;
        log_state_transition!(&old_state, "InstanceList", "Returning to instance list");

        // Restore the list selection if we had a selected instance
        if let Some(instance_index) = self.selected_instance {
            debug!(
                "SCREEN_NAVIGATION: Restoring instance selection to index: {}",
                instance_index
            );
            // Ensure the index is still valid
            if instance_index < self.instances.len() {
                self.list_state.select(Some(instance_index));
                info!(
                    "SCREEN_NAVIGATION: Restored selection to valid index: {}",
                    instance_index
                );
            } else if !self.instances.is_empty() {
                // If the saved index is invalid, select the first item
                self.list_state.select(Some(0));
                debug!(
                    "SCREEN_NAVIGATION: Saved index {} invalid, selected first item",
                    instance_index
                );
            } else {
                self.list_state.select(None);
                debug!("SCREEN_NAVIGATION: No instances available, cleared selection");
            }
        } else {
            debug!("SCREEN_NAVIGATION: No saved instance selection to restore");
        }

        self.selected_instance = None;
        debug!("SCREEN_NAVIGATION: Cleared selected_instance");

        self.reset_scroll();
        debug!("SCREEN_NAVIGATION: Reset scroll position");

        info!("SCREEN_NAVIGATION: Successfully returned to InstanceList");
    }

    /// Get current application state
    pub fn get_current_state(&self) -> &AppState {
        &self.state
    }

    /// Check if we're in a specific state
    pub fn is_in_state(&self, state: AppState) -> bool {
        self.state == state
    }

    /// Check if we can go back (not on the first screen)
    pub fn can_go_back(&self) -> bool {
        !matches!(self.state, AppState::ServiceList)
    }

    /// Perform context-aware back navigation
    pub fn go_back(&mut self) {
        debug!(
            "SCREEN_NAVIGATION: go_back called from state: {:?}",
            self.state
        );

        match self.state {
            AppState::InstanceDetails => {
                log_navigation!("InstanceDetails", "MetricsSummary", "go_back");
                self.back_to_metrics_summary();
            }
            AppState::MetricsSummary => {
                log_navigation!("MetricsSummary", "InstanceList", "go_back");
                self.back_to_list();
            }
            AppState::InstanceList => {
                log_navigation!("InstanceList", "ServiceList", "go_back");
                self.back_to_service_list();
            }
            AppState::ServiceList => {
                debug!("SCREEN_NAVIGATION: Already at root ServiceList, cannot go back further");
            }
        }
    }
}
