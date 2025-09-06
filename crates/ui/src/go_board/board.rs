use crate::go_board::core::*;
use gpui::{px, Pixels, Point, Size};

/// Simplified Go board component with ergonomic API
#[derive(Clone)]
pub struct Board {
    data: BoardData,
    pub theme: Theme,
    show_coordinates: bool,
}

impl Board {
    /// Create a standard 19x19 board
    pub fn new() -> Self {
        Self {
            data: BoardData::standard(),
            theme: Theme::default(),
            show_coordinates: true,
        }
    }

    /// Create a board with custom dimensions
    pub fn with_size(width: usize, height: usize) -> Self {
        Self {
            data: BoardData::new(width, height),
            theme: Theme::default(),
            show_coordinates: true,
        }
    }

    // =============================================================================
    // CONFIGURATION METHODS
    // =============================================================================

    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    pub fn coordinates(mut self, show: bool) -> Self {
        self.show_coordinates = show;
        self
    }

    pub fn range(mut self, range: Range) -> Self {
        self.data.set_range(range);
        self
    }

    // =============================================================================
    // DATA ACCESS
    // =============================================================================

    pub fn data(&self) -> &BoardData {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut BoardData {
        &mut self.data
    }

    pub fn stone_at(&self, pos: Pos) -> Stone {
        self.data.get_stone(pos)
    }

    pub fn marker_at(&self, pos: Pos) -> Option<&Marker> {
        self.data.get_marker(pos)
    }

    // =============================================================================
    // CONTENT METHODS
    // =============================================================================

    /// Add a stone to the board
    pub fn stone(mut self, pos: Pos, stone: Stone) -> Self {
        self.data.set_stone(pos, stone);
        self
    }

    /// Add a marker to the board
    pub fn marker(mut self, pos: Pos, marker: Marker) -> Self {
        self.data.set_marker(pos, Some(marker));
        self
    }

    /// Add a ghost stone to the board
    pub fn ghost(mut self, pos: Pos, ghost: Ghost) -> Self {
        self.data.set_ghost(pos, Some(ghost));
        self
    }

    /// Add a line to the board
    pub fn line(mut self, line: Line) -> Self {
        self.data.add_line(line);
        self
    }

    /// Select a position on the board
    pub fn select(mut self, pos: Pos) -> Self {
        self.data.set_selection(pos, Some(Selection::selected(pos)));
        self
    }

    /// Mark the last move
    pub fn last_move(mut self, pos: Pos) -> Self {
        self.data
            .set_selection(pos, Some(Selection::last_move(pos)));
        self
    }

    /// Add territory marking
    pub fn territory(mut self, pos: Pos, territory: Territory) -> Self {
        self.data.set_territory(pos, Some(territory));
        self
    }

    /// Add heat/influence visualization
    pub fn heat(mut self, pos: Pos, heat: Heat) -> Self {
        self.data.set_heat(pos, Some(heat));
        self
    }

    // =============================================================================
    // CLEAR OPERATIONS
    // =============================================================================

    /// Clear all board content
    pub fn clear_all(mut self) -> Self {
        self.data.clear_all();
        self
    }

    /// Clear selections only
    pub fn clear_selections(mut self) -> Self {
        self.data.clear_selections();
        self
    }

    // =============================================================================
    // UTILITY METHODS
    // =============================================================================

    pub fn dimensions(&self) -> (usize, usize) {
        self.data.size
    }

    pub fn visible_range(&self) -> &Range {
        &self.data.range
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.data.resize(width, height);
    }

    pub fn pixel_size(&self, vertex_size: f32) -> Size<Pixels> {
        let range = &self.data.range;
        let width = range.width() as f32 * vertex_size;
        let height = range.height() as f32 * vertex_size;
        Size::new(px(width), px(height))
    }

    pub fn pos_from_pixel(
        &self,
        pixel: Point<Pixels>,
        offset: Point<Pixels>,
        vertex_size: f32,
    ) -> Option<Pos> {
        let relative_x = (pixel.x - offset.x).0;
        let relative_y = (pixel.y - offset.y).0;

        // Account for the vertex_size/2.0 offset that's added in pixel_from_pos
        let grid_x = ((relative_x - vertex_size / 2.0) / vertex_size).round() as i32;
        let grid_y = ((relative_y - vertex_size / 2.0) / vertex_size).round() as i32;

        if grid_x >= 0 && grid_y >= 0 {
            let pos = Pos::new(
                grid_x as usize + self.data.range.x.0,
                grid_y as usize + self.data.range.y.0,
            );

            if self.data.range.contains(pos) {
                Some(pos)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn pixel_from_pos(
        &self,
        pos: Pos,
        offset: Point<Pixels>,
        vertex_size: f32,
    ) -> Point<Pixels> {
        let range = &self.data.range;
        let relative_x = (pos.x - range.x.0) as f32;
        let relative_y = (pos.y - range.y.0) as f32;

        Point::new(
            offset.x + px(relative_x * vertex_size + vertex_size / 2.0),
            offset.y + px(relative_y * vertex_size + vertex_size / 2.0),
        )
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_creation() {
        let board = Board::new();
        assert_eq!(board.dimensions(), (19, 19));
    }

    #[test]
    fn test_builder_pattern() {
        let board = Board::with_size(9, 9)
            .coordinates(false)
            .stone(Pos::new(4, 4), BLACK)
            .marker(Pos::new(2, 2), Marker::circle());

        assert_eq!(board.dimensions(), (9, 9));
        assert_eq!(board.show_coordinates, false);
        assert_eq!(board.stone_at(Pos::new(4, 4)), BLACK);
        assert!(board.marker_at(Pos::new(2, 2)).is_some());
    }

    #[test]
    fn test_position_conversion() {
        let board = Board::new();
        let offset = Point::new(px(10.0), px(10.0));
        let vertex_size = 20.0;

        // Test with position (1, 1) to avoid edge rounding issues
        let pos = Pos::new(1, 1);
        let pixel = board.pixel_from_pos(pos, offset, vertex_size);
        let converted_pos = board.pos_from_pixel(pixel, offset, vertex_size).unwrap();

        // Now the conversion should work correctly without offset
        assert_eq!(pos, converted_pos);
    }

    #[test]
    fn test_chaining_operations() {
        let board = Board::new()
            .clear_all()
            .stone(Pos::new(3, 3), BLACK)
            .stone(Pos::new(4, 4), WHITE)
            .marker(Pos::new(1, 1), Marker::circle())
            .marker(Pos::new(2, 2), Marker::triangle())
            .ghost(Pos::new(5, 5), Ghost::good(BLACK))
            .ghost(Pos::new(6, 6), Ghost::bad(WHITE));

        assert_eq!(board.stone_at(Pos::new(3, 3)), BLACK);
        assert_eq!(board.stone_at(Pos::new(4, 4)), WHITE);
        assert!(board.marker_at(Pos::new(1, 1)).is_some());
        assert!(board.data().ghosts.contains_key(&Pos::new(5, 5)));
    }
}
