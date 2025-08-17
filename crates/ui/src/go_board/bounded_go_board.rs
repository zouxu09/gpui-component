use crate::go_board::{BoardTheme, GoBoard, Vertex, VertexEventHandlers};
use gpui::*;

/// BoundedGoBoard component that automatically calculates vertex size to fit within constraints
///
/// This component wraps the main GoBoard and provides automatic scaling to fit within
/// maximum width and height constraints while maintaining board proportions.
pub struct BoundedGoBoard {
    board: GoBoard,
    max_width: f32,
    max_height: f32,
    min_vertex_size: f32,
    max_vertex_size: f32,
}

impl BoundedGoBoard {
    /// Creates a new bounded Go board with default 19x19 size
    pub fn new(max_width: f32, max_height: f32) -> Self {
        let mut board = GoBoard::new();
        let calculated_size = Self::calculate_vertex_size(
            max_width,
            max_height,
            board.state().dimensions(),
            1.0,   // min_vertex_size
            100.0, // max_vertex_size
        );
        board.state_mut().vertex_size = calculated_size;

        Self {
            board,
            max_width,
            max_height,
            min_vertex_size: 1.0,
            max_vertex_size: 100.0,
        }
    }

    /// Creates a bounded Go board with specified dimensions
    pub fn with_size(width: usize, height: usize, max_width: f32, max_height: f32) -> Self {
        let mut board = GoBoard::with_size(width, height);
        let calculated_size = Self::calculate_vertex_size(
            max_width,
            max_height,
            (width, height),
            1.0,   // min_vertex_size
            100.0, // max_vertex_size
        );
        board.state_mut().vertex_size = calculated_size;

        Self {
            board,
            max_width,
            max_height,
            min_vertex_size: 1.0,
            max_vertex_size: 100.0,
        }
    }

    /// Creates a bounded Go board with custom vertex size limits
    pub fn with_vertex_size_limits(mut self, min_vertex_size: f32, max_vertex_size: f32) -> Self {
        self.min_vertex_size = min_vertex_size.max(0.1); // Ensure minimum is reasonable
        self.max_vertex_size = max_vertex_size.max(self.min_vertex_size);

        // Recalculate vertex size with new limits
        let calculated_size = Self::calculate_vertex_size(
            self.max_width,
            self.max_height,
            self.board.state().dimensions(),
            self.min_vertex_size,
            self.max_vertex_size,
        );
        self.board.state_mut().vertex_size = calculated_size;

        self
    }

    /// Creates a bounded Go board with a specified theme
    pub fn with_theme(mut self, theme: BoardTheme) -> Self {
        self.board.set_theme(theme);
        self
    }

    /// Creates a bounded Go board with specified board range for partial display
    pub fn with_range(mut self, range: crate::go_board::BoardRange) -> Self {
        let range_dimensions = (range.width(), range.height());
        self.board = self.board.with_range(range);

        // Recalculate vertex size for the new range
        let calculated_size = Self::calculate_vertex_size(
            self.max_width,
            self.max_height,
            range_dimensions,
            self.min_vertex_size,
            self.max_vertex_size,
        );
        self.board.state_mut().vertex_size = calculated_size;

        self
    }

    /// Updates the maximum dimensions and recalculates vertex size
    pub fn set_max_dimensions(&mut self, max_width: f32, max_height: f32) {
        self.max_width = max_width;
        self.max_height = max_height;

        let calculated_size = Self::calculate_vertex_size(
            self.max_width,
            self.max_height,
            self.board.state().dimensions(),
            self.min_vertex_size,
            self.max_vertex_size,
        );
        self.board.state_mut().vertex_size = calculated_size;
    }

    /// Updates the vertex size limits and recalculates vertex size
    pub fn set_vertex_size_limits(&mut self, min_size: f32, max_size: f32) {
        self.min_vertex_size = min_size.max(0.1);
        self.max_vertex_size = max_size.max(self.min_vertex_size);

        let calculated_size = Self::calculate_vertex_size(
            self.max_width,
            self.max_height,
            self.board.state().dimensions(),
            self.min_vertex_size,
            self.max_vertex_size,
        );
        self.board.state_mut().vertex_size = calculated_size;
    }

    /// Calculates the optimal vertex size to fit within constraints
    fn calculate_vertex_size(
        max_width: f32,
        max_height: f32,
        board_dimensions: (usize, usize),
        min_vertex_size: f32,
        max_vertex_size: f32,
    ) -> f32 {
        let (board_width, board_height) = board_dimensions;

        // Calculate the maximum vertex size that fits within each dimension
        let max_vertex_size_by_width = max_width / board_width as f32;
        let max_vertex_size_by_height = max_height / board_height as f32;

        // Use the smaller of the two to ensure the board fits within both constraints
        let calculated_size = max_vertex_size_by_width.min(max_vertex_size_by_height);

        // Clamp to the specified vertex size limits
        calculated_size.clamp(min_vertex_size, max_vertex_size)
    }

    /// Gets the current vertex size
    pub fn vertex_size(&self) -> f32 {
        self.board.state().vertex_size
    }

    /// Gets the current board dimensions
    pub fn board_dimensions(&self) -> (usize, usize) {
        self.board.state().dimensions()
    }

    /// Gets the current actual board size in pixels
    pub fn actual_board_size(&self) -> Size<Pixels> {
        self.board.board_pixel_size()
    }

    /// Gets the maximum allowed dimensions
    pub fn max_dimensions(&self) -> (f32, f32) {
        (self.max_width, self.max_height)
    }

    /// Gets the vertex size limits
    pub fn vertex_size_limits(&self) -> (f32, f32) {
        (self.min_vertex_size, self.max_vertex_size)
    }

    /// Checks if the board is currently constrained by width
    pub fn is_width_constrained(&self) -> bool {
        let (board_width, board_height) = self.board_dimensions();
        let max_vertex_size_by_width = self.max_width / board_width as f32;
        let max_vertex_size_by_height = self.max_height / board_height as f32;

        max_vertex_size_by_width <= max_vertex_size_by_height
    }

    /// Checks if the board is currently constrained by height
    pub fn is_height_constrained(&self) -> bool {
        !self.is_width_constrained()
    }

    /// Gets a reference to the underlying GoBoard
    pub fn board(&self) -> &GoBoard {
        &self.board
    }

    /// Gets a mutable reference to the underlying GoBoard
    pub fn board_mut(&mut self) -> &mut GoBoard {
        &mut self.board
    }

    /// Convenience method: Sets the sign map and updates the board
    pub fn set_sign_map(&mut self, sign_map: crate::go_board::SignMap) {
        self.board.set_sign_map(sign_map);
    }

    /// Convenience method: Sets the marker map
    pub fn set_marker_map(&mut self, marker_map: crate::go_board::MarkerMap) {
        self.board.set_marker_map(marker_map);
    }

    /// Convenience method: Sets the selected vertices
    pub fn set_selected_vertices(&mut self, vertices: Vec<Vertex>) {
        self.board.set_selected_vertices(vertices);
    }

    /// Convenience method: Sets the coordinate display visibility
    pub fn set_show_coordinates(&mut self, show: bool) {
        self.board.set_show_coordinates(show);
    }

    /// Convenience method: Sets the busy state
    pub fn set_busy(&mut self, busy: bool) {
        self.board.set_busy(busy);
    }

    /// Renders the bounded board with vertex event handlers
    pub fn render_with_vertex_handlers(&self, handlers: VertexEventHandlers) -> impl IntoElement {
        self.board.render_with_vertex_handlers(handlers)
    }
}

impl Render for BoundedGoBoard {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let handlers = VertexEventHandlers::new();
        self.board.render_with_vertex_handlers(handlers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::go_board::types::BoardRange;

    #[test]
    fn test_bounded_go_board_creation() {
        let bounded_board = BoundedGoBoard::new(400.0, 400.0);

        // Check that vertex size was calculated
        assert!(bounded_board.vertex_size() > 0.0);
        assert!(bounded_board.vertex_size() <= 100.0); // max_vertex_size
        assert!(bounded_board.vertex_size() >= 1.0); // min_vertex_size

        // For a 19x19 board in 400x400 space, expect roughly 400/19 ≈ 21.05
        let expected_size = 400.0 / 19.0;
        assert!((bounded_board.vertex_size() - expected_size).abs() < 0.1);
    }

    #[test]
    fn test_bounded_go_board_with_size() {
        let bounded_board = BoundedGoBoard::with_size(9, 9, 270.0, 270.0);

        // For a 9x9 board in 270x270 space, expect 270/9 = 30.0
        let expected_size = 270.0 / 9.0;
        assert!((bounded_board.vertex_size() - expected_size).abs() < 0.1);

        assert_eq!(bounded_board.board_dimensions(), (9, 9));
    }

    #[test]
    fn test_vertex_size_calculation() {
        // Test square constraint
        let size = BoundedGoBoard::calculate_vertex_size(400.0, 400.0, (19, 19), 1.0, 100.0);
        assert!((size - 400.0 / 19.0).abs() < 0.1);

        // Test width-constrained (wider than tall)
        let size = BoundedGoBoard::calculate_vertex_size(200.0, 400.0, (10, 10), 1.0, 100.0);
        assert!((size - 20.0).abs() < 0.1); // Limited by width: 200/10 = 20

        // Test height-constrained (taller than wide)
        let size = BoundedGoBoard::calculate_vertex_size(400.0, 200.0, (10, 10), 1.0, 100.0);
        assert!((size - 20.0).abs() < 0.1); // Limited by height: 200/10 = 20
    }

    #[test]
    fn test_vertex_size_limits() {
        let mut bounded_board =
            BoundedGoBoard::new(1000.0, 1000.0).with_vertex_size_limits(5.0, 30.0);

        // Large space should be capped by max_vertex_size
        assert!((bounded_board.vertex_size() - 30.0).abs() < 0.1);

        // Small space should use min_vertex_size
        bounded_board.set_max_dimensions(50.0, 50.0);
        assert!((bounded_board.vertex_size() - 5.0).abs() < 0.1);
    }

    #[test]
    fn test_constraint_detection() {
        // Width-constrained scenario
        let bounded_board = BoundedGoBoard::with_size(19, 19, 200.0, 400.0);
        assert!(bounded_board.is_width_constrained());
        assert!(!bounded_board.is_height_constrained());

        // Height-constrained scenario
        let bounded_board = BoundedGoBoard::with_size(19, 19, 400.0, 200.0);
        assert!(!bounded_board.is_width_constrained());
        assert!(bounded_board.is_height_constrained());

        // Equal constraint (should be width-constrained by default)
        let bounded_board = BoundedGoBoard::with_size(19, 19, 380.0, 380.0);
        assert!(bounded_board.is_width_constrained());
    }

    #[test]
    fn test_set_max_dimensions() {
        let mut bounded_board = BoundedGoBoard::with_size(9, 9, 270.0, 270.0);
        let initial_size = bounded_board.vertex_size();

        // Reduce available space
        bounded_board.set_max_dimensions(180.0, 180.0);
        let new_size = bounded_board.vertex_size();

        assert!(new_size < initial_size);
        assert!((new_size - 20.0).abs() < 0.1); // 180/9 = 20
    }

    #[test]
    fn test_board_range_support() {
        let range = BoardRange::new((3, 15), (3, 15)); // 13x13 visible area of 19x19 board
        let bounded_board = BoundedGoBoard::new(260.0, 260.0).with_range(range);

        // Should calculate size based on visible range (13x13), not full board
        let expected_size = 260.0 / 13.0; // 20.0
        assert!((bounded_board.vertex_size() - expected_size).abs() < 0.1);
    }

    #[test]
    fn test_actual_board_size() {
        let bounded_board = BoundedGoBoard::with_size(9, 9, 270.0, 270.0);
        let actual_size = bounded_board.actual_board_size();

        // For 9x9 board with 30px vertices, expect 8*30 = 240px (ranges are inclusive)
        let expected_width = 8.0 * 30.0; // (9-1) * vertex_size
        let expected_height = 8.0 * 30.0;

        assert!((actual_size.width.0 - expected_width).abs() < 1.0);
        assert!((actual_size.height.0 - expected_height).abs() < 1.0);
    }

    #[test]
    fn test_convenience_methods() {
        let mut bounded_board = BoundedGoBoard::with_size(3, 3, 100.0, 100.0);

        // Test setting sign map
        let sign_map = vec![vec![1, 0, -1], vec![0, 1, 0], vec![-1, 0, 1]];
        bounded_board.set_sign_map(sign_map.clone());
        assert_eq!(bounded_board.board().state().sign_map, sign_map);

        // Test setting coordinates
        bounded_board.set_show_coordinates(true);
        assert!(bounded_board.board().state().show_coordinates);

        // Test setting busy state
        bounded_board.set_busy(true);
        assert!(bounded_board.board().state().busy);
    }

    #[test]
    fn test_extreme_aspect_ratios() {
        // Very wide constraint
        let bounded_board = BoundedGoBoard::with_size(19, 19, 1000.0, 100.0);
        let expected_size = 100.0 / 19.0; // Height-constrained
        assert!((bounded_board.vertex_size() - expected_size).abs() < 0.1);
        assert!(bounded_board.is_height_constrained());

        // Very tall constraint
        let bounded_board = BoundedGoBoard::with_size(19, 19, 100.0, 1000.0);
        let expected_size = 100.0 / 19.0; // Width-constrained
        assert!((bounded_board.vertex_size() - expected_size).abs() < 0.1);
        assert!(bounded_board.is_width_constrained());
    }

    #[test]
    fn test_minimum_vertex_size_protection() {
        let bounded_board =
            BoundedGoBoard::with_size(19, 19, 10.0, 10.0).with_vertex_size_limits(2.0, 50.0);

        // Even with tiny constraints, should respect minimum
        assert!((bounded_board.vertex_size() - 2.0).abs() < 0.1);
    }
}
