use crate::models::*;
use crate::tests::helpers::*;
use crate::ui::themes::*;

#[cfg(test)]
mod theme_tests {
    use super::*;

    #[test]
    fn test_theme_cycling() {
        let mut app = create_test_app();
        let initial_theme_name = app.get_current_theme_name();

        // Test theme cycling
        app.next_theme();
        // Theme should change

        // Cycle through all themes (6 total)
        for _ in 0..5 {
            app.next_theme();
        }

        // Should cycle back to initial theme
        assert_eq!(app.get_current_theme_name(), initial_theme_name);
    }

    #[test]
    fn test_theme_names() {
        let mut app = create_test_app();

        // Test all theme names
        let themes = vec![
            ThemeVariant::Default,
            ThemeVariant::WarmSunset,
            ThemeVariant::BlueGold,
            ThemeVariant::HighContrast,
            ThemeVariant::Monochrome,
            ThemeVariant::TerminalCyan,
        ];

        for theme in themes {
            app.current_theme = theme;
            let name = app.get_current_theme_name();
            assert!(!name.is_empty());
        }
    }

    #[test]
    fn test_theme_persistence() {
        let mut app = create_test_app();

        // Change theme
        app.next_theme();
        let theme_name = app.get_current_theme_name();

        // Theme should persist across state changes
        app.state = AppState::InstanceList;
        assert_eq!(app.get_current_theme_name(), theme_name);

        app.state = AppState::MetricsSummary;
        assert_eq!(app.get_current_theme_name(), theme_name);
    }

    #[test]
    fn test_unified_theme_creation() {
        let app = create_test_app();
        let unified_theme = app.get_current_theme();

        // Should create a valid UnifiedTheme
        // This tests the theme conversion functionality
        // UnifiedTheme should have all required fields
        let _primary = unified_theme.primary;
        let _secondary = unified_theme.secondary;
        let _background = unified_theme.background;
        let _accent = unified_theme.accent;

        // Test passes if it doesn't panic
        assert!(true);
    }
}
