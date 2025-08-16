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
}

impl Default for StoneTheme {
    fn default() -> Self {
        Self {
            black_color: rgb(0x1a1a1a), // Dark gray/black
            white_color: rgb(0xf8f8f8), // Off-white
            stone_size_ratio: 0.9,      // 90% of vertex size
            border_width: 1.0,
            border_color: rgb(0x000000), // Black border
        }
    }
}

/// Individual stone component
pub struct Stone {
    position: Vertex,
    sign: i8, // -1: white, 0: empty, 1: black
    vertex_size: f32,
    theme: StoneTheme,
}

impl Stone {
    /// Creates a new stone
    pub fn new(position: Vertex, sign: i8, vertex_size: f32) -> Self {
        Self {
            position,
            sign,
            vertex_size,
            theme: StoneTheme::default(),
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
        (relative_x * self.vertex_size, relative_y * self.vertex_size)
    }

    /// Calculates the stone size in pixels
    pub fn stone_size(&self) -> f32 {
        self.vertex_size * self.theme.stone_size_ratio
    }

    /// Renders the stone as a circle
    pub fn render(&self, board_range: &BoardRange) -> Option<impl IntoElement> {
        if self.sign == 0 {
            return None; // Empty vertex, no stone to render
        }

        let (pixel_x, pixel_y) = self.pixel_position(board_range);
        let stone_size = self.stone_size();
        let radius = stone_size / 2.0;

        let color = if self.sign == 1 {
            self.theme.black_color
        } else {
            self.theme.white_color
        };

        Some(
            div()
                .absolute()
                .left(px(pixel_x - radius))
                .top(px(pixel_y - radius))
                .w(px(stone_size))
                .h(px(stone_size))
                .rounded_full()
                .bg(color)
                .when(self.theme.border_width > 0.0, |div| {
                    div.border_1().border_color(self.theme.border_color)
                }),
        )
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
        let width = (self.board_range.x.1 - self.board_range.x.0) as f32 * self.vertex_size;
        let height = (self.board_range.y.1 - self.board_range.y.0) as f32 * self.vertex_size;
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
        assert_eq!(width, 120.0); // (6-2) * 30 = 4 * 30
        assert_eq!(height, 90.0); // (4-1) * 30 = 3 * 30
    }
}
