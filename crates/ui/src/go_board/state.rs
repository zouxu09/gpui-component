use crate::go_board::types::*;
use std::time::Duration;

/// Central state management for the Go board widget
/// Follows Shudan's reactive architecture with efficient state updates
#[derive(Clone, Debug)]
pub struct GoBoardState {
    // Core board state - following Shudan's map-based approach
    pub sign_map: SignMap,
    pub marker_map: MarkerMap,
    pub ghost_stone_map: GhostStoneMap,
    pub heat_map: HeatMap,
    pub paint_map: PaintMap,

    // Visual state
    pub selected_vertices: Vec<Vertex>,
    pub dimmed_vertices: Vec<Vertex>,
    pub lines: Vec<Line>,

    // Animation state
    pub animated_vertices: Vec<Vertex>,
    pub animation_duration: Duration,

    // Configuration
    pub vertex_size: f32,
    pub board_range: BoardRange,
    pub show_coordinates: bool,
    pub fuzzy_stone_placement: bool,
    pub animate_stone_placement: bool,
    pub busy: bool,
}

impl GoBoardState {
    /// Creates a new board state with the specified dimensions
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            sign_map: vec![vec![0; width]; height],
            marker_map: vec![vec![None; width]; height],
            ghost_stone_map: vec![vec![None; width]; height],
            heat_map: vec![vec![None; width]; height],
            paint_map: vec![vec![0.0; width]; height],

            selected_vertices: Vec::new(),
            dimmed_vertices: Vec::new(),
            lines: Vec::new(),

            animated_vertices: Vec::new(),
            animation_duration: Duration::from_millis(200),

            vertex_size: 24.0,
            board_range: BoardRange::full(width, height),
            show_coordinates: false,
            fuzzy_stone_placement: false,
            animate_stone_placement: false,
            busy: false,
        }
    }

    /// Creates a standard 19x19 Go board
    pub fn standard() -> Self {
        Self::new(19, 19)
    }

    /// Returns the board dimensions (width, height)
    pub fn dimensions(&self) -> (usize, usize) {
        if let Some(first_row) = self.sign_map.first() {
            (first_row.len(), self.sign_map.len())
        } else {
            (0, 0)
        }
    }

    /// Returns the board width
    pub fn width(&self) -> usize {
        self.dimensions().0
    }

    /// Returns the board height
    pub fn height(&self) -> usize {
        self.dimensions().1
    }

    /// Checks if a vertex is within the board bounds
    pub fn is_valid_vertex(&self, vertex: &Vertex) -> bool {
        let (width, height) = self.dimensions();
        vertex.x < width && vertex.y < height
    }

    /// Gets the sign (stone) at a vertex, returning None if out of bounds
    pub fn get_sign(&self, vertex: &Vertex) -> Option<i8> {
        if self.is_valid_vertex(vertex) {
            Some(self.sign_map[vertex.y][vertex.x])
        } else {
            None
        }
    }

    /// Sets the sign (stone) at a vertex if valid
    pub fn set_sign(&mut self, vertex: &Vertex, sign: i8) -> bool {
        if self.is_valid_vertex(vertex) && (-1..=1).contains(&sign) {
            self.sign_map[vertex.y][vertex.x] = sign;
            true
        } else {
            false
        }
    }

    /// Gets the marker at a vertex, returning None if out of bounds or no marker
    pub fn get_marker(&self, vertex: &Vertex) -> Option<&Marker> {
        if self.is_valid_vertex(vertex) {
            self.marker_map[vertex.y][vertex.x].as_ref()
        } else {
            None
        }
    }

    /// Sets a marker at a vertex if valid
    pub fn set_marker(&mut self, vertex: &Vertex, marker: Option<Marker>) -> bool {
        if self.is_valid_vertex(vertex) {
            self.marker_map[vertex.y][vertex.x] = marker;
            true
        } else {
            false
        }
    }

    /// Clears all stones from the board
    pub fn clear_stones(&mut self) {
        for row in &mut self.sign_map {
            for cell in row {
                *cell = 0;
            }
        }
    }

    /// Clears all markers from the board
    pub fn clear_markers(&mut self) {
        for row in &mut self.marker_map {
            for cell in row {
                *cell = None;
            }
        }
    }

    /// Resizes the board to new dimensions, preserving existing data where possible
    pub fn resize(&mut self, new_width: usize, new_height: usize) {
        // Resize sign_map
        self.sign_map.resize(new_height, vec![0; new_width]);
        for row in &mut self.sign_map {
            row.resize(new_width, 0);
        }

        // Resize marker_map
        self.marker_map.resize(new_height, vec![None; new_width]);
        for row in &mut self.marker_map {
            row.resize(new_width, None);
        }

        // Resize ghost_stone_map
        self.ghost_stone_map
            .resize(new_height, vec![None; new_width]);
        for row in &mut self.ghost_stone_map {
            row.resize(new_width, None);
        }

        // Resize heat_map
        self.heat_map.resize(new_height, vec![None; new_width]);
        for row in &mut self.heat_map {
            row.resize(new_width, None);
        }

        // Resize paint_map
        self.paint_map.resize(new_height, vec![0.0; new_width]);
        for row in &mut self.paint_map {
            row.resize(new_width, 0.0);
        }

        // Update board range
        self.board_range = BoardRange::full(new_width, new_height);

        // Clear invalid vertices from selection lists
        let (width, height) = (new_width, new_height);
        self.selected_vertices
            .retain(|v| v.x < width && v.y < height);
        self.dimmed_vertices.retain(|v| v.x < width && v.y < height);
        self.animated_vertices
            .retain(|v| v.x < width && v.y < height);

        // Clear invalid lines
        self.lines.retain(|line| {
            line.v1.x < width && line.v1.y < height && line.v2.x < width && line.v2.y < height
        });
    }
}

impl Default for GoBoardState {
    fn default() -> Self {
        Self::standard()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_creation() {
        let state = GoBoardState::new(9, 9);
        assert_eq!(state.dimensions(), (9, 9));
        assert_eq!(state.vertex_size, 24.0);
        assert!(!state.show_coordinates);
        assert!(!state.busy);
    }

    #[test]
    fn test_standard_board() {
        let state = GoBoardState::standard();
        assert_eq!(state.dimensions(), (19, 19));
    }

    #[test]
    fn test_vertex_validation() {
        let state = GoBoardState::new(19, 19);
        assert!(state.is_valid_vertex(&Vertex::new(0, 0)));
        assert!(state.is_valid_vertex(&Vertex::new(18, 18)));
        assert!(!state.is_valid_vertex(&Vertex::new(19, 18)));
        assert!(!state.is_valid_vertex(&Vertex::new(18, 19)));
    }

    #[test]
    fn test_sign_operations() {
        let mut state = GoBoardState::new(9, 9);
        let vertex = Vertex::new(4, 4);

        // Test initial state
        assert_eq!(state.get_sign(&vertex), Some(0));

        // Test setting valid signs
        assert!(state.set_sign(&vertex, 1));
        assert_eq!(state.get_sign(&vertex), Some(1));

        assert!(state.set_sign(&vertex, -1));
        assert_eq!(state.get_sign(&vertex), Some(-1));

        // Test invalid sign
        assert!(!state.set_sign(&vertex, 2));
        assert_eq!(state.get_sign(&vertex), Some(-1)); // Should remain unchanged

        // Test out of bounds
        let invalid_vertex = Vertex::new(10, 10);
        assert!(!state.set_sign(&invalid_vertex, 1));
        assert_eq!(state.get_sign(&invalid_vertex), None);
    }

    #[test]
    fn test_marker_operations() {
        let mut state = GoBoardState::new(9, 9);
        let vertex = Vertex::new(3, 3);
        let marker = Marker::new(MarkerType::Circle);

        // Test initial state
        assert!(state.get_marker(&vertex).is_none());

        // Test setting marker
        assert!(state.set_marker(&vertex, Some(marker.clone())));
        assert_eq!(state.get_marker(&vertex), Some(&marker));

        // Test clearing marker
        assert!(state.set_marker(&vertex, None));
        assert!(state.get_marker(&vertex).is_none());
    }

    #[test]
    fn test_clear_operations() {
        let mut state = GoBoardState::new(9, 9);

        // Set some stones and markers
        state.set_sign(&Vertex::new(0, 0), 1);
        state.set_sign(&Vertex::new(1, 1), -1);
        state.set_marker(&Vertex::new(2, 2), Some(Marker::new(MarkerType::Circle)));

        // Clear stones
        state.clear_stones();
        assert_eq!(state.get_sign(&Vertex::new(0, 0)), Some(0));
        assert_eq!(state.get_sign(&Vertex::new(1, 1)), Some(0));
        assert!(state.get_marker(&Vertex::new(2, 2)).is_some()); // Marker should remain

        // Clear markers
        state.clear_markers();
        assert!(state.get_marker(&Vertex::new(2, 2)).is_none());
    }

    #[test]
    fn test_resize() {
        let mut state = GoBoardState::new(9, 9);

        // Set some data
        state.set_sign(&Vertex::new(4, 4), 1);
        state.set_marker(&Vertex::new(3, 3), Some(Marker::new(MarkerType::Circle)));
        state.selected_vertices.push(Vertex::new(8, 8));

        // Resize to larger
        state.resize(13, 13);
        assert_eq!(state.dimensions(), (13, 13));
        assert_eq!(state.get_sign(&Vertex::new(4, 4)), Some(1)); // Data preserved
        assert!(state.get_marker(&Vertex::new(3, 3)).is_some()); // Data preserved
        assert!(state.selected_vertices.contains(&Vertex::new(8, 8))); // Selection preserved

        // Resize to smaller
        state.selected_vertices.push(Vertex::new(12, 12)); // Add vertex that will become invalid
        state.resize(7, 7);
        assert_eq!(state.dimensions(), (7, 7));
        assert_eq!(state.get_sign(&Vertex::new(4, 4)), Some(1)); // Data preserved
        assert!(!state.selected_vertices.contains(&Vertex::new(8, 8))); // Invalid vertex removed
        assert!(!state.selected_vertices.contains(&Vertex::new(12, 12))); // Invalid vertex removed
    }
}
