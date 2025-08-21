use crate::go_board::types::BoardRange;
use gpui::*;

/// Coordinate labeling theme for styling coordinate labels
#[derive(Clone, Debug)]
pub struct CoordinateTheme {
    pub color: Rgba,
    pub font_size: f32,
    pub font_family: String,
    pub margin: f32, // Distance from board edge
}

impl Default for CoordinateTheme {
    fn default() -> Self {
        Self {
            color: rgb(0x000000), // Black labels
            font_size: 12.0,
            font_family: "system-ui".to_string(),
            margin: 8.0,
        }
    }
}

/// Function type for custom coordinate labeling
pub type CoordFunction = fn(usize) -> String;

/// Default coordinate functions for standard Go notation
pub fn default_coord_x(x: usize) -> String {
    // A-T columns, skipping I (A=0, B=1, C=2, ..., H=7, J=8, K=9, ...)
    let letter = if x < 8 {
        (b'A' + x as u8) as char
    } else {
        (b'A' + x as u8 + 1) as char // Skip 'I'
    };
    letter.to_string()
}

pub fn default_coord_y(y: usize) -> String {
    // Numbers 1-19, but inverted (1 is at bottom, 19 is at top)
    (y + 1).to_string()
}

/// Coordinate labels component for Go board
pub struct CoordinateLabels {
    board_range: BoardRange,
    vertex_size: f32,
    theme: CoordinateTheme,
    coord_x: CoordFunction,
    coord_y: CoordFunction,
    show_top: bool,
    show_bottom: bool,
    show_left: bool,
    show_right: bool,
}

impl CoordinateLabels {
    /// Creates new coordinate labels
    pub fn new(board_range: BoardRange, vertex_size: f32) -> Self {
        Self {
            board_range,
            vertex_size,
            theme: CoordinateTheme::default(),
            coord_x: default_coord_x,
            coord_y: default_coord_y,
            show_top: true,
            show_bottom: true,
            show_left: true,
            show_right: true,
        }
    }

    /// Sets the coordinate theme
    pub fn with_theme(mut self, theme: CoordinateTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Sets custom coordinate functions
    pub fn with_coord_functions(mut self, coord_x: CoordFunction, coord_y: CoordFunction) -> Self {
        self.coord_x = coord_x;
        self.coord_y = coord_y;
        self
    }

    /// Sets which sides to show coordinates on
    pub fn with_sides(mut self, top: bool, bottom: bool, left: bool, right: bool) -> Self {
        self.show_top = top;
        self.show_bottom = bottom;
        self.show_left = left;
        self.show_right = right;
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

    /// Calculates the total dimensions including coordinate labels
    pub fn total_dimensions(&self) -> (f32, f32) {
        // Use the same dimension calculation as the grid
        let grid_intervals_x = (self.board_range.x.1 - self.board_range.x.0) as f32;
        let grid_intervals_y = (self.board_range.y.1 - self.board_range.y.0) as f32;
        let grid_width = grid_intervals_x * self.vertex_size + self.vertex_size;
        let grid_height = grid_intervals_y * self.vertex_size + self.vertex_size;

        let margin = self.theme.margin + self.theme.font_size;

        let width = grid_width
            + if self.show_left { margin } else { 0.0 }
            + if self.show_right { margin } else { 0.0 };

        let height = grid_height
            + if self.show_top { margin } else { 0.0 }
            + if self.show_bottom { margin } else { 0.0 };

        (width, height)
    }

    /// Gets the offset for the grid within the coordinate container
    pub fn grid_offset(&self) -> (f32, f32) {
        let margin = self.theme.margin + self.theme.font_size;
        let x_offset = if self.show_left { margin } else { 0.0 };
        let y_offset = if self.show_top { margin } else { 0.0 };

        (x_offset, y_offset)
    }

    /// Renders coordinate labels for all enabled sides
    pub fn render_coordinates(&self) -> impl IntoElement {
        let (total_width, total_height) = self.total_dimensions();
        let (grid_offset_x, grid_offset_y) = self.grid_offset();
        let _grid_width = (self.board_range.x.1 - self.board_range.x.0) as f32 * self.vertex_size;
        let _grid_height = (self.board_range.y.1 - self.board_range.y.0) as f32 * self.vertex_size;

        let mut container = div().relative().w(px(total_width)).h(px(total_height));

        // Top coordinates
        if self.show_top {
            for x in self.board_range.x.0..=self.board_range.x.1 {
                let relative_x = (x - self.board_range.x.0) as f32;
                // Add half vertex size offset to align with grid intersections
                let pixel_x =
                    grid_offset_x + relative_x * self.vertex_size + self.vertex_size / 2.0;
                let label = (self.coord_x)(x);

                container = container.child(
                    div()
                        .absolute()
                        .left(px(pixel_x - self.theme.font_size / 2.0))
                        .top(px(0.0))
                        .w(px(self.theme.font_size))
                        .h(px(self.theme.font_size))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(self.theme.font_size))
                        .text_color(self.theme.color)
                        .child(label),
                );
            }
        }

        // Bottom coordinates
        if self.show_bottom {
            for x in self.board_range.x.0..=self.board_range.x.1 {
                let relative_x = (x - self.board_range.x.0) as f32;
                // Add half vertex size offset to align with grid intersections
                let pixel_x =
                    grid_offset_x + relative_x * self.vertex_size + self.vertex_size / 2.0;
                let label = (self.coord_x)(x);

                container = container.child(
                    div()
                        .absolute()
                        .left(px(pixel_x - self.theme.font_size / 2.0))
                        .top(px(total_height - self.theme.font_size))
                        .w(px(self.theme.font_size))
                        .h(px(self.theme.font_size))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(self.theme.font_size))
                        .text_color(self.theme.color)
                        .child(label),
                );
            }
        }

        // Left coordinates (numbers, inverted for Go board)
        if self.show_left {
            for y in self.board_range.y.0..=self.board_range.y.1 {
                let relative_y = (y - self.board_range.y.0) as f32;
                // Add half vertex size offset to align with grid intersections
                let pixel_y =
                    grid_offset_y + relative_y * self.vertex_size + self.vertex_size / 2.0;
                // For Go boards, y=0 is top, but coordinate 1 should be at bottom
                let inverted_y = self.board_range.y.1 - y + self.board_range.y.0;
                let label = (self.coord_y)(inverted_y);

                container = container.child(
                    div()
                        .absolute()
                        .left(px(0.0))
                        .top(px(pixel_y - self.theme.font_size / 2.0))
                        .w(px(self.theme.font_size))
                        .h(px(self.theme.font_size))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(self.theme.font_size))
                        .text_color(self.theme.color)
                        .child(label),
                );
            }
        }

        // Right coordinates (numbers, inverted for Go board)
        if self.show_right {
            for y in self.board_range.y.0..=self.board_range.y.1 {
                let relative_y = (y - self.board_range.y.0) as f32;
                // Add half vertex size offset to align with grid intersections
                let pixel_y =
                    grid_offset_y + relative_y * self.vertex_size + self.vertex_size / 2.0;
                // For Go boards, y=0 is top, but coordinate 1 should be at bottom
                let inverted_y = self.board_range.y.1 - y + self.board_range.y.0;
                let label = (self.coord_y)(inverted_y);

                container = container.child(
                    div()
                        .absolute()
                        .left(px(total_width - self.theme.font_size))
                        .top(px(pixel_y - self.theme.font_size / 2.0))
                        .w(px(self.theme.font_size))
                        .h(px(self.theme.font_size))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(self.theme.font_size))
                        .text_color(self.theme.color)
                        .child(label),
                );
            }
        }

        container
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_coord_x() {
        assert_eq!(default_coord_x(0), "A");
        assert_eq!(default_coord_x(7), "H");
        assert_eq!(default_coord_x(8), "J"); // Skip I
        assert_eq!(default_coord_x(17), "S");
        assert_eq!(default_coord_x(18), "T");
    }

    #[test]
    fn test_default_coord_y() {
        assert_eq!(default_coord_y(0), "1");
        assert_eq!(default_coord_y(9), "10");
        assert_eq!(default_coord_y(18), "19");
    }

    #[test]
    fn test_coordinate_labels_creation() {
        let range = BoardRange::new((0, 18), (0, 18));
        let coords = CoordinateLabels::new(range.clone(), 20.0);

        assert_eq!(coords.board_range, range);
        assert_eq!(coords.vertex_size, 20.0);
        assert!(coords.show_top);
        assert!(coords.show_bottom);
        assert!(coords.show_left);
        assert!(coords.show_right);
    }

    #[test]
    fn test_coordinate_dimensions() {
        let range = BoardRange::new((0, 8), (0, 8)); // 9x9 board
        let coords = CoordinateLabels::new(range, 20.0);

        let (total_width, total_height) = coords.total_dimensions();
        // Grid: 8 * 20 = 160, margins: 2 * (8 + 12) = 40
        assert_eq!(total_width, 200.0);
        assert_eq!(total_height, 200.0);
    }

    #[test]
    fn test_grid_offset() {
        let range = BoardRange::new((0, 8), (0, 8));
        let coords = CoordinateLabels::new(range, 20.0);

        let (x_offset, y_offset) = coords.grid_offset();
        // margin + font_size = 8 + 12 = 20
        assert_eq!(x_offset, 20.0);
        assert_eq!(y_offset, 20.0);
    }

    #[test]
    fn test_coordinate_with_sides() {
        let range = BoardRange::new((0, 8), (0, 8));
        let coords = CoordinateLabels::new(range, 20.0).with_sides(true, false, true, false); // Only top and left

        assert!(coords.show_top);
        assert!(!coords.show_bottom);
        assert!(coords.show_left);
        assert!(!coords.show_right);

        let (total_width, total_height) = coords.total_dimensions();
        // Grid: 160, left margin: 20, no right margin
        assert_eq!(total_width, 180.0);
        // Grid: 160, top margin: 20, no bottom margin
        assert_eq!(total_height, 180.0);
    }
}
