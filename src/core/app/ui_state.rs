use crate::models::{App, AppState, FocusedPanel};
use log::{debug, info};

// Include debug logging macros
use crate::{log_metric_operation, log_navigation};

impl App {
    /// Handle scroll up based on current state and focused panel
    pub fn scroll_up(&mut self) {
        debug!(
            "UI_STATE: scroll_up called in state: {:?}, panel: {:?}",
            self.state, self.focused_panel
        );

        match self.state {
            AppState::MetricsSummary => match self.focused_panel {
                FocusedPanel::Timezone => {
                    debug!("UI_STATE: Scrolling up in timezone panel");
                    self.timezone_scroll_up();
                }
                FocusedPanel::Period => {
                    debug!("UI_STATE: Scrolling up in period panel");
                    self.period_scroll_up();
                }
                FocusedPanel::TimeRanges => {
                    debug!("UI_STATE: Scrolling up in time ranges panel");
                    self.time_range_scroll_up();
                }
                FocusedPanel::SparklineGrid => {
                    debug!("UI_STATE: Scrolling up in sparkline grid");
                    let old_index = self.sparkline_grid_selected_index;
                    self.sparkline_grid_scroll_up();
                    let new_index = self.sparkline_grid_selected_index;
                    if old_index != new_index {
                        self.log_metric_selection_change(old_index, new_index, "scroll up");
                    }
                }
            },
            AppState::InstanceDetails => {
                debug!("UI_STATE: Scrolling up in instance details (sequential)");
                let old_index = self.sparkline_grid_selected_index;
                self.sequential_scroll_up();
                let new_index = self.sparkline_grid_selected_index;
                if old_index != new_index {
                    self.log_metric_selection_change(old_index, new_index, "detail scroll up");
                }
            }
            _ => {
                debug!(
                    "UI_STATE: Scroll up - no action for state: {:?}",
                    self.state
                );
            }
        }
    }

    /// Handle scroll down based on current state and focused panel
    pub fn scroll_down(&mut self) {
        debug!(
            "UI_STATE: scroll_down called in state: {:?}, panel: {:?}",
            self.state, self.focused_panel
        );

        match self.state {
            AppState::MetricsSummary => match self.focused_panel {
                FocusedPanel::Timezone => {
                    debug!("UI_STATE: Scrolling down in timezone panel");
                    self.timezone_scroll_down();
                }
                FocusedPanel::Period => {
                    debug!("UI_STATE: Scrolling down in period panel");
                    self.period_scroll_down();
                }
                FocusedPanel::TimeRanges => {
                    debug!("UI_STATE: Scrolling down in time ranges panel");
                    self.time_range_scroll_down();
                }
                FocusedPanel::SparklineGrid => {
                    debug!("UI_STATE: Scrolling down in sparkline grid");
                    let old_index = self.sparkline_grid_selected_index;
                    self.sparkline_grid_scroll_down();
                    let new_index = self.sparkline_grid_selected_index;
                    if old_index != new_index {
                        self.log_metric_selection_change(old_index, new_index, "scroll down");
                    }
                }
            },
            AppState::InstanceDetails => {
                debug!("UI_STATE: Scrolling down in instance details (sequential)");
                let old_index = self.sparkline_grid_selected_index;
                self.sequential_scroll_down();
                let new_index = self.sparkline_grid_selected_index;
                if old_index != new_index {
                    self.log_metric_selection_change(old_index, new_index, "detail scroll down");
                }
            }
            _ => {
                debug!(
                    "UI_STATE: Scroll down - no action for state: {:?}",
                    self.state
                );
            }
        }
    }

    /// Handle scroll left based on current state and focused panel
    pub fn scroll_left(&mut self) {
        debug!(
            "UI_STATE: scroll_left called in state: {:?}, panel: {:?}",
            self.state, self.focused_panel
        );

        match self.state {
            AppState::MetricsSummary => match self.focused_panel {
                FocusedPanel::Timezone => {
                    debug!("UI_STATE: Scrolling left in timezone panel");
                    self.timezone_scroll_up(); // Use up for left in timezone
                }
                FocusedPanel::Period => {
                    debug!("UI_STATE: Scrolling left in period panel");
                    self.period_scroll_up(); // Use up for left in period
                }
                FocusedPanel::TimeRanges => {
                    debug!("UI_STATE: Scrolling left in time ranges panel");
                    self.time_range_scroll_left();
                }
                FocusedPanel::SparklineGrid => {
                    debug!("UI_STATE: Scrolling left in sparkline grid");
                    let old_index = self.sparkline_grid_selected_index;
                    self.sparkline_grid_scroll_left();
                    let new_index = self.sparkline_grid_selected_index;
                    if old_index != new_index {
                        self.log_metric_selection_change(old_index, new_index, "scroll left");
                    }
                }
            },
            _ => {
                debug!(
                    "UI_STATE: Scroll left - no action for state: {:?}",
                    self.state
                );
            }
        }
    }

    /// Handle scroll right based on current state and focused panel
    pub fn scroll_right(&mut self) {
        debug!(
            "UI_STATE: scroll_right called in state: {:?}, panel: {:?}",
            self.state, self.focused_panel
        );

        match self.state {
            AppState::MetricsSummary => match self.focused_panel {
                FocusedPanel::Timezone => {
                    debug!("UI_STATE: Scrolling right in timezone panel");
                    self.timezone_scroll_down(); // Use down for right in timezone
                }
                FocusedPanel::Period => {
                    debug!("UI_STATE: Scrolling right in period panel");
                    self.period_scroll_down(); // Use down for right in period
                }
                FocusedPanel::TimeRanges => {
                    debug!("UI_STATE: Scrolling right in time ranges panel");
                    self.time_range_scroll_right();
                }
                FocusedPanel::SparklineGrid => {
                    debug!("UI_STATE: Scrolling right in sparkline grid");
                    let old_index = self.sparkline_grid_selected_index;
                    self.sparkline_grid_scroll_right();
                    let new_index = self.sparkline_grid_selected_index;
                    if old_index != new_index {
                        self.log_metric_selection_change(old_index, new_index, "scroll right");
                    }
                }
            },
            _ => {
                debug!(
                    "UI_STATE: Scroll right - no action for state: {:?}",
                    self.state
                );
            }
        }
    }

    /// Reset all scroll positions and UI state
    pub fn reset_scroll(&mut self) {
        info!("UI_STATE: Resetting all scroll positions and UI state");

        let old_panel = self.focused_panel.clone();
        let old_index = self.sparkline_grid_selected_index;

        self.focused_panel = FocusedPanel::Timezone;
        self.saved_focused_panel = FocusedPanel::Timezone;
        self.sparkline_grid_selected_index = 0;
        self.saved_sparkline_grid_selected_index = 0;
        self.sparkline_grid_list_state = ratatui::widgets::ListState::default();
        self.initialize_sparkline_grid();

        debug!(
            "UI_STATE: Reset complete - Panel: {:?} -> {:?}, Index: {} -> {}",
            old_panel, self.focused_panel, old_index, self.sparkline_grid_selected_index
        );
    }

    /// Switch to the next panel in focus rotation
    pub fn switch_panel(&mut self) {
        let old_panel = self.focused_panel.clone();

        self.focused_panel = match self.focused_panel {
            FocusedPanel::Timezone => FocusedPanel::Period,
            FocusedPanel::Period => FocusedPanel::TimeRanges,
            FocusedPanel::TimeRanges => FocusedPanel::SparklineGrid,
            FocusedPanel::SparklineGrid => FocusedPanel::Timezone,
        };

        log_navigation!(
            "Panel switch",
            format!("{:?} -> {:?}", old_panel, self.focused_panel),
            "Tab key"
        );
        debug!(
            "UI_STATE: Panel switched from {:?} to {:?}",
            old_panel, self.focused_panel
        );
    }

    /// Get currently focused panel
    pub fn get_focused_panel(&self) -> &FocusedPanel {
        &self.focused_panel
    }

    /// Navigate up in the sparkline grid (grid-aware navigation)
    pub fn sparkline_grid_scroll_up(&mut self) {
        let available_metrics = self.get_available_metrics();
        if available_metrics.is_empty() {
            debug!("UI_STATE: Cannot scroll up - no metrics available");
            return;
        }

        let current_index = self.sparkline_grid_list_state.selected().unwrap_or(0);
        let metrics_per_row = 2;

        debug!(
            "UI_STATE: Grid scroll up - Current index: {}, Metrics per row: {}",
            current_index, metrics_per_row
        );

        if current_index >= metrics_per_row {
            let new_index = current_index - metrics_per_row;
            self.sparkline_grid_list_state.select(Some(new_index));
            self.sparkline_grid_selected_index = new_index;
            debug!(
                "UI_STATE: Grid scrolled up from {} to {}",
                current_index, new_index
            );

            // Update selected metric name
            self.update_selected_metric();
        } else {
            debug!("UI_STATE: Grid scroll up - already at top");
        }
    }

    /// Navigate down in the sparkline grid (grid-aware navigation)
    pub fn sparkline_grid_scroll_down(&mut self) {
        let available_metrics = self.get_available_metrics();
        if available_metrics.is_empty() {
            debug!("UI_STATE: Cannot scroll down - no metrics available");
            return;
        }

        let current_index = self.sparkline_grid_list_state.selected().unwrap_or(0);
        let metrics_per_row = 2;
        let total_metrics = available_metrics.len();

        debug!(
            "UI_STATE: Grid scroll down - Current index: {}, Total metrics: {}",
            current_index, total_metrics
        );

        let new_index = current_index + metrics_per_row;
        if new_index < total_metrics {
            self.sparkline_grid_list_state.select(Some(new_index));
            self.sparkline_grid_selected_index = new_index;
            debug!(
                "UI_STATE: Grid scrolled down from {} to {}",
                current_index, new_index
            );

            // Update selected metric name
            self.update_selected_metric();
        } else {
            debug!("UI_STATE: Grid scroll down - already at bottom");
        }
    }

    /// Navigate up sequentially through all metrics (for instance details)
    pub fn sequential_scroll_up(&mut self) {
        let available_metrics = self.get_available_metrics();
        if available_metrics.is_empty() {
            debug!("UI_STATE: Cannot scroll up - no metrics available");
            return;
        }

        let current_index = self.sparkline_grid_list_state.selected().unwrap_or(0);
        debug!(
            "UI_STATE: Sequential scroll up - Current index: {}",
            current_index
        );

        if current_index > 0 {
            let new_index = current_index - 1;
            self.sparkline_grid_list_state.select(Some(new_index));
            self.sparkline_grid_selected_index = new_index;
            debug!(
                "UI_STATE: Sequential scrolled up from {} to {}",
                current_index, new_index
            );

            // Update selected metric name
            self.update_selected_metric();
        } else {
            debug!("UI_STATE: Sequential scroll up - already at top");
        }
    }

    /// Navigate down sequentially through all metrics (for instance details)
    pub fn sequential_scroll_down(&mut self) {
        let available_metrics = self.get_available_metrics();
        if available_metrics.is_empty() {
            debug!("UI_STATE: Cannot scroll down - no metrics available");
            return;
        }

        let current_index = self.sparkline_grid_list_state.selected().unwrap_or(0);
        let total_metrics = available_metrics.len();

        debug!(
            "UI_STATE: Sequential scroll down - Current index: {}, Total metrics: {}",
            current_index, total_metrics
        );

        if current_index < total_metrics - 1 {
            let new_index = current_index + 1;
            self.sparkline_grid_list_state.select(Some(new_index));
            self.sparkline_grid_selected_index = new_index;
            debug!(
                "UI_STATE: Sequential scrolled down from {} to {}",
                current_index, new_index
            );

            // Update selected metric name
            self.update_selected_metric();
        } else {
            debug!("UI_STATE: Sequential scroll down - already at bottom");
        }
    }

    /// Navigate left in the sparkline grid
    pub fn sparkline_grid_scroll_left(&mut self) {
        let available_metrics = self.get_available_metrics();
        if available_metrics.is_empty() {
            debug!("UI_STATE: Cannot scroll left - no metrics available");
            return;
        }

        let current_index = self.sparkline_grid_list_state.selected().unwrap_or(0);

        debug!(
            "UI_STATE: Grid scroll left - Current index: {}",
            current_index
        );

        if current_index > 0 && current_index % 2 == 1 {
            let new_index = current_index - 1;
            self.sparkline_grid_list_state.select(Some(new_index));
            self.sparkline_grid_selected_index = new_index;
            debug!(
                "UI_STATE: Grid scrolled left from {} to {}",
                current_index, new_index
            );

            // Update selected metric name and log the change
            self.update_selected_metric();
            self.log_metric_selection_change(current_index, new_index, "scroll left");
        } else {
            debug!(
                "UI_STATE: Grid scroll left - cannot move left from index {}",
                current_index
            );
        }
    }

    /// Navigate right in the sparkline grid
    pub fn sparkline_grid_scroll_right(&mut self) {
        let available_metrics = self.get_available_metrics();
        if available_metrics.is_empty() {
            debug!("UI_STATE: Cannot scroll right - no metrics available");
            return;
        }

        let current_index = self.sparkline_grid_list_state.selected().unwrap_or(0);
        let total_metrics = available_metrics.len();

        debug!(
            "UI_STATE: Grid scroll right - Current index: {}, Total metrics: {}",
            current_index, total_metrics
        );

        if current_index % 2 == 0 && current_index + 1 < total_metrics {
            let new_index = current_index + 1;
            self.sparkline_grid_list_state.select(Some(new_index));
            self.sparkline_grid_selected_index = new_index;
            debug!(
                "UI_STATE: Grid scrolled right from {} to {}",
                current_index, new_index
            );

            // Update selected metric name and log the change
            self.update_selected_metric();
            self.log_metric_selection_change(current_index, new_index, "scroll right");
        } else {
            debug!(
                "UI_STATE: Grid scroll right - cannot move right from index {}",
                current_index
            );
        }
    }

    /// Set focus to a specific panel
    pub fn set_focused_panel(&mut self, panel: FocusedPanel) {
        let old_panel = self.focused_panel.clone();
        self.focused_panel = panel;
        debug!(
            "UI_STATE: Set focused panel from {:?} to {:?}",
            old_panel, self.focused_panel
        );
    }

    /// Check if a specific panel is focused
    pub fn is_panel_focused(&self, panel: &FocusedPanel) -> bool {
        &self.focused_panel == panel
    }

    /// Helper function to log metric selection changes with detailed information
    fn log_metric_selection_change(&self, old_index: usize, new_index: usize, action: &str) {
        if let Some(ref dynamic_metrics) = self.dynamic_metrics {
            let available_metrics = dynamic_metrics.get_available_metric_names();

            let old_metric_name = available_metrics
                .get(old_index)
                .map(|s| s.as_str())
                .unwrap_or("unknown");
            let new_metric_name = available_metrics
                .get(new_index)
                .map(|s| s.as_str())
                .unwrap_or("unknown");

            info!(
                "UI_STATE: Metric selection changed via {} - {} ({}) -> {} ({})",
                action, old_metric_name, old_index, new_metric_name, new_index
            );

            log_metric_operation!(
                "Select metric",
                new_metric_name,
                format!(
                    "Action: {}, Position: {}/{}",
                    action,
                    new_index,
                    available_metrics.len()
                )
            );

            // Log additional context about the newly selected metric
            if let Some(metric_data) = dynamic_metrics
                .metrics
                .iter()
                .find(|m| &m.display_name == new_metric_name)
            {
                debug!("UI_STATE: Selected metric details - Unit: '{:?}', Current Value: {}, History Points: {}", 
                       metric_data.unit, metric_data.current_value, metric_data.history.len());
            }
        } else {
            debug!(
                "UI_STATE: Metric selection changed {} -> {} via {} (no dynamic metrics)",
                old_index, new_index, action
            );
        }
    }
}
