# Service Organization Refactoring Plan

## Goal
Make it easy to understand and work with multiple AWS services by organizing code in a clear, service-specific structure.

## Current Structure Analysis
- `src/aws/rds_service.rs` - RDS specific code
- `src/aws/cloudwatch_service.rs` - Mixed service code
- `src/models.rs` - Already has AwsService enum and ServiceInstance enum

## Proposed Simple Structure
```
src/aws/
├── services/
│   ├── rds/
│   │   ├── mod.rs           # RDS service implementation
│   │   ├── instances.rs     # RDS instance loading
│   │   ├── metrics.rs       # RDS-specific metrics
│   │   └── types.rs         # RDS-specific types
│   ├── sqs/
│   │   ├── mod.rs           # SQS service implementation  
│   │   ├── instances.rs     # SQS queue loading
│   │   ├── metrics.rs       # SQS-specific metrics
│   │   └── types.rs         # SQS-specific types
│   └── mod.rs               # Service registry and common traits
├── cloudwatch_service.rs    # Generic CloudWatch operations
└── mod.rs
```

## Implementation Strategy
1. **Enum-based service provider** (avoid async trait dyn issues)
2. **Service-specific modules** (clear separation)
3. **Backward compatibility** (minimal breaking changes)
4. **Progressive migration** (can be done incrementally)

## Benefits
- Clear service boundaries
- Easy to add new services
- Type-safe service operations
- Maintainable code structure
- No complex trait object issues