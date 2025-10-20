use crate::aws::load_rds_instances;
use crate::models::{App, AppState, AwsService, ServiceInstance};
use anyhow::Result;
use log::{debug, info, warn};

// Include debug logging macros
use crate::log_state_transition;

impl App {
    /// Select the currently highlighted service and transition to instance list
    pub fn select_service(&mut self) -> Option<&AwsService> {
        debug!("SERVICE_MANAGEMENT: select_service called");

        if let Some(index) = self.service_list_state.selected() {
            if let Some(service) = self.available_services.get(index) {
                info!(
                    "SERVICE_MANAGEMENT: Selected service at index {}: {:?}",
                    index, service
                );

                self.selected_service = Some(service.clone());
                let old_state = format!("{:?}", self.state);
                self.state = AppState::InstanceList;
                log_state_transition!(
                    &old_state,
                    "InstanceList",
                    format!("Service selected: {:?}", service)
                );

                self.list_state.select(Some(0));
                debug!("SERVICE_MANAGEMENT: Reset instance list selection to index 0");

                return Some(service);
            } else {
                warn!(
                    "SERVICE_MANAGEMENT: Selected index {} out of bounds for {} services",
                    index,
                    self.available_services.len()
                );
            }
        } else {
            warn!("SERVICE_MANAGEMENT: No service selected in list_state");
        }
        None
    }

    /// Go back to the service selection screen
    pub fn back_to_service_list(&mut self) {
        debug!("SERVICE_MANAGEMENT: back_to_service_list called");

        let old_state = format!("{:?}", self.state);
        self.state = AppState::ServiceList;
        log_state_transition!(&old_state, "ServiceList", "Returning to service selection");

        let old_service = self.selected_service.clone();
        self.selected_service = None;
        debug!(
            "SERVICE_MANAGEMENT: Cleared selected service: {:?}",
            old_service
        );

        let instance_count = self.instances.len();
        self.instances.clear();
        self.rds_instances.clear();
        self.sqs_queues.clear();
        debug!(
            "SERVICE_MANAGEMENT: Cleared {} instances and all service data",
            instance_count
        );

        self.start_loading();
        debug!("SERVICE_MANAGEMENT: Started loading state");

        info!("SERVICE_MANAGEMENT: Successfully returned to ServiceList");
    }

    /// Load instances for the specified service
    pub async fn load_service_instances(&mut self, service: &AwsService) -> Result<()> {
        info!(
            "SERVICE_MANAGEMENT: Loading service instances for: {:?}",
            service
        );
        debug!(
            "SERVICE_MANAGEMENT: Current instance count before loading: {}",
            self.instances.len()
        );

        match service {
            AwsService::Rds => {
                debug!("SERVICE_MANAGEMENT: Loading RDS instances");
                self.load_rds_instances().await
            }
            AwsService::Sqs => {
                debug!("SERVICE_MANAGEMENT: Loading SQS instances");
                self.load_sqs_instances().await
            }
        }
    }

    /// Load RDS instances specifically
    async fn load_rds_instances(&mut self) -> Result<()> {
        debug!("SERVICE_MANAGEMENT: load_rds_instances called");

        match load_rds_instances().await {
            Ok(rds_instances) => {
                info!(
                    "SERVICE_MANAGEMENT: Successfully loaded {} RDS instances",
                    rds_instances.len()
                );

                self.rds_instances = rds_instances.clone();
                self.instances = rds_instances
                    .into_iter()
                    .map(ServiceInstance::Rds)
                    .collect();

                debug!(
                    "SERVICE_MANAGEMENT: Converted RDS instances to generic ServiceInstance format"
                );

                self.finalize_instance_loading();
                Ok(())
            }
            Err(e) => {
                warn!("SERVICE_MANAGEMENT: Failed to load RDS instances: {}", e);
                self.set_error(format!("AWS Error: {e}"));
                self.clear_instances();
                Ok(())
            }
        }
    }

    /// Load SQS instances specifically
    async fn load_sqs_instances(&mut self) -> Result<()> {
        debug!("SERVICE_MANAGEMENT: load_sqs_instances called");

        match crate::aws::sqs_service::load_sqs_queues().await {
            Ok(sqs_queues) => {
                info!(
                    "SERVICE_MANAGEMENT: Successfully loaded {} SQS queues",
                    sqs_queues.len()
                );

                self.sqs_queues = sqs_queues.clone();
                self.instances = sqs_queues.into_iter().map(ServiceInstance::Sqs).collect();

                debug!(
                    "SERVICE_MANAGEMENT: Converted SQS queues to generic ServiceInstance format"
                );

                self.finalize_instance_loading();
                Ok(())
            }
            Err(e) => {
                warn!("SERVICE_MANAGEMENT: Failed to load SQS queues: {}", e);
                self.set_error(format!("AWS SQS Error: {e}"));
                self.clear_instances();
                Ok(())
            }
        }
    }

    /// Common logic to finalize instance loading
    fn finalize_instance_loading(&mut self) {
        debug!(
            "SERVICE_MANAGEMENT: finalize_instance_loading called with {} instances",
            self.instances.len()
        );

        self.clear_error();
        self.stop_loading();
        self.mark_refreshed();

        if !self.instances.is_empty() {
            let current_selection = self.list_state.selected().unwrap_or(0);
            let new_selection = if current_selection < self.instances.len() {
                current_selection
            } else {
                0
            };
            self.list_state.select(Some(new_selection));
            info!(
                "SERVICE_MANAGEMENT: Set instance selection to index: {}",
                new_selection
            );

            if let Some(instance_id) = self.get_selected_instance_id() {
                debug!("SERVICE_MANAGEMENT: Selected instance ID: {}", instance_id);
            }
        } else {
            self.list_state.select(None);
            warn!("SERVICE_MANAGEMENT: No instances loaded, cleared selection");
        }

        info!("SERVICE_MANAGEMENT: Successfully finalized instance loading");
    }

    /// Clear all instance data
    fn clear_instances(&mut self) {
        debug!("SERVICE_MANAGEMENT: clear_instances called");

        let instance_count = self.instances.len();
        let rds_count = self.rds_instances.len();
        let sqs_count = self.sqs_queues.len();

        self.instances.clear();
        self.rds_instances.clear();
        self.sqs_queues.clear();
        self.list_state.select(None);

        debug!(
            "SERVICE_MANAGEMENT: Cleared {} instances ({} RDS, {} SQS)",
            instance_count, rds_count, sqs_count
        );

        info!("SERVICE_MANAGEMENT: Instance data cleared due to error");
    }
}
