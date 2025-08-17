use crate::go_board::types::{PaintMap, Vertex};
use gpui::*;

/// Directional paint mapping for edge-based territory marking
/// Supports painting on specific sides of vertices for precise territory control
#[derive(Clone, Debug, PartialEq)]
pub struct DirectionalPaintMap {
    pub left: Vec<Vec<f32>>,
    pub right: Vec<Vec<f32>>,
    pub top: Vec<Vec<f32>>,
    pub bottom: Vec<Vec<f32>>,
    pub corners: Vec<Vec<CornerPaint>>,
}

impl DirectionalPaintMap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            left: vec![vec![0.0; width]; height],
            right: vec![vec![0.0; width]; height],
            top: vec![vec![0.0; width]; height],
            bottom: vec![vec![0.0; width]; height],
            corners: vec![vec![CornerPaint::default(); width]; height],
        }
    }

    /// Checks if any directional paint exists at a vertex
    pub fn has_paint(&self, vertex: &Vertex) -> bool {
        if vertex.y >= self.left.len() || vertex.x >= self.left[vertex.y].len() {
            return false;
        }

        self.left[vertex.y][vertex.x] != 0.0
            || self.right[vertex.y][vertex.x] != 0.0
            || self.top[vertex.y][vertex.x] != 0.0
            || self.bottom[vertex.y][vertex.x] != 0.0
            || self.corners[vertex.y][vertex.x] != CornerPaint::default()
    }
}

/// Corner paint configuration for precise corner territory marking
#[derive(Clone, Debug, PartialEq)]
pub struct CornerPaint {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_left: f32,
    pub bottom_right: f32,
}

impl Default for CornerPaint {
    fn default() -> Self {
        Self {
            top_left: 0.0,
            top_right: 0.0,
            bottom_left: 0.0,
            bottom_right: 0.0,
        }
    }
}

impl CornerPaint {
    pub fn new(top_left: f32, top_right: f32, bottom_left: f32, bottom_right: f32) -> Self {
        Self {
            top_left: top_left.clamp(-1.0, 1.0),
            top_right: top_right.clamp(-1.0, 1.0),
            bottom_left: bottom_left.clamp(-1.0, 1.0),
            bottom_right: bottom_right.clamp(-1.0, 1.0),
        }
    }

    /// Checks if any corner has paint
    pub fn has_paint(&self) -> bool {
        self.top_left != 0.0
            || self.top_right != 0.0
            || self.bottom_left != 0.0
            || self.bottom_right != 0.0
    }
}

/// Paint overlay renderer for territory marking and analysis visualization
/// Supports full vertex painting and directional edge painting
#[derive(Clone)]
pub struct PaintOverlayRenderer {
    pub vertex_size: f32,
    pub grid_offset: Point<Pixels>,
}

impl PaintOverlayRenderer {
    pub fn new(vertex_size: f32, grid_offset: Point<Pixels>) -> Self {
        Self {
            vertex_size,
            grid_offset,
        }
    }

    /// Renders a single paint cell with the specified intensity
    pub fn render_paint_cell(&self, vertex: &Vertex, intensity: f32) -> impl IntoElement {
        let position = self.calculate_paint_position(vertex);
        let size = self.vertex_size * 0.8; // Slightly smaller to avoid grid line overlap
        let color = self.get_paint_color(intensity);

        div()
            .absolute()
            .left(position.x)
            .top(position.y)
            .w(px(size))
            .h(px(size))
            .bg(color)
            .rounded(px(size * 0.1)) // Slight rounding for smooth appearance
    }

    /// Renders directional paint on specific edges of a vertex
    pub fn render_directional_paint(
        &self,
        vertex: &Vertex,
        direction: PaintDirection,
        intensity: f32,
    ) -> impl IntoElement {
        let base_position = self.calculate_paint_position(vertex);
        let (position, size) = self.calculate_directional_dimensions(base_position, direction);
        let color = self.get_paint_color(intensity);

        div()
            .absolute()
            .left(position.x)
            .top(position.y)
            .w(size.width)
            .h(size.height)
            .bg(color)
            .rounded(px(2.0)) // Small rounding for directional paint
    }

    /// Renders corner paint with precise triangular regions
    pub fn render_corner_paint(
        &self,
        vertex: &Vertex,
        corner: CornerPosition,
        intensity: f32,
    ) -> impl IntoElement {
        let base_position = self.calculate_paint_position(vertex);
        let corner_size = self.vertex_size * 0.2;
        let color = self.get_paint_color(intensity);

        // Use small colored squares for corner paint
        // Since GPUI doesn't have transforms, we just position small squares at corners
        let position = self.calculate_corner_position(base_position, corner);

        div()
            .absolute()
            .left(position.x)
            .top(position.y)
            .w(px(corner_size))
            .h(px(corner_size))
            .bg(color)
            .rounded(px(1.0))
    }

    /// Calculates the pixel position for paint at the given vertex
    fn calculate_paint_position(&self, vertex: &Vertex) -> Point<Pixels> {
        let offset = self.vertex_size * 0.1; // Small offset to center the paint
        let x = self.grid_offset.x
            + px(vertex.x as f32 * self.vertex_size - self.vertex_size * 0.4 + offset);
        let y = self.grid_offset.y
            + px(vertex.y as f32 * self.vertex_size - self.vertex_size * 0.4 + offset);
        point(x, y)
    }

    /// Calculates position and size for directional paint elements
    fn calculate_directional_dimensions(
        &self,
        base_position: Point<Pixels>,
        direction: PaintDirection,
    ) -> (Point<Pixels>, Size<Pixels>) {
        let full_size = self.vertex_size * 0.8;
        let edge_thickness = self.vertex_size * 0.15;

        match direction {
            PaintDirection::Left => (
                point(base_position.x, base_position.y),
                size(px(edge_thickness), px(full_size)),
            ),
            PaintDirection::Right => (
                point(
                    base_position.x + px(full_size - edge_thickness),
                    base_position.y,
                ),
                size(px(edge_thickness), px(full_size)),
            ),
            PaintDirection::Top => (
                point(base_position.x, base_position.y),
                size(px(full_size), px(edge_thickness)),
            ),
            PaintDirection::Bottom => (
                point(
                    base_position.x,
                    base_position.y + px(full_size - edge_thickness),
                ),
                size(px(full_size), px(edge_thickness)),
            ),
        }
    }

    /// Calculates corner position for corner paint elements
    fn calculate_corner_position(
        &self,
        base_position: Point<Pixels>,
        corner: CornerPosition,
    ) -> Point<Pixels> {
        let offset = self.vertex_size * 0.1;

        match corner {
            CornerPosition::TopLeft => {
                point(base_position.x + px(offset), base_position.y + px(offset))
            }
            CornerPosition::TopRight => point(
                base_position.x + px(self.vertex_size * 0.6),
                base_position.y + px(offset),
            ),
            CornerPosition::BottomLeft => point(
                base_position.x + px(offset),
                base_position.y + px(self.vertex_size * 0.6),
            ),
            CornerPosition::BottomRight => point(
                base_position.x + px(self.vertex_size * 0.6),
                base_position.y + px(self.vertex_size * 0.6),
            ),
        }
    }

    /// Converts paint intensity to color with proper alpha blending
    fn get_paint_color(&self, intensity: f32) -> Hsla {
        let clamped_intensity = intensity.clamp(-1.0, 1.0);
        let alpha = clamped_intensity.abs() * 0.4; // Semi-transparent for subtle overlay

        if clamped_intensity > 0.0 {
            // Positive intensity: black territory (dark blue)
            let base_color: Hsla = rgb(0x1f3a93).into();
            base_color.alpha(alpha)
        } else if clamped_intensity < 0.0 {
            // Negative intensity: white territory (light gray)
            let base_color: Hsla = rgb(0xe5e5e5).into();
            base_color.alpha(alpha)
        } else {
            // No paint
            gpui::transparent_black().into()
        }
    }
}

/// Direction for edge-based directional painting
#[derive(Clone, Debug, PartialEq)]
pub enum PaintDirection {
    Left,
    Right,
    Top,
    Bottom,
}

/// Corner position for precise corner painting
#[derive(Clone, Debug, PartialEq)]
pub enum CornerPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// Main paint overlay component that manages and renders all paint elements
#[derive(Clone)]
pub struct PaintOverlay {
    renderer: PaintOverlayRenderer,
}

impl PaintOverlay {
    pub fn new(vertex_size: f32, grid_offset: Point<Pixels>) -> Self {
        Self {
            renderer: PaintOverlayRenderer::new(vertex_size, grid_offset),
        }
    }

    /// Renders the complete paint overlay from paint map data
    pub fn render_paint_overlay(
        &self,
        paint_map: &PaintMap,
        directional_paint: Option<&DirectionalPaintMap>,
    ) -> impl IntoElement {
        let mut paint_elements = Vec::new();

        // Render basic paint map
        for (y, row) in paint_map.iter().enumerate() {
            for (x, &intensity) in row.iter().enumerate() {
                if intensity != 0.0 {
                    let vertex = Vertex::new(x, y);
                    paint_elements.push(
                        self.renderer
                            .render_paint_cell(&vertex, intensity)
                            .into_any_element(),
                    );
                }
            }
        }

        // Render directional paint if provided
        if let Some(dir_paint) = directional_paint {
            paint_elements.extend(self.render_directional_overlay(dir_paint));
        }

        div()
            .absolute()
            .top(px(0.0))
            .left(px(0.0))
            .w_full()
            .h_full()
            .children(paint_elements)
    }

    /// Renders all directional paint elements
    fn render_directional_overlay(
        &self,
        directional_paint: &DirectionalPaintMap,
    ) -> Vec<AnyElement> {
        let mut elements = Vec::new();

        // Render edge-based directional paint
        for (y, row) in directional_paint.left.iter().enumerate() {
            for (x, &intensity) in row.iter().enumerate() {
                if intensity != 0.0 {
                    let vertex = Vertex::new(x, y);
                    elements.push(
                        self.renderer
                            .render_directional_paint(&vertex, PaintDirection::Left, intensity)
                            .into_any_element(),
                    );
                }
            }
        }

        for (y, row) in directional_paint.right.iter().enumerate() {
            for (x, &intensity) in row.iter().enumerate() {
                if intensity != 0.0 {
                    let vertex = Vertex::new(x, y);
                    elements.push(
                        self.renderer
                            .render_directional_paint(&vertex, PaintDirection::Right, intensity)
                            .into_any_element(),
                    );
                }
            }
        }

        for (y, row) in directional_paint.top.iter().enumerate() {
            for (x, &intensity) in row.iter().enumerate() {
                if intensity != 0.0 {
                    let vertex = Vertex::new(x, y);
                    elements.push(
                        self.renderer
                            .render_directional_paint(&vertex, PaintDirection::Top, intensity)
                            .into_any_element(),
                    );
                }
            }
        }

        for (y, row) in directional_paint.bottom.iter().enumerate() {
            for (x, &intensity) in row.iter().enumerate() {
                if intensity != 0.0 {
                    let vertex = Vertex::new(x, y);
                    elements.push(
                        self.renderer
                            .render_directional_paint(&vertex, PaintDirection::Bottom, intensity)
                            .into_any_element(),
                    );
                }
            }
        }

        // Render corner paint
        for (y, row) in directional_paint.corners.iter().enumerate() {
            for (x, corner_paint) in row.iter().enumerate() {
                if corner_paint.has_paint() {
                    let vertex = Vertex::new(x, y);

                    if corner_paint.top_left != 0.0 {
                        elements.push(
                            self.renderer
                                .render_corner_paint(
                                    &vertex,
                                    CornerPosition::TopLeft,
                                    corner_paint.top_left,
                                )
                                .into_any_element(),
                        );
                    }
                    if corner_paint.top_right != 0.0 {
                        elements.push(
                            self.renderer
                                .render_corner_paint(
                                    &vertex,
                                    CornerPosition::TopRight,
                                    corner_paint.top_right,
                                )
                                .into_any_element(),
                        );
                    }
                    if corner_paint.bottom_left != 0.0 {
                        elements.push(
                            self.renderer
                                .render_corner_paint(
                                    &vertex,
                                    CornerPosition::BottomLeft,
                                    corner_paint.bottom_left,
                                )
                                .into_any_element(),
                        );
                    }
                    if corner_paint.bottom_right != 0.0 {
                        elements.push(
                            self.renderer
                                .render_corner_paint(
                                    &vertex,
                                    CornerPosition::BottomRight,
                                    corner_paint.bottom_right,
                                )
                                .into_any_element(),
                        );
                    }
                }
            }
        }

        elements
    }

    /// Creates a paint overlay from simple paint map data for basic usage
    pub fn from_paint_map(paint_map: &PaintMap) -> Vec<(Vertex, f32)> {
        let mut paint_data = Vec::new();

        for (y, row) in paint_map.iter().enumerate() {
            for (x, &intensity) in row.iter().enumerate() {
                if intensity != 0.0 {
                    paint_data.push((Vertex::new(x, y), intensity));
                }
            }
        }

        paint_data
    }

    /// Updates paint renderer configuration
    pub fn update_renderer(&mut self, vertex_size: f32, grid_offset: Point<Pixels>) {
        self.renderer = PaintOverlayRenderer::new(vertex_size, grid_offset);
    }

    /// Efficiently renders only changed paint areas for performance optimization
    pub fn render_paint_updates(&self, updates: &[(Vertex, f32)]) -> impl IntoElement {
        let mut update_elements = Vec::new();

        for (vertex, intensity) in updates {
            if *intensity != 0.0 {
                update_elements.push(self.renderer.render_paint_cell(vertex, *intensity));
            }
            // Skip rendering zero-intensity updates (they're clearing operations)
        }

        div()
            .absolute()
            .top(px(0.0))
            .left(px(0.0))
            .w_full()
            .h_full()
            .children(update_elements)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_directional_paint_map_creation() {
        let dir_paint = DirectionalPaintMap::new(9, 9);
        assert_eq!(dir_paint.left.len(), 9);
        assert_eq!(dir_paint.left[0].len(), 9);
        assert_eq!(dir_paint.right.len(), 9);
        assert_eq!(dir_paint.top.len(), 9);
        assert_eq!(dir_paint.bottom.len(), 9);
        assert_eq!(dir_paint.corners.len(), 9);
    }

    #[test]
    fn test_directional_paint_map_has_paint() {
        let mut dir_paint = DirectionalPaintMap::new(3, 3);
        let vertex = Vertex::new(1, 1);

        // Initially no paint
        assert!(!dir_paint.has_paint(&vertex));

        // Add paint to left side
        dir_paint.left[1][1] = 0.5;
        assert!(dir_paint.has_paint(&vertex));

        // Out of bounds should return false
        let invalid_vertex = Vertex::new(10, 10);
        assert!(!dir_paint.has_paint(&invalid_vertex));
    }

    #[test]
    fn test_corner_paint_creation() {
        let corner = CornerPaint::new(0.5, -0.3, 0.8, -0.1);
        assert_eq!(corner.top_left, 0.5);
        assert_eq!(corner.top_right, -0.3);
        assert_eq!(corner.bottom_left, 0.8);
        assert_eq!(corner.bottom_right, -0.1);
        assert!(corner.has_paint());

        let empty_corner = CornerPaint::default();
        assert!(!empty_corner.has_paint());
    }

    #[test]
    fn test_corner_paint_clamping() {
        let corner = CornerPaint::new(1.5, -1.5, 0.5, -0.5);
        assert_eq!(corner.top_left, 1.0); // Clamped to 1.0
        assert_eq!(corner.top_right, -1.0); // Clamped to -1.0
        assert_eq!(corner.bottom_left, 0.5); // Within range
        assert_eq!(corner.bottom_right, -0.5); // Within range
    }

    #[test]
    fn test_paint_overlay_renderer_creation() {
        let renderer = PaintOverlayRenderer::new(24.0, point(px(10.0), px(10.0)));
        assert_eq!(renderer.vertex_size, 24.0);
        assert_eq!(renderer.grid_offset.x, px(10.0));
        assert_eq!(renderer.grid_offset.y, px(10.0));
    }

    #[test]
    fn test_paint_position_calculation() {
        let renderer = PaintOverlayRenderer::new(24.0, point(px(10.0), px(10.0)));
        let vertex = Vertex::new(2, 3);
        let position = renderer.calculate_paint_position(&vertex);

        // Expected: offset + vertex * size - paint_offset + small_offset
        let expected_x = px(10.0 + 2.0 * 24.0 - 24.0 * 0.4 + 24.0 * 0.1); // 56.4
        let expected_y = px(10.0 + 3.0 * 24.0 - 24.0 * 0.4 + 24.0 * 0.1); // 80.4
        assert_eq!(position.x, expected_x);
        assert_eq!(position.y, expected_y);
    }

    #[test]
    fn test_paint_color_mapping() {
        let renderer = PaintOverlayRenderer::new(24.0, point(px(0.0), px(0.0)));

        // Test positive intensity (black territory)
        let black_color = renderer.get_paint_color(0.5);
        assert!(black_color.a > 0.0);
        assert!(black_color.a <= 0.4);

        // Test negative intensity (white territory)
        let white_color = renderer.get_paint_color(-0.5);
        assert!(white_color.a > 0.0);
        assert!(white_color.a <= 0.4);

        // Test zero intensity (no paint)
        let no_color = renderer.get_paint_color(0.0);
        assert_eq!(no_color.a, 0.0);

        // Test clamping
        let clamped_color = renderer.get_paint_color(2.0);
        assert_eq!(clamped_color.a, 0.4); // Should be clamped to 1.0 * 0.4
    }

    #[test]
    fn test_paint_overlay_creation() {
        let overlay = PaintOverlay::new(24.0, point(px(5.0), px(5.0)));
        assert_eq!(overlay.renderer.vertex_size, 24.0);
    }

    #[test]
    fn test_paint_map_conversion() {
        let paint_map = vec![
            vec![0.0, 0.5, 0.0],
            vec![-0.3, 0.0, 0.8],
            vec![0.0, 0.0, 0.0],
        ];

        let paint_data = PaintOverlay::from_paint_map(&paint_map);
        assert_eq!(paint_data.len(), 3); // Only non-zero values

        // Check specific paint data
        assert!(paint_data.contains(&(Vertex::new(1, 0), 0.5)));
        assert!(paint_data.contains(&(Vertex::new(0, 1), -0.3)));
        assert!(paint_data.contains(&(Vertex::new(2, 1), 0.8)));
    }

    #[test]
    fn test_directional_paint_directions() {
        let directions = vec![
            PaintDirection::Left,
            PaintDirection::Right,
            PaintDirection::Top,
            PaintDirection::Bottom,
        ];

        // Test that all directions can be cloned and compared
        for direction in directions {
            let cloned = direction.clone();
            assert_eq!(direction, cloned);
        }
    }

    #[test]
    fn test_corner_positions() {
        let positions = vec![
            CornerPosition::TopLeft,
            CornerPosition::TopRight,
            CornerPosition::BottomLeft,
            CornerPosition::BottomRight,
        ];

        // Test that all positions can be cloned and compared
        for position in positions {
            let cloned = position.clone();
            assert_eq!(position, cloned);
        }
    }

    #[test]
    fn test_paint_overlay_rendering() {
        let overlay = PaintOverlay::new(24.0, point(px(0.0), px(0.0)));
        let paint_map = vec![vec![0.5, 0.0, -0.3], vec![0.0, 0.8, 0.0]];

        let _element = overlay.render_paint_overlay(&paint_map, None);
        // Should render without panicking
    }

    #[test]
    fn test_paint_overlay_with_directional_paint() {
        let overlay = PaintOverlay::new(24.0, point(px(0.0), px(0.0)));
        let paint_map = vec![vec![0.5], vec![0.0]];
        let mut dir_paint = DirectionalPaintMap::new(1, 2);
        dir_paint.left[0][0] = 0.3;
        dir_paint.corners[1][0] = CornerPaint::new(0.2, 0.0, 0.0, 0.0);

        let _element = overlay.render_paint_overlay(&paint_map, Some(&dir_paint));
        // Should render with directional paint without panicking
    }

    #[test]
    fn test_paint_update_rendering() {
        let overlay = PaintOverlay::new(24.0, point(px(0.0), px(0.0)));
        let updates = vec![
            (Vertex::new(1, 1), 0.5),
            (Vertex::new(2, 2), 0.0), // Should be skipped
            (Vertex::new(3, 3), -0.7),
        ];

        let _element = overlay.render_paint_updates(&updates);
        // Should render only non-zero updates without panicking
    }

    #[test]
    fn test_renderer_update() {
        let mut overlay = PaintOverlay::new(20.0, point(px(0.0), px(0.0)));
        overlay.update_renderer(30.0, point(px(5.0), px(5.0)));

        assert_eq!(overlay.renderer.vertex_size, 30.0);
        assert_eq!(overlay.renderer.grid_offset.x, px(5.0));
        assert_eq!(overlay.renderer.grid_offset.y, px(5.0));
    }

    #[test]
    fn test_directional_dimensions_calculation() {
        let renderer = PaintOverlayRenderer::new(30.0, point(px(0.0), px(0.0)));
        let base_position = point(px(10.0), px(10.0));

        // Test left direction
        let (pos, size) =
            renderer.calculate_directional_dimensions(base_position, PaintDirection::Left);
        assert_eq!(pos, base_position);
        assert_eq!(size.width, px(30.0 * 0.15)); // Edge thickness
        assert_eq!(size.height, px(30.0 * 0.8)); // Full size

        // Test top direction
        let (pos, size) =
            renderer.calculate_directional_dimensions(base_position, PaintDirection::Top);
        assert_eq!(pos, base_position);
        assert_eq!(size.width, px(30.0 * 0.8)); // Full size
        assert_eq!(size.height, px(30.0 * 0.15)); // Edge thickness
    }

    #[test]
    fn test_comprehensive_paint_rendering() {
        let overlay = PaintOverlay::new(25.0, point(px(2.0), px(2.0)));

        // Create comprehensive test data
        let paint_map = vec![
            vec![0.5, -0.3, 0.0],
            vec![0.0, 0.8, -0.1],
            vec![-0.9, 0.0, 0.2],
        ];

        let mut dir_paint = DirectionalPaintMap::new(3, 3);
        dir_paint.left[1][1] = 0.4;
        dir_paint.right[0][2] = -0.6;
        dir_paint.top[2][0] = 0.7;
        dir_paint.bottom[1][2] = -0.2;
        dir_paint.corners[0][0] = CornerPaint::new(0.3, 0.0, 0.0, 0.5);

        let _element = overlay.render_paint_overlay(&paint_map, Some(&dir_paint));
        // Should handle complex paint configuration without panicking
    }
}
