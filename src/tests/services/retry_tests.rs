//! Tests for retry logic and circuit breaker patterns
//!
//! Tests the retry mechanisms and circuit breaker functionality
//! to ensure robust error handling and failure recovery.

use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;

use crate::services::{
    retry_with_backoff, CircuitBreaker, CircuitBreakerConfig, CircuitBreakerState, RetryConfig,
    ServiceError,
};

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
async fn test_retry_success_after_failures() {
    let config = RetryConfig {
        max_attempts: 3,
        initial_delay: Duration::from_millis(10),
        max_delay: Duration::from_millis(100),
        backoff_multiplier: 2.0,
        jitter: false,
    };

    let attempt_count = Arc::new(Mutex::new(0));
    let attempt_count_clone = attempt_count.clone();

    let result = retry_with_backoff(
        move || {
            let count = attempt_count_clone.clone();
            async move {
                let mut attempts = count.lock().unwrap();
                *attempts += 1;

                if *attempts < 3 {
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
    assert_eq!(*attempt_count.lock().unwrap(), 3);
}

#[tokio::test]
async fn test_retry_non_retryable_error() {
    let config = RetryConfig::default();
    let attempt_count = Arc::new(Mutex::new(0));
    let attempt_count_clone = attempt_count.clone();

    let result: Result<(), ServiceError> = retry_with_backoff(
        move || {
            let count = attempt_count_clone.clone();
            async move {
                let mut attempts = count.lock().unwrap();
                *attempts += 1;

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
    assert_eq!(*attempt_count.lock().unwrap(), 1); // Should not retry

    if let Err(ServiceError::Authentication { message }) = result {
        assert_eq!(message, "Invalid credentials");
    } else {
        panic!("Expected Authentication error");
    }
}

#[tokio::test]
async fn test_retry_max_attempts_exceeded() {
    let config = RetryConfig {
        max_attempts: 2,
        initial_delay: Duration::from_millis(10),
        max_delay: Duration::from_millis(100),
        backoff_multiplier: 2.0,
        jitter: false,
    };

    let attempt_count = Arc::new(Mutex::new(0));
    let attempt_count_clone = attempt_count.clone();

    let result: Result<(), ServiceError> = retry_with_backoff(
        move || {
            let count = attempt_count_clone.clone();
            async move {
                let mut attempts = count.lock().unwrap();
                *attempts += 1;

                Err(ServiceError::Network {
                    message: "Persistent failure".to_string(),
                })
            }
        },
        &config,
        "test_operation",
    )
    .await;

    assert!(result.is_err());
    assert_eq!(*attempt_count.lock().unwrap(), 2); // Should try max_attempts times
}

#[tokio::test]
async fn test_circuit_breaker_creation() {
    let config = CircuitBreakerConfig::default();
    let circuit_breaker = CircuitBreaker::new(config);

    assert_eq!(circuit_breaker.state(), &CircuitBreakerState::Closed);
    assert_eq!(circuit_breaker.failure_count(), 0);
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
    assert_eq!(circuit_breaker.failure_count(), 1);

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
    assert_eq!(circuit_breaker.failure_count(), 2);
}

#[tokio::test]
async fn test_circuit_breaker_rejects_calls_when_open() {
    let config = CircuitBreakerConfig {
        failure_threshold: 1,
        recovery_timeout: Duration::from_millis(100),
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

    // Next call should be rejected
    let result = circuit_breaker
        .execute(|| async { Ok::<(), ServiceError>(()) }, "test_service")
        .await;

    assert!(result.is_err());
    if let Err(ServiceError::CircuitBreakerOpen { service }) = result {
        assert_eq!(service, "test_service");
    } else {
        panic!("Expected CircuitBreakerOpen error");
    }
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
    assert_eq!(circuit_breaker.failure_count(), 0);
}

#[tokio::test]
async fn test_circuit_breaker_half_open_failure_reopens() {
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

    // Wait for recovery timeout
    sleep(Duration::from_millis(60)).await;

    // Next call should transition to half-open but fail
    let result = circuit_breaker
        .execute(
            || async {
                Err::<(), ServiceError>(ServiceError::ServiceUnavailable {
                    message: "Still down".to_string(),
                })
            },
            "test_service",
        )
        .await;

    assert!(result.is_err());
    assert_eq!(circuit_breaker.state(), &CircuitBreakerState::Open);
}

#[tokio::test]
async fn test_circuit_breaker_ignores_non_failure_errors() {
    let config = CircuitBreakerConfig {
        failure_threshold: 1,
        recovery_timeout: Duration::from_millis(100),
        half_open_max_calls: 1,
    };
    let mut circuit_breaker = CircuitBreaker::new(config);

    // Authentication error should not count as failure
    let result = circuit_breaker
        .execute(
            || async {
                Err::<(), ServiceError>(ServiceError::Authentication {
                    message: "Invalid credentials".to_string(),
                })
            },
            "test_service",
        )
        .await;

    assert!(result.is_err());
    assert_eq!(circuit_breaker.state(), &CircuitBreakerState::Closed);
    assert_eq!(circuit_breaker.failure_count(), 0);
}

#[tokio::test]
async fn test_circuit_breaker_reset() {
    let config = CircuitBreakerConfig {
        failure_threshold: 1,
        recovery_timeout: Duration::from_millis(100),
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
    assert_eq!(circuit_breaker.failure_count(), 1);

    // Reset circuit breaker
    circuit_breaker.reset();

    assert_eq!(circuit_breaker.state(), &CircuitBreakerState::Closed);
    assert_eq!(circuit_breaker.failure_count(), 0);

    // Should accept calls again
    let result = circuit_breaker
        .execute(|| async { Ok::<(), ServiceError>(()) }, "test_service")
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_retry_with_jitter() {
    let config = RetryConfig {
        max_attempts: 3,
        initial_delay: Duration::from_millis(100),
        max_delay: Duration::from_millis(1000),
        backoff_multiplier: 2.0,
        jitter: true,
    };

    let attempt_count = Arc::new(Mutex::new(0));
    let attempt_count_clone = attempt_count.clone();

    let start_time = std::time::Instant::now();

    let result = retry_with_backoff(
        move || {
            let count = attempt_count_clone.clone();
            async move {
                let mut attempts = count.lock().unwrap();
                *attempts += 1;

                if *attempts < 3 {
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

    let elapsed = start_time.elapsed();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
    assert_eq!(*attempt_count.lock().unwrap(), 3);

    // Should have taken some time due to delays (with jitter, timing is less predictable)
    assert!(elapsed >= Duration::from_millis(50)); // At least some delay
    assert!(elapsed <= Duration::from_millis(500)); // But not too much
}
