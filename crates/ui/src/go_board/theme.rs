use gpui::*;

/// Comprehensive theme system for the Go board widget
/// Inspired by Shudan's CSS custom property approach but adapted for GPUI
#[derive(Clone, Debug)]
pub struct BoardTheme {
    // Board appearance
    pub board_background_color: Rgba,
    pub board_foreground_color: Rgba,
    pub board_border_color: Rgba,
    pub board_border_width: f32,

    // Grid styling
    pub grid_line_color: Rgba,
    pub grid_line_width: f32,
    pub star_point_color: Rgba,
    pub star_point_size: f32,

    // Stone styling
    pub black_stone_color: Rgba,
    pub white_stone_color: Rgba,
    pub stone_size_ratio: f32, // Ratio of stone size to vertex size (0.0 to 1.0)
    pub stone_border_width: f32,
    pub stone_border_color: Rgba,
    pub stone_shadow_enabled: bool,
    pub stone_shadow_color: Rgba,
    pub stone_shadow_offset: (f32, f32),

    // Stone visual effects
    pub fuzzy_placement: bool,
    pub fuzzy_max_offset: f32,
    pub random_variation: bool,
    pub max_rotation: f32,

    // Coordinate styling
    pub coordinate_color: Rgba,
    pub coordinate_font_size: f32,
    pub coordinate_font_family: String,
    pub coordinate_opacity: f32,

    // Marker styling
    pub marker_default_color: Rgba,
    pub marker_default_size: f32,
    pub marker_stroke_width: f32,
    pub marker_label_font_size: f32,
    pub marker_label_color: Rgba,

    // Selection and highlight styling
    pub selection_color: Rgba,
    pub selection_opacity: f32,
    pub dimmed_opacity: f32,
    pub selection_border_width: f32,

    // Directional selection colors
    pub selection_left_color: Rgba,
    pub selection_right_color: Rgba,
    pub selection_top_color: Rgba,
    pub selection_bottom_color: Rgba,

    // Overlay styling
    pub paint_overlay_opacity: f32,
    pub heat_overlay_opacity: f32,
    pub heat_gradient_colors: Vec<Rgba>, // Color gradient for heat values 0-9

    // Ghost stone styling
    pub ghost_stone_opacity: f32,
    pub ghost_stone_faint_opacity: f32,
    pub ghost_good_color: Rgba,
    pub ghost_interesting_color: Rgba,
    pub ghost_doubtful_color: Rgba,
    pub ghost_bad_color: Rgba,

    // Line and arrow styling
    pub line_default_color: Rgba,
    pub line_default_width: f32,
    pub arrow_head_size: f32,
    pub line_dash_pattern: Option<Vec<f32>>,
}

impl Default for BoardTheme {
    fn default() -> Self {
        Self {
            // Board appearance - Shudan-inspired wood theme
            board_background_color: rgb(0xebb55b), // Classic Go board wood color
            board_foreground_color: rgb(0x5e2e0c), // Dark wood accent
            board_border_color: rgb(0xca933a),     // Wood border
            board_border_width: 4.0,

            // Grid styling
            grid_line_color: rgb(0x000000), // Black grid lines
            grid_line_width: 1.0,
            star_point_color: rgb(0x000000), // Black star points
            star_point_size: 6.0,

            // Stone styling
            black_stone_color: rgb(0x1a1a1a), // Dark gray/black
            white_stone_color: rgb(0xf8f8f8), // Off-white
            stone_size_ratio: 0.9,            // 90% of vertex size
            stone_border_width: 1.0,
            stone_border_color: rgb(0x000000), // Black border
            stone_shadow_enabled: true,
            stone_shadow_color: rgb(0x000000), // Black shadow without alpha
            stone_shadow_offset: (2.0, 2.0),

            // Stone visual effects
            fuzzy_placement: false,
            fuzzy_max_offset: 2.0, // 2 pixels max offset
            random_variation: false,
            max_rotation: 5.0, // 5 degrees max rotation

            // Coordinate styling
            coordinate_color: rgb(0x5e2e0c), // Dark wood color
            coordinate_font_size: 12.0,
            coordinate_font_family: "system-ui".to_string(),
            coordinate_opacity: 0.8,

            // Marker styling
            marker_default_color: rgb(0xff0000), // Red markers
            marker_default_size: 0.4,            // 40% of vertex size
            marker_stroke_width: 2.0,
            marker_label_font_size: 10.0,
            marker_label_color: rgb(0x000000),

            // Selection and highlight styling
            selection_color: rgb(0x0066cc), // Blue selection
            selection_opacity: 0.6,
            dimmed_opacity: 0.3,
            selection_border_width: 2.0,

            // Directional selection colors
            selection_left_color: rgb(0xff0000),   // Red for left
            selection_right_color: rgb(0x00aa00),  // Green for right
            selection_top_color: rgb(0xff8800),    // Orange for top
            selection_bottom_color: rgb(0x8800ff), // Purple for bottom

            // Overlay styling
            paint_overlay_opacity: 0.4,
            heat_overlay_opacity: 0.6,
            heat_gradient_colors: vec![
                rgb(0x000080), // Blue (0)
                rgb(0x0040a0), // 1
                rgb(0x0080c0), // 2
                rgb(0x00a0e0), // Cyan (3)
                rgb(0x40c040), // 4
                rgb(0x80e000), // 5
                rgb(0xc0ff00), // Yellow (6)
                rgb(0xff8000), // 7
                rgb(0xff4000), // 8
                rgb(0xff0000), // Red (9)
            ],

            // Ghost stone styling
            ghost_stone_opacity: 0.7,
            ghost_stone_faint_opacity: 0.4,
            ghost_good_color: rgb(0x00aa00),        // Green
            ghost_interesting_color: rgb(0x0066cc), // Blue
            ghost_doubtful_color: rgb(0xffaa00),    // Yellow
            ghost_bad_color: rgb(0xff0000),         // Red

            // Line and arrow styling
            line_default_color: rgb(0x2c2c2c), // Dark gray
            line_default_width: 2.0,
            arrow_head_size: 8.0,
            line_dash_pattern: None,
        }
    }
}

impl BoardTheme {
    /// Creates a new board theme with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a minimalist theme with clean lines and subtle colors
    pub fn minimalist() -> Self {
        Self {
            board_background_color: rgb(0xfafafa), // Light gray
            board_border_color: rgb(0xe0e0e0),     // Lighter border
            grid_line_color: rgb(0x666666),        // Gray grid lines
            star_point_color: rgb(0x999999),       // Lighter star points
            coordinate_color: rgb(0x666666),       // Gray coordinates
            marker_default_color: rgb(0x333333),   // Dark gray markers
            ..Self::default()
        }
    }

    /// Creates a dark theme suitable for night mode
    pub fn dark() -> Self {
        Self {
            board_background_color: rgb(0x2d2d2d), // Dark gray
            board_border_color: rgb(0x404040),     // Lighter dark border
            grid_line_color: rgb(0x606060),        // Light gray grid lines
            star_point_color: rgb(0x808080),       // Light gray star points
            coordinate_color: rgb(0xa0a0a0),       // Light gray coordinates
            marker_default_color: rgb(0xffffff),   // White markers
            selection_color: rgb(0x4488ff),        // Brighter blue selection
            ..Self::default()
        }
    }

    /// Creates a high contrast theme for accessibility
    pub fn high_contrast() -> Self {
        Self {
            board_background_color: rgb(0xffffff), // Pure white
            board_border_color: rgb(0x000000),     // Pure black border
            grid_line_color: rgb(0x000000),        // Pure black grid lines
            star_point_color: rgb(0x000000),       // Pure black star points
            black_stone_color: rgb(0x000000),      // Pure black stones
            white_stone_color: rgb(0xffffff),      // Pure white stones
            stone_border_width: 2.0,               // Thicker borders
            coordinate_color: rgb(0x000000),       // Black coordinates
            marker_default_color: rgb(0xff0000),   // Red markers
            selection_color: rgb(0x0000ff),        // Blue selection
            selection_opacity: 0.8,                // Higher opacity
            ..Self::default()
        }
    }

    /// Sets the board background color
    pub fn with_board_background(mut self, color: Rgba) -> Self {
        self.board_background_color = color;
        self
    }

    /// Sets the grid line styling
    pub fn with_grid_lines(mut self, color: Rgba, width: f32) -> Self {
        self.grid_line_color = color;
        self.grid_line_width = width;
        self
    }

    /// Sets the stone colors
    pub fn with_stone_colors(mut self, black: Rgba, white: Rgba) -> Self {
        self.black_stone_color = black;
        self.white_stone_color = white;
        self
    }

    /// Enables fuzzy stone placement with specified maximum offset
    pub fn with_fuzzy_placement(mut self, enabled: bool, max_offset: f32) -> Self {
        self.fuzzy_placement = enabled;
        self.fuzzy_max_offset = max_offset;
        self
    }

    /// Enables random stone variation with rotation
    pub fn with_random_variation(mut self, enabled: bool, max_rotation: f32) -> Self {
        self.random_variation = enabled;
        self.max_rotation = max_rotation;
        self
    }

    /// Sets coordinate styling
    pub fn with_coordinates(mut self, color: Rgba, font_size: f32, opacity: f32) -> Self {
        self.coordinate_color = color;
        self.coordinate_font_size = font_size;
        self.coordinate_opacity = opacity;
        self
    }

    /// Sets marker styling
    pub fn with_markers(mut self, color: Rgba, size: f32, stroke_width: f32) -> Self {
        self.marker_default_color = color;
        self.marker_default_size = size;
        self.marker_stroke_width = stroke_width;
        self
    }

    /// Sets selection styling
    pub fn with_selection(mut self, color: Rgba, opacity: f32, border_width: f32) -> Self {
        self.selection_color = color;
        self.selection_opacity = opacity;
        self.selection_border_width = border_width;
        self
    }

    /// Sets directional selection colors
    pub fn with_directional_selection(
        mut self,
        left: Rgba,
        right: Rgba,
        top: Rgba,
        bottom: Rgba,
    ) -> Self {
        self.selection_left_color = left;
        self.selection_right_color = right;
        self.selection_top_color = top;
        self.selection_bottom_color = bottom;
        self
    }

    /// Sets ghost stone colors
    pub fn with_ghost_stones(
        mut self,
        good: Rgba,
        interesting: Rgba,
        doubtful: Rgba,
        bad: Rgba,
    ) -> Self {
        self.ghost_good_color = good;
        self.ghost_interesting_color = interesting;
        self.ghost_doubtful_color = doubtful;
        self.ghost_bad_color = bad;
        self
    }

    /// Sets line and arrow styling
    pub fn with_lines(mut self, color: Rgba, width: f32, arrow_size: f32) -> Self {
        self.line_default_color = color;
        self.line_default_width = width;
        self.arrow_head_size = arrow_size;
        self
    }

    /// Sets heat map gradient colors
    pub fn with_heat_gradient(mut self, colors: Vec<Rgba>) -> Self {
        self.heat_gradient_colors = colors;
        self
    }

    /// Gets the heat map color for a given strength value (0-9)
    pub fn get_heat_color(&self, strength: u8) -> Rgba {
        let index = (strength as usize).min(self.heat_gradient_colors.len() - 1);
        self.heat_gradient_colors
            .get(index)
            .copied()
            .unwrap_or(rgb(0x888888)) // Fallback gray
    }

    /// Validates the theme configuration and returns any issues
    pub fn validate(&self) -> Vec<String> {
        let mut issues = Vec::new();

        // Validate ratios are in valid range
        if self.stone_size_ratio < 0.1 || self.stone_size_ratio > 1.0 {
            issues.push(format!(
                "stone_size_ratio ({}) should be between 0.1 and 1.0",
                self.stone_size_ratio
            ));
        }

        if self.selection_opacity < 0.0 || self.selection_opacity > 1.0 {
            issues.push(format!(
                "selection_opacity ({}) should be between 0.0 and 1.0",
                self.selection_opacity
            ));
        }

        if self.dimmed_opacity < 0.0 || self.dimmed_opacity > 1.0 {
            issues.push(format!(
                "dimmed_opacity ({}) should be between 0.0 and 1.0",
                self.dimmed_opacity
            ));
        }

        // Validate positive values
        if self.grid_line_width <= 0.0 {
            issues.push("grid_line_width must be positive".to_string());
        }

        if self.star_point_size <= 0.0 {
            issues.push("star_point_size must be positive".to_string());
        }

        if self.coordinate_font_size <= 0.0 {
            issues.push("coordinate_font_size must be positive".to_string());
        }

        // Validate heat gradient has at least 2 colors
        if self.heat_gradient_colors.len() < 2 {
            issues.push("heat_gradient_colors must have at least 2 colors".to_string());
        }

        issues
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_theme() {
        let theme = BoardTheme::default();
        assert_eq!(theme.board_background_color, rgb(0xebb55b));
        assert_eq!(theme.grid_line_width, 1.0);
        assert_eq!(theme.stone_size_ratio, 0.9);
    }

    #[test]
    fn test_minimalist_theme() {
        let theme = BoardTheme::minimalist();
        assert_eq!(theme.board_background_color, rgb(0xfafafa));
        assert_eq!(theme.grid_line_color, rgb(0x666666));
    }

    #[test]
    fn test_dark_theme() {
        let theme = BoardTheme::dark();
        assert_eq!(theme.board_background_color, rgb(0x2d2d2d));
        assert_eq!(theme.marker_default_color, rgb(0xffffff));
    }

    #[test]
    fn test_high_contrast_theme() {
        let theme = BoardTheme::high_contrast();
        assert_eq!(theme.board_background_color, rgb(0xffffff));
        assert_eq!(theme.black_stone_color, rgb(0x000000));
        assert_eq!(theme.white_stone_color, rgb(0xffffff));
        assert_eq!(theme.stone_border_width, 2.0);
    }

    #[test]
    fn test_theme_builder_methods() {
        let theme = BoardTheme::new()
            .with_board_background(rgb(0xff0000))
            .with_grid_lines(rgb(0x00ff00), 2.0)
            .with_stone_colors(rgb(0x000000), rgb(0xffffff));

        assert_eq!(theme.board_background_color, rgb(0xff0000));
        assert_eq!(theme.grid_line_color, rgb(0x00ff00));
        assert_eq!(theme.grid_line_width, 2.0);
        assert_eq!(theme.black_stone_color, rgb(0x000000));
        assert_eq!(theme.white_stone_color, rgb(0xffffff));
    }

    #[test]
    fn test_heat_color_mapping() {
        let theme = BoardTheme::default();

        // Test valid indices
        let color_0 = theme.get_heat_color(0);
        let color_5 = theme.get_heat_color(5);
        let color_9 = theme.get_heat_color(9);

        assert_eq!(color_0, rgb(0x000080)); // Blue
        assert_eq!(color_9, rgb(0xff0000)); // Red

        // Test out of bounds
        let color_15 = theme.get_heat_color(15);
        assert_eq!(color_15, rgb(0xff0000)); // Should clamp to max (red)
    }

    #[test]
    fn test_theme_validation() {
        let mut theme = BoardTheme::default();

        // Valid theme should have no issues
        assert!(theme.validate().is_empty());

        // Invalid stone size ratio
        theme.stone_size_ratio = 1.5;
        let issues = theme.validate();
        assert!(issues
            .iter()
            .any(|issue| issue.contains("stone_size_ratio")));

        // Invalid opacity
        theme.stone_size_ratio = 0.9; // Fix previous issue
        theme.selection_opacity = -0.5;
        let issues = theme.validate();
        assert!(issues
            .iter()
            .any(|issue| issue.contains("selection_opacity")));

        // Invalid grid line width
        theme.selection_opacity = 0.6; // Fix previous issue
        theme.grid_line_width = 0.0;
        let issues = theme.validate();
        assert!(issues.iter().any(|issue| issue.contains("grid_line_width")));
    }
}
