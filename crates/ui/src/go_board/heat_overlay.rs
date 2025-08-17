use crate::go_board::types::{HeatData, HeatMap, Vertex};
use gpui::*;

/// Heat overlay renderer for influence and positional strength visualization
/// Supports strength values from 0-9 with gradient visualization and optional text labels
#[derive(Clone)]
pub struct HeatOverlayRenderer {
    pub vertex_size: f32,
    pub grid_offset: Point<Pixels>,
}

impl HeatOverlayRenderer {
    pub fn new(vertex_size: f32, grid_offset: Point<Pixels>) -> Self {
        Self {
            vertex_size,
            grid_offset,
        }
    }

    /// Renders a single heat cell with the specified strength and optional text
    pub fn render_heat_cell(&self, vertex: &Vertex, heat_data: &HeatData) -> impl IntoElement {
        let position = self.calculate_heat_position(vertex);
        let size = self.vertex_size * 0.75; // Slightly smaller than vertex for good visual separation
        let color = self.get_heat_color(heat_data.strength);

        let base_cell = div()
            .absolute()
            .left(position.x)
            .top(position.y)
            .w(px(size))
            .h(px(size))
            .bg(color)
            .rounded(px(size * 0.1)) // Slight rounding for smooth appearance
            .flex()
            .items_center()
            .justify_center();

        // Add text label if present
        if let Some(ref text) = heat_data.text {
            base_cell.child(
                div()
                    .text_xs()
                    .text_color(self.get_text_color(heat_data.strength))
                    .line_height(relative(1.0))
                    .text_align(gpui::TextAlign::Center)
                    .child(text.clone()),
            )
        } else {
            base_cell
        }
    }

    /// Calculates the pixel position for heat visualization at the given vertex
    fn calculate_heat_position(&self, vertex: &Vertex) -> Point<Pixels> {
        // Add half vertex size offset to center heat on grid intersections
        // This matches the grid's vertex_to_pixel logic
        let grid_offset = self.vertex_size / 2.0;
        let heat_size = self.vertex_size * 0.75; // Heat cell size
        let heat_center_offset = heat_size / 2.0; // Offset to center the heat cell

        let x = self.grid_offset.x
            + px(vertex.x as f32 * self.vertex_size + grid_offset - heat_center_offset);
        let y = self.grid_offset.y
            + px(vertex.y as f32 * self.vertex_size + grid_offset - heat_center_offset);
        point(x, y)
    }

    /// Converts heat strength (0-9) to color with gradient visualization
    fn get_heat_color(&self, strength: u8) -> Hsla {
        let clamped_strength = strength.min(9);
        let intensity = clamped_strength as f32 / 9.0; // Normalize to 0.0-1.0
        let alpha = (0.3 + intensity * 0.5).min(0.8); // Range from 0.3 to 0.8 alpha

        // Create gradient from cool blue (low) to hot red (high)
        if intensity == 0.0 {
            // Transparent for zero strength
            gpui::transparent_black().into()
        } else if intensity <= 0.33 {
            // Blue to cyan range (cool)
            let local_intensity = intensity / 0.33;
            let hue = 240.0 - (local_intensity * 60.0); // 240° (blue) to 180° (cyan)
            hsla(hue / 360.0, 0.8, 0.6, alpha)
        } else if intensity <= 0.66 {
            // Cyan to yellow range (warm)
            let local_intensity = (intensity - 0.33) / 0.33;
            let hue = 180.0 - (local_intensity * 120.0); // 180° (cyan) to 60° (yellow)
            hsla(hue / 360.0, 0.8, 0.6, alpha)
        } else {
            // Yellow to red range (hot)
            let local_intensity = (intensity - 0.66) / 0.34;
            let hue = 60.0 - (local_intensity * 60.0); // 60° (yellow) to 0° (red)
            hsla(hue / 360.0, 0.9, 0.5, alpha)
        }
    }

    /// Gets appropriate text color based on heat strength for good contrast
    fn get_text_color(&self, strength: u8) -> Hsla {
        let intensity = strength as f32 / 9.0;

        if intensity > 0.5 {
            // White text for darker backgrounds
            gpui::white().alpha(0.9).into()
        } else {
            // Dark text for lighter backgrounds
            gpui::black().alpha(0.8).into()
        }
    }
}

/// Main heat overlay component that manages and renders heat map visualization
#[derive(Clone)]
pub struct HeatOverlay {
    renderer: HeatOverlayRenderer,
}

impl HeatOverlay {
    pub fn new(vertex_size: f32, grid_offset: Point<Pixels>) -> Self {
        Self {
            renderer: HeatOverlayRenderer::new(vertex_size, grid_offset),
        }
    }

    /// Renders the complete heat overlay from heat map data
    pub fn render_heat_overlay(&self, heat_map: &HeatMap) -> impl IntoElement {
        let mut heat_elements = Vec::new();

        // Render heat cells with strength and text labels
        for (y, row) in heat_map.iter().enumerate() {
            for (x, heat_option) in row.iter().enumerate() {
                if let Some(heat_data) = heat_option {
                    if heat_data.strength > 0 {
                        let vertex = Vertex::new(x, y);
                        heat_elements.push(
                            self.renderer
                                .render_heat_cell(&vertex, heat_data)
                                .into_any_element(),
                        );
                    }
                }
            }
        }

        div()
            .absolute()
            .top(px(0.0))
            .left(px(0.0))
            .w_full()
            .h_full()
            .children(heat_elements)
    }

    /// Creates a heat overlay from simple strength values for basic usage
    pub fn from_strength_map(strength_map: &[Vec<u8>]) -> Vec<(Vertex, HeatData)> {
        let mut heat_data = Vec::new();

        for (y, row) in strength_map.iter().enumerate() {
            for (x, &strength) in row.iter().enumerate() {
                if strength > 0 {
                    heat_data.push((Vertex::new(x, y), HeatData::new(strength)));
                }
            }
        }

        heat_data
    }

    /// Creates a heat overlay with text labels from strength and text maps
    pub fn from_strength_and_text_maps(
        strength_map: &[Vec<u8>],
        text_map: &[Vec<Option<String>>],
    ) -> Vec<(Vertex, HeatData)> {
        let mut heat_data = Vec::new();

        for (y, (strength_row, text_row)) in strength_map.iter().zip(text_map.iter()).enumerate() {
            for (x, (&strength, text_option)) in
                strength_row.iter().zip(text_row.iter()).enumerate()
            {
                if strength > 0 {
                    let heat = if let Some(ref text) = text_option {
                        HeatData::with_text(strength, text.clone())
                    } else {
                        HeatData::new(strength)
                    };
                    heat_data.push((Vertex::new(x, y), heat));
                }
            }
        }

        heat_data
    }

    /// Updates heat renderer configuration
    pub fn update_renderer(&mut self, vertex_size: f32, grid_offset: Point<Pixels>) {
        self.renderer = HeatOverlayRenderer::new(vertex_size, grid_offset);
    }

    /// Efficiently renders only changed heat areas for performance optimization
    pub fn render_heat_updates(&self, updates: &[(Vertex, HeatData)]) -> impl IntoElement {
        let mut update_elements = Vec::new();

        for (vertex, heat_data) in updates {
            if heat_data.strength > 0 {
                update_elements.push(self.renderer.render_heat_cell(vertex, heat_data));
            }
            // Skip rendering zero-strength updates (they're clearing operations)
        }

        div()
            .absolute()
            .top(px(0.0))
            .left(px(0.0))
            .w_full()
            .h_full()
            .children(update_elements)
    }

    /// Creates a gradient demonstration for testing heat visualization
    pub fn create_gradient_demonstration(width: usize, height: usize) -> HeatMap {
        let mut heat_map = vec![vec![None; width]; height];

        for y in 0..height {
            for x in 0..width {
                // Create circular gradient from center
                let center_x = width as f32 / 2.0;
                let center_y = height as f32 / 2.0;
                let max_distance = ((center_x * center_x) + (center_y * center_y)).sqrt();

                let distance =
                    ((x as f32 - center_x).powi(2) + (y as f32 - center_y).powi(2)).sqrt();
                let normalized_distance = (distance / max_distance).min(1.0);

                // Inverse distance to create stronger values near center
                let strength = ((1.0 - normalized_distance) * 9.0) as u8;

                if strength > 0 {
                    heat_map[y][x] = Some(HeatData::new(strength));
                }
            }
        }

        heat_map
    }

    /// Creates a heat map with text labels for demonstration
    pub fn create_labeled_demonstration(width: usize, height: usize) -> HeatMap {
        let mut heat_map = vec![vec![None; width]; height];

        // Create a pattern with labeled strength values
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                if (x + y) % 3 == 0 {
                    let strength = ((x + y) % 10) as u8;
                    if strength > 0 {
                        let label = if strength >= 7 {
                            format!("H{}", strength) // High
                        } else if strength >= 4 {
                            format!("M{}", strength) // Medium
                        } else {
                            format!("L{}", strength) // Low
                        };
                        heat_map[y][x] = Some(HeatData::with_text(strength, label));
                    }
                }
            }
        }

        heat_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heat_overlay_renderer_creation() {
        let renderer = HeatOverlayRenderer::new(24.0, point(px(10.0), px(10.0)));
        assert_eq!(renderer.vertex_size, 24.0);
        assert_eq!(renderer.grid_offset.x, px(10.0));
        assert_eq!(renderer.grid_offset.y, px(10.0));
    }

    #[test]
    fn test_heat_position_calculation() {
        let renderer = HeatOverlayRenderer::new(24.0, point(px(10.0), px(10.0)));
        let vertex = Vertex::new(2, 3);
        let position = renderer.calculate_heat_position(&vertex);

        // Expected calculation with new alignment logic:
        // grid_offset = 24.0 / 2.0 = 12.0
        // heat_size = 24.0 * 0.75 = 18.0
        // heat_center_offset = 18.0 / 2.0 = 9.0
        // x = 10.0 + 2.0 * 24.0 + 12.0 - 9.0 = 61.0
        // y = 10.0 + 3.0 * 24.0 + 12.0 - 9.0 = 85.0
        let expected_x = px(61.0);
        let expected_y = px(85.0);
        assert_eq!(position.x, expected_x);
        assert_eq!(position.y, expected_y);
    }

    #[test]
    fn test_heat_color_gradient() {
        let renderer = HeatOverlayRenderer::new(24.0, point(px(0.0), px(0.0)));

        // Test strength 0 (transparent)
        let color_0 = renderer.get_heat_color(0);
        assert_eq!(color_0.a, 0.0);

        // Test low strength (blue range)
        let color_3 = renderer.get_heat_color(3);
        assert!(color_3.a > 0.0);
        assert!(color_3.h > 0.5); // Blue-cyan range

        // Test medium strength (yellow range)
        let color_6 = renderer.get_heat_color(6);
        assert!(color_6.a > 0.0);
        assert!(color_6.h < 0.5 && color_6.h > 0.1); // Yellow range

        // Test high strength (red range)
        let color_9 = renderer.get_heat_color(9);
        assert!(color_9.a > 0.0);
        assert!(color_9.h < 0.1); // Red range

        // Test clamping
        let color_clamped = renderer.get_heat_color(15);
        assert_eq!(color_clamped, renderer.get_heat_color(9));
    }

    #[test]
    fn test_text_color_contrast() {
        let renderer = HeatOverlayRenderer::new(24.0, point(px(0.0), px(0.0)));

        // Low strength should use dark text (for light backgrounds)
        let text_color_low = renderer.get_text_color(2);
        assert!(text_color_low.l < 0.5); // Dark text

        // High strength should use light text (for dark backgrounds)
        let text_color_high = renderer.get_text_color(8);
        assert!(text_color_high.l > 0.5); // Light text
    }

    #[test]
    fn test_heat_overlay_creation() {
        let overlay = HeatOverlay::new(24.0, point(px(5.0), px(5.0)));
        assert_eq!(overlay.renderer.vertex_size, 24.0);
    }

    #[test]
    fn test_strength_map_conversion() {
        let strength_map = vec![vec![0, 3, 0], vec![6, 0, 9], vec![0, 0, 0]];

        let heat_data = HeatOverlay::from_strength_map(&strength_map);
        assert_eq!(heat_data.len(), 3); // Only non-zero values

        // Check specific heat data
        assert!(heat_data
            .iter()
            .any(|(v, h)| v == &Vertex::new(1, 0) && h.strength == 3));
        assert!(heat_data
            .iter()
            .any(|(v, h)| v == &Vertex::new(0, 1) && h.strength == 6));
        assert!(heat_data
            .iter()
            .any(|(v, h)| v == &Vertex::new(2, 1) && h.strength == 9));
    }

    #[test]
    fn test_strength_and_text_maps_conversion() {
        let strength_map = vec![vec![5, 0], vec![0, 7]];
        let text_map = vec![
            vec![Some("Low".to_string()), None],
            vec![None, Some("High".to_string())],
        ];

        let heat_data = HeatOverlay::from_strength_and_text_maps(&strength_map, &text_map);
        assert_eq!(heat_data.len(), 2);

        // Check that text labels are correctly assigned
        let low_entry = heat_data
            .iter()
            .find(|(v, _)| v == &Vertex::new(0, 0))
            .unwrap();
        assert_eq!(low_entry.1.text, Some("Low".to_string()));

        let high_entry = heat_data
            .iter()
            .find(|(v, _)| v == &Vertex::new(1, 1))
            .unwrap();
        assert_eq!(high_entry.1.text, Some("High".to_string()));
    }

    #[test]
    fn test_heat_overlay_rendering() {
        let overlay = HeatOverlay::new(24.0, point(px(0.0), px(0.0)));
        let heat_map = vec![
            vec![Some(HeatData::new(3)), None, Some(HeatData::new(7))],
            vec![None, Some(HeatData::with_text(5, "Test".to_string())), None],
        ];

        let _element = overlay.render_heat_overlay(&heat_map);
        // Should render without panicking
    }

    #[test]
    fn test_gradient_demonstration() {
        let heat_map = HeatOverlay::create_gradient_demonstration(5, 5);

        // Center should have highest strength
        assert!(heat_map[2][2].is_some());
        let center_strength = heat_map[2][2].as_ref().unwrap().strength;

        // Corners should have lower strength or be empty
        let corner_strength = heat_map[0][0].as_ref().map(|h| h.strength).unwrap_or(0);
        assert!(center_strength >= corner_strength);
    }

    #[test]
    fn test_labeled_demonstration() {
        let heat_map = HeatOverlay::create_labeled_demonstration(6, 6);

        // Check that some cells have text labels
        let has_labeled_cell = heat_map
            .iter()
            .flatten()
            .any(|cell| cell.as_ref().map(|h| h.text.is_some()).unwrap_or(false));
        assert!(has_labeled_cell);
    }

    #[test]
    fn test_heat_update_rendering() {
        let overlay = HeatOverlay::new(24.0, point(px(0.0), px(0.0)));
        let updates = vec![
            (Vertex::new(1, 1), HeatData::new(5)),
            (Vertex::new(2, 2), HeatData::new(0)), // Should be skipped
            (
                Vertex::new(3, 3),
                HeatData::with_text(8, "Strong".to_string()),
            ),
        ];

        let _element = overlay.render_heat_updates(&updates);
        // Should render only non-zero updates without panicking
    }

    #[test]
    fn test_renderer_update() {
        let mut overlay = HeatOverlay::new(20.0, point(px(0.0), px(0.0)));
        overlay.update_renderer(30.0, point(px(5.0), px(5.0)));

        assert_eq!(overlay.renderer.vertex_size, 30.0);
        assert_eq!(overlay.renderer.grid_offset.x, px(5.0));
        assert_eq!(overlay.renderer.grid_offset.y, px(5.0));
    }

    #[test]
    fn test_heat_strength_clamping() {
        let renderer = HeatOverlayRenderer::new(24.0, point(px(0.0), px(0.0)));

        // Test that strength values above 9 are handled correctly
        let color_10 = renderer.get_heat_color(10);
        let color_9 = renderer.get_heat_color(9);

        // Should be clamped to 9
        assert_eq!(color_10, color_9);
    }

    #[test]
    fn test_comprehensive_heat_rendering() {
        let overlay = HeatOverlay::new(25.0, point(px(2.0), px(2.0)));

        // Create comprehensive test data
        let heat_map = vec![
            vec![
                Some(HeatData::new(1)),
                Some(HeatData::new(3)),
                Some(HeatData::new(5)),
            ],
            vec![
                Some(HeatData::with_text(2, "Low".to_string())),
                None,
                Some(HeatData::with_text(7, "High".to_string())),
            ],
            vec![
                Some(HeatData::new(9)),
                Some(HeatData::new(0)),
                Some(HeatData::with_text(4, "Med".to_string())),
            ],
        ];

        let _element = overlay.render_heat_overlay(&heat_map);
        // Should handle complex heat configuration without panicking
    }
}
