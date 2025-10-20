#[cfg(test)]
mod tests {
    use crate::config::{AppConfig, ConfigManager};
    use std::env;
    use tempfile::tempdir;

    #[test]
    fn test_config_manager_creation() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.json");

        let manager = ConfigManager::with_path(&config_path).unwrap();
        let config = manager.get_config();

        assert_eq!(config.version, "1.0.0");
        assert!(config_path.exists());
    }

    #[test]
    fn test_config_persistence() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.json");

        {
            let manager = ConfigManager::with_path(&config_path).unwrap();
            manager.set_auto_refresh_interval(60).unwrap();
        }

        // Create new manager with same path
        let manager = ConfigManager::with_path(&config_path).unwrap();
        let config = manager.get_config();

        assert_eq!(config.ui.auto_refresh_interval, 60);
    }

    #[test]
    fn test_environment_overrides() {
        env::set_var("AWS_DEFAULT_REGION", "us-west-2");
        env::set_var("AWSCW_REFRESH_INTERVAL", "45");

        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.json");

        let manager = ConfigManager::with_path(&config_path).unwrap();
        let config = manager.get_config();

        assert_eq!(config.aws.region, Some("us-west-2".to_string()));
        assert_eq!(config.ui.auto_refresh_interval, 45);

        // Clean up
        env::remove_var("AWS_DEFAULT_REGION");
        env::remove_var("AWSCW_REFRESH_INTERVAL");
    }

    #[test]
    fn test_corrupted_config_recovery() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.json");

        // Write invalid JSON
        std::fs::write(&config_path, "{ invalid json }").unwrap();

        let manager = ConfigManager::with_path(&config_path).unwrap();
        let config = manager.get_config();

        // Should have default values
        assert_eq!(config.version, "1.0.0");
        assert_eq!(config.ui.auto_refresh_interval, 30);

        // Backup file should exist
        let backup_path = config_path.with_extension("json.backup");
        assert!(backup_path.exists());
    }

    #[test]
    fn test_config_validation() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.json");

        let manager = ConfigManager::with_path(&config_path).unwrap();

        // Test invalid refresh interval
        let result = manager.set_auto_refresh_interval(0);
        assert!(result.is_err());
    }

    #[test]
    fn test_theme_setting() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.json");

        let manager = ConfigManager::with_path(&config_path).unwrap();

        // Set theme
        manager
            .set_theme(crate::ui::themes::ThemeVariant::BlueGold)
            .unwrap();

        let config = manager.get_config();
        assert!(matches!(
            config.theme.current_theme,
            crate::ui::themes::ThemeVariant::BlueGold
        ));
    }

    #[test]
    fn test_aws_config_setting() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.json");

        let manager = ConfigManager::with_path(&config_path).unwrap();

        // Set AWS region
        manager
            .set_aws_region(Some("eu-west-1".to_string()))
            .unwrap();

        let config = manager.get_config();
        assert_eq!(config.aws.region, Some("eu-west-1".to_string()));
    }

    #[test]
    fn test_duration_helpers() {
        let config = AppConfig::default();

        assert_eq!(config.refresh_interval().as_secs(), 30);
        assert_eq!(config.loading_timeout().as_secs(), 30);
        assert_eq!(config.request_timeout().as_secs(), 10);
        assert_eq!(config.metric_data_period().as_secs(), 300);
        assert_eq!(config.cache_ttl().as_secs(), 300);
    }

    #[test]
    fn test_comprehensive_environment_overrides() {
        // Set multiple environment variables
        env::set_var("AWS_DEFAULT_REGION", "eu-central-1");
        env::set_var("AWS_PROFILE", "test-profile");
        env::set_var("AWSCW_THEME", "blue_gold");
        env::set_var("AWSCW_REFRESH_INTERVAL", "60");
        env::set_var("AWSCW_AUTO_REFRESH", "false");
        env::set_var("AWSCW_MAX_CONCURRENT_REQUESTS", "20");

        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.json");

        let manager = ConfigManager::with_path(&config_path).unwrap();
        let config = manager.get_config();

        // Verify all environment overrides were applied
        assert_eq!(config.aws.region, Some("eu-central-1".to_string()));
        assert_eq!(config.aws.profile, Some("test-profile".to_string()));
        assert!(matches!(
            config.theme.current_theme,
            crate::ui::themes::ThemeVariant::BlueGold
        ));
        assert_eq!(config.ui.auto_refresh_interval, 60);
        assert_eq!(config.ui.auto_refresh_enabled, false);
        assert_eq!(config.performance.max_concurrent_requests, 20);

        // Clean up
        env::remove_var("AWS_DEFAULT_REGION");
        env::remove_var("AWS_PROFILE");
        env::remove_var("AWSCW_THEME");
        env::remove_var("AWSCW_REFRESH_INTERVAL");
        env::remove_var("AWSCW_AUTO_REFRESH");
        env::remove_var("AWSCW_MAX_CONCURRENT_REQUESTS");
    }

    #[test]
    fn test_configuration_file_structure() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.json");

        let manager = ConfigManager::with_path(&config_path).unwrap();

        // Verify config file was created and has proper structure
        assert!(config_path.exists());

        let content = std::fs::read_to_string(&config_path).unwrap();
        let json_value: serde_json::Value = serde_json::from_str(&content).unwrap();

        // Verify all major sections exist
        assert!(json_value.get("version").is_some());
        assert!(json_value.get("theme").is_some());
        assert!(json_value.get("ui").is_some());
        assert!(json_value.get("aws").is_some());
        assert!(json_value.get("performance").is_some());

        // Verify nested structure
        let theme = json_value.get("theme").unwrap();
        assert!(theme.get("current_theme").is_some());

        let ui = json_value.get("ui").unwrap();
        assert!(ui.get("auto_refresh_interval").is_some());
        assert!(ui.get("default_time_range").is_some());
    }

    #[test]
    fn test_configuration_update_methods() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.json");

        let manager = ConfigManager::with_path(&config_path).unwrap();

        // Test individual update methods
        manager
            .set_theme(crate::ui::themes::ThemeVariant::HighContrast)
            .unwrap();
        manager.set_auto_refresh_interval(120).unwrap();
        manager.set_auto_refresh_enabled(false).unwrap();
        manager.set_default_time_range("6h".to_string()).unwrap();
        manager
            .set_aws_region(Some("ap-southeast-1".to_string()))
            .unwrap();
        manager
            .set_aws_profile(Some("production".to_string()))
            .unwrap();

        // Verify all updates were applied
        let config = manager.get_config();
        assert!(matches!(
            config.theme.current_theme,
            crate::ui::themes::ThemeVariant::HighContrast
        ));
        assert_eq!(config.ui.auto_refresh_interval, 120);
        assert_eq!(config.ui.auto_refresh_enabled, false);
        assert_eq!(config.ui.default_time_range, "6h");
        assert_eq!(config.aws.region, Some("ap-southeast-1".to_string()));
        assert_eq!(config.aws.profile, Some("production".to_string()));
    }

    #[test]
    fn test_configuration_reload() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.json");

        let manager = ConfigManager::with_path(&config_path).unwrap();

        // Make initial changes
        manager
            .set_theme(crate::ui::themes::ThemeVariant::Monochrome)
            .unwrap();

        // Manually modify the config file
        let mut config = manager.get_config();
        config.ui.auto_refresh_interval = 90;
        std::fs::write(&config_path, serde_json::to_string_pretty(&config).unwrap()).unwrap();

        // Reload configuration
        manager.reload().unwrap();

        // Verify changes were loaded
        let reloaded_config = manager.get_config();
        assert_eq!(reloaded_config.ui.auto_refresh_interval, 90);
        assert!(matches!(
            reloaded_config.theme.current_theme,
            crate::ui::themes::ThemeVariant::Monochrome
        ));
    }

    #[test]
    fn test_configuration_validation_edge_cases() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.json");

        let manager = ConfigManager::with_path(&config_path).unwrap();

        // Test validation failures
        assert!(manager.set_auto_refresh_interval(0).is_err());

        // Test that valid values still work
        assert!(manager.set_auto_refresh_interval(1).is_ok());
        assert!(manager.set_auto_refresh_interval(3600).is_ok());
    }

    #[test]
    fn test_reset_to_defaults() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.json");

        let manager = ConfigManager::with_path(&config_path).unwrap();

        // Make some changes
        manager
            .set_theme(crate::ui::themes::ThemeVariant::BlueGold)
            .unwrap();
        manager.set_auto_refresh_interval(300).unwrap();
        manager
            .set_aws_region(Some("us-west-1".to_string()))
            .unwrap();

        // Reset to defaults
        manager.reset_to_defaults().unwrap();

        // Verify defaults were restored
        let config = manager.get_config();
        let default_config = AppConfig::default();

        assert_eq!(config.version, default_config.version);
        assert_eq!(
            config.ui.auto_refresh_interval,
            default_config.ui.auto_refresh_interval
        );
        assert_eq!(config.aws.region, default_config.aws.region);
        assert!(matches!(
            config.theme.current_theme,
            crate::ui::themes::ThemeVariant::Default
        ));
    }

    #[test]
    fn test_config_path_handling() {
        let manager = ConfigManager::new().unwrap();
        let config_path = manager.config_path();

        // Verify config path is reasonable
        assert!(config_path.to_string_lossy().contains("awscw"));
        assert!(config_path.extension().unwrap() == "json");

        // Verify config file exists after creation
        assert!(config_path.exists());
    }
}
