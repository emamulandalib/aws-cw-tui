#[cfg(test)]
mod tests {
    use crate::ui::components::list_styling::border_factory;
    use crate::ui::themes::{warm_sunset_colors, UnifiedTheme};
    use ratatui::style::{Color, Style};

    /// Test that create_theme_border_style returns correct colors for focused state
    #[test]
    fn test_create_theme_border_style_focused() {
        let theme = UnifiedTheme::warm_sunset();
        let style = border_factory::create_theme_border_style(&theme, true);

        // Focused state should return VIBRANT_ORANGE (#F77F00)
        assert_eq!(style.fg, Some(warm_sunset_colors::VIBRANT_ORANGE));
        assert_eq!(style.fg, Some(Color::Rgb(247, 127, 0)));
    }

    /// Test that create_theme_border_style returns correct colors for unfocused state
    #[test]
    fn test_create_theme_border_style_unfocused() {
        let theme = UnifiedTheme::warm_sunset();
        let style = border_factory::create_theme_border_style(&theme, false);

        // Unfocused state should return WARM_CREAM (#EAE2B7)
        assert_eq!(style.fg, Some(warm_sunset_colors::WARM_CREAM));
        assert_eq!(style.fg, Some(Color::Rgb(234, 226, 183)));
    }

    /// Test that theme border colors match expected hex values
    #[test]
    fn test_theme_border_color_values() {
        let theme = UnifiedTheme::warm_sunset();

        // Verify theme has correct border colors
        assert_eq!(theme.border, warm_sunset_colors::WARM_CREAM);
        assert_eq!(theme.border, Color::Rgb(234, 226, 183)); // #EAE2B7

        assert_eq!(theme.border_focused, warm_sunset_colors::VIBRANT_ORANGE);
        assert_eq!(theme.border_focused, Color::Rgb(247, 127, 0)); // #F77F00
    }

    /// Test get_theme_border_color utility function
    #[test]
    fn test_get_theme_border_color() {
        let theme = UnifiedTheme::warm_sunset();

        // Test focused color
        let focused_color = border_factory::get_theme_border_color(&theme, true);
        assert_eq!(focused_color, warm_sunset_colors::VIBRANT_ORANGE);
        assert_eq!(focused_color, Color::Rgb(247, 127, 0));

        // Test unfocused color
        let unfocused_color = border_factory::get_theme_border_color(&theme, false);
        assert_eq!(unfocused_color, warm_sunset_colors::WARM_CREAM);
        assert_eq!(unfocused_color, Color::Rgb(234, 226, 183));
    }

    /// Test validate_border_consistency function
    #[test]
    fn test_validate_border_consistency() {
        let theme = UnifiedTheme::warm_sunset();

        // Test valid focused style
        let focused_style = Style::default().fg(theme.border_focused);
        assert!(border_factory::validate_border_consistency(
            &focused_style,
            &theme,
            true
        ));

        // Test valid unfocused style
        let unfocused_style = Style::default().fg(theme.border);
        assert!(border_factory::validate_border_consistency(
            &unfocused_style,
            &theme,
            false
        ));

        // Test invalid style (wrong color)
        let invalid_style = Style::default().fg(Color::Red);
        assert!(!border_factory::validate_border_consistency(
            &invalid_style,
            &theme,
            true
        ));
        assert!(!border_factory::validate_border_consistency(
            &invalid_style,
            &theme,
            false
        ));

        // Test style with no color
        let no_color_style = Style::default();
        assert!(!border_factory::validate_border_consistency(
            &no_color_style,
            &theme,
            true
        ));
        assert!(!border_factory::validate_border_consistency(
            &no_color_style,
            &theme,
            false
        ));
    }

    /// Test create_theme_status_border_style function
    #[test]
    fn test_create_theme_status_border_style() {
        let theme = UnifiedTheme::warm_sunset();

        // Test normal status
        let normal_style = border_factory::create_theme_status_border_style(
            &theme,
            border_factory::BorderStatus::Normal,
        );
        assert_eq!(normal_style.fg, Some(theme.border));

        // Test focused status
        let focused_style = border_factory::create_theme_status_border_style(
            &theme,
            border_factory::BorderStatus::Focused,
        );
        assert_eq!(focused_style.fg, Some(theme.border_focused));

        // Test error status
        let error_style = border_factory::create_theme_status_border_style(
            &theme,
            border_factory::BorderStatus::Error,
        );
        assert_eq!(error_style.fg, Some(theme.error));

        // Test success status
        let success_style = border_factory::create_theme_status_border_style(
            &theme,
            border_factory::BorderStatus::Success,
        );
        assert_eq!(success_style.fg, Some(theme.success));

        // Test warning status
        let warning_style = border_factory::create_theme_status_border_style(
            &theme,
            border_factory::BorderStatus::Warning,
        );
        assert_eq!(warning_style.fg, Some(theme.warning));

        // Test info status
        let info_style = border_factory::create_theme_status_border_style(
            &theme,
            border_factory::BorderStatus::Info,
        );
        assert_eq!(info_style.fg, Some(theme.info));
    }

    /// Test page-specific border behavior simulation
    /// This tests the expected behavior for metrics page vs non-metrics pages
    #[test]
    fn test_page_specific_border_behavior() {
        let theme = UnifiedTheme::warm_sunset();

        // Simulate metrics page behavior - should use focus/unfocus behavior
        let metrics_page_focused = border_factory::create_theme_border_style(&theme, true);
        let metrics_page_unfocused = border_factory::create_theme_border_style(&theme, false);

        assert_eq!(metrics_page_focused.fg, Some(Color::Rgb(247, 127, 0))); // #F77F00 orange
        assert_eq!(metrics_page_unfocused.fg, Some(Color::Rgb(234, 226, 183))); // #EAE2B7 beige

        // Simulate non-metrics pages - should consistently use unfocused color
        let non_metrics_page_always_unfocused =
            border_factory::create_theme_border_style(&theme, false);
        let non_metrics_page_ignore_focus =
            border_factory::create_theme_border_style(&theme, false);

        // Both should be the same (unfocused color) regardless of focus parameter for non-metrics pages
        assert_eq!(
            non_metrics_page_always_unfocused.fg,
            Some(Color::Rgb(234, 226, 183))
        ); // #EAE2B7
        assert_eq!(
            non_metrics_page_ignore_focus.fg,
            Some(Color::Rgb(234, 226, 183))
        ); // #EAE2B7
    }

    /// Test border factory with different theme variants
    #[test]
    fn test_border_factory_with_different_themes() {
        // Test with warm sunset theme (default)
        let warm_theme = UnifiedTheme::warm_sunset();
        let warm_focused = border_factory::create_theme_border_style(&warm_theme, true);
        let warm_unfocused = border_factory::create_theme_border_style(&warm_theme, false);

        assert_eq!(warm_focused.fg, Some(Color::Rgb(247, 127, 0))); // VIBRANT_ORANGE
        assert_eq!(warm_unfocused.fg, Some(Color::Rgb(234, 226, 183))); // WARM_CREAM

        // Test with blue gold theme
        let blue_theme = UnifiedTheme::blue_gold();
        let blue_focused = border_factory::create_theme_border_style(&blue_theme, true);
        let blue_unfocused = border_factory::create_theme_border_style(&blue_theme, false);

        // Should use different colors but same pattern
        assert_ne!(blue_focused.fg, warm_focused.fg);
        assert_ne!(blue_unfocused.fg, warm_unfocused.fg);
        assert_eq!(blue_focused.fg, Some(blue_theme.border_focused));
        assert_eq!(blue_unfocused.fg, Some(blue_theme.border));

        // Test with high contrast theme
        let contrast_theme = UnifiedTheme::high_contrast();
        let contrast_focused = border_factory::create_theme_border_style(&contrast_theme, true);
        let contrast_unfocused = border_factory::create_theme_border_style(&contrast_theme, false);

        assert_eq!(contrast_focused.fg, Some(contrast_theme.border_focused));
        assert_eq!(contrast_unfocused.fg, Some(contrast_theme.border));
    }

    /// Test that border factory functions handle edge cases properly
    #[test]
    fn test_border_factory_edge_cases() {
        let theme = UnifiedTheme::warm_sunset();

        // Test with custom focus color override
        let custom_focus_color = Color::Magenta;
        let custom_style = border_factory::create_theme_border_style_with_focus_override(
            &theme,
            true,
            custom_focus_color,
        );
        assert_eq!(custom_style.fg, Some(custom_focus_color));

        // Test unfocused with custom override (should still use theme border)
        let custom_unfocused = border_factory::create_theme_border_style_with_focus_override(
            &theme,
            false,
            custom_focus_color,
        );
        assert_eq!(custom_unfocused.fg, Some(theme.border));
        assert_ne!(custom_unfocused.fg, Some(custom_focus_color));
    }

    /// Test color constant values match expected hex codes
    #[test]
    fn test_color_constants_hex_values() {
        // Test VIBRANT_ORANGE is #F77F00
        assert_eq!(warm_sunset_colors::VIBRANT_ORANGE, Color::Rgb(247, 127, 0));

        // Test WARM_CREAM is #EAE2B7
        assert_eq!(warm_sunset_colors::WARM_CREAM, Color::Rgb(234, 226, 183));

        // Verify these are the colors used in the theme
        let theme = UnifiedTheme::warm_sunset();
        assert_eq!(theme.border_focused, Color::Rgb(247, 127, 0));
        assert_eq!(theme.border, Color::Rgb(234, 226, 183));
    }

    /// Test that border styles maintain consistency across multiple calls
    #[test]
    fn test_border_style_consistency() {
        let theme = UnifiedTheme::warm_sunset();

        // Multiple calls should return identical styles
        let style1 = border_factory::create_theme_border_style(&theme, true);
        let style2 = border_factory::create_theme_border_style(&theme, true);
        let style3 = border_factory::create_theme_border_style(&theme, false);
        let style4 = border_factory::create_theme_border_style(&theme, false);

        assert_eq!(style1.fg, style2.fg);
        assert_eq!(style3.fg, style4.fg);
        assert_ne!(style1.fg, style3.fg); // focused vs unfocused should be different
    }
}
