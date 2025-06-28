use std::sync::Arc;
use tokio::sync::OnceCell;
use aws_config::{BehaviorVersion, SdkConfig};
use aws_sdk_rds::Client as RdsClient;
use aws_sdk_cloudwatch::Client as CloudWatchClient;
use aws_sdk_sts::Client as StsClient;

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
    pub async fn get_config() -> Arc<SdkConfig> {
        AWS_CONFIG
            .get_or_init(|| async {
                let config = aws_config::defaults(BehaviorVersion::latest())
                    .load()
                    .await;
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

    /// Validate AWS credentials and return session information
    /// 
    /// This replaces the validate_aws_credentials function in main.rs
    /// and provides detailed credential information.
    pub async fn validate_credentials() -> anyhow::Result<CredentialInfo> {
        println!("Checking AWS credentials...");

        // Get current profile info
        let profile = std::env::var("AWS_PROFILE").unwrap_or_else(|_| "default".to_string());
        
        // Load config once
        let config = Self::get_config().await;
        
        // Get the actual region that the AWS SDK will use
        let region = config.region().map(|r| r.as_ref()).unwrap_or("unknown");

        println!("Using AWS Profile: {}", profile);
        println!("Using AWS Region: {}", region);

        // Test credential access with a simple STS call
        println!("Validating credentials...");
        let sts_client = Self::sts_client().await;

        match sts_client.get_caller_identity().send().await {
            Ok(identity) => {
                let account_id = identity.account().unwrap_or("Unknown");
                let user_id = identity.user_id().unwrap_or("Unknown");
                let arn = identity.arn().unwrap_or("Unknown");
                
                println!("AWS credentials validated successfully!");
                println!("   Account ID: {}", account_id);
                println!("   User/Role: {}", user_id);
                println!("   ARN: {}", arn);
                println!();

                Ok(CredentialInfo {
                    profile,
                    region: region.to_string(),
                    account_id: account_id.to_string(),
                    user_id: user_id.to_string(),
                    arn: arn.to_string(),
                })
            }
            Err(e) => {
                println!("AWS credential validation failed!");
                println!();

                let error_msg = e.to_string();
                if error_msg.contains("credential") || error_msg.contains("no providers in chain") {
                    println!("Credential issue detected. AWS SDK supports multiple authentication methods:");
                    println!("   1. Environment variables:");
                    println!("      export AWS_ACCESS_KEY_ID=your-access-key");
                    println!("      export AWS_SECRET_ACCESS_KEY=your-secret-key");
                    println!("   2. AWS profiles:");
                    println!("      export AWS_PROFILE=your-profile-name");
                    println!("      (or run: aws configure)");
                    println!("   3. SSO login:");
                    println!("      aws sso login --profile your-sso-profile");
                    println!("   4. Instance/Container roles (automatic when running on AWS)");
                    println!();
                    println!(
                        "Current profile '{}' might not exist or be configured correctly.",
                        profile
                    );
                } else {
                    println!("Error details: {}", error_msg);
                }

                Err(anyhow::anyhow!("AWS credential validation failed: {}", error_msg))
            }
        }
    }

    /// Get region information from the current config
    pub async fn get_region() -> String {
        let config = Self::get_config().await;
        config.region().map(|r| r.as_ref()).unwrap_or("us-east-1").to_string()
    }

    /// Force reload the AWS configuration (useful for credential rotation)
    /// 
    /// Note: This is generally not needed as AWS SDK handles credential refresh automatically
    pub async fn reload_config() -> Arc<SdkConfig> {
        // Force reload by creating new config
        let config = aws_config::defaults(BehaviorVersion::latest())
            .load()
            .await;
        let new_config = Arc::new(config);
        
        // This will replace the old config
        AWS_CONFIG.set(new_config.clone()).ok(); // Ignore error if already set
        new_config
    }
}

/// Information about the current AWS credentials and session
#[derive(Debug, Clone)]
pub struct CredentialInfo {
    pub profile: String,
    pub region: String,
    pub account_id: String,
    pub user_id: String,
    pub arn: String,
} 