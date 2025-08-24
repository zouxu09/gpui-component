use crate::go_board::core::*;
use gpui::*;
use std::rc::Rc;

/// Simplified Go board component with ergonomic API
/// This replaces the complex GoBoard + GoBoardState + multiple overlay system
#[derive(Clone)]
pub struct Board {
    data: BoardData,
    pub theme: Theme,
    pub vertex_size: f32,
    show_coordinates: bool,

    // Event handlers - much simpler than the previous complex system
    on_click: Option<Rc<dyn Fn(PosEvent)>>,
    on_hover: Option<Rc<dyn Fn(Option<Pos>)>>,
    on_key: Option<Rc<dyn Fn(KeyDownEvent) -> Option<NavEvent>>>,
}

impl Board {
    /// Create a new board with default 19x19 size
    pub fn new() -> Self {
        Self {
            data: BoardData::standard(),
            theme: Theme::default(),
            vertex_size: 24.0,
            show_coordinates: true,
            on_click: None,
            on_hover: None,
            on_key: None,
        }
    }

    /// Create board with specific size
    pub fn with_size(width: usize, height: usize) -> Self {
        Self {
            data: BoardData::new(width, height),
            theme: Theme::default(),
            vertex_size: 24.0,
            show_coordinates: true,
            on_click: None,
            on_hover: None,
            on_key: None,
        }
    }

    // =============================================================================
    // BUILDER PATTERN API - Much more ergonomic than previous scattered methods
    // =============================================================================

    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    pub fn vertex_size(mut self, size: f32) -> Self {
        self.vertex_size = size;
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

    // Event handlers with simple closures
    pub fn on_click<F>(mut self, handler: F) -> Self
    where
        F: Fn(PosEvent) + 'static,
    {
        self.on_click = Some(Rc::new(handler));
        self
    }

    pub fn on_hover<F>(mut self, handler: F) -> Self
    where
        F: Fn(Option<Pos>) + 'static,
    {
        self.on_hover = Some(Rc::new(handler));
        self
    }

    pub fn on_key<F>(mut self, handler: F) -> Self
    where
        F: Fn(KeyDownEvent) -> Option<NavEvent> + 'static,
    {
        self.on_key = Some(Rc::new(handler));
        self
    }

    // =============================================================================
    // DATA ACCESS - Direct, simple API
    // =============================================================================

    /// Get reference to board data for reading
    pub fn data(&self) -> &BoardData {
        &self.data
    }

    /// Get mutable reference to board data for modifications
    pub fn data_mut(&mut self) -> &mut BoardData {
        &mut self.data
    }

    /// Immutable access to specific data
    pub fn stone_at(&self, pos: Pos) -> Stone {
        self.data.get_stone(pos)
    }

    pub fn marker_at(&self, pos: Pos) -> Option<&Marker> {
        self.data.get_marker(pos)
    }

    // =============================================================================
    // CONVENIENT MUTATION METHODS
    // =============================================================================

    /// Set a stone and return self for chaining
    pub fn stone(mut self, pos: Pos, stone: Stone) -> Self {
        self.data.set_stone(pos, stone);
        self
    }

    /// Set multiple stones from iterator
    pub fn stones<I>(mut self, stones: I) -> Self
    where
        I: IntoIterator<Item = (Pos, Stone)>,
    {
        for (pos, stone) in stones {
            self.data.set_stone(pos, stone);
        }
        self
    }

    /// Set a marker
    pub fn marker(mut self, pos: Pos, marker: Marker) -> Self {
        self.data.set_marker(pos, Some(marker));
        self
    }

    /// Set multiple markers
    pub fn markers<I>(mut self, markers: I) -> Self
    where
        I: IntoIterator<Item = (Pos, Marker)>,
    {
        for (pos, marker) in markers {
            self.data.set_marker(pos, Some(marker));
        }
        self
    }

    /// Set ghost stone
    pub fn ghost(mut self, pos: Pos, ghost: Ghost) -> Self {
        self.data.set_ghost(pos, Some(ghost));
        self
    }

    /// Set multiple ghost stones
    pub fn ghosts<I>(mut self, ghosts: I) -> Self
    where
        I: IntoIterator<Item = (Pos, Ghost)>,
    {
        for (pos, ghost) in ghosts {
            self.data.set_ghost(pos, Some(ghost));
        }
        self
    }

    /// Add a line
    pub fn line(mut self, line: Line) -> Self {
        self.data.add_line(line);
        self
    }

    /// Add multiple lines
    pub fn lines<I>(mut self, lines: I) -> Self
    where
        I: IntoIterator<Item = Line>,
    {
        for line in lines {
            self.data.add_line(line);
        }
        self
    }

    /// Select a position
    pub fn select(mut self, pos: Pos) -> Self {
        self.data.set_selection(pos, Some(Selection::selected(pos)));
        self
    }

    /// Select multiple positions
    pub fn selections<I>(mut self, positions: I) -> Self
    where
        I: IntoIterator<Item = Pos>,
    {
        for pos in positions {
            self.data.set_selection(pos, Some(Selection::selected(pos)));
        }
        self
    }

    /// Mark last move
    pub fn last_move(mut self, pos: Pos) -> Self {
        self.data
            .set_selection(pos, Some(Selection::last_move(pos)));
        self
    }

    /// Set territory
    pub fn territory(mut self, pos: Pos, territory: Territory) -> Self {
        self.data.set_territory(pos, Some(territory));
        self
    }

    /// Set heat/influence
    pub fn heat(mut self, pos: Pos, heat: Heat) -> Self {
        self.data.set_heat(pos, Some(heat));
        self
    }

    // =============================================================================
    // CLEAR OPERATIONS
    // =============================================================================

    pub fn clear_stones(mut self) -> Self {
        self.data.clear_stones();
        self
    }

    pub fn clear_markers(mut self) -> Self {
        self.data.clear_markers();
        self
    }

    pub fn clear_ghosts(mut self) -> Self {
        self.data.clear_ghosts();
        self
    }

    pub fn clear_selections(mut self) -> Self {
        self.data.clear_selections();
        self
    }

    pub fn clear_lines(mut self) -> Self {
        self.data.clear_lines();
        self
    }

    pub fn clear_all(mut self) -> Self {
        self.data.clear_stones();
        self.data.clear_markers();
        self.data.clear_ghosts();
        self.data.clear_selections();
        self.data.clear_lines();
        self.data.clear_heat();
        self.data.clear_territory();
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

    /// Calculate board pixel size
    pub fn pixel_size(&self) -> Size<Pixels> {
        let range = &self.data.range;
        let width = range.width() as f32 * self.vertex_size;
        let height = range.height() as f32 * self.vertex_size;
        Size::new(px(width), px(height))
    }

    /// Get position from pixel coordinates (for click handling)
    pub fn pos_from_pixel(&self, pixel: Point<Pixels>, offset: Point<Pixels>) -> Option<Pos> {
        let relative_x = (pixel.x - offset.x).0;
        let relative_y = (pixel.y - offset.y).0;

        let grid_x = (relative_x / self.vertex_size).round() as i32;
        let grid_y = (relative_y / self.vertex_size).round() as i32;

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

    /// Get pixel coordinates for a position
    pub fn pixel_from_pos(&self, pos: Pos, offset: Point<Pixels>) -> Point<Pixels> {
        let range = &self.data.range;
        let relative_x = (pos.x - range.x.0) as f32;
        let relative_y = (pos.y - range.y.0) as f32;

        Point::new(
            offset.x + px(relative_x * self.vertex_size + self.vertex_size / 2.0),
            offset.y + px(relative_y * self.vertex_size + self.vertex_size / 2.0),
        )
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// BOUNDED BOARD - Simple replacement for BoundedGoBoard
// =============================================================================

/// Auto-sizing board that fits within given constraints
pub struct BoundedBoard {
    board: Board,
    max_width: f32,
    max_height: f32,
    min_vertex_size: f32,
    max_vertex_size: f32,
}

impl BoundedBoard {
    pub fn new(max_width: f32, max_height: f32) -> Self {
        let mut board = Board::new();
        let vertex_size =
            Self::calculate_vertex_size(max_width, max_height, board.dimensions(), 10.0, 50.0);
        board.vertex_size = vertex_size;

        Self {
            board,
            max_width,
            max_height,
            min_vertex_size: 10.0,
            max_vertex_size: 50.0,
        }
    }

    pub fn with_size(width: usize, height: usize, max_width: f32, max_height: f32) -> Self {
        let mut board = Board::with_size(width, height);
        let vertex_size =
            Self::calculate_vertex_size(max_width, max_height, (width, height), 10.0, 50.0);
        board.vertex_size = vertex_size;

        Self {
            board,
            max_width,
            max_height,
            min_vertex_size: 10.0,
            max_vertex_size: 50.0,
        }
    }

    pub fn vertex_size_limits(mut self, min: f32, max: f32) -> Self {
        self.min_vertex_size = min;
        self.max_vertex_size = max;
        self.recalculate_vertex_size();
        self
    }

    pub fn max_dimensions(mut self, width: f32, height: f32) -> Self {
        self.max_width = width;
        self.max_height = height;
        self.recalculate_vertex_size();
        self
    }

    /// Delegate all Board methods
    pub fn inner(&self) -> &Board {
        &self.board
    }

    pub fn inner_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    /// Consume the BoundedBoard and return the inner Board
    pub fn into_inner(self) -> Board {
        self.board
    }

    /// Apply closure to inner board and recalculate size
    pub fn update<F>(mut self, f: F) -> Self
    where
        F: FnOnce(Board) -> Board,
    {
        self.board = f(self.board);
        self.recalculate_vertex_size();
        self
    }

    fn recalculate_vertex_size(&mut self) {
        let vertex_size = Self::calculate_vertex_size(
            self.max_width,
            self.max_height,
            self.board.dimensions(),
            self.min_vertex_size,
            self.max_vertex_size,
        );
        self.board.vertex_size = vertex_size;
    }

    fn calculate_vertex_size(
        max_width: f32,
        max_height: f32,
        (board_width, board_height): (usize, usize),
        min_vertex_size: f32,
        max_vertex_size: f32,
    ) -> f32 {
        let max_by_width = max_width / board_width as f32;
        let max_by_height = max_height / board_height as f32;
        let calculated = max_by_width.min(max_by_height);
        calculated.clamp(min_vertex_size, max_vertex_size)
    }
}

// =============================================================================
// CONVENIENT HELPER FUNCTIONS
// =============================================================================

/// Create a standard 19x19 board with some stones
pub fn demo_board() -> Board {
    Board::new()
        .stone(Pos::new(3, 3), BLACK)
        .stone(Pos::new(15, 15), WHITE)
        .stone(Pos::new(9, 9), BLACK)
        .marker(Pos::new(3, 15), Marker::circle().with_color(rgb(0xff0000).into()))
        .marker(
            Pos::new(15, 3),
            Marker::triangle().with_color(rgb(0x0000ff).into()),
        )
        .ghost(Pos::new(4, 4), Ghost::good(WHITE))
        .ghost(Pos::new(5, 5), Ghost::bad(BLACK))
        .select(Pos::new(3, 3))
        .last_move(Pos::new(9, 9))
}

/// Create 9x9 board for quick testing
pub fn small_board() -> Board {
    Board::with_size(9, 9)
        .vertex_size(30.0)
        .stone(Pos::new(4, 4), BLACK)
        .stone(Pos::new(2, 2), WHITE)
        .stone(Pos::new(6, 6), BLACK)
}

/// Load board from SGF-like position string
pub fn from_position_string(size: (usize, usize), position: &str) -> Board {
    let mut board = Board::with_size(size.0, size.1);

    // Simple format: "B[dd],W[pp],B[pd],W[dp]"
    for move_str in position.split(',') {
        if let Some((color, coords)) = move_str.split_once('[') {
            if let Some(coords) = coords.strip_suffix(']') {
                if coords.len() == 2 {
                    let x = coords.chars().nth(0).unwrap() as usize - 'a' as usize;
                    let y = coords.chars().nth(1).unwrap() as usize - 'a' as usize;
                    let stone = match color {
                        "B" => BLACK,
                        "W" => WHITE,
                        _ => continue,
                    };
                    board.data.set_stone(Pos::new(x, y), stone);
                }
            }
        }
    }

    board
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_creation() {
        let board = Board::new();
        assert_eq!(board.dimensions(), (19, 19));
        assert_eq!(board.vertex_size, 24.0);
    }

    #[test]
    fn test_builder_pattern() {
        let board = Board::with_size(9, 9)
            .vertex_size(30.0)
            .coordinates(false)
            .stone(Pos::new(4, 4), BLACK)
            .marker(Pos::new(2, 2), Marker::circle());

        assert_eq!(board.dimensions(), (9, 9));
        assert_eq!(board.vertex_size, 30.0);
        assert_eq!(board.show_coordinates, false);
        assert_eq!(board.stone_at(Pos::new(4, 4)), BLACK);
        assert!(board.marker_at(Pos::new(2, 2)).is_some());
    }

    #[test]
    fn test_bounded_board() {
        let bounded = BoundedBoard::new(200.0, 200.0);
        // For 19x19 board in 200x200, expect roughly 200/19 ≈ 10.5
        assert!(bounded.inner().vertex_size >= 10.0);
        assert!(bounded.inner().vertex_size <= 11.0);
    }

    #[test]
    fn test_position_conversion() {
        let board = Board::new().vertex_size(20.0);
        let offset = Point::new(px(10.0), px(10.0));

        // Convert position to pixel and back
        let pos = Pos::new(5, 5);
        let pixel = board.pixel_from_pos(pos, offset);
        let converted_pos = board.pos_from_pixel(pixel, offset).unwrap();
        assert_eq!(pos, converted_pos);
    }

    #[test]
    fn test_demo_board() {
        let board = demo_board();
        assert_eq!(board.stone_at(Pos::new(3, 3)), BLACK);
        assert_eq!(board.stone_at(Pos::new(15, 15)), WHITE);
        assert!(board.marker_at(Pos::new(3, 15)).is_some());
    }

    #[test]
    fn test_position_string_parsing() {
        let board = from_position_string((19, 19), "B[dd],W[pp],B[pd]");
        assert_eq!(board.stone_at(Pos::new(3, 3)), BLACK); // 'd' = 3
        assert_eq!(board.stone_at(Pos::new(15, 15)), WHITE); // 'p' = 15
        assert_eq!(board.stone_at(Pos::new(15, 3)), BLACK);
    }

    #[test]
    fn test_chaining_operations() {
        let board = Board::new()
            .clear_all()
            .stones([(Pos::new(3, 3), BLACK), (Pos::new(4, 4), WHITE)])
            .markers([
                (Pos::new(1, 1), Marker::circle()),
                (Pos::new(2, 2), Marker::triangle()),
            ])
            .ghosts([
                (Pos::new(5, 5), Ghost::good(BLACK)),
                (Pos::new(6, 6), Ghost::bad(WHITE)),
            ]);

        assert_eq!(board.stone_at(Pos::new(3, 3)), BLACK);
        assert_eq!(board.stone_at(Pos::new(4, 4)), WHITE);
        assert!(board.marker_at(Pos::new(1, 1)).is_some());
        assert!(board.data().ghosts.contains_key(&Pos::new(5, 5)));
    }
}
