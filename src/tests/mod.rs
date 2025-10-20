// === Existing Test Modules ===
pub mod border_factory_tests;
pub mod model_tests;
pub mod test_utils;

// === Core Module Integration Tests ===
pub mod core_integration_tests;

// === Focus Integration Tests ===
pub mod focus_integration_tests;
pub mod simple_focus_test;

// === Page System Tests ===
pub mod page_system_tests;

// === New Modular Test Structure ===
pub mod helpers;
pub mod integration;
pub mod services;
pub mod ui;
pub mod unit;

// === Test Utilities ===
pub use test_utils::*;

#[cfg(test)]
mod smoke_tests {
    use super::*;

    #[test]
    fn test_all_modules_import() {
        // Test creating basic structures
        let _app = helpers::create_test_app();

        // If this compiles and runs, all modules are accessible
        assert!(true);
    }
}
