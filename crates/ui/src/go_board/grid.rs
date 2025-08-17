use crate::go_board::coordinates::{
    default_coord_x, default_coord_y, CoordFunction, CoordinateLabels, CoordinateTheme,
};
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
    pub star_point_color: Rgba,
    pub star_point_size: f32,
}

impl Default for GridTheme {
    fn default() -> Self {
        Self {
            background_color: rgb(0xebb55b), // Shudan default wood color
            grid_line_color: rgb(0x000000),  // Black grid lines
            grid_line_width: 1.0,
            border_color: rgb(0xca933a), // Wood border color
            border_width: 4.0,
            star_point_color: rgb(0x000000), // Black star points
            star_point_size: 6.0,            // Default star point size
        }
    }
}

pub struct Grid {
    board_range: BoardRange,
    vertex_size: f32,
    theme: GridTheme,
    show_coordinates: bool,
    coordinate_theme: CoordinateTheme,
    coord_x: CoordFunction,
    coord_y: CoordFunction,
}

impl Grid {
    /// Creates a new Grid component
    pub fn new(board_range: BoardRange, vertex_size: f32) -> Self {
        Self {
            board_range,
            vertex_size,
            theme: GridTheme::default(),
            show_coordinates: false,
            coordinate_theme: CoordinateTheme::default(),
            coord_x: default_coord_x,
            coord_y: default_coord_y,
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

    /// Sets the coordinate theme
    pub fn with_coordinate_theme(mut self, theme: CoordinateTheme) -> Self {
        self.coordinate_theme = theme;
        self
    }

    /// Sets custom coordinate functions
    pub fn with_coordinate_functions(
        mut self,
        coord_x: CoordFunction,
        coord_y: CoordFunction,
    ) -> Self {
        self.coord_x = coord_x;
        self.coord_y = coord_y;
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

        // Add half vertex size offset to center stones on grid intersections
        let offset = self.vertex_size / 2.0;
        (
            relative_x * self.vertex_size + offset,
            relative_y * self.vertex_size + offset,
        )
    }

    /// Gets the visible board dimensions
    fn visible_dimensions(&self) -> (f32, f32) {
        // Calculate dimensions based on the number of grid intervals, not vertices
        // For n vertices, we need (n-1) intervals plus some padding for the border
        let grid_intervals_x = (self.board_range.x.1 - self.board_range.x.0) as f32;
        let grid_intervals_y = (self.board_range.y.1 - self.board_range.y.0) as f32;

        // Add half vertex size padding on each side for proper stone placement
        let width = grid_intervals_x * self.vertex_size + self.vertex_size;
        let height = grid_intervals_y * self.vertex_size + self.vertex_size;
        (width, height)
    }

    /// Calculates hoshi (star point) positions for standard Go board sizes
    pub fn calculate_hoshi_positions(&self) -> Vec<Vertex> {
        let full_width = self.board_range.x.1 + 1;
        let full_height = self.board_range.y.1 + 1;

        // Only calculate for the full board, then filter for visible range
        let mut positions = Vec::new();

        match (full_width, full_height) {
            // 19x19 board - standard pattern
            (19, 19) => {
                positions.extend([
                    Vertex::new(3, 3),
                    Vertex::new(9, 3),
                    Vertex::new(15, 3),
                    Vertex::new(3, 9),
                    Vertex::new(9, 9),
                    Vertex::new(15, 9),
                    Vertex::new(3, 15),
                    Vertex::new(9, 15),
                    Vertex::new(15, 15),
                ]);
            }
            // 13x13 board - standard pattern
            (13, 13) => {
                positions.extend([
                    Vertex::new(3, 3),
                    Vertex::new(9, 3),
                    Vertex::new(6, 6),
                    Vertex::new(3, 9),
                    Vertex::new(9, 9),
                ]);
            }
            // 9x9 board - standard pattern
            (9, 9) => {
                positions.extend([
                    Vertex::new(2, 2),
                    Vertex::new(6, 2),
                    Vertex::new(4, 4),
                    Vertex::new(2, 6),
                    Vertex::new(6, 6),
                ]);
            }
            // Custom sizes - generate star points based on board size
            (w, h) if w >= 7 && h >= 7 => {
                let center_x = w / 2;
                let center_y = h / 2;
                let corner_offset = if w <= 11 { 2 } else { 3 };

                // Add corner points
                if w >= 7 && h >= 7 {
                    positions.extend([
                        Vertex::new(corner_offset, corner_offset),
                        Vertex::new(w - 1 - corner_offset, corner_offset),
                        Vertex::new(corner_offset, h - 1 - corner_offset),
                        Vertex::new(w - 1 - corner_offset, h - 1 - corner_offset),
                    ]);
                }

                // Add center point for odd-sized boards
                if w % 2 == 1 && h % 2 == 1 {
                    positions.push(Vertex::new(center_x, center_y));
                }
            }
            _ => {} // No star points for very small boards
        }

        // Filter positions to only include those within the visible range
        positions
            .into_iter()
            .filter(|pos| {
                pos.x >= self.board_range.x.0
                    && pos.x <= self.board_range.x.1
                    && pos.y >= self.board_range.y.0
                    && pos.y <= self.board_range.y.1
            })
            .collect()
    }

    /// Renders star points (hoshi) as circles
    pub fn render_star_points(&self) -> Vec<impl IntoElement> {
        let hoshi_positions = self.calculate_hoshi_positions();
        let mut star_points = Vec::new();

        for pos in hoshi_positions {
            let (pixel_x, pixel_y) = self.vertex_to_pixel(&pos);
            let radius = self.theme.star_point_size / 2.0;

            star_points.push(
                div()
                    .absolute()
                    .left(px(pixel_x - radius))
                    .top(px(pixel_y - radius))
                    .w(px(self.theme.star_point_size))
                    .h(px(self.theme.star_point_size))
                    .rounded_full()
                    .bg(self.theme.star_point_color),
            );
        }

        star_points
    }

    /// Renders horizontal grid lines
    fn render_horizontal_lines(&self) -> Vec<impl IntoElement> {
        let mut lines = Vec::new();
        let (grid_width, _) = self.visible_dimensions();
        let offset = self.vertex_size / 2.0;

        for y in self.board_range.y.0..=self.board_range.y.1 {
            let relative_y = (y - self.board_range.y.0) as f32;
            let pixel_y = relative_y * self.vertex_size + offset;

            lines.push(
                div()
                    .absolute()
                    .left(px(offset))
                    .top(px(pixel_y - self.theme.grid_line_width / 2.0))
                    .w(px(grid_width - self.vertex_size))
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
        let offset = self.vertex_size / 2.0;

        for x in self.board_range.x.0..=self.board_range.x.1 {
            let relative_x = (x - self.board_range.x.0) as f32;
            let pixel_x = relative_x * self.vertex_size + offset;

            lines.push(
                div()
                    .absolute()
                    .left(px(pixel_x - self.theme.grid_line_width / 2.0))
                    .top(px(offset))
                    .w(px(self.theme.grid_line_width))
                    .h(px(grid_height - self.vertex_size))
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

    /// Renders the complete grid with background, lines, and optional coordinates
    pub fn render(&self) -> AnyElement {
        if self.show_coordinates {
            self.render_with_coordinates().into_any_element()
        } else {
            self.render_grid_only().into_any_element()
        }
    }

    /// Renders the grid with texture support
    pub fn render_with_texture(&self, texture_element: Option<impl IntoElement>) -> AnyElement {
        if self.show_coordinates {
            self.render_with_coordinates_and_texture(texture_element)
                .into_any_element()
        } else {
            self.render_grid_only_with_texture(texture_element)
                .into_any_element()
        }
    }

    /// Renders the grid without coordinates
    fn render_grid_only(&self) -> impl IntoElement {
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

        // Add star points (hoshi)
        for star_point in self.render_star_points() {
            container = container.child(star_point);
        }

        container
    }

    /// Renders the grid with coordinate labels
    fn render_with_coordinates(&self) -> impl IntoElement {
        let coordinate_labels = CoordinateLabels::new(self.board_range.clone(), self.vertex_size)
            .with_theme(self.coordinate_theme.clone())
            .with_coord_functions(self.coord_x, self.coord_y);

        let (grid_offset_x, grid_offset_y) = coordinate_labels.grid_offset();
        let (total_width, total_height) = coordinate_labels.total_dimensions();
        let (width, height) = self.visible_dimensions();

        // Create main container with relative positioning
        let mut main_container = div().relative().w(px(total_width)).h(px(total_height));

        // Add coordinate labels as background layer
        main_container = main_container.child(
            div()
                .absolute()
                .inset_0()
                .child(coordinate_labels.render_coordinates()),
        );

        // Create grid container positioned within the coordinate space
        let mut grid_container = div()
            .absolute()
            .left(px(grid_offset_x))
            .top(px(grid_offset_y))
            .w(px(width))
            .h(px(height))
            .bg(self.theme.background_color)
            .border_1()
            .border_color(self.theme.border_color)
            .relative();

        // Add horizontal lines
        for line in self.render_horizontal_lines() {
            grid_container = grid_container.child(line);
        }

        // Add vertical lines
        for line in self.render_vertical_lines() {
            grid_container = grid_container.child(line);
        }

        // Add star points (hoshi)
        for star_point in self.render_star_points() {
            grid_container = grid_container.child(star_point);
        }

        main_container.child(grid_container)
    }

    /// Renders the grid without coordinates but with texture support
    fn render_grid_only_with_texture(
        &self,
        texture_element: Option<impl IntoElement>,
    ) -> impl IntoElement {
        let (width, height) = self.visible_dimensions();

        // Create container with background
        let mut container = div()
            .w(px(width))
            .h(px(height))
            .bg(self.theme.background_color)
            .border_1()
            .border_color(self.theme.border_color)
            .relative();

        // Add texture as background if provided
        if let Some(texture) = texture_element {
            container = container.child(div().absolute().inset_0().child(texture));
        }

        // Add horizontal lines
        for line in self.render_horizontal_lines() {
            container = container.child(line);
        }

        // Add vertical lines
        for line in self.render_vertical_lines() {
            container = container.child(line);
        }

        // Add star points
        for star_point in self.render_star_points() {
            container = container.child(star_point);
        }

        container
    }

    /// Renders the grid with coordinates and texture support
    fn render_with_coordinates_and_texture(
        &self,
        texture_element: Option<impl IntoElement>,
    ) -> impl IntoElement {
        let coordinate_labels = CoordinateLabels::new(self.board_range.clone(), self.vertex_size)
            .with_theme(self.coordinate_theme.clone())
            .with_coord_functions(self.coord_x, self.coord_y);

        let (grid_offset_x, grid_offset_y) = coordinate_labels.grid_offset();
        let (total_width, total_height) = coordinate_labels.total_dimensions();
        let (width, height) = self.visible_dimensions();

        // Create main container with relative positioning
        let mut main_container = div().relative().w(px(total_width)).h(px(total_height));

        // Add coordinate labels as background layer
        main_container = main_container.child(
            div()
                .absolute()
                .inset_0()
                .child(coordinate_labels.render_coordinates()),
        );

        // Create grid container positioned within the coordinate space with texture
        let mut grid_container = div()
            .absolute()
            .left(px(grid_offset_x))
            .top(px(grid_offset_y))
            .w(px(width))
            .h(px(height))
            .bg(self.theme.background_color)
            .border_1()
            .border_color(self.theme.border_color)
            .relative();

        // Add texture as background if provided
        if let Some(texture) = texture_element {
            grid_container = grid_container.child(div().absolute().inset_0().child(texture));
        }

        // Add horizontal lines
        for line in self.render_horizontal_lines() {
            grid_container = grid_container.child(line);
        }

        // Add vertical lines
        for line in self.render_vertical_lines() {
            grid_container = grid_container.child(line);
        }

        // Add star points
        for star_point in self.render_star_points() {
            grid_container = grid_container.child(star_point);
        }

        main_container.child(grid_container)
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

        // Test corner vertices (now centered on grid intersections)
        assert_eq!(grid.vertex_to_pixel(&Vertex::new(0, 0)), (10.0, 10.0)); // Half vertex offset
        assert_eq!(grid.vertex_to_pixel(&Vertex::new(18, 18)), (370.0, 370.0)); // 18 * 20 + 10
        assert_eq!(grid.vertex_to_pixel(&Vertex::new(9, 9)), (190.0, 190.0)); // 9 * 20 + 10
    }

    #[test]
    fn test_vertex_to_pixel_with_range() {
        let range = BoardRange::new((3, 15), (3, 15)); // 13x13 visible area
        let grid = Grid::new(range, 20.0);

        // Vertices relative to the range (now centered on grid intersections)
        assert_eq!(grid.vertex_to_pixel(&Vertex::new(3, 3)), (10.0, 10.0)); // Top-left of range + offset
        assert_eq!(grid.vertex_to_pixel(&Vertex::new(15, 15)), (250.0, 250.0)); // Bottom-right: (15-3)*20 + 10 = 250
        assert_eq!(grid.vertex_to_pixel(&Vertex::new(9, 9)), (130.0, 130.0)); // Center: (9-3)*20 + 10 = 130
    }

    #[test]
    fn test_visible_dimensions() {
        let range = BoardRange::new((0, 18), (0, 18)); // 19x19 board
        let grid = Grid::new(range, 25.0);

        assert_eq!(grid.visible_dimensions(), (475.0, 475.0)); // 18 * 25 + 25 = 475

        let partial_range = BoardRange::new((5, 14), (5, 14)); // 10x10 partial board
        let partial_grid = Grid::new(partial_range, 30.0);

        assert_eq!(partial_grid.visible_dimensions(), (300.0, 300.0)); // 9 * 30 + 30 = 300
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

    #[test]
    fn test_hoshi_calculation_19x19() {
        let range = BoardRange::new((0, 18), (0, 18)); // Full 19x19 board
        let grid = Grid::new(range, 20.0);

        let hoshi_positions = grid.calculate_hoshi_positions();

        // 19x19 should have 9 star points
        assert_eq!(hoshi_positions.len(), 9);

        // Verify some key positions
        assert!(hoshi_positions.contains(&Vertex::new(3, 3)));
        assert!(hoshi_positions.contains(&Vertex::new(9, 9))); // Center
        assert!(hoshi_positions.contains(&Vertex::new(15, 15)));
    }

    #[test]
    fn test_hoshi_calculation_13x13() {
        let range = BoardRange::new((0, 12), (0, 12)); // Full 13x13 board
        let grid = Grid::new(range, 20.0);

        let hoshi_positions = grid.calculate_hoshi_positions();

        // 13x13 should have 5 star points
        assert_eq!(hoshi_positions.len(), 5);

        // Verify center position
        assert!(hoshi_positions.contains(&Vertex::new(6, 6)));
    }

    #[test]
    fn test_hoshi_calculation_9x9() {
        let range = BoardRange::new((0, 8), (0, 8)); // Full 9x9 board
        let grid = Grid::new(range, 20.0);

        let hoshi_positions = grid.calculate_hoshi_positions();

        // 9x9 should have 5 star points
        assert_eq!(hoshi_positions.len(), 5);

        // Verify center position
        assert!(hoshi_positions.contains(&Vertex::new(4, 4)));
    }

    #[test]
    fn test_hoshi_filtering_partial_range() {
        let range = BoardRange::new((5, 14), (5, 14)); // Partial view of 19x19
        let grid = Grid::new(range, 20.0);

        let hoshi_positions = grid.calculate_hoshi_positions();

        // Should only include hoshi points within the visible range
        for pos in &hoshi_positions {
            assert!(pos.x >= 5 && pos.x <= 14);
            assert!(pos.y >= 5 && pos.y <= 14);
        }
    }
}
