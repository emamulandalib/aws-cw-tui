//! Factory for creating and managing metric service providers

use crate::aws::metrics::providers::{MetricProvider, RdsMetricProvider};
use crate::models::AwsService;
use anyhow::{anyhow, Result};
use std::collections::HashMap;

/// Factory for creating metric service providers
///
/// This factory manages the registration and creation of service-specific
/// metric providers, allowing for extensible support of different AWS services.
pub struct MetricServiceFactory {
    providers: HashMap<AwsService, Box<dyn MetricProvider>>,
}

impl MetricServiceFactory {
    /// Create a new factory with default providers registered
    pub fn new() -> Self {
        let mut factory = Self {
            providers: HashMap::new(),
        };

        // Register default providers
        factory.register_provider(AwsService::Rds, Box::new(RdsMetricProvider::new()));

        factory
    }

    /// Register a new provider for a service type
    pub fn register_provider(
        &mut self,
        service_type: AwsService,
        provider: Box<dyn MetricProvider>,
    ) {
        self.providers.insert(service_type, provider);
    }

    /// Get a provider for the specified service type
    pub fn get_provider(&self, service_type: &AwsService) -> Result<&dyn MetricProvider> {
        self.providers
            .get(service_type)
            .map(|provider| provider.as_ref())
            .ok_or_else(|| {
                anyhow!(
                    "No provider registered for service type: {:?}",
                    service_type
                )
            })
    }

    /// Check if a provider is registered for the given service type
    pub fn has_provider(&self, service_type: &AwsService) -> bool {
        self.providers.contains_key(service_type)
    }

    /// Get all registered service types
    pub fn get_supported_services(&self) -> Vec<AwsService> {
        self.providers.keys().cloned().collect()
    }
}

impl Default for MetricServiceFactory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_creation() {
        let factory = MetricServiceFactory::new();
        assert!(factory.has_provider(&AwsService::Rds));
    }

    #[test]
    fn test_get_provider() {
        let factory = MetricServiceFactory::new();
        let provider = factory.get_provider(&AwsService::Rds);
        assert!(provider.is_ok());
    }
}
