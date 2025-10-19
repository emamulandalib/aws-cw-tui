//! Retry logic and circuit breaker patterns for service calls
//!
//! Provides robust error handling with exponential backoff retry
//! and circuit breaker patterns to prevent cascading failures.

use rand::Rng;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{debug, error, warn};

use super::{CircuitBreakerConfig, RetryConfig, ServiceError, ServiceResult};

/// Retry a service operation with exponential backoff
pub async fn retry_with_backoff<F, Fut, T>(
    mut operation: F,
    config: &RetryConfig,
    operation_name: &str,
) -> ServiceResult<T>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = ServiceResult<T>>,
{
    let mut attempt = 0;
    let mut delay = config.initial_delay;

    loop {
        attempt += 1;

        debug!(
            operation = operation_name,
            attempt = attempt,
            max_attempts = config.max_attempts,
            "Attempting service operation"
        );

        match operation().await {
            Ok(result) => {
                if attempt > 1 {
                    debug!(
                        operation = operation_name,
                        attempt = attempt,
                        "Service operation succeeded after retry"
                    );
                }
                return Ok(result);
            }
            Err(error) => {
                if attempt >= config.max_attempts {
                    error!(
                        operation = operation_name,
                        attempt = attempt,
                        error = %error,
                        "Service operation failed after all retry attempts"
                    );
                    return Err(error);
                }

                if !error.is_retryable() {
                    error!(
                        operation = operation_name,
                        attempt = attempt,
                        error = %error,
                        "Service operation failed with non-retryable error"
                    );
                    return Err(error);
                }

                warn!(
                    operation = operation_name,
                    attempt = attempt,
                    error = %error,
                    delay_ms = delay.as_millis(),
                    "Service operation failed, retrying after delay"
                );

                // Apply jitter if configured
                let actual_delay = if config.jitter {
                    let jitter_range = delay.as_millis() as f64 * 0.1; // 10% jitter
                    let jitter = rand::thread_rng().gen_range(-jitter_range..=jitter_range);
                    Duration::from_millis((delay.as_millis() as f64 + jitter) as u64)
                } else {
                    delay
                };

                sleep(actual_delay).await;

                // Calculate next delay with exponential backoff
                delay = Duration::from_millis(
                    ((delay.as_millis() as f64) * config.backoff_multiplier) as u64,
                )
                .min(config.max_delay);
            }
        }
    }
}

/// Circuit breaker state
#[derive(Debug, Clone, PartialEq)]
pub enum CircuitBreakerState {
    Closed,
    Open,
    HalfOpen,
}

/// Circuit breaker for preventing cascading failures
#[derive(Debug)]
pub struct CircuitBreaker {
    config: CircuitBreakerConfig,
    state: CircuitBreakerState,
    failure_count: u32,
    last_failure_time: Option<Instant>,
    half_open_calls: u32,
}

impl CircuitBreaker {
    /// Create a new circuit breaker with the given configuration
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            config,
            state: CircuitBreakerState::Closed,
            failure_count: 0,
            last_failure_time: None,
            half_open_calls: 0,
        }
    }

    /// Execute an operation through the circuit breaker
    pub async fn execute<F, Fut, T>(&mut self, operation: F, service_name: &str) -> ServiceResult<T>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = ServiceResult<T>>,
    {
        // Check if circuit breaker should transition states
        self.update_state();

        match self.state {
            CircuitBreakerState::Open => {
                debug!(
                    service = service_name,
                    state = ?self.state,
                    "Circuit breaker is open, rejecting call"
                );
                return Err(ServiceError::CircuitBreakerOpen {
                    service: service_name.to_string(),
                });
            }
            CircuitBreakerState::HalfOpen => {
                if self.half_open_calls >= self.config.half_open_max_calls {
                    debug!(
                        service = service_name,
                        state = ?self.state,
                        half_open_calls = self.half_open_calls,
                        "Circuit breaker half-open call limit reached"
                    );
                    return Err(ServiceError::CircuitBreakerOpen {
                        service: service_name.to_string(),
                    });
                }
                self.half_open_calls += 1;
            }
            CircuitBreakerState::Closed => {
                // Normal operation
            }
        }

        debug!(
            service = service_name,
            state = ?self.state,
            failure_count = self.failure_count,
            "Executing operation through circuit breaker"
        );

        match operation().await {
            Ok(result) => {
                self.on_success(service_name);
                Ok(result)
            }
            Err(error) => {
                self.on_failure(service_name, &error);
                Err(error)
            }
        }
    }

    /// Handle successful operation
    fn on_success(&mut self, service_name: &str) {
        match self.state {
            CircuitBreakerState::HalfOpen => {
                debug!(
                    service = service_name,
                    "Circuit breaker half-open call succeeded, closing circuit"
                );
                self.state = CircuitBreakerState::Closed;
                self.failure_count = 0;
                self.half_open_calls = 0;
                self.last_failure_time = None;
            }
            CircuitBreakerState::Closed => {
                // Reset failure count on success
                if self.failure_count > 0 {
                    debug!(
                        service = service_name,
                        previous_failures = self.failure_count,
                        "Circuit breaker call succeeded, resetting failure count"
                    );
                    self.failure_count = 0;
                    self.last_failure_time = None;
                }
            }
            CircuitBreakerState::Open => {
                // This shouldn't happen as we reject calls when open
                warn!(
                    service = service_name,
                    "Unexpected success while circuit breaker is open"
                );
            }
        }
    }

    /// Handle failed operation
    fn on_failure(&mut self, service_name: &str, error: &ServiceError) {
        // Only count certain types of failures
        if matches!(
            error,
            ServiceError::ServiceUnavailable { .. }
                | ServiceError::Network { .. }
                | ServiceError::Timeout { .. }
        ) {
            self.failure_count += 1;
            self.last_failure_time = Some(Instant::now());

            debug!(
                service = service_name,
                failure_count = self.failure_count,
                threshold = self.config.failure_threshold,
                error = %error,
                "Circuit breaker recorded failure"
            );

            if self.failure_count >= self.config.failure_threshold {
                warn!(
                    service = service_name,
                    failure_count = self.failure_count,
                    "Circuit breaker opening due to failure threshold"
                );
                self.state = CircuitBreakerState::Open;
                self.half_open_calls = 0;
            }
        }
    }

    /// Update circuit breaker state based on time and conditions
    fn update_state(&mut self) {
        if self.state == CircuitBreakerState::Open {
            if let Some(last_failure) = self.last_failure_time {
                if last_failure.elapsed() >= self.config.recovery_timeout {
                    debug!("Circuit breaker transitioning from Open to HalfOpen");
                    self.state = CircuitBreakerState::HalfOpen;
                    self.half_open_calls = 0;
                }
            }
        }
    }

    /// Get current circuit breaker state
    pub fn state(&self) -> &CircuitBreakerState {
        &self.state
    }

    /// Get current failure count
    pub fn failure_count(&self) -> u32 {
        self.failure_count
    }

    /// Reset the circuit breaker to closed state
    pub fn reset(&mut self) {
        self.state = CircuitBreakerState::Closed;
        self.failure_count = 0;
        self.last_failure_time = None;
        self.half_open_calls = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_retry_success_on_first_attempt() {
        let config = RetryConfig::default();

        let result = retry_with_backoff(
            || async { Ok::<i32, ServiceError>(42) },
            &config,
            "test_operation",
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_retry_success_after_retries() {
        let config = RetryConfig::default();
        let mut attempt_count = 0;

        let result = retry_with_backoff(
            || {
                attempt_count += 1;
                async move {
                    if attempt_count < 3 {
                        Err(ServiceError::Network {
                            message: "Temporary failure".to_string(),
                        })
                    } else {
                        Ok(42)
                    }
                }
            },
            &config,
            "test_operation",
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(attempt_count, 3);
    }

    #[tokio::test]
    async fn test_retry_non_retryable_error() {
        let config = RetryConfig::default();
        let mut attempt_count = 0;

        let result: Result<(), ServiceError> = retry_with_backoff(
            || {
                attempt_count += 1;
                async move {
                    Err(ServiceError::Authentication {
                        message: "Invalid credentials".to_string(),
                    })
                }
            },
            &config,
            "test_operation",
        )
        .await;

        assert!(result.is_err());
        assert_eq!(attempt_count, 1); // Should not retry
    }

    #[tokio::test]
    async fn test_circuit_breaker_opens_after_failures() {
        let config = CircuitBreakerConfig {
            failure_threshold: 2,
            recovery_timeout: Duration::from_millis(100),
            half_open_max_calls: 1,
        };
        let mut circuit_breaker = CircuitBreaker::new(config);

        // First failure
        let result1 = circuit_breaker
            .execute(
                || async {
                    Err::<(), ServiceError>(ServiceError::ServiceUnavailable {
                        message: "Service down".to_string(),
                    })
                },
                "test_service",
            )
            .await;
        assert!(result1.is_err());
        assert_eq!(circuit_breaker.state(), &CircuitBreakerState::Closed);

        // Second failure - should open circuit
        let result2 = circuit_breaker
            .execute(
                || async {
                    Err::<(), ServiceError>(ServiceError::ServiceUnavailable {
                        message: "Service down".to_string(),
                    })
                },
                "test_service",
            )
            .await;
        assert!(result2.is_err());
        assert_eq!(circuit_breaker.state(), &CircuitBreakerState::Open);

        // Third call should be rejected
        let result3 = circuit_breaker
            .execute(|| async { Ok::<(), ServiceError>(()) }, "test_service")
            .await;
        assert!(result3.is_err());
        assert!(matches!(
            result3.unwrap_err(),
            ServiceError::CircuitBreakerOpen { .. }
        ));
    }

    #[tokio::test]
    async fn test_circuit_breaker_half_open_recovery() {
        let config = CircuitBreakerConfig {
            failure_threshold: 1,
            recovery_timeout: Duration::from_millis(50),
            half_open_max_calls: 1,
        };
        let mut circuit_breaker = CircuitBreaker::new(config);

        // Cause failure to open circuit
        let _ = circuit_breaker
            .execute(
                || async {
                    Err::<(), ServiceError>(ServiceError::ServiceUnavailable {
                        message: "Service down".to_string(),
                    })
                },
                "test_service",
            )
            .await;
        assert_eq!(circuit_breaker.state(), &CircuitBreakerState::Open);

        // Wait for recovery timeout
        sleep(Duration::from_millis(60)).await;

        // Next call should transition to half-open and succeed
        let result = circuit_breaker
            .execute(|| async { Ok::<(), ServiceError>(()) }, "test_service")
            .await;
        assert!(result.is_ok());
        assert_eq!(circuit_breaker.state(), &CircuitBreakerState::Closed);
    }
}
