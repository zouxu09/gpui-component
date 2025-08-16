#[cfg(test)]
mod grid_tests {
    use super::*;
    use crate::go_board::types::*;

    #[test]
    fn test_grid_creation() {
        let range = BoardRange::new((0, 18), (0, 18));
        let grid = Grid::new(range.clone(), 24.0);

        assert_eq!(grid.board_range, range);
        assert_eq!(grid.vertex_size, 24.0);
        assert!(!grid.show_coordinates);
    }

    #[test]
    fn test_vertex_to_pixel() {
        let range = BoardRange::new((0, 18), (0, 18));
        let grid = Grid::new(range, 20.0);

        // Test corner vertices
        assert_eq!(grid.vertex_to_pixel(&Vertex::new(0, 0)), (0.0, 0.0));
        assert_eq!(grid.vertex_to_pixel(&Vertex::new(18, 18)), (360.0, 360.0)); // 18 * 20
        assert_eq!(grid.vertex_to_pixel(&Vertex::new(9, 9)), (180.0, 180.0)); // 9 * 20
    }

    #[test]
    fn test_visible_dimensions() {
        let range = BoardRange::new((0, 18), (0, 18)); // 19x19 board
        let grid = Grid::new(range, 25.0);

        assert_eq!(grid.visible_dimensions(), (475.0, 475.0)); // 19 * 25
    }
}
