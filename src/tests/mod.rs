// === Test Modules ===
pub mod model_tests;
pub mod test_utils;

// === Core Module Integration Tests ===
pub mod core_integration_tests;

// === Test Utilities ===
pub use test_utils::*;

#[cfg(test)]
mod tests {
    use super::*;
    
    /// Basic smoke test to ensure all modules compile and can be imported
    #[test]
    fn test_module_imports() {
        // Test that all core modules can be imported
        use crate::core::app::*;
        use crate::models::*;
        use crate::ui::*;
        use crate::utils::*;
        use crate::config::*;
        
        // If this compiles and runs, all modules are accessible
        assert!(true);
    }
}
