//! Service layer abstractions and implementations
//!
//! This module provides the enhanced service architecture with:
//! - Standardized service interfaces via AwsService trait
//! - Dependency injection through ServiceContainer
//! - Mock implementations for testing
//! - Retry logic and circuit breaker patterns
//! - Standardized error handling

pub mod abstractions;
pub mod aws;

// Re-export core abstractions
pub use abstractions::{
    retry_with_backoff, AwsService, CircuitBreaker, CircuitBreakerConfig, CircuitBreakerState,
    RetryConfig, ServiceConfig, ServiceContainer, ServiceError, ServiceResult,
};

// Re-export AWS service implementations
pub use aws::{
    CloudWatchServiceImpl, EnhancedRdsService, EnhancedSqsService, MockRdsService, MockSqsService,
    RdsServiceImpl, SqsServiceImpl,
};
