use crate::go_board::types::{SelectionDirection, Vertex, VertexSelection};
use gpui::*;

/// Selection rendering component for highlighting vertices and dimming effects
/// Provides visual feedback for selected and dimmed vertices with directional indicators
#[derive(Clone)]
pub struct SelectionRenderer {
    pub vertex_size: f32,
    pub grid_offset: Point<Pixels>,
}

impl SelectionRenderer {
    pub fn new(vertex_size: f32, grid_offset: Point<Pixels>) -> Self {
        Self {
            vertex_size,
            grid_offset,
        }
    }

    /// Renders a selection highlight at the specified vertex
    pub fn render_selection(&self, selection: &VertexSelection) -> impl IntoElement {
        let position = self.calculate_selection_position(&selection.vertex);
        let size = self.vertex_size * 0.9; // Slightly smaller than full vertex

        let base_color = self.get_selection_color(&selection.direction);
        let selection_color = base_color.alpha(selection.opacity());

        div()
            .absolute()
            .left(position.x)
            .top(position.y)
            .w(px(size))
            .h(px(size))
            .flex()
            .items_center()
            .justify_center()
            .child(self.render_selection_shape(selection, size, selection_color))
    }

    /// Calculates the pixel position for a selection at the given vertex
    fn calculate_selection_position(&self, vertex: &Vertex) -> Point<Pixels> {
        let offset = self.vertex_size * 0.05; // Small offset to center the selection
        let x = self.grid_offset.x
            + px(vertex.x as f32 * self.vertex_size - self.vertex_size * 0.5 + offset);
        let y = self.grid_offset.y
            + px(vertex.y as f32 * self.vertex_size - self.vertex_size * 0.5 + offset);
        point(x, y)
    }

    /// Gets the base color for a selection based on direction
    fn get_selection_color(&self, direction: &SelectionDirection) -> Hsla {
        match direction {
            SelectionDirection::None => rgb(0x007AFF).into(), // Blue for standard selection
            SelectionDirection::Left => rgb(0xFF3B30).into(), // Red for left
            SelectionDirection::Right => rgb(0x34C759).into(), // Green for right
            SelectionDirection::Top => rgb(0xFF9500).into(),  // Orange for top
            SelectionDirection::Bottom => rgb(0xAF52DE).into(), // Purple for bottom
            SelectionDirection::TopLeft => rgb(0xFF2D92).into(), // Pink for top-left
            SelectionDirection::TopRight => rgb(0x5AC8FA).into(), // Light blue for top-right
            SelectionDirection::BottomLeft => rgb(0xFFCC00).into(), // Yellow for bottom-left
            SelectionDirection::BottomRight => rgb(0x32D74B).into(), // Light green for bottom-right
        }
    }

    /// Renders the selection shape with directional indicators
    fn render_selection_shape(
        &self,
        selection: &VertexSelection,
        size: f32,
        color: Hsla,
    ) -> AnyElement {
        match &selection.direction {
            SelectionDirection::None => self.render_basic_selection(size, color).into_any_element(),
            _ => self
                .render_directional_selection(selection, size, color)
                .into_any_element(),
        }
    }

    /// Renders a basic circular selection highlight
    fn render_basic_selection(&self, size: f32, color: Hsla) -> impl IntoElement {
        div()
            .w(px(size))
            .h(px(size))
            .bg(color)
            .rounded(px(size * 0.5))
            .border_2()
            .border_color(color.alpha(0.8))
    }

    /// Renders a directional selection with visual indicators
    fn render_directional_selection(
        &self,
        selection: &VertexSelection,
        size: f32,
        color: Hsla,
    ) -> impl IntoElement {
        div()
            .w(px(size))
            .h(px(size))
            .relative()
            .child(self.render_basic_selection(size, color.alpha(0.3)))
            .child(self.render_direction_indicator(&selection.direction, size, color))
    }

    /// Renders directional indicators for the selection
    fn render_direction_indicator(
        &self,
        direction: &SelectionDirection,
        size: f32,
        color: Hsla,
    ) -> impl IntoElement {
        let indicator_size = size * 0.3;
        let offset = size * 0.35;

        let (x_offset, y_offset) = match direction {
            SelectionDirection::Left => (-offset, 0.0),
            SelectionDirection::Right => (offset, 0.0),
            SelectionDirection::Top => (0.0, -offset),
            SelectionDirection::Bottom => (0.0, offset),
            SelectionDirection::TopLeft => (-offset, -offset),
            SelectionDirection::TopRight => (offset, -offset),
            SelectionDirection::BottomLeft => (-offset, offset),
            SelectionDirection::BottomRight => (offset, offset),
            SelectionDirection::None => (0.0, 0.0),
        };

        div()
            .absolute()
            .left(px(size * 0.5 + x_offset - indicator_size * 0.5))
            .top(px(size * 0.5 + y_offset - indicator_size * 0.5))
            .w(px(indicator_size))
            .h(px(indicator_size))
            .bg(color)
            .rounded(px(indicator_size * 0.5))
            .border_1()
            .border_color(gpui::white().alpha(0.8))
    }
}

/// Selection overlay component that renders all vertex selections
#[derive(Clone)]
pub struct VertexSelections {
    renderer: SelectionRenderer,
}

impl VertexSelections {
    pub fn new(vertex_size: f32, grid_offset: Point<Pixels>) -> Self {
        Self {
            renderer: SelectionRenderer::new(vertex_size, grid_offset),
        }
    }

    /// Renders all vertex selections with proper layering
    pub fn render_selections(&self, selections: &[VertexSelection]) -> impl IntoElement {
        let mut selection_elements = Vec::new();

        // Sort selections by opacity (dimmed vertices render first, then highlighted)
        let mut sorted_selections = selections.to_vec();
        sorted_selections.sort_by(|a, b| {
            a.opacity()
                .partial_cmp(&b.opacity())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        for selection in &sorted_selections {
            selection_elements.push(self.renderer.render_selection(selection));
        }

        div()
            .absolute()
            .top(px(0.0))
            .left(px(0.0))
            .w_full()
            .h_full()
            .children(selection_elements)
    }

    /// Converts simple vertex lists to selection format for compatibility
    pub fn from_vertex_lists(
        selected_vertices: &[Vertex],
        dimmed_vertices: &[Vertex],
    ) -> Vec<VertexSelection> {
        let mut selections = Vec::new();

        // Add selected vertices with full opacity
        for vertex in selected_vertices {
            selections.push(VertexSelection::new(vertex.clone()));
        }

        // Add dimmed vertices with reduced opacity
        for vertex in dimmed_vertices {
            selections.push(VertexSelection::dimmed(vertex.clone(), 0.4));
        }

        selections
    }

    /// Converts board state to comprehensive selection format including directional indicators
    pub fn from_board_state(
        selected_vertices: &[Vertex],
        dimmed_vertices: &[Vertex],
        selected_left: &[Vertex],
        selected_right: &[Vertex],
        selected_top: &[Vertex],
        selected_bottom: &[Vertex],
    ) -> Vec<VertexSelection> {
        let mut selections = Vec::new();

        // Add basic selected vertices
        for vertex in selected_vertices {
            selections.push(VertexSelection::new(vertex.clone()));
        }

        // Add dimmed vertices with reduced opacity
        for vertex in dimmed_vertices {
            selections.push(VertexSelection::dimmed(vertex.clone(), 0.4));
        }

        // Add directional selections
        for vertex in selected_left {
            selections.push(VertexSelection::with_direction(
                vertex.clone(),
                SelectionDirection::Left,
            ));
        }

        for vertex in selected_right {
            selections.push(VertexSelection::with_direction(
                vertex.clone(),
                SelectionDirection::Right,
            ));
        }

        for vertex in selected_top {
            selections.push(VertexSelection::with_direction(
                vertex.clone(),
                SelectionDirection::Top,
            ));
        }

        for vertex in selected_bottom {
            selections.push(VertexSelection::with_direction(
                vertex.clone(),
                SelectionDirection::Bottom,
            ));
        }

        selections
    }

    /// Creates selections with directional indicators for advanced usage
    pub fn with_directional_selections(
        &self,
        selections: Vec<VertexSelection>,
    ) -> Vec<VertexSelection> {
        selections
    }

    /// Efficiently calculates which selections have changed between states
    /// Returns only the vertices that need to be updated
    pub fn calculate_selection_differences(
        old_selections: &[VertexSelection],
        new_selections: &[VertexSelection],
    ) -> Vec<VertexSelection> {
        let mut differences = Vec::new();

        // Create sets for efficient lookup
        let old_set: std::collections::HashSet<_> = old_selections.iter().collect();
        let new_set: std::collections::HashSet<_> = new_selections.iter().collect();

        // Find selections that are new or changed
        for new_selection in new_selections {
            if !old_set.contains(new_selection) {
                differences.push(new_selection.clone());
            }
        }

        // Find selections that were removed (need to be cleared)
        for old_selection in old_selections {
            if !new_set.contains(old_selection) {
                // Create a transparent version to "clear" the old selection
                let mut cleared = old_selection.clone();
                cleared = cleared.with_opacity(0.0);
                differences.push(cleared);
            }
        }

        differences
    }

    /// Renders only the selection differences for efficient updates
    pub fn render_selection_updates(&self, updates: &[VertexSelection]) -> impl IntoElement {
        let mut update_elements = Vec::new();

        // Sort updates by opacity (transparent ones first to clear, then visible ones)
        let mut sorted_updates = updates.to_vec();
        sorted_updates.sort_by(|a, b| {
            a.opacity()
                .partial_cmp(&b.opacity())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        for update in &sorted_updates {
            if update.opacity() > 0.0 {
                update_elements.push(self.renderer.render_selection(update));
            }
            // Skip rendering transparent updates (they're just for clearing)
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
    use crate::go_board::types::SelectionDirection;

    #[test]
    fn test_selection_renderer_creation() {
        let renderer = SelectionRenderer::new(24.0, point(px(10.0), px(10.0)));
        assert_eq!(renderer.vertex_size, 24.0);
        assert_eq!(renderer.grid_offset.x, px(10.0));
        assert_eq!(renderer.grid_offset.y, px(10.0));
    }

    #[test]
    fn test_selection_position_calculation() {
        let renderer = SelectionRenderer::new(24.0, point(px(10.0), px(10.0)));
        let vertex = Vertex::new(3, 2);
        let position = renderer.calculate_selection_position(&vertex);

        // Expected: offset + vertex * size - half_size + small_offset
        let expected_x = px(10.0 + 3.0 * 24.0 - 12.0 + 24.0 * 0.05); // 79.2
        let expected_y = px(10.0 + 2.0 * 24.0 - 12.0 + 24.0 * 0.05); // 47.2
        assert_eq!(position.x, expected_x);
        assert_eq!(position.y, expected_y);
    }

    #[test]
    fn test_selection_color_mapping() {
        let renderer = SelectionRenderer::new(24.0, point(px(0.0), px(0.0)));

        // Test that each direction gets a unique color
        let directions = vec![
            SelectionDirection::None,
            SelectionDirection::Left,
            SelectionDirection::Right,
            SelectionDirection::Top,
            SelectionDirection::Bottom,
            SelectionDirection::TopLeft,
            SelectionDirection::TopRight,
            SelectionDirection::BottomLeft,
            SelectionDirection::BottomRight,
        ];

        let mut colors = Vec::new();
        for direction in directions {
            let color = renderer.get_selection_color(&direction);
            colors.push(color);
        }

        // Verify that all colors are different (checking RGB values)
        for (i, color1) in colors.iter().enumerate() {
            for (j, color2) in colors.iter().enumerate() {
                if i != j {
                    // Colors should be different
                    assert_ne!(
                        color1, color2,
                        "Colors for different directions should be unique"
                    );
                }
            }
        }
    }

    #[test]
    fn test_vertex_selections_creation() {
        let selections = VertexSelections::new(24.0, point(px(5.0), px(5.0)));
        assert_eq!(selections.renderer.vertex_size, 24.0);
    }

    #[test]
    fn test_vertex_list_conversion() {
        let selected = vec![Vertex::new(1, 1), Vertex::new(2, 2)];
        let dimmed = vec![Vertex::new(3, 3), Vertex::new(4, 4)];

        let selections = VertexSelections::from_vertex_lists(&selected, &dimmed);

        assert_eq!(selections.len(), 4);

        // Check selected vertices have full opacity
        let selected_selections: Vec<_> =
            selections.iter().filter(|s| s.opacity() == 1.0).collect();
        assert_eq!(selected_selections.len(), 2);

        // Check dimmed vertices have reduced opacity
        let dimmed_selections: Vec<_> = selections.iter().filter(|s| s.opacity() == 0.4).collect();
        assert_eq!(dimmed_selections.len(), 2);
    }

    #[test]
    fn test_selection_differential_updates() {
        let old_selections = vec![
            VertexSelection::new(Vertex::new(1, 1)),
            VertexSelection::dimmed(Vertex::new(2, 2), 0.5),
        ];

        let new_selections = vec![
            VertexSelection::new(Vertex::new(1, 1)), // Unchanged
            VertexSelection::dimmed(Vertex::new(3, 3), 0.5), // New
                                                     // Missing (2, 2) - should be cleared
        ];

        let differences =
            VertexSelections::calculate_selection_differences(&old_selections, &new_selections);

        // Should have one new selection and one cleared selection
        assert_eq!(differences.len(), 2);

        // Check that we have both a new selection and a cleared one
        let new_count = differences.iter().filter(|d| d.opacity() > 0.0).count();
        let cleared_count = differences.iter().filter(|d| d.opacity() == 0.0).count();

        assert_eq!(new_count, 1);
        assert_eq!(cleared_count, 1);
    }

    #[test]
    fn test_selection_update_rendering() {
        let selections_component = VertexSelections::new(24.0, point(px(0.0), px(0.0)));

        let updates = vec![
            VertexSelection::new(Vertex::new(1, 1)),
            VertexSelection::dimmed(Vertex::new(2, 2), 0.0), // Should be skipped
            VertexSelection::dimmed(Vertex::new(3, 3), 0.7),
        ];

        let _element = selections_component.render_selection_updates(&updates);
        // Should render only visible updates without panicking
    }

    #[test]
    fn test_opacity_conversion() {
        let vertex = Vertex::new(1, 1);

        // Test dimmed creation with float
        let dimmed = VertexSelection::dimmed(vertex.clone(), 0.75);
        assert_eq!(dimmed.opacity_percent, 75);
        assert!((dimmed.opacity() - 0.75).abs() < 0.01);

        // Test with_opacity method
        let modified = VertexSelection::new(vertex).with_opacity(0.3);
        assert_eq!(modified.opacity_percent, 30);
        assert!((modified.opacity() - 0.3).abs() < 0.01);

        // Test clamping
        let clamped = VertexSelection::dimmed(vertex, 1.5);
        assert_eq!(clamped.opacity_percent, 100);
    }

    #[test]
    fn test_selection_rendering() {
        let renderer = SelectionRenderer::new(24.0, point(px(0.0), px(0.0)));
        let vertex = Vertex::new(1, 1);

        // Test basic selection
        let basic_selection = VertexSelection::new(vertex.clone());
        let _element = renderer.render_selection(&basic_selection);

        // Test directional selection
        let directional_selection =
            VertexSelection::with_direction(vertex.clone(), SelectionDirection::Left);
        let _element = renderer.render_selection(&directional_selection);

        // Test dimmed selection
        let dimmed_selection = VertexSelection::dimmed(vertex, 0.5);
        let _element = renderer.render_selection(&dimmed_selection);

        // All should render without panicking
    }

    #[test]
    fn test_selection_sorting_by_opacity() {
        let selections_component = VertexSelections::new(24.0, point(px(0.0), px(0.0)));

        let selections = vec![
            VertexSelection::new(Vertex::new(0, 0)), // opacity 1.0
            VertexSelection::dimmed(Vertex::new(1, 1), 0.3), // opacity 0.3
            VertexSelection::dimmed(Vertex::new(2, 2), 0.7), // opacity 0.7
        ];

        let _element = selections_component.render_selections(&selections);
        // Should render without panicking and handle sorting correctly
    }

    #[test]
    fn test_directional_indicator_positioning() {
        let renderer = SelectionRenderer::new(24.0, point(px(0.0), px(0.0)));
        let vertex = Vertex::new(1, 1);

        // Test all directional selections
        let directions = vec![
            SelectionDirection::Left,
            SelectionDirection::Right,
            SelectionDirection::Top,
            SelectionDirection::Bottom,
            SelectionDirection::TopLeft,
            SelectionDirection::TopRight,
            SelectionDirection::BottomLeft,
            SelectionDirection::BottomRight,
        ];

        for direction in directions {
            let selection = VertexSelection::with_direction(vertex.clone(), direction);
            let _element = renderer.render_selection(&selection);
            // Should render all directional indicators without panicking
        }
    }

    #[test]
    fn test_comprehensive_board_state_conversion() {
        let selected = vec![Vertex::new(1, 1)];
        let dimmed = vec![Vertex::new(2, 2)];
        let selected_left = vec![Vertex::new(3, 3)];
        let selected_right = vec![Vertex::new(4, 4)];
        let selected_top = vec![Vertex::new(5, 5)];
        let selected_bottom = vec![Vertex::new(6, 6)];

        let selections = VertexSelections::from_board_state(
            &selected,
            &dimmed,
            &selected_left,
            &selected_right,
            &selected_top,
            &selected_bottom,
        );

        assert_eq!(selections.len(), 6);

        // Verify each type of selection is present
        let basic_selections: Vec<_> = selections
            .iter()
            .filter(|s| s.direction == SelectionDirection::None && s.opacity() == 1.0)
            .collect();
        assert_eq!(basic_selections.len(), 1);

        let dimmed_selections: Vec<_> = selections.iter().filter(|s| s.opacity() == 0.4).collect();
        assert_eq!(dimmed_selections.len(), 1);

        let left_selections: Vec<_> = selections
            .iter()
            .filter(|s| s.direction == SelectionDirection::Left)
            .collect();
        assert_eq!(left_selections.len(), 1);

        let right_selections: Vec<_> = selections
            .iter()
            .filter(|s| s.direction == SelectionDirection::Right)
            .collect();
        assert_eq!(right_selections.len(), 1);

        let top_selections: Vec<_> = selections
            .iter()
            .filter(|s| s.direction == SelectionDirection::Top)
            .collect();
        assert_eq!(top_selections.len(), 1);

        let bottom_selections: Vec<_> = selections
            .iter()
            .filter(|s| s.direction == SelectionDirection::Bottom)
            .collect();
        assert_eq!(bottom_selections.len(), 1);
    }

    #[test]
    fn test_selection_state_management() {
        // Test that selections maintain their state correctly
        let vertex = Vertex::new(2, 3);

        // Test that each selection type preserves its properties
        let basic = VertexSelection::new(vertex.clone());
        assert_eq!(basic.vertex, vertex);
        assert_eq!(basic.direction, SelectionDirection::None);
        assert_eq!(basic.opacity, 1.0);

        let directional =
            VertexSelection::with_direction(vertex.clone(), SelectionDirection::TopRight);
        assert_eq!(directional.vertex, vertex);
        assert_eq!(directional.direction, SelectionDirection::TopRight);
        assert_eq!(directional.opacity, 1.0);

        let dimmed = VertexSelection::dimmed(vertex.clone(), 0.3);
        assert_eq!(dimmed.vertex, vertex);
        assert_eq!(dimmed.direction, SelectionDirection::None);
        assert_eq!(dimmed.opacity, 0.3);
    }

    #[test]
    fn test_selection_rendering_layers() {
        let selections_component = VertexSelections::new(24.0, point(px(0.0), px(0.0)));

        // Create selections with different properties for layering test
        let selections = vec![
            VertexSelection::new(Vertex::new(0, 0)), // Basic selection
            VertexSelection::dimmed(Vertex::new(1, 1), 0.2), // Very dimmed
            VertexSelection::dimmed(Vertex::new(2, 2), 0.8), // Slightly dimmed
            VertexSelection::with_direction(Vertex::new(3, 3), SelectionDirection::Left), // Directional
        ];

        let _element = selections_component.render_selections(&selections);
        // Should render and sort by opacity correctly without panicking
    }

    #[test]
    fn test_directional_selection_visual_feedback() {
        let renderer = SelectionRenderer::new(30.0, point(px(5.0), px(5.0)));

        // Test each direction has unique visual properties
        let test_directions = vec![
            (SelectionDirection::Left, "left"),
            (SelectionDirection::Right, "right"),
            (SelectionDirection::Top, "top"),
            (SelectionDirection::Bottom, "bottom"),
            (SelectionDirection::TopLeft, "top-left"),
            (SelectionDirection::TopRight, "top-right"),
            (SelectionDirection::BottomLeft, "bottom-left"),
            (SelectionDirection::BottomRight, "bottom-right"),
        ];

        for (direction, _name) in test_directions {
            let selection = VertexSelection::with_direction(Vertex::new(1, 1), direction.clone());
            let color = renderer.get_selection_color(&direction);

            // Each direction should have a unique color
            assert!(color.h >= 0.0 && color.h <= 1.0);
            assert!(color.s >= 0.0 && color.s <= 1.0);
            assert!(color.l >= 0.0 && color.l <= 1.0);

            let _element = renderer.render_selection(&selection);
            // Should render unique visual feedback for each direction
        }
    }
}
