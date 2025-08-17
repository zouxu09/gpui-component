use gpui::*;
use std::collections::HashMap;

use super::theme::BoardTheme;

/// CSS property adapter for GPUI integration
/// Provides utilities for applying BoardTheme properties to GPUI elements
pub struct ThemeCSSAdapter {
    css_properties: HashMap<String, String>,
}

impl ThemeCSSAdapter {
    /// Creates a new CSS adapter from a BoardTheme
    pub fn from_theme(theme: &BoardTheme) -> Self {
        Self {
            css_properties: theme.generate_css_properties(),
        }
    }

    /// Gets a CSS property value by name
    pub fn get_property(&self, name: &str) -> Option<&String> {
        self.css_properties.get(name)
    }

    /// Gets all CSS properties as a map
    pub fn get_all_properties(&self) -> &HashMap<String, String> {
        &self.css_properties
    }

    /// Applies board background styling to a GPUI element
    pub fn apply_board_background<E: Styled>(&self, element: E, theme: &BoardTheme) -> E {
        element.bg(theme.board_background_color)
    }

    /// Applies grid line styling to a GPUI element
    pub fn apply_grid_line<E: Styled>(&self, element: E, theme: &BoardTheme) -> E {
        element.border_color(theme.grid_line_color).border_1()
    }

    /// Applies stone styling to a GPUI element
    pub fn apply_stone<E: Styled>(&self, element: E, theme: &BoardTheme, is_black: bool) -> E {
        let stone_color = if is_black {
            theme.black_stone_color
        } else {
            theme.white_stone_color
        };

        let mut styled = element.bg(stone_color);

        if theme.stone_border_width > 0.0 {
            styled = styled.border_color(theme.stone_border_color).border_1();
        }

        styled
    }

    /// Applies coordinate label styling to a GPUI element
    pub fn apply_coordinate<E: Styled>(&self, element: E, theme: &BoardTheme) -> E {
        element
            .text_color(theme.coordinate_color)
            .text_size(px(theme.coordinate_font_size))
    }

    /// Applies marker styling to a GPUI element
    pub fn apply_marker<E: Styled>(&self, element: E, theme: &BoardTheme) -> E {
        element.border_color(theme.marker_default_color).border_1()
    }

    /// Applies selection styling to a GPUI element
    pub fn apply_selection<E: Styled>(&self, element: E, theme: &BoardTheme) -> E {
        let mut selection_color = theme.selection_color;
        selection_color.a = theme.selection_opacity;

        element
            .bg(selection_color)
            .border_color(theme.selection_color)
            .border_1()
    }

    /// Applies directional selection styling based on direction
    pub fn apply_directional_selection<E: Styled>(
        &self,
        element: E,
        theme: &BoardTheme,
        direction: SelectionDirection,
    ) -> E {
        let color = match direction {
            SelectionDirection::Left => theme.selection_left_color,
            SelectionDirection::Right => theme.selection_right_color,
            SelectionDirection::Top => theme.selection_top_color,
            SelectionDirection::Bottom => theme.selection_bottom_color,
            _ => theme.selection_color,
        };

        let mut bg_color = color;
        bg_color.a = theme.selection_opacity;

        element.bg(bg_color).border_color(color).border_1()
    }

    /// Applies dimmed styling to a GPUI element
    pub fn apply_dimmed<E: Styled>(&self, element: E, theme: &BoardTheme) -> E {
        element.opacity(theme.dimmed_opacity)
    }

    /// Applies ghost stone styling based on ghost stone type
    pub fn apply_ghost_stone<E: Styled>(
        &self,
        element: E,
        theme: &BoardTheme,
        ghost_type: &crate::go_board::GhostStoneType,
        is_faint: bool,
    ) -> E {
        let color = match ghost_type {
            crate::go_board::GhostStoneType::Good => theme.ghost_good_color,
            crate::go_board::GhostStoneType::Interesting => theme.ghost_interesting_color,
            crate::go_board::GhostStoneType::Doubtful => theme.ghost_doubtful_color,
            crate::go_board::GhostStoneType::Bad => theme.ghost_bad_color,
        };

        let opacity = if is_faint {
            theme.ghost_stone_faint_opacity
        } else {
            theme.ghost_stone_opacity
        };

        let mut bg_color = color;
        bg_color.a = opacity;

        element.bg(bg_color).border_color(color)
    }

    /// Applies heat overlay styling based on strength
    pub fn apply_heat_overlay<E: Styled>(&self, element: E, theme: &BoardTheme, strength: u8) -> E {
        let mut color = theme.get_heat_color(strength);
        color.a = theme.heat_overlay_opacity;
        element.bg(color)
    }

    /// Applies paint overlay styling
    pub fn apply_paint_overlay<E: Styled>(
        &self,
        element: E,
        theme: &BoardTheme,
        paint_value: f32,
    ) -> E {
        let mut color = if paint_value > 0.0 {
            theme.black_stone_color // Positive values use black territory
        } else {
            theme.white_stone_color // Negative values use white territory
        };

        color.a = paint_value.abs() * theme.paint_overlay_opacity;
        element.bg(color)
    }

    /// Applies line styling to a GPUI element
    pub fn apply_line<E: Styled>(&self, element: E, theme: &BoardTheme) -> E {
        element
            .bg(theme.line_default_color)
            .border_color(theme.line_default_color)
    }

    /// Generates CSS text for external styling systems
    pub fn generate_css_text(&self) -> String {
        let mut css = String::from(":root {\n");

        for (property, value) in &self.css_properties {
            css.push_str(&format!("  {}: {};\n", property, value));
        }

        css.push_str("}\n");
        css
    }
}

// Import SelectionDirection from types
use crate::go_board::SelectionDirection;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::go_board::{BoardTheme, GhostStoneType};

    #[test]
    fn test_css_adapter_creation() {
        let theme = BoardTheme::default();
        let adapter = ThemeCSSAdapter::from_theme(&theme);

        assert!(adapter.get_property("--board-background-color").is_some());
        assert!(adapter.get_property("--grid-line-color").is_some());
        assert!(adapter.get_property("--black-stone-color").is_some());
    }

    #[test]
    fn test_css_text_generation() {
        let theme = BoardTheme::default();
        let adapter = ThemeCSSAdapter::from_theme(&theme);

        let css_text = adapter.generate_css_text();
        assert!(css_text.starts_with(":root {"));
        assert!(css_text.ends_with("}\n"));
        assert!(css_text.contains("--board-background-color"));
        assert!(css_text.contains("--grid-line-color"));
    }

    #[test]
    fn test_property_retrieval() {
        let theme = BoardTheme::default();
        let adapter = ThemeCSSAdapter::from_theme(&theme);

        let bg_color = adapter.get_property("--board-background-color");
        assert!(bg_color.is_some());
        assert!(bg_color.unwrap().contains("rgba"));

        let invalid_prop = adapter.get_property("--nonexistent-property");
        assert!(invalid_prop.is_none());
    }

    #[test]
    fn test_all_properties_access() {
        let theme = BoardTheme::default();
        let adapter = ThemeCSSAdapter::from_theme(&theme);

        let all_props = adapter.get_all_properties();
        assert!(!all_props.is_empty());
        assert!(all_props.contains_key("--board-background-color"));
        assert!(all_props.contains_key("--stone-size-ratio"));
    }
}
