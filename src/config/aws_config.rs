use std::time::Duration;

/// AWS-specific configuration settings
#[derive(Debug, Clone)]
pub struct AwsConfig {
    pub region: Option<String>,
    pub profile: Option<String>,
    pub max_retries: u32,
    pub request_timeout: Duration,
    pub metric_data_period: Duration,
}

impl Default for AwsConfig {
    fn default() -> Self {
        Self {
            region: None,  // Will use AWS SDK defaults
            profile: None, // Will use AWS SDK defaults
            max_retries: 3,
            request_timeout: Duration::from_secs(10),
            metric_data_period: Duration::from_secs(300), // 5 minutes
        }
    }
}

impl AwsConfig {
    /// Create a new AWS configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the AWS region
    pub fn with_region(mut self, region: String) -> Self {
        self.region = Some(region);
        self
    }

    /// Set the AWS profile
    pub fn with_profile(mut self, profile: String) -> Self {
        self.profile = Some(profile);
        self
    }

    /// Set maximum retries for AWS requests
    pub fn with_max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }

    /// Set request timeout
    pub fn with_request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self
    }
}
