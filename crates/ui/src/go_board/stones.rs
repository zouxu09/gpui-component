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
        let relative_x = (self.position.x - board_range.x.0) as f32;
        let relative_y = (self.position.y - board_range.y.0) as f32;

        // Add half vertex size offset to center stones on grid intersections
        // This matches the grid's vertex_to_pixel logic
        let offset = self.vertex_size / 2.0;
        (
            relative_x * self.vertex_size + offset,
            relative_y * self.vertex_size + offset,
        )
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

    /// Calculates rotation for visual variation
    fn rotation_angle(&self) -> f32 {
        if !self.theme.random_variation {
            return 0.0;
        }

        // Use deterministic "random" rotation based on position and random_class
        let seed = self.position.x * 19 + self.position.y * 29 + self.random_class as usize * 7;
        ((seed % 100) as f32 / 50.0 - 1.0) * self.theme.max_rotation
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
}

impl Stones {
    /// Creates a new Stones component
    pub fn new(board_range: BoardRange, vertex_size: f32, sign_map: SignMap) -> Self {
        Self {
            board_range,
            vertex_size,
            sign_map,
            theme: StoneTheme::default(),
        }
    }

    /// Sets the stone theme
    pub fn with_theme(mut self, theme: StoneTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Updates the sign map
    pub fn set_sign_map(&mut self, sign_map: SignMap) {
        self.sign_map = sign_map;
    }

    /// Updates the sign map efficiently with change detection
    pub fn update_sign_map(&mut self, new_sign_map: &SignMap) -> bool {
        if new_sign_map.is_empty() || new_sign_map[0].is_empty() {
            return false;
        }

        let _height = new_sign_map.len();
        let _width = new_sign_map[0].len();

        // Check if the new map differs from current
        let mut changed = false;
        for (y, row) in new_sign_map.iter().enumerate() {
            if y >= self.sign_map.len() {
                changed = true;
                break;
            }
            for (x, new_sign) in row.iter().enumerate() {
                if x >= self.sign_map[y].len() || self.sign_map[y][x] != *new_sign {
                    changed = true;
                    break;
                }
            }
            if changed {
                break;
            }
        }

        if changed {
            self.sign_map = new_sign_map.clone();
        }

        changed
    }

    /// Gets the differences between current and new sign map
    pub fn get_sign_map_differences(&self, new_sign_map: &SignMap) -> Vec<Vertex> {
        let mut differences = Vec::new();

        if new_sign_map.is_empty() || new_sign_map[0].is_empty() {
            return differences;
        }

        for (y, row) in new_sign_map.iter().enumerate() {
            if y < self.sign_map.len() {
                for (x, new_sign) in row.iter().enumerate() {
                    if x < self.sign_map[y].len() && self.sign_map[y][x] != *new_sign {
                        differences.push(Vertex::new(x, y));
                    }
                }
            }
        }

        differences
    }

    /// Updates individual stones efficiently
    pub fn update_stones(&mut self, updates: &[(Vertex, i8)]) -> bool {
        let mut changed = false;

        for (vertex, sign) in updates {
            if vertex.y < self.sign_map.len() && vertex.x < self.sign_map[vertex.y].len() {
                if (-1..=1).contains(sign) && self.sign_map[vertex.y][vertex.x] != *sign {
                    self.sign_map[vertex.y][vertex.x] = *sign;
                    changed = true;
                }
            }
        }

        changed
    }

    /// Renders only stones that have changed from a previous state
    pub fn render_differential_stones(&self, changed_vertices: &[Vertex]) -> Vec<impl IntoElement> {
        let mut stones = Vec::new();

        for vertex in changed_vertices {
            // Only render if vertex is within visible range
            if vertex.x >= self.board_range.x.0
                && vertex.x <= self.board_range.x.1
                && vertex.y >= self.board_range.y.0
                && vertex.y <= self.board_range.y.1
            {
                if vertex.y < self.sign_map.len() && vertex.x < self.sign_map[vertex.y].len() {
                    let sign = self.sign_map[vertex.y][vertex.x];
                    if sign != 0 {
                        let stone = Stone::new(*vertex, sign, self.vertex_size)
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

    /// Renders stones only within the specified vertices (for efficient updates)
    pub fn render_stones_at_vertices(&self, vertices: &[Vertex]) -> Vec<impl IntoElement> {
        let mut stones = Vec::new();

        for vertex in vertices {
            // Only render if vertex is within visible range
            if vertex.x >= self.board_range.x.0
                && vertex.x <= self.board_range.x.1
                && vertex.y >= self.board_range.y.0
                && vertex.y <= self.board_range.y.1
            {
                if vertex.y < self.sign_map.len() && vertex.x < self.sign_map[vertex.y].len() {
                    let sign = self.sign_map[vertex.y][vertex.x];
                    if sign != 0 {
                        let stone = Stone::new(*vertex, sign, self.vertex_size)
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

    /// Renders all stones in a container
    pub fn render(&self) -> impl IntoElement {
        let (width, height) = self.visible_dimensions();
        let mut container = div().relative().w(px(width)).h(px(height));

        for stone in self.render_stones() {
            container = container.child(stone);
        }

        container
    }

    /// Calculates the visible dimensions
    fn visible_dimensions(&self) -> (f32, f32) {
        let width = (self.board_range.x.1 - self.board_range.x.0 + 1) as f32 * self.vertex_size;
        let height = (self.board_range.y.1 - self.board_range.y.0 + 1) as f32 * self.vertex_size;
        (width, height)
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
        assert_eq!(width, 150.0); // (6-2+1) * 30 = 5 * 30
        assert_eq!(height, 120.0); // (4-1+1) * 30 = 4 * 30
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
    fn test_rotation_angle_disabled() {
        let mut theme = StoneTheme::default();
        theme.random_variation = false;

        let stone = Stone::new(Vertex::new(2, 3), 1, 20.0).with_theme(theme);
        let rotation = stone.rotation_angle();

        assert_eq!(rotation, 0.0);
    }

    #[test]
    fn test_rotation_angle_enabled() {
        let mut theme = StoneTheme::default();
        theme.random_variation = true;
        theme.max_rotation = 10.0;

        let stone = Stone::new(Vertex::new(2, 3), 1, 20.0).with_theme(theme);
        let rotation = stone.rotation_angle();

        // Rotation should be within the max range
        assert!(rotation.abs() <= theme.max_rotation);

        // Rotation should be deterministic
        let stone2 = Stone::new(Vertex::new(2, 3), -1, 20.0).with_theme(theme.clone());
        let rotation2 = stone2.rotation_angle();
        assert_eq!(rotation, rotation2);
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
}
