use anyhow::Result;
use std::time::Duration;

/// Simple mock for simulating AWS service delays
pub struct MockAwsDelay {
    pub delay_ms: u64,
}

impl MockAwsDelay {
    pub fn new(delay_ms: u64) -> Self {
        Self { delay_ms }
    }

    pub async fn simulate_delay(&self) {
        if self.delay_ms > 0 {
            tokio::time::sleep(Duration::from_millis(self.delay_ms)).await;
        }
    }
}

/// Mock error scenarios for testing
pub enum MockError {
    NetworkTimeout,
    AuthenticationFailed,
    InvalidConfiguration,
    DataParsingError,
}

impl MockError {
    pub fn to_error_message(&self) -> String {
        match self {
            MockError::NetworkTimeout => "Network timeout".to_string(),
            MockError::AuthenticationFailed => "Authentication failed".to_string(),
            MockError::InvalidConfiguration => "Invalid configuration".to_string(),
            MockError::DataParsingError => "Data parsing error".to_string(),
        }
    }
}

/// Simple mock service for testing
pub struct MockService {
    pub should_fail: bool,
    pub error_type: MockError,
    pub delay: MockAwsDelay,
}

impl MockService {
    pub fn new() -> Self {
        Self {
            should_fail: false,
            error_type: MockError::NetworkTimeout,
            delay: MockAwsDelay::new(0),
        }
    }

    pub fn with_failure(mut self, error_type: MockError) -> Self {
        self.should_fail = true;
        self.error_type = error_type;
        self
    }

    pub fn with_delay(mut self, delay_ms: u64) -> Self {
        self.delay = MockAwsDelay::new(delay_ms);
        self
    }

    pub async fn simulate_operation(&self) -> Result<()> {
        self.delay.simulate_delay().await;

        if self.should_fail {
            return Err(anyhow::anyhow!(self.error_type.to_error_message()));
        }

        Ok(())
    }
}
