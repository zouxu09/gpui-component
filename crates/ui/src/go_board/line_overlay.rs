use crate::go_board::types::{Line, LineType, Vertex};
use gpui::{prelude::*, *};

/// Theme configuration for line and arrow appearance
#[derive(Clone, Debug)]
pub struct LineTheme {
    pub default_color: Hsla,
    pub default_width: f32,
    pub arrow_head_size: f32,
    pub dash_pattern: Option<Vec<f32>>,
}

impl Default for LineTheme {
    fn default() -> Self {
        Self {
            default_color: hsla(0.0, 0.0, 0.2, 0.8), // Dark gray with transparency
            default_width: 2.0,
            arrow_head_size: 8.0,
            dash_pattern: None,
        }
    }
}

/// Line renderer for drawing connections between vertices
/// Supports both simple lines and directional arrows
#[derive(Clone)]
pub struct LineRenderer {
    pub vertex_size: f32,
    pub grid_offset: Point<Pixels>,
    pub theme: LineTheme,
}

impl LineRenderer {
    pub fn new(vertex_size: f32, grid_offset: Point<Pixels>) -> Self {
        Self {
            vertex_size,
            grid_offset,
            theme: LineTheme::default(),
        }
    }

    pub fn with_theme(mut self, theme: LineTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Renders a single line between two vertices
    pub fn render_line(&self, line: &Line) -> AnyElement {
        let start_pos = self.calculate_vertex_center(&line.v1);
        let end_pos = self.calculate_vertex_center(&line.v2);

        match line.line_type {
            LineType::Line => self
                .render_simple_line(start_pos, end_pos)
                .into_any_element(),
            LineType::Arrow => self
                .render_arrow_line(start_pos, end_pos)
                .into_any_element(),
        }
    }

    /// Calculates the center position of a vertex in pixels
    fn calculate_vertex_center(&self, vertex: &Vertex) -> Point<Pixels> {
        // Add half vertex size offset to center on grid intersections
        // This matches the grid's vertex_to_pixel logic
        let grid_offset = self.vertex_size / 2.0;
        let x = self.grid_offset.x + px(vertex.x as f32 * self.vertex_size + grid_offset);
        let y = self.grid_offset.y + px(vertex.y as f32 * self.vertex_size + grid_offset);
        point(x, y)
    }

    /// Renders a simple straight line
    fn render_simple_line(&self, start: Point<Pixels>, end: Point<Pixels>) -> impl IntoElement {
        // For now, use a simple div-based approach
        // Calculate line properties
        let dx = end.x.0 - start.x.0;
        let dy = end.y.0 - start.y.0;

        // Create a simple line using a div positioned between the two points
        div()
            .absolute()
            .left(start.x.min(end.x))
            .top(start.y.min(end.y))
            .w(px(dx.abs().max(self.theme.default_width)))
            .h(px(dy.abs().max(self.theme.default_width)))
            .bg(self.theme.default_color)
    }

    /// Renders a line with an arrow head
    fn render_arrow_line(&self, start: Point<Pixels>, end: Point<Pixels>) -> impl IntoElement {
        // For now, render as a simple line with a marker at the end
        div()
            .absolute()
            .left(start.x.min(end.x))
            .top(start.y.min(end.y))
            .w(px((end.x.0 - start.x.0)
                .abs()
                .max(self.theme.default_width)))
            .h(px((end.y.0 - start.y.0)
                .abs()
                .max(self.theme.default_width)))
            .child(
                // Main line
                div()
                    .w_full()
                    .h(px(self.theme.default_width))
                    .bg(self.theme.default_color),
            )
            .child(
                // Arrow head as a simple triangle marker
                self.render_arrow_head_simple(end),
            )
    }

    /// Renders a simple arrow head at the end position
    fn render_arrow_head_simple(&self, position: Point<Pixels>) -> impl IntoElement {
        let size = self.theme.arrow_head_size;

        // Create a simple triangle using div positioning
        div()
            .absolute()
            .left(px(position.x.0 - size / 2.0))
            .top(px(position.y.0 - size / 2.0))
            .w(px(size))
            .h(px(size))
            .bg(self.theme.default_color)
            .rounded(px(2.0))
    }

    /// Calculates the distance between two points
    fn calculate_distance(&self, start: Point<Pixels>, end: Point<Pixels>) -> f32 {
        let dx = end.x.0 - start.x.0;
        let dy = end.y.0 - start.y.0;
        (dx * dx + dy * dy).sqrt()
    }

    /// Calculates the angle in degrees from start to end point
    fn calculate_angle(&self, start: Point<Pixels>, end: Point<Pixels>) -> f32 {
        let dx = end.x.0 - start.x.0;
        let dy = end.y.0 - start.y.0;
        dy.atan2(dx).to_degrees()
    }

    /// Updates the renderer configuration
    pub fn update_config(&mut self, vertex_size: f32, grid_offset: Point<Pixels>) {
        self.vertex_size = vertex_size;
        self.grid_offset = grid_offset;
    }
}

/// Main line overlay component that manages and renders line visualization
#[derive(Clone)]
pub struct LineOverlay {
    renderer: LineRenderer,
}

impl LineOverlay {
    pub fn new(vertex_size: f32, grid_offset: Point<Pixels>) -> Self {
        Self {
            renderer: LineRenderer::new(vertex_size, grid_offset),
        }
    }

    pub fn with_theme(mut self, theme: LineTheme) -> Self {
        self.renderer = self.renderer.with_theme(theme);
        self
    }

    /// Renders all lines from a line array
    pub fn render_lines(&self, lines: &[Line]) -> impl IntoElement {
        let mut line_elements: Vec<AnyElement> = Vec::new();

        for line in lines {
            line_elements.push(self.renderer.render_line(line));
        }

        div()
            .absolute()
            .top(px(0.0))
            .left(px(0.0))
            .w_full()
            .h_full()
            .children(line_elements)
    }

    /// Creates lines from vertex pairs for basic usage
    pub fn from_vertex_pairs(pairs: &[(Vertex, Vertex)], line_type: LineType) -> Vec<Line> {
        pairs
            .iter()
            .map(|(v1, v2)| Line::new(v1.clone(), v2.clone(), line_type.clone()))
            .collect()
    }

    /// Creates a demonstration pattern showing different line types
    pub fn create_line_demonstration() -> Vec<Line> {
        vec![
            // Horizontal line
            Line::line(Vertex::new(1, 1), Vertex::new(4, 1)),
            // Vertical line
            Line::line(Vertex::new(1, 3), Vertex::new(1, 6)),
            // Diagonal line
            Line::line(Vertex::new(3, 3), Vertex::new(6, 6)),
            // Arrow line
            Line::arrow(Vertex::new(2, 5), Vertex::new(5, 2)),
            // Multiple arrows showing direction
            Line::arrow(Vertex::new(7, 1), Vertex::new(7, 4)),
            Line::arrow(Vertex::new(8, 4), Vertex::new(8, 1)),
        ]
    }

    /// Efficiently renders only specific lines for performance optimization
    pub fn render_line_subset(&self, lines: &[Line], indices: &[usize]) -> impl IntoElement {
        let mut line_elements: Vec<AnyElement> = Vec::new();

        for &index in indices {
            if let Some(line) = lines.get(index) {
                line_elements.push(self.renderer.render_line(line));
            }
        }

        div()
            .absolute()
            .top(px(0.0))
            .left(px(0.0))
            .w_full()
            .h_full()
            .children(line_elements)
    }

    /// Updates renderer configuration
    pub fn update_renderer(&mut self, vertex_size: f32, grid_offset: Point<Pixels>) {
        self.renderer.update_config(vertex_size, grid_offset);
    }

    /// Updates the theme for all lines
    pub fn update_theme(&mut self, theme: LineTheme) {
        self.renderer.theme = theme;
    }

    /// Gets a reference to the current renderer
    pub fn renderer(&self) -> &LineRenderer {
        &self.renderer
    }

    /// Gets a mutable reference to the current renderer
    pub fn renderer_mut(&mut self) -> &mut LineRenderer {
        &mut self.renderer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_theme_default() {
        let theme = LineTheme::default();
        assert_eq!(theme.default_width, 2.0);
        assert_eq!(theme.arrow_head_size, 8.0);
        assert!(theme.dash_pattern.is_none());
    }

    #[test]
    fn test_line_renderer_creation() {
        let renderer = LineRenderer::new(24.0, point(px(10.0), px(10.0)));
        assert_eq!(renderer.vertex_size, 24.0);
        assert_eq!(renderer.grid_offset.x, px(10.0));
        assert_eq!(renderer.grid_offset.y, px(10.0));
    }

    #[test]
    fn test_vertex_center_calculation() {
        let renderer = LineRenderer::new(24.0, point(px(10.0), px(10.0)));
        let vertex = Vertex::new(2, 3);
        let center = renderer.calculate_vertex_center(&vertex);

        // Expected calculation with grid intersection alignment:
        // grid_offset = 24.0 / 2.0 = 12.0
        // x = 10.0 + 2.0 * 24.0 + 12.0 = 70.0
        // y = 10.0 + 3.0 * 24.0 + 12.0 = 94.0
        let expected_x = px(70.0);
        let expected_y = px(94.0);
        assert_eq!(center.x, expected_x);
        assert_eq!(center.y, expected_y);
    }

    #[test]
    fn test_distance_calculation() {
        let renderer = LineRenderer::new(24.0, point(px(0.0), px(0.0)));
        let start = point(px(0.0), px(0.0));
        let end = point(px(3.0), px(4.0));
        let distance = renderer.calculate_distance(start, end);
        assert_eq!(distance, 5.0); // 3-4-5 triangle
    }

    #[test]
    fn test_angle_calculation() {
        let renderer = LineRenderer::new(24.0, point(px(0.0), px(0.0)));

        // Horizontal line (0 degrees)
        let start = point(px(0.0), px(0.0));
        let end = point(px(1.0), px(0.0));
        let angle = renderer.calculate_angle(start, end);
        assert!((angle - 0.0).abs() < 0.001);

        // Vertical line (90 degrees)
        let end_vertical = point(px(0.0), px(1.0));
        let angle_vertical = renderer.calculate_angle(start, end_vertical);
        assert!((angle_vertical - 90.0).abs() < 0.001);
    }

    #[test]
    fn test_line_overlay_creation() {
        let overlay = LineOverlay::new(24.0, point(px(5.0), px(5.0)));
        assert_eq!(overlay.renderer.vertex_size, 24.0);
    }

    #[test]
    fn test_vertex_pairs_conversion() {
        let pairs = vec![
            (Vertex::new(0, 0), Vertex::new(1, 1)),
            (Vertex::new(2, 2), Vertex::new(3, 3)),
        ];

        let lines = LineOverlay::from_vertex_pairs(&pairs, LineType::Line);
        assert_eq!(lines.len(), 2);

        assert_eq!(lines[0].v1, Vertex::new(0, 0));
        assert_eq!(lines[0].v2, Vertex::new(1, 1));
        assert_eq!(lines[0].line_type, LineType::Line);
    }

    #[test]
    fn test_line_demonstration() {
        let lines = LineOverlay::create_line_demonstration();
        assert!(lines.len() > 0);

        // Check we have both line and arrow types
        let has_line = lines.iter().any(|l| l.line_type == LineType::Line);
        let has_arrow = lines.iter().any(|l| l.line_type == LineType::Arrow);
        assert!(has_line);
        assert!(has_arrow);
    }

    #[test]
    fn test_line_rendering() {
        let overlay = LineOverlay::new(24.0, point(px(0.0), px(0.0)));
        let lines = vec![
            Line::line(Vertex::new(0, 0), Vertex::new(2, 2)),
            Line::arrow(Vertex::new(1, 1), Vertex::new(3, 1)),
        ];

        let _element = overlay.render_lines(&lines);
        // Should render without panicking
    }

    #[test]
    fn test_line_subset_rendering() {
        let overlay = LineOverlay::new(24.0, point(px(0.0), px(0.0)));
        let lines = vec![
            Line::line(Vertex::new(0, 0), Vertex::new(1, 1)),
            Line::arrow(Vertex::new(2, 2), Vertex::new(3, 3)),
            Line::line(Vertex::new(4, 4), Vertex::new(5, 5)),
        ];

        let indices = vec![0, 2]; // Render only first and third lines
        let _element = overlay.render_line_subset(&lines, &indices);
        // Should render without panicking
    }

    #[test]
    fn test_renderer_update() {
        let mut overlay = LineOverlay::new(20.0, point(px(0.0), px(0.0)));
        overlay.update_renderer(30.0, point(px(5.0), px(5.0)));

        assert_eq!(overlay.renderer.vertex_size, 30.0);
        assert_eq!(overlay.renderer.grid_offset.x, px(5.0));
        assert_eq!(overlay.renderer.grid_offset.y, px(5.0));
    }

    #[test]
    fn test_theme_update() {
        let mut overlay = LineOverlay::new(24.0, point(px(0.0), px(0.0)));
        let custom_theme = LineTheme {
            default_color: hsla(0.5, 0.8, 0.6, 1.0),
            default_width: 3.0,
            arrow_head_size: 10.0,
            dash_pattern: Some(vec![5.0, 3.0]),
        };

        overlay.update_theme(custom_theme.clone());
        assert_eq!(overlay.renderer.theme.default_width, 3.0);
        assert_eq!(overlay.renderer.theme.arrow_head_size, 10.0);
    }

    #[test]
    fn test_comprehensive_line_rendering() {
        let overlay = LineOverlay::new(25.0, point(px(2.0), px(2.0)));

        // Create comprehensive test data
        let lines = vec![
            Line::line(Vertex::new(0, 0), Vertex::new(1, 1)), // Diagonal line
            Line::line(Vertex::new(2, 0), Vertex::new(2, 2)), // Vertical line
            Line::line(Vertex::new(0, 3), Vertex::new(3, 3)), // Horizontal line
            Line::arrow(Vertex::new(1, 4), Vertex::new(4, 1)), // Diagonal arrow
            Line::arrow(Vertex::new(5, 0), Vertex::new(5, 3)), // Vertical arrow
            Line::arrow(Vertex::new(0, 5), Vertex::new(3, 5)), // Horizontal arrow
        ];

        let _element = overlay.render_lines(&lines);
        // Should handle complex line configuration without panicking
    }
}
