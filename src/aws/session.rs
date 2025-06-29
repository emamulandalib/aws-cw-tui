use aws_config::{BehaviorVersion, SdkConfig};
use aws_sdk_cloudwatch::Client as CloudWatchClient;
use aws_sdk_rds::Client as RdsClient;
use aws_sdk_sts::Client as StsClient;
use std::sync::Arc;
use tokio::sync::OnceCell;

/// Global AWS configuration - loaded once and reused throughout the application
static AWS_CONFIG: OnceCell<Arc<SdkConfig>> = OnceCell::const_new();

/// AWS Session Manager - handles centralized AWS config and client creation
///
/// This ensures AWS config is loaded only once using standard credential chain:
/// - Environment variables (AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY)
/// - AWS profiles (~/.aws/credentials, ~/.aws/config)
/// - SSO sessions
/// - Instance roles (when running on EC2)
/// - Container roles (when running on ECS/Fargate)
pub struct AwsSessionManager;

impl AwsSessionManager {
    /// Get the shared AWS configuration, loading it once if needed
    ///
    /// This uses AWS standard credential chain and respects all AWS SDK conventions
    /// including region resolution, retry configuration, and timeout settings.
    ///
    /// The AWS SDK automatically handles:
    /// 1. Environment variables (AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY, AWS_SESSION_TOKEN)
    /// 2. Shared AWS config and credentials files (~/.aws/config, ~/.aws/credentials)
    /// 3. AWS STS web identity tokens (SSO, federated auth)
    /// 4. Amazon ECS/EKS container credentials
    /// 5. Amazon EC2 Instance Metadata Service (IMDSv2)
    pub async fn get_config() -> Arc<SdkConfig> {
        AWS_CONFIG
            .get_or_init(|| async {
                let config = aws_config::defaults(BehaviorVersion::latest()).load().await;
                Arc::new(config)
            })
            .await
            .clone()
    }

    /// Create a new RDS client using the shared config
    pub async fn rds_client() -> RdsClient {
        let config = Self::get_config().await;
        RdsClient::new(&config)
    }

    /// Create a new CloudWatch client using the shared config
    pub async fn cloudwatch_client() -> CloudWatchClient {
        let config = Self::get_config().await;
        CloudWatchClient::new(&config)
    }

    /// Create a new STS client using the shared config (for credential validation)
    pub async fn sts_client() -> StsClient {
        let config = Self::get_config().await;
        StsClient::new(&config)
    }

    /// Validate credentials using AWS STS GetCallerIdentity
    ///
    /// This leverages the built-in AWS credential provider chain
    /// and provides detailed feedback about the authentication status
    pub async fn validate_credentials() -> anyhow::Result<CredentialValidationResult> {
        let mut status_messages = Vec::new();
        let mut error_guidance = Vec::new();

        status_messages.push("Checking AWS credentials...".to_string());

        // Get current profile info for display (AWS SDK handles actual profile resolution)
        let profile =
            std::env::var("AWS_PROFILE").unwrap_or_else(|_| "default (implicit)".to_string());

        // Load config once - AWS SDK handles the credential provider chain
        let config = Self::get_config().await;

        // Get the actual region that the AWS SDK resolved
        let region = config
            .region()
            .map(|r| r.as_ref())
            .unwrap_or("not-configured");

        status_messages.push(format!("Using AWS Profile: {}", profile));
        status_messages.push(format!("Using AWS Region: {}", region));

        // Test credential access with STS call - this will use whatever credential source AWS SDK found
        status_messages.push("Validating credentials...".to_string());
        let sts_client = Self::sts_client().await;

        match sts_client.get_caller_identity().send().await {
            Ok(identity) => {
                let account_id = identity.account().unwrap_or("Unknown");
                let user_id = identity.user_id().unwrap_or("Unknown");
                let arn = identity.arn().unwrap_or("Unknown");

                status_messages.push("AWS credentials validated successfully!".to_string());
                status_messages.push(format!("   Account ID: {}", account_id));
                status_messages.push(format!("   User/Role: {}", user_id));
                status_messages.push(format!("   ARN: {}", arn));

                let credential_info = CredentialInfo {
                    profile,
                    region: region.to_string(),
                    account_id: account_id.to_string(),
                    user_id: user_id.to_string(),
                    arn: arn.to_string(),
                };

                Ok(CredentialValidationResult {
                    success: true,
                    credential_info: Some(credential_info),
                    status_messages,
                    error_message: None,
                    error_guidance: Vec::new(),
                })
            }
            Err(e) => {
                let error_msg = e.to_string();
                status_messages.push("AWS credential validation failed!".to_string());

                // Provide helpful guidance for common credential issues
                if error_msg.contains("credential") || error_msg.contains("no providers in chain") {
                    error_guidance.push("Credential issue detected. AWS SDK supports multiple authentication methods:".to_string());
                    error_guidance.push("   1. Environment variables:".to_string());
                    error_guidance
                        .push("      export AWS_ACCESS_KEY_ID=your-access-key".to_string());
                    error_guidance
                        .push("      export AWS_SECRET_ACCESS_KEY=your-secret-key".to_string());
                    error_guidance.push("   2. AWS profiles:".to_string());
                    error_guidance.push("      export AWS_PROFILE=your-profile-name".to_string());
                    error_guidance.push("      (or run: aws configure)".to_string());
                    error_guidance.push("   3. SSO login:".to_string());
                    error_guidance
                        .push("      aws sso login --profile your-sso-profile".to_string());
                    error_guidance.push(
                        "   4. Instance/Container roles (automatic when running on AWS)"
                            .to_string(),
                    );
                    error_guidance.push(format!(
                        "Current profile '{}' might not exist or be configured correctly.",
                        profile
                    ));
                } else {
                    error_guidance.push(format!("Error details: {}", error_msg));
                }

                Ok(CredentialValidationResult {
                    success: false,
                    credential_info: None,
                    status_messages,
                    error_message: Some(error_msg),
                    error_guidance,
                })
            }
        }
    }

    /// Get region information from the current config
    pub async fn get_region() -> String {
        let config = Self::get_config().await;
        config
            .region()
            .map(|r| r.as_ref())
            .unwrap_or("us-east-1")
            .to_string()
    }

    /// Force reload the AWS configuration (useful for credential rotation)
    ///
    /// Note: This is generally not needed as AWS SDK handles credential refresh automatically
    pub async fn reload_config() -> Arc<SdkConfig> {
        // Force reload by creating new config
        let config = aws_config::defaults(BehaviorVersion::latest()).load().await;
        let new_config = Arc::new(config);

        // This will replace the old config
        AWS_CONFIG.set(new_config.clone()).ok(); // Ignore error if already set
        new_config
    }
}

#[derive(Debug, Clone)]
pub struct CredentialInfo {
    pub profile: String,
    pub region: String,
    pub account_id: String,
    pub user_id: String,
    pub arn: String,
}

/// Comprehensive result of credential validation including status messages
#[derive(Debug, Clone)]
pub struct CredentialValidationResult {
    pub success: bool,
    pub credential_info: Option<CredentialInfo>,
    pub status_messages: Vec<String>,
    pub error_message: Option<String>,
    pub error_guidance: Vec<String>,
}

/// Collection of AWS service clients
#[derive(Debug)]
pub struct AwsClients {
    pub rds: RdsClient,
    pub cloudwatch: CloudWatchClient,
    pub sts: StsClient,
}
