// Refactored metrics chart module
// All functionality has been split into focused modules:
// - validation: Data validation and sanitization
// - chart_utils: Utility functions for bounds, labels, colors
// - formatter: Value formatting for different metric types
// - error_display: Error message rendering
// - data_collector: Metrics collection from app state
// - renderer: Chart rendering functions
// - types: Chart data structures and types

// Note: Backwards compatibility re-exports removed as they were unused

// Note: All tests have been moved to their respective modules
// This provides better organization and faster compilation
