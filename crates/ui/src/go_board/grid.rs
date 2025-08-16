use crate::go_board::types::{BoardRange, Vertex};
use gpui::*;

/// Grid component for drawing board lines and background
/// Handles the visual grid structure of the Go board
#[derive(Clone, Debug)]
pub struct GridTheme {
    pub background_color: Rgba,
    pub grid_line_color: Rgba,
    pub grid_line_width: f32,
    pub border_color: Rgba,
    pub border_width: f32,
}

impl Default for GridTheme {
    fn default() -> Self {
        Self {
            background_color: rgb(0xebb55b), // Shudan default wood color
            grid_line_color: rgb(0x000000),  // Black grid lines
            grid_line_width: 1.0,
            border_color: rgb(0xca933a), // Wood border color
            border_width: 4.0,
        }
    }
}

pub struct Grid {
    board_range: BoardRange,
    vertex_size: f32,
    theme: GridTheme,
    show_coordinates: bool,
}

impl Grid {
    /// Creates a new Grid component
    pub fn new(board_range: BoardRange, vertex_size: f32) -> Self {
        Self {
            board_range,
            vertex_size,
            theme: GridTheme::default(),
            show_coordinates: false,
        }
    }

    /// Sets the grid theme
    pub fn with_theme(mut self, theme: GridTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Sets coordinate visibility
    pub fn with_coordinates(mut self, show: bool) -> Self {
        self.show_coordinates = show;
        self
    }

    /// Updates the board range
    pub fn set_board_range(&mut self, range: BoardRange) {
        self.board_range = range;
    }

    /// Updates the vertex size
    pub fn set_vertex_size(&mut self, size: f32) {
        self.vertex_size = size;
    }

    /// Calculates the pixel position of a vertex
    pub fn vertex_to_pixel(&self, vertex: &Vertex) -> (f32, f32) {
        let relative_x = vertex.x.saturating_sub(self.board_range.x.0) as f32;
        let relative_y = vertex.y.saturating_sub(self.board_range.y.0) as f32;

        (relative_x * self.vertex_size, relative_y * self.vertex_size)
    }

    /// Gets the visible board dimensions
    fn visible_dimensions(&self) -> (f32, f32) {
        let width = self.board_range.width() as f32 * self.vertex_size;
        let height = self.board_range.height() as f32 * self.vertex_size;
        (width, height)
    }

    /// Renders horizontal grid lines
    fn render_horizontal_lines(&self) -> Vec<impl IntoElement> {
        let mut lines = Vec::new();
        let (grid_width, _) = self.visible_dimensions();

        for y in self.board_range.y.0..=self.board_range.y.1 {
            let relative_y = (y - self.board_range.y.0) as f32;
            let pixel_y = relative_y * self.vertex_size;

            lines.push(
                div()
                    .absolute()
                    .left(px(0.0))
                    .top(px(pixel_y))
                    .w(px(grid_width))
                    .h(px(self.theme.grid_line_width))
                    .bg(self.theme.grid_line_color),
            );
        }

        lines
    }

    /// Renders vertical grid lines
    fn render_vertical_lines(&self) -> Vec<impl IntoElement> {
        let mut lines = Vec::new();
        let (_, grid_height) = self.visible_dimensions();

        for x in self.board_range.x.0..=self.board_range.x.1 {
            let relative_x = (x - self.board_range.x.0) as f32;
            let pixel_x = relative_x * self.vertex_size;

            lines.push(
                div()
                    .absolute()
                    .left(px(pixel_x))
                    .top(px(0.0))
                    .w(px(self.theme.grid_line_width))
                    .h(px(grid_height))
                    .bg(self.theme.grid_line_color),
            );
        }

        lines
    }

    /// Renders all grid lines
    pub fn render_grid_lines(&self) -> impl IntoElement {
        let (width, height) = self.visible_dimensions();

        let mut grid_container = div().relative().w(px(width)).h(px(height));

        // Add horizontal lines
        for line in self.render_horizontal_lines() {
            grid_container = grid_container.child(line);
        }

        // Add vertical lines
        for line in self.render_vertical_lines() {
            grid_container = grid_container.child(line);
        }

        grid_container
    }

    /// Renders the complete grid with background and lines
    pub fn render(&self) -> impl IntoElement {
        let (width, height) = self.visible_dimensions();

        // Create container with background
        let mut container = div()
            .w(px(width))
            .h(px(height))
            .bg(self.theme.background_color)
            .border_1()
            .border_color(self.theme.border_color)
            .relative();

        // Add horizontal lines
        for line in self.render_horizontal_lines() {
            container = container.child(line);
        }

        // Add vertical lines
        for line in self.render_vertical_lines() {
            container = container.child(line);
        }

        container
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_vertex_to_pixel_with_range() {
        let range = BoardRange::new((3, 15), (3, 15)); // 13x13 visible area
        let grid = Grid::new(range, 20.0);

        // Vertices relative to the range
        assert_eq!(grid.vertex_to_pixel(&Vertex::new(3, 3)), (0.0, 0.0)); // Top-left of range
        assert_eq!(grid.vertex_to_pixel(&Vertex::new(15, 15)), (240.0, 240.0)); // Bottom-right: (15-3)*20 = 240
        assert_eq!(grid.vertex_to_pixel(&Vertex::new(9, 9)), (120.0, 120.0)); // Center: (9-3)*20 = 120
    }

    #[test]
    fn test_visible_dimensions() {
        let range = BoardRange::new((0, 18), (0, 18)); // 19x19 board
        let grid = Grid::new(range, 25.0);

        assert_eq!(grid.visible_dimensions(), (475.0, 475.0)); // 19 * 25

        let partial_range = BoardRange::new((5, 14), (5, 14)); // 10x10 partial board
        let partial_grid = Grid::new(partial_range, 30.0);

        assert_eq!(partial_grid.visible_dimensions(), (300.0, 300.0)); // 10 * 30
    }

    #[test]
    fn test_grid_theme() {
        let range = BoardRange::new((0, 8), (0, 8));
        let custom_theme = GridTheme {
            background_color: rgb(0x123456),
            grid_line_color: rgb(0x654321),
            grid_line_width: 2.0,
            border_color: rgb(0xabcdef),
            border_width: 3.0,
        };

        let grid = Grid::new(range, 24.0).with_theme(custom_theme.clone());

        assert_eq!(grid.theme.background_color, custom_theme.background_color);
        assert_eq!(grid.theme.grid_line_width, 2.0);
        assert_eq!(grid.theme.border_width, 3.0);
    }

    #[test]
    fn test_grid_line_count() {
        let range = BoardRange::new((0, 8), (0, 8)); // 9x9 board
        let grid = Grid::new(range, 24.0);

        // Should have 9 horizontal lines (y=0 to y=8)
        let horizontal_lines = grid.render_horizontal_lines();
        assert_eq!(horizontal_lines.len(), 9);

        // Should have 9 vertical lines (x=0 to x=8)
        let vertical_lines = grid.render_vertical_lines();
        assert_eq!(vertical_lines.len(), 9);
    }

    #[test]
    fn test_partial_board_grid_lines() {
        let range = BoardRange::new((3, 7), (2, 6)); // 5x5 visible area
        let grid = Grid::new(range, 20.0);

        // Should have 5 horizontal lines (y=2 to y=6)
        let horizontal_lines = grid.render_horizontal_lines();
        assert_eq!(horizontal_lines.len(), 5);

        // Should have 5 vertical lines (x=3 to x=7)
        let vertical_lines = grid.render_vertical_lines();
        assert_eq!(vertical_lines.len(), 5);
    }
}
