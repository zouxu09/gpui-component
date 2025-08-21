use crate::go_board::types::{BoardRange, Vertex};
use gpui::*;

/// Utility functions for calculating pixel positions of Go board elements
/// Provides consistent position calculations across all board components
#[derive(Clone, Debug)]
pub struct PositionCalculator {
    vertex_size: f32,
    grid_offset: Point<Pixels>,
    board_range: Option<BoardRange>,
}

impl PositionCalculator {
    /// Creates a new position calculator with vertex size and grid offset
    pub fn new(vertex_size: f32, grid_offset: Point<Pixels>) -> Self {
        Self {
            vertex_size,
            grid_offset,
            board_range: None,
        }
    }

    /// Creates a position calculator with board range for relative positioning
    pub fn with_board_range(
        vertex_size: f32,
        grid_offset: Point<Pixels>,
        board_range: BoardRange,
    ) -> Self {
        Self {
            vertex_size,
            grid_offset,
            board_range: Some(board_range),
        }
    }

    /// Updates the vertex size
    pub fn set_vertex_size(&mut self, vertex_size: f32) {
        self.vertex_size = vertex_size;
    }

    /// Updates the grid offset
    pub fn set_grid_offset(&mut self, grid_offset: Point<Pixels>) {
        self.grid_offset = grid_offset;
    }

    /// Updates the board range
    pub fn set_board_range(&mut self, board_range: BoardRange) {
        self.board_range = Some(board_range);
    }

    /// Gets the current vertex size
    pub fn vertex_size(&self) -> f32 {
        self.vertex_size
    }

    /// Gets the current grid offset
    pub fn grid_offset(&self) -> Point<Pixels> {
        self.grid_offset
    }

    /// Gets the current board range
    pub fn board_range(&self) -> Option<&BoardRange> {
        self.board_range.as_ref()
    }

    /// Calculates the center position of a vertex (grid intersection point)
    /// This is the canonical position calculation used by all components
    pub fn vertex_center(&self, vertex: &Vertex) -> Point<Pixels> {
        let grid_offset = self.vertex_size / 2.0;
        let x = self.grid_offset.x + px(vertex.x as f32 * self.vertex_size + grid_offset);
        let y = self.grid_offset.y + px(vertex.y as f32 * self.vertex_size + grid_offset);
        point(x, y)
    }

    /// Calculates the center position of a vertex relative to board range
    /// Used by components that need board-relative positioning
    pub fn vertex_center_relative(&self, vertex: &Vertex) -> Point<Pixels> {
        let board_range = self
            .board_range
            .as_ref()
            .expect("Board range must be set for relative positioning");
        let relative_x = (vertex.x - board_range.x.0) as f32;
        let relative_y = (vertex.y - board_range.y.0) as f32;
        let grid_offset = self.vertex_size / 2.0;

        let x = self.grid_offset.x + px(relative_x * self.vertex_size + grid_offset);
        let y = self.grid_offset.y + px(relative_y * self.vertex_size + grid_offset);
        point(x, y)
    }

    /// Calculates position for an element with specific size, centered on vertex
    pub fn element_position(&self, vertex: &Vertex, element_size: f32) -> Point<Pixels> {
        let center = self.vertex_center(vertex);
        let element_offset = element_size / 2.0;
        point(center.x - px(element_offset), center.y - px(element_offset))
    }

    /// Calculates position for an element with specific size, relative to board range
    pub fn element_position_relative(&self, vertex: &Vertex, element_size: f32) -> Point<Pixels> {
        let center = self.vertex_center_relative(vertex);
        let element_offset = element_size / 2.0;
        point(center.x - px(element_offset), center.y - px(element_offset))
    }

    /// Calculates position for stone elements (commonly 90% of vertex size)
    pub fn stone_position(&self, vertex: &Vertex, size_ratio: f32) -> Point<Pixels> {
        let stone_size = self.vertex_size * size_ratio;
        self.element_position(vertex, stone_size)
    }

    /// Calculates position for stone elements relative to board range
    pub fn stone_position_relative(&self, vertex: &Vertex, size_ratio: f32) -> Point<Pixels> {
        let stone_size = self.vertex_size * size_ratio;
        self.element_position_relative(vertex, stone_size)
    }

    /// Calculates position for selection elements (commonly 90% of vertex size)
    pub fn selection_position(&self, vertex: &Vertex) -> Point<Pixels> {
        self.stone_position(vertex, 0.9)
    }

    /// Calculates position for marker elements (same size as vertex)
    pub fn marker_position(&self, vertex: &Vertex) -> Point<Pixels> {
        self.element_position(vertex, self.vertex_size)
    }

    /// Calculates position for paint overlay elements (commonly 80% of vertex size)
    pub fn paint_position(&self, vertex: &Vertex) -> Point<Pixels> {
        self.stone_position(vertex, 0.8)
    }

    /// Calculates position for heat overlay elements (commonly 75% of vertex size)
    pub fn heat_position(&self, vertex: &Vertex) -> Point<Pixels> {
        self.stone_position(vertex, 0.75)
    }

    /// Calculates position for ghost stone elements
    pub fn ghost_stone_position(&self, vertex: &Vertex, ghost_size: f32) -> Point<Pixels> {
        self.element_position(vertex, ghost_size)
    }

    /// Converts vertex coordinates to pixel coordinates (f32 values)
    pub fn vertex_to_pixel(&self, vertex: &Vertex) -> (f32, f32) {
        let grid_offset = self.vertex_size / 2.0;
        (
            vertex.x as f32 * self.vertex_size + grid_offset,
            vertex.y as f32 * self.vertex_size + grid_offset,
        )
    }

    /// Converts vertex coordinates to pixel coordinates relative to board range
    pub fn vertex_to_pixel_relative(&self, vertex: &Vertex) -> (f32, f32) {
        let board_range = self
            .board_range
            .as_ref()
            .expect("Board range must be set for relative positioning");
        let relative_x = (vertex.x - board_range.x.0) as f32;
        let relative_y = (vertex.y - board_range.y.0) as f32;
        let grid_offset = self.vertex_size / 2.0;
        (
            relative_x * self.vertex_size + grid_offset,
            relative_y * self.vertex_size + grid_offset,
        )
    }

    /// Calculates the visible dimensions of the board area
    pub fn visible_dimensions(&self) -> (f32, f32) {
        let board_range = self
            .board_range
            .as_ref()
            .expect("Board range must be set for dimension calculation");
        let grid_intervals_x = (board_range.x.1 - board_range.x.0) as f32;
        let grid_intervals_y = (board_range.y.1 - board_range.y.0) as f32;
        let width = grid_intervals_x * self.vertex_size + self.vertex_size;
        let height = grid_intervals_y * self.vertex_size + self.vertex_size;
        (width, height)
    }

    /// Calculates the visible dimensions for stone rendering
    pub fn stone_visible_dimensions(&self) -> (f32, f32) {
        let board_range = self
            .board_range
            .as_ref()
            .expect("Board range must be set for dimension calculation");
        let width = (board_range.x.1 - board_range.x.0 + 1) as f32 * self.vertex_size;
        let height = (board_range.y.1 - board_range.y.0 + 1) as f32 * self.vertex_size;
        (width, height)
    }
}

/// Utility functions for common position calculations without creating a calculator instance
#[derive(Clone, Copy, Debug)]
pub struct PositionUtils;

impl PositionUtils {
    /// Calculates vertex center position from basic parameters
    pub fn vertex_center(
        vertex: &Vertex,
        vertex_size: f32,
        grid_offset: Point<Pixels>,
    ) -> Point<Pixels> {
        let grid_offset_f32 = vertex_size / 2.0;
        let x = grid_offset.x + px(vertex.x as f32 * vertex_size + grid_offset_f32);
        let y = grid_offset.y + px(vertex.y as f32 * vertex_size + grid_offset_f32);
        point(x, y)
    }

    /// Calculates vertex center position relative to board range
    pub fn vertex_center_relative(
        vertex: &Vertex,
        vertex_size: f32,
        grid_offset: Point<Pixels>,
        board_range: &BoardRange,
    ) -> Point<Pixels> {
        let relative_x = (vertex.x - board_range.x.0) as f32;
        let relative_y = (vertex.y - board_range.y.0) as f32;
        let grid_offset_f32 = vertex_size / 2.0;

        let x = grid_offset.x + px(relative_x * vertex_size + grid_offset_f32);
        let y = grid_offset.y + px(relative_y * vertex_size + grid_offset_f32);
        point(x, y)
    }

    /// Calculates element position centered on vertex
    pub fn element_position(
        vertex: &Vertex,
        vertex_size: f32,
        grid_offset: Point<Pixels>,
        element_size: f32,
    ) -> Point<Pixels> {
        let center = Self::vertex_center(vertex, vertex_size, grid_offset);
        let element_offset = element_size / 2.0;
        point(center.x - px(element_offset), center.y - px(element_offset))
    }

    /// Converts vertex coordinates to pixel coordinates
    pub fn vertex_to_pixel(vertex: &Vertex, vertex_size: f32) -> (f32, f32) {
        let grid_offset = vertex_size / 2.0;
        (
            vertex.x as f32 * vertex_size + grid_offset,
            vertex.y as f32 * vertex_size + grid_offset,
        )
    }

    /// Converts vertex coordinates to pixel coordinates relative to board range
    pub fn vertex_to_pixel_relative(
        vertex: &Vertex,
        vertex_size: f32,
        board_range: &BoardRange,
    ) -> (f32, f32) {
        let relative_x = (vertex.x - board_range.x.0) as f32;
        let relative_y = (vertex.y - board_range.y.0) as f32;
        let grid_offset = vertex_size / 2.0;
        (
            relative_x * vertex_size + grid_offset,
            relative_y * vertex_size + grid_offset,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_calculator_creation() {
        let calculator = PositionCalculator::new(24.0, point(px(10.0), px(10.0)));
        assert_eq!(calculator.vertex_size(), 24.0);
        assert_eq!(calculator.grid_offset().x, px(10.0));
        assert_eq!(calculator.grid_offset().y, px(10.0));
        assert!(calculator.board_range().is_none());
    }

    #[test]
    fn test_vertex_center_calculation() {
        let calculator = PositionCalculator::new(24.0, point(px(10.0), px(10.0)));
        let vertex = Vertex::new(2, 3);
        let position = calculator.vertex_center(&vertex);

        // Expected calculation: grid_offset = 24.0 / 2.0 = 12.0
        // x = 10.0 + 2.0 * 24.0 + 12.0 = 70.0
        // y = 10.0 + 3.0 * 24.0 + 12.0 = 94.0
        assert_eq!(position.x, px(70.0));
        assert_eq!(position.y, px(94.0));
    }

    #[test]
    fn test_element_position_calculation() {
        let calculator = PositionCalculator::new(24.0, point(px(10.0), px(10.0)));
        let vertex = Vertex::new(1, 1);
        let element_size = 20.0;
        let position = calculator.element_position(&vertex, element_size);

        // Vertex center: 10.0 + 1.0 * 24.0 + 12.0 = 46.0
        // Element offset: 20.0 / 2.0 = 10.0
        // Position: 46.0 - 10.0 = 36.0
        assert_eq!(position.x, px(36.0));
        assert_eq!(position.y, px(36.0));
    }

    #[test]
    fn test_stone_position_calculation() {
        let calculator = PositionCalculator::new(24.0, point(px(10.0), px(10.0)));
        let vertex = Vertex::new(1, 1);
        let position = calculator.stone_position(&vertex, 0.9);

        // Stone size: 24.0 * 0.9 = 21.6
        // Vertex center: 10.0 + 1.0 * 24.0 + 12.0 = 46.0
        // Stone offset: 21.6 / 2.0 = 10.8
        // Position: 46.0 - 10.8 = 35.2
        assert_eq!(position.x, px(35.2));
        assert_eq!(position.y, px(35.2));
    }

    #[test]
    fn test_vertex_to_pixel_conversion() {
        let calculator = PositionCalculator::new(24.0, point(px(0.0), px(0.0)));
        let vertex = Vertex::new(2, 3);
        let (pixel_x, pixel_y) = calculator.vertex_to_pixel(&vertex);

        // grid_offset = 24.0 / 2.0 = 12.0
        // pixel_x = 2.0 * 24.0 + 12.0 = 60.0
        // pixel_y = 3.0 * 24.0 + 12.0 = 84.0
        assert_eq!(pixel_x, 60.0);
        assert_eq!(pixel_y, 84.0);
    }

    #[test]
    fn test_static_utility_functions() {
        let vertex = Vertex::new(2, 3);
        let grid_offset = point(px(10.0), px(10.0));

        let position = PositionUtils::vertex_center(&vertex, 24.0, grid_offset);
        assert_eq!(position.x, px(70.0)); // 10.0 + 2.0 * 24.0 + 12.0
        assert_eq!(position.y, px(94.0)); // 10.0 + 3.0 * 24.0 + 12.0

        let (pixel_x, pixel_y) = PositionUtils::vertex_to_pixel(&vertex, 24.0);
        assert_eq!(pixel_x, 60.0); // 2.0 * 24.0 + 12.0
        assert_eq!(pixel_y, 84.0); // 3.0 * 24.0 + 12.0
    }

    #[test]
    fn test_relative_position_calculation() {
        let board_range = BoardRange::new((0, 8), (0, 8));
        let calculator =
            PositionCalculator::with_board_range(24.0, point(px(10.0), px(10.0)), board_range);
        let vertex = Vertex::new(2, 3);

        let position = calculator.vertex_center_relative(&vertex);
        // Relative calculation should be same as absolute for full board range
        assert_eq!(position.x, px(70.0)); // 10.0 + 2.0 * 24.0 + 12.0
        assert_eq!(position.y, px(94.0)); // 10.0 + 3.0 * 24.0 + 12.0

        let (pixel_x, pixel_y) = calculator.vertex_to_pixel_relative(&vertex);
        assert_eq!(pixel_x, 60.0); // 2.0 * 24.0 + 12.0
        assert_eq!(pixel_y, 84.0); // 3.0 * 24.0 + 12.0
    }

    #[test]
    fn test_visible_dimensions_calculation() {
        let board_range = BoardRange::new((2, 6), (1, 4)); // 5x4 area
        let calculator =
            PositionCalculator::with_board_range(30.0, point(px(0.0), px(0.0)), board_range);

        let (width, height) = calculator.visible_dimensions();
        // grid_intervals_x = (6-2) = 4, grid_intervals_y = (4-1) = 3
        // width = 4 * 30 + 30 = 150.0
        // height = 3 * 30 + 30 = 120.0
        assert_eq!(width, 150.0);
        assert_eq!(height, 120.0);

        let (stone_width, stone_height) = calculator.stone_visible_dimensions();
        // width = (6-2+1) * 30 = 5 * 30 = 150.0
        // height = (4-1+1) * 30 = 4 * 30 = 120.0
        assert_eq!(stone_width, 150.0);
        assert_eq!(stone_height, 120.0);
    }
}
