/// AWS error handling utilities for consistent credential error management
pub struct AwsErrorHandler;

impl AwsErrorHandler {
    /// Check if an AWS SDK error is credential-related and return a formatted error
    /// 
    /// # Arguments
    /// * `error` - The AWS SDK error to check
    /// * `context` - Context describing what operation failed (e.g., "fetch RDS instances")
    /// * `service_permissions` - Service-specific permission requirements (e.g., "RDS describe permissions")
    /// 
    /// # Returns
    /// An `anyhow::Error` with either a detailed credential error message or a generic error message
    pub fn handle_aws_error<E: std::error::Error + Send + Sync + 'static>(
        error: E,
        context: &str,
        service_permissions: &str,
    ) -> anyhow::Error {
        let error_msg = error.to_string();
        
        // Check if it's a credential-related error
        if error_msg.contains("credential")
            || error_msg.contains("authentication")
            || error_msg.contains("access")
            || error_msg.contains("no providers in chain")
        {
            anyhow::anyhow!(
                "AWS credentials error: {}\\n\\n\
                 Please ensure:\\n\
                 - Your AWS credentials are configured correctly\\n\
                 - You have the correct AWS_PROFILE set (currently: {})\\n\
                 - Your credentials have {}\\n\
                 - Try: export AWS_PROFILE=your-profile-name",
                error_msg,
                std::env::var("AWS_PROFILE").unwrap_or_else(|_| "default".to_string()),
                service_permissions
            )
        } else {
            anyhow::anyhow!("Failed to {}: {}", context, error)
        }
    }
} 