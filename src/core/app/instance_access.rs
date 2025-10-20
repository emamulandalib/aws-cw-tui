use crate::models::{App, AppState, RdsInstance, ServiceInstance, SqsQueue};
use log::warn;

impl App {
    /// Get reference to current instances list
    pub fn get_current_instances(&self) -> &Vec<ServiceInstance> {
        &self.instances
    }

    /// Get the currently selected instance with safe bounds checking
    /// Uses saved selected_instance for metrics views, list_state for list navigation
    pub fn get_selected_instance(&self) -> Option<&ServiceInstance> {
        // For metrics views (summary and details), use the saved selected_instance
        if matches!(
            self.state,
            AppState::MetricsSummary | AppState::InstanceDetails
        ) {
            if let Some(saved_index) = self.selected_instance {
                if saved_index < self.instances.len() {
                    return Some(&self.instances[saved_index]);
                } else {
                    warn!(
                        "GET_SELECTED_INSTANCE: Saved selected_instance index {} is out of bounds for {} instances",
                        saved_index, self.instances.len()
                    );
                }
            }
        }

        // For list navigation or fallback, use current list_state selection
        if let Some(index) = self.list_state.selected() {
            self.instances.get(index)
        } else {
            None
        }
    }

    /// Get the ID of the currently selected instance
    pub fn get_selected_instance_id(&self) -> Option<String> {
        self.get_selected_instance()
            .map(|instance| instance.as_aws_instance().id().to_string())
    }

    /// Safely get the selected RDS instance with bounds checking
    pub fn get_selected_rds_instance(&self) -> Option<&RdsInstance> {
        if let Some(instance) = self.get_selected_instance() {
            match instance {
                ServiceInstance::Rds(rds) => Some(rds),
                _ => None,
            }
        } else {
            None
        }
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

    /// Check if there are instances available for the current service
    pub fn has_instances(&self) -> bool {
        !self.instances.is_empty()
    }

    /// Get the number of available instances
    pub fn instance_count(&self) -> usize {
        self.instances.len()
    }
}
