use std::collections::HashMap;

/// AWS service enumeration - defines supported AWS services
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, serde::Serialize, serde::Deserialize)]
pub enum AwsService {
    Rds,
    Sqs,
}

impl AwsService {
    /// Get the full display name for the service
    pub fn display_name(&self) -> &'static str {
        match self {
            AwsService::Rds => "RDS (Relational Database Service)",
            AwsService::Sqs => "SQS (Simple Queue Service)",
        }
    }

    /// Get the short name for the service
    pub fn short_name(&self) -> &'static str {
        match self {
            AwsService::Rds => "RDS",
            AwsService::Sqs => "SQS",
        }
    }
}

/// Generic AWS instance trait that different services can implement
pub trait AwsInstance {
    fn id(&self) -> &str;
}

/// RDS database instance representation
#[derive(Debug, Clone)]
pub struct RdsInstance {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub engine: String,
    pub status: String,
    pub instance_class: String,
    pub endpoint: Option<String>,
    pub port: Option<i32>,
    pub engine_version: Option<String>,
    pub allocated_storage: Option<i32>,
    pub storage_type: Option<String>,
    pub availability_zone: Option<String>,
    pub backup_retention_period: Option<i32>,
    pub multi_az: Option<bool>,
    pub storage_encrypted: Option<bool>,
    pub performance_insights_enabled: Option<bool>,
    pub deletion_protection: Option<bool>,
    pub creation_time: Option<std::time::SystemTime>,
}

impl AwsInstance for RdsInstance {
    fn id(&self) -> &str {
        &self.id
    }
}

/// SQS queue representation
#[derive(Debug, Clone)]
pub struct SqsQueue {
    pub url: String,
    pub name: String,
    pub queue_type: String, // "Standard" or "FIFO"
    pub attributes: HashMap<String, String>,
}

impl AwsInstance for SqsQueue {
    fn id(&self) -> &str {
        &self.name
    }
}

/// Generic service instance container
/// Supports different AWS service instances with type safety
#[derive(Debug, Clone)]
pub enum ServiceInstance {
    Rds(RdsInstance),
    Sqs(SqsQueue),
    // Future services can be added here
    // Ec2(Ec2Instance),
    // Lambda(LambdaFunction),
}

impl ServiceInstance {
    /// Get a reference to the underlying AWS instance trait
    pub fn as_aws_instance(&self) -> &dyn AwsInstance {
        match self {
            ServiceInstance::Rds(instance) => instance,
            ServiceInstance::Sqs(queue) => queue,
        }
    }

    /// Get the service type for this instance
    pub fn service_type(&self) -> AwsService {
        match self {
            ServiceInstance::Rds(_) => AwsService::Rds,
            ServiceInstance::Sqs(_) => AwsService::Sqs,
        }
    }
}
