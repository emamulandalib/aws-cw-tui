use crate::models::{App, AwsService};
use crate::ui::charts::chart_utils::{calculate_dynamic_metric_max, get_dynamic_metric_color};
use crate::ui::charts::formatter::{
    format_dynamic_metric_value, get_rds_metric_display_info, get_sqs_metric_display_info,
};
use crate::ui::charts::types::MetricTuple;
use crate::utils::validation::validate_metric_data;
use std::time::SystemTime;

/// Collect all available metrics from the app state
pub fn collect_available_metrics_unified(app: &App) -> Vec<MetricTuple<'_>> {
    let mut individual_metrics = vec![];

    // Check if dynamic metrics are available, otherwise fall back to legacy
    if let Some(ref dynamic_metrics) = app.dynamic_metrics {
        if !dynamic_metrics.is_empty() {
            log::info!(
                "Using dynamic metrics system with {} metrics",
                dynamic_metrics.len()
            );

            // Use dynamic metrics system
            for metric_data in &dynamic_metrics.metrics {
                // Validate metric data before adding
                if let Err(error) = validate_metric_data(
                    &metric_data.metric_name,
                    &metric_data.history,
                    &metric_data.timestamps,
                ) {
                    log::warn!(
                        "Skipping dynamic metric {}: {}",
                        metric_data.metric_name,
                        error
                    );
                    continue;
                }

                if !metric_data.history.is_empty() && !metric_data.timestamps.is_empty() {
                    let display_name = metric_data.display_name.as_str(); // Use AWS SDK metric name directly
                    let formatted_value = format_dynamic_metric_value(metric_data);
                    let color = get_dynamic_metric_color(&metric_data.metric_name);
                    let max_val = calculate_dynamic_metric_max(metric_data);

                    individual_metrics.push((
                        display_name,
                        formatted_value,
                        &metric_data.history,
                        color,
                        max_val,
                        true,
                    ));
                } else {
                    log::debug!(
                        "Skipping dynamic metric {} with empty data",
                        metric_data.metric_name
                    );
                }
            }

            if individual_metrics.is_empty() {
                log::warn!(
                    "Dynamic metrics available but none passed validation - falling back to legacy"
                );
            } else {
                // Sort metrics alphabetically by display name
                individual_metrics.sort_by(|a, b| a.0.cmp(b.0));
                log::info!(
                    "Successfully loaded {} dynamic metrics",
                    individual_metrics.len()
                );
                return individual_metrics;
            }
        } else {
            log::warn!("Dynamic metrics available but empty - falling back to legacy");
        }
    } else {
        log::info!("No dynamic metrics available - using legacy system");
    }

    // Fall back to hardcoded metrics if dynamic metrics not available or empty
    log::info!("Using legacy metrics system");
    collect_legacy_metrics(app)
}

/// Collect metrics from the legacy system
fn collect_legacy_metrics(app: &App) -> Vec<MetricTuple<'_>> {
    let mut individual_metrics = vec![];

    // Get available metrics based on service type
    let available_metrics = match app.selected_service.as_ref().unwrap_or(&AwsService::Rds) {
        AwsService::Rds => {
            let metrics = &app.metrics;
            if metrics.timestamps.is_empty() {
                log::warn!("Legacy RDS metrics have no timestamps");
                Vec::new()
            } else {
                // Only return metrics that have actual data
                let available = metrics.get_available_metrics_with_data();
                log::info!("Found {} legacy RDS metrics with data", available.len());
                available
            }
        }
        AwsService::Sqs => {
            let metrics = &app.sqs_metrics;
            if metrics.timestamps.is_empty() {
                log::warn!("Legacy SQS metrics have no timestamps");
                Vec::new()
            } else {
                let available = metrics.get_available_metrics();
                log::info!("Found {} legacy SQS metrics with data", available.len());
                available
            }
        }
    };

    // Convert available metrics to display format
    for metric_type in available_metrics {
        let (name, value, history, color, max_val) =
            match app.selected_service.as_ref().unwrap_or(&AwsService::Rds) {
                AwsService::Rds => get_rds_metric_display_info(&metric_type, &app.metrics),
                AwsService::Sqs => get_sqs_metric_display_info(&metric_type, &app.sqs_metrics),
            };

        individual_metrics.push((name, value, history, color, max_val, false));
    }

    log::info!(
        "Successfully loaded {} legacy metrics",
        individual_metrics.len()
    );
    individual_metrics
}

/// Get timestamps for the current app state
pub fn get_timestamps_for_app(app: &App) -> Option<&[SystemTime]> {
    if let Some(ref dynamic_metrics) = app.dynamic_metrics {
        // Use first dynamic metric's timestamps for rendering
        dynamic_metrics
            .metrics
            .iter()
            .find(|m| !m.timestamps.is_empty())
            .map(|m| m.timestamps.as_slice())
    } else {
        // Use legacy timestamps
        match app.selected_service.as_ref().unwrap_or(&AwsService::Rds) {
            AwsService::Rds => {
                if app.metrics.timestamps.is_empty() {
                    None
                } else {
                    Some(&app.metrics.timestamps)
                }
            }
            AwsService::Sqs => {
                if app.sqs_metrics.timestamps.is_empty() {
                    None
                } else {
                    Some(&app.sqs_metrics.timestamps)
                }
            }
        }
    }
}

/// Check if any metrics data is available
pub fn has_metrics_data(app: &App) -> bool {
    if let Some(ref dynamic_metrics) = app.dynamic_metrics {
        !dynamic_metrics.is_empty()
            && dynamic_metrics
                .metrics
                .iter()
                .any(|m| !m.history.is_empty())
    } else {
        match app.selected_service.as_ref().unwrap_or(&AwsService::Rds) {
            AwsService::Rds => !app.metrics.get_available_metrics_with_data().is_empty(),
            AwsService::Sqs => !app.sqs_metrics.get_available_metrics().is_empty(),
        }
    }
}

/// Get the count of available metrics
pub fn get_metrics_count(app: &App) -> usize {
    if let Some(ref dynamic_metrics) = app.dynamic_metrics {
        dynamic_metrics.metrics.len()
    } else {
        match app.selected_service.as_ref().unwrap_or(&AwsService::Rds) {
            AwsService::Rds => app.metrics.get_available_metrics_with_data().len(),
            AwsService::Sqs => app.sqs_metrics.get_available_metrics().len(),
        }
    }
}

/// Get the service name for display purposes
pub fn get_service_name(app: &App) -> &'static str {
    match app.selected_service.as_ref().unwrap_or(&AwsService::Rds) {
        AwsService::Rds => "RDS",
        AwsService::Sqs => "SQS",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{MetricData, SqsMetricData};
    use std::time::SystemTime;

    #[test]
    fn test_has_metrics_data_empty() {
        let app = App {
            metrics: MetricData::default(),
            sqs_metrics: SqsMetricData::default(),
            dynamic_metrics: None,
            selected_service: Some(AwsService::Rds),
            ..Default::default()
        };

        assert!(!has_metrics_data(&app));
    }

    #[test]
    fn test_get_service_name() {
        let mut app = App::default();
        app.selected_service = Some(AwsService::Rds);
        assert_eq!(get_service_name(&app), "RDS");

        app.selected_service = Some(AwsService::Sqs);
        assert_eq!(get_service_name(&app), "SQS");
    }

    #[test]
    fn test_get_metrics_count_empty() {
        let app = App {
            metrics: MetricData::default(),
            sqs_metrics: SqsMetricData::default(),
            dynamic_metrics: None,
            selected_service: Some(AwsService::Rds),
            ..Default::default()
        };

        assert_eq!(get_metrics_count(&app), 0);
    }
}
