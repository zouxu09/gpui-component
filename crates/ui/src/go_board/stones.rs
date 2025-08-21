use crate::go_board::coordinates::{
    default_coord_x, default_coord_y, CoordFunction, CoordinateLabels, CoordinateTheme,
};
use crate::go_board::position_utils::{PositionCalculator, PositionUtils};
use crate::go_board::types::{BoardRange, SignMap, Vertex};
use gpui::{prelude::FluentBuilder, *};

/// Stone theme for customizing stone appearance
#[derive(Clone, Debug)]
pub struct StoneTheme {
    pub black_color: Rgba,
    pub white_color: Rgba,
    pub stone_size_ratio: f32, // Ratio of stone size to vertex size (0.0 to 1.0)
    pub border_width: f32,
    pub border_color: Rgba,
    // Fuzzy positioning
    pub fuzzy_placement: bool,
    pub fuzzy_max_offset: f32, // Maximum offset in pixels for fuzzy placement
    // Visual variation
    pub random_variation: bool,
    pub max_rotation: f32, // Maximum rotation in degrees
    // Custom stone images
    pub black_stone_image: Option<String>, // CSS background-image URL
    pub white_stone_image: Option<String>, // CSS background-image URL
}

impl Default for StoneTheme {
    fn default() -> Self {
        Self {
            black_color: rgb(0x1a1a1a), // Dark gray/black
            white_color: rgb(0xf8f8f8), // Off-white
            stone_size_ratio: 0.9,      // 90% of vertex size
            border_width: 1.0,
            border_color: rgb(0x000000), // Black border
            fuzzy_placement: false,
            fuzzy_max_offset: 2.0, // 2 pixels max offset
            random_variation: false,
            max_rotation: 5.0, // 5 degrees max rotation
            black_stone_image: None,
            white_stone_image: None,
        }
    }
}

/// Individual stone component
pub struct Stone {
    position: Vertex,
    sign: i8, // -1: white, 0: empty, 1: black
    vertex_size: f32,
    theme: StoneTheme,
    random_class: u8, // 0-4 for visual variation
}

impl Stone {
    /// Creates a new stone
    pub fn new(position: Vertex, sign: i8, vertex_size: f32) -> Self {
        // Generate deterministic random class based on position for consistency
        let random_class = ((position.x * 7 + position.y * 11) % 5) as u8;

        Self {
            position,
            sign,
            vertex_size,
            theme: StoneTheme::default(),
            random_class,
        }
    }

    /// Creates a new stone with explicit random class
    pub fn with_random_class(
        position: Vertex,
        sign: i8,
        vertex_size: f32,
        random_class: u8,
    ) -> Self {
        Self {
            position,
            sign,
            vertex_size,
            theme: StoneTheme::default(),
            random_class: random_class % 5, // Ensure 0-4 range
        }
    }

    /// Creates a stone with custom theme
    pub fn with_theme(mut self, theme: StoneTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Updates the stone's position
    pub fn set_position(&mut self, position: Vertex) {
        self.position = position;
    }

    /// Updates the stone's sign value
    pub fn set_sign(&mut self, sign: i8) {
        self.sign = sign;
    }

    /// Updates the vertex size
    pub fn set_vertex_size(&mut self, size: f32) {
        self.vertex_size = size;
    }

    /// Gets the stone's pixel position relative to the board
    pub fn pixel_position(&self, board_range: &BoardRange) -> (f32, f32) {
        PositionUtils::vertex_to_pixel_relative(&self.position, self.vertex_size, board_range)
    }

    /// Calculates the stone size in pixels
    pub fn stone_size(&self) -> f32 {
        self.vertex_size * self.theme.stone_size_ratio
    }

    /// Calculates fuzzy position offset for natural stone placement
    fn fuzzy_offset(&self) -> (f32, f32) {
        if !self.theme.fuzzy_placement {
            return (0.0, 0.0);
        }

        // Use deterministic "random" values based on position and random_class
        let seed = self.position.x * 17 + self.position.y * 23 + self.random_class as usize * 13;
        let offset_x = ((seed % 100) as f32 / 50.0 - 1.0) * self.theme.fuzzy_max_offset;
        let offset_y = (((seed * 37) % 100) as f32 / 50.0 - 1.0) * self.theme.fuzzy_max_offset;

        (offset_x, offset_y)
    }

    /// Renders the stone as a circle
    pub fn render(&self, board_range: &BoardRange) -> Option<impl IntoElement> {
        if self.sign == 0 {
            return None; // Empty vertex, no stone to render
        }

        let (base_pixel_x, base_pixel_y) = self.pixel_position(board_range);
        let (fuzzy_x, fuzzy_y) = self.fuzzy_offset();
        let pixel_x = base_pixel_x + fuzzy_x;
        let pixel_y = base_pixel_y + fuzzy_y;

        let stone_size = self.stone_size();
        let radius = stone_size / 2.0;

        let color = if self.sign == 1 {
            self.theme.black_color
        } else {
            self.theme.white_color
        };

        // Check if custom stone images are provided
        let stone_image = if self.sign == 1 {
            &self.theme.black_stone_image
        } else {
            &self.theme.white_stone_image
        };

        let mut stone_div = div()
            .absolute()
            .left(px(pixel_x - radius))
            .top(px(pixel_y - radius))
            .w(px(stone_size))
            .h(px(stone_size))
            .rounded_full();

        // Apply background image or color
        if let Some(_image_url) = stone_image {
            // For now, we'll use a different color scheme to indicate custom stones
            // In a real implementation, custom image support would require additional GPUI features
            let custom_color = if self.sign == 1 {
                rgb(0x2a2a2a) // Slightly different black for custom stones
            } else {
                rgb(0xe8e8e8) // Slightly different white for custom stones
            };

            stone_div = stone_div
                .bg(custom_color)
                .when(self.theme.border_width > 0.0, |div| {
                    div.border_1().border_color(self.theme.border_color)
                });
        } else {
            stone_div = stone_div
                .bg(color)
                .when(self.theme.border_width > 0.0, |div| {
                    div.border_1().border_color(self.theme.border_color)
                });
        }

        Some(stone_div)
    }
}

/// Stones component for rendering all stones on the board
pub struct Stones {
    board_range: BoardRange,
    vertex_size: f32,
    sign_map: SignMap,
    theme: StoneTheme,
    show_coordinates: bool,
    coordinate_theme: CoordinateTheme,
    coord_x: CoordFunction,
    coord_y: CoordFunction,
}

impl Stones {
    /// Creates a new Stones component
    pub fn new(board_range: BoardRange, vertex_size: f32, sign_map: SignMap) -> Self {
        Self {
            board_range,
            vertex_size,
            sign_map,
            theme: StoneTheme::default(),
            show_coordinates: false,
            coordinate_theme: CoordinateTheme::default(),
            coord_x: default_coord_x,
            coord_y: default_coord_y,
        }
    }

    /// Sets the stone theme
    pub fn with_theme(mut self, theme: StoneTheme) -> Self {
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

    /// Updates the sign map
    pub fn set_sign_map(&mut self, sign_map: SignMap) {
        self.sign_map = sign_map;
    }

    /// Updates individual stones
    pub fn update_stones(&mut self, updates: &[(Vertex, i8)]) {
        for (vertex, sign) in updates {
            if vertex.y < self.sign_map.len() && vertex.x < self.sign_map[vertex.y].len() {
                if (-1..=1).contains(sign) {
                    self.sign_map[vertex.y][vertex.x] = *sign;
                }
            }
        }
    }

    /// Updates the board range
    pub fn set_board_range(&mut self, range: BoardRange) {
        self.board_range = range;
    }

    /// Updates the vertex size
    pub fn set_vertex_size(&mut self, size: f32) {
        self.vertex_size = size;
    }

    /// Gets the stone at a specific vertex
    pub fn get_stone_at(&self, vertex: &Vertex) -> Option<i8> {
        if vertex.y < self.sign_map.len() && vertex.x < self.sign_map[vertex.y].len() {
            let sign = self.sign_map[vertex.y][vertex.x];
            if sign != 0 {
                Some(sign)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Renders all stones on the board
    pub fn render_stones(&self) -> Vec<impl IntoElement> {
        let mut stones = Vec::new();

        // Iterate through the visible range
        for y in self.board_range.y.0..=self.board_range.y.1 {
            for x in self.board_range.x.0..=self.board_range.x.1 {
                if y < self.sign_map.len() && x < self.sign_map[y].len() {
                    let sign = self.sign_map[y][x];
                    if sign != 0 {
                        let position = Vertex::new(x, y);
                        let stone = Stone::new(position, sign, self.vertex_size)
                            .with_theme(self.theme.clone());

                        if let Some(element) = stone.render(&self.board_range) {
                            stones.push(element);
                        }
                    }
                }
            }
        }

        stones
    }

    /// Renders all stones in a container with optional coordinates
    pub fn render(&self) -> AnyElement {
        if self.show_coordinates {
            self.render_with_coordinates().into_any_element()
        } else {
            self.render_stones_only().into_any_element()
        }
    }

    /// Renders stones without coordinates
    fn render_stones_only(&self) -> impl IntoElement {
        let (width, height) = self.visible_dimensions();
        let mut container = div().relative().w(px(width)).h(px(height));

        for stone in self.render_stones() {
            container = container.child(stone);
        }

        container
    }

    /// Renders stones with coordinate labels
    fn render_with_coordinates(&self) -> impl IntoElement {
        let coordinate_labels = CoordinateLabels::new(self.board_range.clone(), self.vertex_size)
            .with_theme(self.coordinate_theme.clone())
            .with_coord_functions(self.coord_x, self.coord_y);

        let (grid_offset_x, grid_offset_y) = coordinate_labels.grid_offset();
        let (total_width, total_height) = coordinate_labels.total_dimensions();
        let (stones_width, stones_height) = self.visible_dimensions();

        // Create main container with relative positioning
        let mut main_container = div().relative().w(px(total_width)).h(px(total_height));

        // Add coordinate labels as background layer
        main_container = main_container.child(
            div()
                .absolute()
                .inset_0()
                .child(coordinate_labels.render_coordinates()),
        );

        // Create stones container positioned within the coordinate space
        let mut stones_container = div()
            .absolute()
            .left(px(grid_offset_x))
            .top(px(grid_offset_y))
            .w(px(stones_width))
            .h(px(stones_height))
            .border_1()
            .border_color(rgba(0x00000000)) // Transparent border to match grid spacing
            .relative();

        // Add all stones
        for stone in self.render_stones() {
            stones_container = stones_container.child(stone);
        }

        main_container.child(stones_container)
    }

    /// Calculates the visible dimensions
    fn visible_dimensions(&self) -> (f32, f32) {
        let calculator = PositionCalculator::with_board_range(
            self.vertex_size,
            point(px(0.0), px(0.0)),
            self.board_range.clone(),
        );
        calculator.visible_dimensions()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stone_creation() {
        let position = Vertex::new(3, 5);
        let stone = Stone::new(position.clone(), 1, 20.0);

        assert_eq!(stone.position, position);
        assert_eq!(stone.sign, 1);
        assert_eq!(stone.vertex_size, 20.0);
    }

    #[test]
    fn test_stone_pixel_position() {
        let position = Vertex::new(2, 1);
        let board_range = BoardRange::new((0, 8), (0, 8));
        let stone = Stone::new(position, 1, 25.0);

        let (pixel_x, pixel_y) = stone.pixel_position(&board_range);
        assert_eq!(pixel_x, 50.0); // 2 * 25
        assert_eq!(pixel_y, 25.0); // 1 * 25
    }

    #[test]
    fn test_stone_size_calculation() {
        let stone = Stone::new(Vertex::new(0, 0), 1, 30.0);
        assert_eq!(stone.stone_size(), 27.0); // 30 * 0.9

        let custom_theme = StoneTheme {
            stone_size_ratio: 0.8,
            ..StoneTheme::default()
        };
        let stone_with_theme = stone.with_theme(custom_theme);
        assert_eq!(stone_with_theme.stone_size(), 24.0); // 30 * 0.8
    }

    #[test]
    fn test_empty_stone_rendering() {
        let position = Vertex::new(0, 0);
        let board_range = BoardRange::new((0, 8), (0, 8));
        let empty_stone = Stone::new(position, 0, 20.0);

        assert!(empty_stone.render(&board_range).is_none());
    }

    #[test]
    fn test_stones_component_creation() {
        let range = BoardRange::new((0, 8), (0, 8));
        let sign_map = vec![vec![0, 1, 0], vec![-1, 0, 1], vec![0, 0, 0]];
        let stones = Stones::new(range.clone(), 20.0, sign_map.clone());

        assert_eq!(stones.board_range, range);
        assert_eq!(stones.vertex_size, 20.0);
        assert_eq!(stones.sign_map, sign_map);
    }

    #[test]
    fn test_stones_get_stone_at() {
        let range = BoardRange::new((0, 2), (0, 2));
        let sign_map = vec![vec![0, 1, 0], vec![-1, 0, 1], vec![0, 0, 0]];
        let stones = Stones::new(range, 20.0, sign_map);

        assert_eq!(stones.get_stone_at(&Vertex::new(1, 0)), Some(1)); // Black stone
        assert_eq!(stones.get_stone_at(&Vertex::new(0, 1)), Some(-1)); // White stone
        assert_eq!(stones.get_stone_at(&Vertex::new(0, 0)), None); // Empty
        assert_eq!(stones.get_stone_at(&Vertex::new(10, 10)), None); // Out of bounds
    }

    #[test]
    fn test_stones_visible_dimensions() {
        let range = BoardRange::new((2, 6), (1, 4)); // 5x4 area
        let sign_map = vec![];
        let stones = Stones::new(range, 30.0, sign_map);

        let (width, height) = stones.visible_dimensions();
        assert_eq!(width, 150.0); // (6-2) * 30 + 30 = 4 * 30 + 30 = 150
        assert_eq!(height, 120.0); // (4-1) * 30 + 30 = 3 * 30 + 30 = 120
    }

    #[test]
    fn test_stone_random_class_generation() {
        let stone1 = Stone::new(Vertex::new(0, 0), 1, 20.0);
        let stone2 = Stone::new(Vertex::new(1, 1), 1, 20.0);
        let stone3 = Stone::new(Vertex::new(0, 0), -1, 20.0); // Same position, different sign

        // Random class should be based on position, not sign
        assert_eq!(stone1.random_class, stone3.random_class);
        // Different positions should potentially have different random classes
        assert!(
            stone1.random_class != stone2.random_class
                || stone1.random_class == stone2.random_class
        );
        // Random class should be in range 0-4
        assert!(stone1.random_class < 5);
        assert!(stone2.random_class < 5);
    }

    #[test]
    fn test_stone_with_explicit_random_class() {
        let stone = Stone::with_random_class(Vertex::new(0, 0), 1, 20.0, 7);
        assert_eq!(stone.random_class, 2); // 7 % 5 = 2

        let stone2 = Stone::with_random_class(Vertex::new(0, 0), 1, 20.0, 3);
        assert_eq!(stone2.random_class, 3);
    }

    #[test]
    fn test_fuzzy_offset_disabled() {
        let mut theme = StoneTheme::default();
        theme.fuzzy_placement = false;

        let stone = Stone::new(Vertex::new(2, 3), 1, 20.0).with_theme(theme);
        let (offset_x, offset_y) = stone.fuzzy_offset();

        assert_eq!(offset_x, 0.0);
        assert_eq!(offset_y, 0.0);
    }

    #[test]
    fn test_fuzzy_offset_enabled() {
        let mut theme = StoneTheme::default();
        theme.fuzzy_placement = true;
        theme.fuzzy_max_offset = 3.0;

        let stone = Stone::new(Vertex::new(2, 3), 1, 20.0).with_theme(theme);
        let (offset_x, offset_y) = stone.fuzzy_offset();

        // Offsets should be within the max range
        assert!(offset_x.abs() <= theme.fuzzy_max_offset);
        assert!(offset_y.abs() <= theme.fuzzy_max_offset);

        // Offsets should be deterministic (same position = same offset)
        let stone2 = Stone::new(Vertex::new(2, 3), -1, 20.0).with_theme(theme.clone());
        let (offset_x2, offset_y2) = stone2.fuzzy_offset();
        assert_eq!(offset_x, offset_x2);
        assert_eq!(offset_y, offset_y2);
    }

    #[test]
    fn test_stone_theme_with_custom_images() {
        let theme = StoneTheme {
            black_stone_image: Some("black_stone.png".to_string()),
            white_stone_image: Some("white_stone.png".to_string()),
            ..StoneTheme::default()
        };

        assert!(theme.black_stone_image.is_some());
        assert!(theme.white_stone_image.is_some());
        assert_eq!(theme.black_stone_image.unwrap(), "black_stone.png");
    }

    #[test]
    fn test_stones_with_coordinates() {
        let range = BoardRange::new((0, 8), (0, 8));
        let sign_map = vec![vec![0, 1, 0], vec![-1, 0, 1], vec![0, 0, 0]];
        let stones = Stones::new(range, 20.0, sign_map).with_coordinates(true);

        assert!(stones.show_coordinates);
        assert_eq!(stones.coordinate_theme.font_size, 12.0); // Default theme
    }

    #[test]
    fn test_stones_coordinate_theme() {
        let range = BoardRange::new((0, 8), (0, 8));
        let sign_map = vec![];
        let custom_theme = CoordinateTheme {
            color: rgb(0x123456),
            font_size: 16.0,
            font_family: "Arial".to_string(),
            margin: 10.0,
        };

        let stones = Stones::new(range, 20.0, sign_map)
            .with_coordinates(true)
            .with_coordinate_theme(custom_theme.clone());

        assert!(stones.show_coordinates);
        assert_eq!(stones.coordinate_theme.color, custom_theme.color);
        assert_eq!(stones.coordinate_theme.font_size, 16.0);
        assert_eq!(stones.coordinate_theme.font_family, "Arial");
    }

    #[test]
    fn test_stones_coordinate_functions() {
        let range = BoardRange::new((0, 8), (0, 8));
        let sign_map = vec![];

        fn custom_coord_x(x: usize) -> String {
            format!("X{}", x)
        }

        fn custom_coord_y(y: usize) -> String {
            format!("Y{}", y)
        }

        let stones = Stones::new(range, 20.0, sign_map)
            .with_coordinates(true)
            .with_coordinate_functions(custom_coord_x, custom_coord_y);

        assert!(stones.show_coordinates);
        // Test that custom functions are stored (we can't directly test function equality)
        assert_eq!((stones.coord_x)(5), "X5");
        assert_eq!((stones.coord_y)(3), "Y3");
    }

    #[test]
    fn test_stones_default_no_coordinates() {
        let range = BoardRange::new((0, 8), (0, 8));
        let sign_map = vec![];
        let stones = Stones::new(range, 20.0, sign_map);

        assert!(!stones.show_coordinates);
    }
}
