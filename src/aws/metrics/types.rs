//! Core types for the modular CloudWatch metrics system

use crate::models::AwsService;
use std::collections::HashMap;
use std::time::SystemTime;

/// Defines a CloudWatch metric with its configuration
#[derive(Debug, Clone)]
pub struct MetricDefinition {
    pub name: String,
    pub unit: Option<String>,
    pub statistic: StatisticType,
    pub category: MetricCategory,
}

/// Statistic types supported by CloudWatch
#[derive(Debug, Clone, PartialEq)]
pub enum StatisticType {
    Average,
    Sum,
    Maximum,
    Minimum,
}

/// Categories for organizing metrics
#[derive(Debug, Clone, PartialEq)]
pub enum MetricCategory {
    Core,
    Advanced,
    Performance,
    Storage,
    Network,
}

/// Contains raw metrics data from CloudWatch
#[derive(Debug)]
pub struct ServiceMetrics {
    pub raw_metrics: HashMap<String, MetricValue>,
    pub timestamps: Vec<SystemTime>,
    pub service_type: AwsService,
}

/// A single metric value with current and historical data
#[derive(Debug, Clone)]
pub struct MetricValue {
    pub current: f64,
    pub history: Vec<f64>,
}

impl MetricValue {
    pub fn new(current: f64, history: Vec<f64>) -> Self {
        Self { current, history }
    }
    
    pub fn empty() -> Self {
        Self {
            current: 0.0,
            history: Vec::new(),
        }
    }
}

impl ServiceMetrics {
    pub fn new(service_type: AwsService) -> Self {
        Self {
            raw_metrics: HashMap::new(),
            timestamps: Vec::new(),
            service_type,
        }
    }
    
    pub fn add_metric(&mut self, name: String, value: MetricValue) {
        self.raw_metrics.insert(name, value);
    }
}