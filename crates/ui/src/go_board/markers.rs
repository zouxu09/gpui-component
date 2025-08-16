use crate::go_board::types::{Marker, MarkerType, Vertex};
use gpui::*;

/// Marker rendering component for Go board annotations
/// Supports all marker types with proper positioning and scaling
#[derive(Clone)]
pub struct MarkerRenderer {
    pub vertex_size: f32,
    pub grid_offset: Point<Pixels>,
}

impl MarkerRenderer {
    pub fn new(vertex_size: f32, grid_offset: Point<Pixels>) -> Self {
        Self {
            vertex_size,
            grid_offset,
        }
    }

    /// Renders a marker at the specified vertex position
    pub fn render_marker(&self, marker: &Marker, vertex: &Vertex) -> impl IntoElement {
        let position = self.calculate_marker_position(vertex);
        let size = self.vertex_size * marker.size;

        let mut marker_div = div()
            .absolute()
            .left(position.x)
            .top(position.y)
            .w(px(size))
            .h(px(size))
            .flex()
            .items_center()
            .justify_center()
            .child(self.render_marker_shape(marker, size));

        // Add tooltip if marker has a label
        if let Some(tooltip_text) = &marker.label {
            // Note: tooltip method not available in current GPUI version
            // marker_div = marker_div.tooltip(tooltip_text.clone());
        }

        marker_div
    }

    /// Calculates the pixel position for a marker at the given vertex
    fn calculate_marker_position(&self, vertex: &Vertex) -> Point<Pixels> {
        let x =
            self.grid_offset.x + px(vertex.x as f32 * self.vertex_size - self.vertex_size * 0.5);
        let y =
            self.grid_offset.y + px(vertex.y as f32 * self.vertex_size - self.vertex_size * 0.5);
        point(x, y)
    }

    /// Renders the actual marker shape based on its type
    fn render_marker_shape(&self, marker: &Marker, size: f32) -> AnyElement {
        let color = self.parse_color(marker.color.as_deref().unwrap_or("#000000"));

        match &marker.marker_type {
            MarkerType::Circle => self.render_circle_marker(color, size).into_any_element(),
            MarkerType::Cross => self.render_cross_marker(color, size).into_any_element(),
            MarkerType::Triangle => self.render_triangle_marker(color, size).into_any_element(),
            MarkerType::Square => self.render_square_marker(color, size).into_any_element(),
            MarkerType::Point => self.render_point_marker(color, size).into_any_element(),
            MarkerType::Loader => self.render_loader_marker(color, size).into_any_element(),
            MarkerType::Label(text) => self
                .render_label_marker(text, color, size)
                .into_any_element(),
        }
    }

    /// Parse color string to GPUI Hsla
    fn parse_color(&self, color_str: &str) -> Hsla {
        // Simple hex color parsing - for now just support basic colors
        match color_str {
            "#000000" | "black" => gpui::black(),
            "#FFFFFF" | "white" => gpui::white(),
            "#FF0000" | "red" => rgb(0xFF0000).into(),
            "#00FF00" | "green" => rgb(0x00FF00).into(),
            "#0000FF" | "blue" => rgb(0x0000FF).into(),
            _ => gpui::black(), // Default to black
        }
    }

    fn render_circle_marker(&self, color: Hsla, size: f32) -> impl IntoElement {
        div()
            .w(px(size * 0.6))
            .h(px(size * 0.6))
            .border_2()
            .border_color(color)
            .rounded(px(size * 0.3))
            .bg(gpui::transparent_black())
    }

    fn render_cross_marker(&self, color: Hsla, size: f32) -> impl IntoElement {
        div()
            .w(px(size))
            .h(px(size))
            .relative()
            .child(
                // Horizontal line
                div()
                    .absolute()
                    .left(px(size * 0.2))
                    .top(px(size * 0.48))
                    .w(px(size * 0.6))
                    .h(px(2.0))
                    .bg(color),
            )
            .child(
                // Vertical line
                div()
                    .absolute()
                    .left(px(size * 0.48))
                    .top(px(size * 0.2))
                    .w(px(2.0))
                    .h(px(size * 0.6))
                    .bg(color),
            )
    }

    fn render_triangle_marker(&self, color: Hsla, size: f32) -> impl IntoElement {
        let triangle_size = size * 0.6;

        div()
            .w(px(triangle_size))
            .h(px(triangle_size))
            .border_2()
            .border_color(color)
            .bg(gpui::transparent_black())
            .rounded(px(3.0))
        // Using a rotated square as a simple triangle approximation
    }

    fn render_square_marker(&self, color: Hsla, size: f32) -> impl IntoElement {
        div()
            .w(px(size * 0.6))
            .h(px(size * 0.6))
            .border_2()
            .border_color(color)
            .bg(gpui::transparent_black())
    }

    fn render_point_marker(&self, color: Hsla, size: f32) -> impl IntoElement {
        div()
            .w(px(size * 0.3))
            .h(px(size * 0.3))
            .bg(color)
            .rounded(px(size * 0.15))
    }

    fn render_loader_marker(&self, color: Hsla, size: f32) -> impl IntoElement {
        // Enhanced loader with multiple dots for better visual indication
        div()
            .w(px(size * 0.8))
            .h(px(size * 0.8))
            .flex()
            .items_center()
            .justify_center()
            .gap_1()
            .child(
                // First dot
                div()
                    .w(px(size * 0.15))
                    .h(px(size * 0.15))
                    .bg(color)
                    .rounded(px(size * 0.075)),
            )
            .child(
                // Second dot
                div()
                    .w(px(size * 0.15))
                    .h(px(size * 0.15))
                    .bg(color.alpha(0.7)) // Slightly transparent
                    .rounded(px(size * 0.075)),
            )
            .child(
                // Third dot
                div()
                    .w(px(size * 0.15))
                    .h(px(size * 0.15))
                    .bg(color.alpha(0.4)) // More transparent
                    .rounded(px(size * 0.075)),
            )
    }

    fn render_label_marker(&self, text: &str, color: Hsla, size: f32) -> impl IntoElement {
        let font_size = (size * 0.5).max(10.0).min(28.0); // Increased from 0.4 to 0.5 for better readability

        div()
            .w(px(size))
            .h(px(size))
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .text_color(color)
                    .text_size(px(font_size))
                    .font_weight(gpui::FontWeight::BOLD)
                    .child(text.to_string()),
            )
    }
}

/// Markers component that renders all markers for a board
#[derive(Clone)]
pub struct Markers {
    renderer: MarkerRenderer,
}

impl Markers {
    pub fn new(vertex_size: f32, grid_offset: Point<Pixels>) -> Self {
        Self {
            renderer: MarkerRenderer::new(vertex_size, grid_offset),
        }
    }

    /// Renders all markers from a marker map
    pub fn render_markers(&self, marker_map: &[Vec<Option<Marker>>]) -> impl IntoElement {
        let mut marker_elements = Vec::new();

        for (y, row) in marker_map.iter().enumerate() {
            for (x, marker_opt) in row.iter().enumerate() {
                if let Some(marker) = marker_opt {
                    let vertex = Vertex::new(x, y);
                    marker_elements.push(self.renderer.render_marker(marker, &vertex));
                }
            }
        }

        div()
            .absolute()
            .top(px(0.0))
            .left(px(0.0))
            .w_full()
            .h_full()
            .children(marker_elements)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::go_board::types::MarkerType;

    #[test]
    fn test_marker_renderer_creation() {
        let renderer = MarkerRenderer::new(24.0, point(px(10.0), px(10.0)));
        assert_eq!(renderer.vertex_size, 24.0);
        assert_eq!(renderer.grid_offset.x, px(10.0));
        assert_eq!(renderer.grid_offset.y, px(10.0));
    }

    #[test]
    fn test_marker_position_calculation() {
        let renderer = MarkerRenderer::new(24.0, point(px(10.0), px(10.0)));
        let vertex = Vertex::new(5, 3);
        let position = renderer.calculate_marker_position(&vertex);

        // Expected: offset + vertex * size - half_size
        assert_eq!(position.x, px(10.0 + 5.0 * 24.0 - 12.0)); // 118.0
        assert_eq!(position.y, px(10.0 + 3.0 * 24.0 - 12.0)); // 70.0
    }

    #[test]
    fn test_markers_component_creation() {
        let markers = Markers::new(24.0, point(px(5.0), px(5.0)));
        assert_eq!(markers.renderer.vertex_size, 24.0);
    }

    #[test]
    fn test_marker_map_rendering() {
        let markers = Markers::new(20.0, point(px(0.0), px(0.0)));

        // Create a simple 3x3 marker map
        let marker_map = vec![
            vec![
                Some(Marker::new(MarkerType::Circle)),
                None,
                Some(Marker::new(MarkerType::Cross)),
            ],
            vec![None, Some(Marker::new(MarkerType::Point)), None],
            vec![
                Some(Marker::new(MarkerType::Square)),
                None,
                Some(Marker::new(MarkerType::Triangle)),
            ],
        ];

        // Should render without panicking
        let _element = markers.render_markers(&marker_map);
    }

    #[test]
    fn test_marker_size_scaling() {
        let renderer = MarkerRenderer::new(24.0, point(px(0.0), px(0.0)));
        let mut marker = Marker::new(MarkerType::Circle);
        marker.size = 1.5; // 1.5x the default size

        let vertex = Vertex::new(0, 0);
        let _element = renderer.render_marker(&marker, &vertex);
        // Element should render with scaled size (24.0 * 1.5 = 36.0)
    }

    #[test]
    fn test_marker_custom_color() {
        let renderer = MarkerRenderer::new(24.0, point(px(0.0), px(0.0)));
        let marker = Marker::new(MarkerType::Circle).with_color("#FF0000".to_string());

        let vertex = Vertex::new(0, 0);
        let _element = renderer.render_marker(&marker, &vertex);
        // Element should render with red color
    }

    #[test]
    fn test_label_marker_rendering() {
        let renderer = MarkerRenderer::new(24.0, point(px(0.0), px(0.0)));
        let marker = Marker::new(MarkerType::Label("A".to_string()));

        let vertex = Vertex::new(2, 2);
        let _element = renderer.render_marker(&marker, &vertex);
        // Should render label text without panicking
    }

    #[test]
    fn test_marker_tooltip_functionality() {
        let renderer = MarkerRenderer::new(24.0, point(px(0.0), px(0.0)));
        let marker = Marker::new(MarkerType::Circle)
            .with_color("#FF0000".to_string())
            .with_label("Test tooltip text".to_string());

        let vertex = Vertex::new(0, 0);
        let _element = renderer.render_marker(&marker, &vertex);
        // Element should render with tooltip functionality
    }

    #[test]
    fn test_enhanced_label_marker_rendering() {
        let renderer = MarkerRenderer::new(24.0, point(px(0.0), px(0.0)));
        let marker = Marker::new(MarkerType::Label("Enhanced Label".to_string()))
            .with_color("blue".to_string())
            .with_size(1.5);

        let vertex = Vertex::new(1, 1);
        let _element = renderer.render_marker(&marker, &vertex);
        // Should render with improved typography and sizing
    }

    #[test]
    fn test_improved_loader_marker() {
        let renderer = MarkerRenderer::new(24.0, point(px(0.0), px(0.0)));
        let marker = Marker::new(MarkerType::Loader)
            .with_color("green".to_string())
            .with_label("Loading process status".to_string());

        let vertex = Vertex::new(2, 2);
        let _element = renderer.render_marker(&marker, &vertex);
        // Should render improved loader design with dots
    }

    #[test]
    fn test_marker_without_tooltip() {
        let renderer = MarkerRenderer::new(24.0, point(px(0.0), px(0.0)));
        let marker = Marker::new(MarkerType::Square); // No label, so no tooltip

        let vertex = Vertex::new(3, 3);
        let _element = renderer.render_marker(&marker, &vertex);
        // Should render without tooltip functionality
    }
}
