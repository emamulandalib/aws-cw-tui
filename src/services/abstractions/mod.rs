//! Service abstractions and core traits
//!
//! This module defines the core service architecture including:
//! - AwsService trait for standardized service interfaces
//! - ServiceContainer for dependency injection
//! - Error handling and configuration types
//! - Retry and circuit breaker patterns

pub mod retry;

use crate::aws::cloudwatch_service::TimeRange;
use crate::models::{AwsService as AwsServiceType, DynamicMetrics, ServiceInstance};
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

// Re-export retry utilities
pub use retry::{retry_with_backoff, CircuitBreaker, CircuitBreakerState};

/// Result type for service operations
pub type ServiceResult<T> = Result<T, ServiceError>;

/// Standardized error types for all AWS services
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("AWS API error: {message}")]
    AwsApi {
        message: String,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Configuration error: {message}")]
    Configuration { message: String },

    #[error("Network error: {message}")]
    Network { message: String },

    #[error("Authentication error: {message}")]
    Authentication { message: String },

    #[error("Service unavailable: {message}")]
    ServiceUnavailable { message: String },

    #[error("Rate limit exceeded: {message}")]
    RateLimit { message: String },

    #[error("Circuit breaker open for service: {service}")]
    CircuitBreakerOpen { service: String },

    #[error("Timeout error: {message}")]
    Timeout { message: String },

    #[error("Validation error: {message}")]
    Validation { message: String },
}

impl ServiceError {
    /// Check if the error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            ServiceError::Network { .. }
                | ServiceError::ServiceUnavailable { .. }
                | ServiceError::RateLimit { .. }
                | ServiceError::Timeout { .. }
        )
    }

    /// Get the error category for metrics and logging
    pub fn category(&self) -> &'static str {
        match self {
            ServiceError::AwsApi { .. } => "aws_api",
            ServiceError::Configuration { .. } => "configuration",
            ServiceError::Network { .. } => "network",
            ServiceError::Authentication { .. } => "authentication",
            ServiceError::ServiceUnavailable { .. } => "service_unavailable",
            ServiceError::RateLimit { .. } => "rate_limit",
            ServiceError::CircuitBreakerOpen { .. } => "circuit_breaker",
            ServiceError::Timeout { .. } => "timeout",
            ServiceError::Validation { .. } => "validation",
        }
    }
}

/// Configuration for retry behavior
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
    pub jitter: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
            jitter: true,
        }
    }
}

/// Configuration for circuit breaker pattern
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    pub failure_threshold: u32,
    pub recovery_timeout: Duration,
    pub half_open_max_calls: u32,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            recovery_timeout: Duration::from_secs(60),
            half_open_max_calls: 3,
        }
    }
}

/// Service-specific configuration
#[derive(Debug, Clone)]
pub struct ServiceConfig {
    pub retry: RetryConfig,
    pub circuit_breaker: CircuitBreakerConfig,
    pub timeout: Duration,
    pub custom_settings: HashMap<String, String>,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            retry: RetryConfig::default(),
            circuit_breaker: CircuitBreakerConfig::default(),
            timeout: Duration::from_secs(30),
            custom_settings: HashMap::new(),
        }
    }
}

/// Core trait for all AWS service implementations
///
/// This trait provides a standardized interface for all AWS services,
/// enabling consistent error handling, retry logic, and testing.
#[async_trait]
pub trait AwsService: Send + Sync {
    /// Get the service type this implementation handles
    fn service_type(&self) -> AwsServiceType;

    /// Get the service name for logging and metrics
    fn service_name(&self) -> &'static str;

    /// List all instances for this service
    async fn list_instances(&self) -> ServiceResult<Vec<ServiceInstance>>;

    /// Get metrics for a specific instance
    async fn get_metrics(
        &self,
        instance_id: &str,
        time_range: TimeRange,
    ) -> ServiceResult<DynamicMetrics>;

    /// Validate service configuration and connectivity
    async fn health_check(&self) -> ServiceResult<()>;

    /// Get service-specific configuration
    fn get_config(&self) -> &ServiceConfig;

    /// Update service configuration
    fn update_config(&mut self, config: ServiceConfig);
}

/// Dependency injection container for services
///
/// Manages service instances and their dependencies, providing
/// a clean way to inject services into the application.
pub struct ServiceContainer {
    services: HashMap<AwsServiceType, Arc<dyn AwsService>>,
    config: Arc<crate::config::AppConfig>,
}

impl ServiceContainer {
    /// Create a new service container with the given configuration
    pub fn new(config: Arc<crate::config::AppConfig>) -> Self {
        Self {
            services: HashMap::new(),
            config,
        }
    }

    /// Register a service implementation
    pub fn register<T>(&mut self, service: T)
    where
        T: AwsService + 'static,
    {
        let service_type = service.service_type();
        self.services.insert(service_type, Arc::new(service));
    }

    /// Get a service by type
    pub fn get_service(&self, service_type: &AwsServiceType) -> Option<Arc<dyn AwsService>> {
        self.services.get(service_type).cloned()
    }

    /// Get all registered services
    pub fn get_all_services(&self) -> Vec<Arc<dyn AwsService>> {
        self.services.values().cloned().collect()
    }

    /// Get the application configuration
    pub fn get_config(&self) -> Arc<crate::config::AppConfig> {
        self.config.clone()
    }

    /// Check health of all registered services
    pub async fn health_check_all(&self) -> HashMap<AwsServiceType, ServiceResult<()>> {
        let mut results = HashMap::new();

        for (service_type, service) in &self.services {
            let result = service.health_check().await;
            results.insert(service_type.clone(), result);
        }

        results
    }
}

impl std::fmt::Debug for ServiceContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ServiceContainer")
            .field("services", &self.services.keys().collect::<Vec<_>>())
            .finish()
    }
}
