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
    pub selected_left: Vec<Vertex>, // Directional selection indicators
    pub selected_right: Vec<Vertex>,
    pub selected_top: Vec<Vertex>,
    pub selected_bottom: Vec<Vertex>,
    pub lines: Vec<Line>,

    // Selection state tracking for efficient updates
    pub previous_selection_state: Option<SelectionStateSnapshot>,

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
            selected_left: Vec::new(),
            selected_right: Vec::new(),
            selected_top: Vec::new(),
            selected_bottom: Vec::new(),
            lines: Vec::new(),

            previous_selection_state: None,

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

    /// Gets the ghost stone at a vertex, returning None if out of bounds or no ghost stone
    pub fn get_ghost_stone(&self, vertex: &Vertex) -> Option<&GhostStone> {
        if self.is_valid_vertex(vertex) {
            self.ghost_stone_map[vertex.y][vertex.x].as_ref()
        } else {
            None
        }
    }

    /// Sets a ghost stone at a vertex if valid
    pub fn set_ghost_stone(&mut self, vertex: &Vertex, ghost_stone: Option<GhostStone>) -> bool {
        if self.is_valid_vertex(vertex) {
            self.ghost_stone_map[vertex.y][vertex.x] = ghost_stone;
            true
        } else {
            false
        }
    }

    /// Clears all ghost stones from the board
    pub fn clear_ghost_stones(&mut self) {
        for row in &mut self.ghost_stone_map {
            for cell in row {
                *cell = None;
            }
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
        self.selected_left.retain(|v| v.x < width && v.y < height);
        self.selected_right.retain(|v| v.x < width && v.y < height);
        self.selected_top.retain(|v| v.x < width && v.y < height);
        self.selected_bottom.retain(|v| v.x < width && v.y < height);
        self.animated_vertices
            .retain(|v| v.x < width && v.y < height);

        // Clear invalid lines
        self.lines.retain(|line| {
            line.v1.x < width && line.v1.y < height && line.v2.x < width && line.v2.y < height
        });
    }

    /// Creates a snapshot of current selection state for differential updates
    pub fn capture_selection_snapshot(&mut self) -> SelectionStateSnapshot {
        let snapshot = SelectionStateSnapshot::from_board_state(self);
        self.previous_selection_state = Some(snapshot.clone());
        snapshot
    }

    /// Checks if selection state has changed since last snapshot
    pub fn has_selection_changed(&self) -> bool {
        match &self.previous_selection_state {
            Some(previous) => {
                let current = SelectionStateSnapshot::from_board_state(self);
                current.differs_from(previous)
            }
            None => true, // No previous state means it's changed
        }
    }

    /// Updates multiple ghost stones efficiently with change tracking
    pub fn update_ghost_stones(&mut self, updates: &[(Vertex, Option<GhostStone>)]) -> bool {
        let mut changed = false;

        for (vertex, ghost_stone) in updates {
            if self.is_valid_vertex(vertex) {
                let current = &self.ghost_stone_map[vertex.y][vertex.x];
                let different = match (current, ghost_stone) {
                    (None, None) => false,
                    (Some(_), None) | (None, Some(_)) => true,
                    (Some(a), Some(b)) => a != b,
                };

                if different {
                    self.ghost_stone_map[vertex.y][vertex.x] = ghost_stone.clone();
                    changed = true;
                }
            }
        }

        changed
    }

    /// Bulk update ghost stones from a complete map with change tracking
    pub fn update_ghost_stone_map(&mut self, new_ghost_map: &GhostStoneMap) -> bool {
        if new_ghost_map.is_empty() || new_ghost_map[0].is_empty() {
            return false;
        }

        let height = new_ghost_map.len();
        let width = new_ghost_map[0].len();
        let (current_width, current_height) = self.dimensions();

        if width != current_width || height != current_height {
            return false; // Size mismatch
        }

        // Check if the new map differs from current
        let mut changed = false;
        for (y, row) in new_ghost_map.iter().enumerate() {
            for (x, new_ghost) in row.iter().enumerate() {
                let current_ghost = &self.ghost_stone_map[y][x];
                let different = match (current_ghost, new_ghost) {
                    (None, None) => false,
                    (Some(_), None) | (None, Some(_)) => true,
                    (Some(a), Some(b)) => a != b,
                };

                if different {
                    changed = true;
                    break;
                }
            }
            if changed {
                break;
            }
        }

        if changed {
            self.ghost_stone_map = new_ghost_map.clone();
        }

        changed
    }

    /// Updates multiple stones efficiently with change tracking
    pub fn update_stones(&mut self, updates: &[(Vertex, i8)]) -> bool {
        let mut changed = false;

        for (vertex, sign) in updates {
            if self.is_valid_vertex(vertex) && (-1..=1).contains(sign) {
                let current_sign = self.sign_map[vertex.y][vertex.x];
                if current_sign != *sign {
                    self.sign_map[vertex.y][vertex.x] = *sign;
                    changed = true;
                }
            }
        }

        changed
    }

    /// Bulk update the sign map from a complete map with change tracking
    pub fn update_sign_map(&mut self, new_sign_map: &SignMap) -> bool {
        if new_sign_map.is_empty() || new_sign_map[0].is_empty() {
            return false;
        }

        let height = new_sign_map.len();
        let width = new_sign_map[0].len();
        let (current_width, current_height) = self.dimensions();

        if width != current_width || height != current_height {
            return false; // Size mismatch
        }

        // Check if the new map differs from current
        let mut changed = false;
        for (y, row) in new_sign_map.iter().enumerate() {
            for (x, new_sign) in row.iter().enumerate() {
                if self.sign_map[y][x] != *new_sign {
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

    /// Gets a list of vertices that differ between current and new sign map
    pub fn get_sign_map_differences(&self, new_sign_map: &SignMap) -> Vec<Vertex> {
        let mut differences = Vec::new();

        if new_sign_map.is_empty() || new_sign_map[0].is_empty() {
            return differences;
        }

        let height = new_sign_map.len();
        let width = new_sign_map[0].len();
        let (current_width, current_height) = self.dimensions();

        if width != current_width || height != current_height {
            return differences; // Size mismatch
        }

        for (y, row) in new_sign_map.iter().enumerate() {
            for (x, new_sign) in row.iter().enumerate() {
                if self.sign_map[y][x] != *new_sign {
                    differences.push(Vertex::new(x, y));
                }
            }
        }

        differences
    }

    /// Updates selection state efficiently with change tracking
    pub fn update_selected_vertices(&mut self, vertices: Vec<Vertex>) -> bool {
        let changed = self.selected_vertices != vertices;
        if changed {
            self.selected_vertices = vertices
                .into_iter()
                .filter(|v| self.is_valid_vertex(v))
                .collect();
        }
        changed
    }

    /// Updates dimmed vertices efficiently with change tracking
    pub fn update_dimmed_vertices(&mut self, vertices: Vec<Vertex>) -> bool {
        let changed = self.dimmed_vertices != vertices;
        if changed {
            self.dimmed_vertices = vertices
                .into_iter()
                .filter(|v| self.is_valid_vertex(v))
                .collect();
        }
        changed
    }

    /// Updates directional selection efficiently with change tracking
    pub fn update_directional_selections(
        &mut self,
        selected_left: Option<Vec<Vertex>>,
        selected_right: Option<Vec<Vertex>>,
        selected_top: Option<Vec<Vertex>>,
        selected_bottom: Option<Vec<Vertex>>,
    ) -> bool {
        let mut changed = false;

        if let Some(vertices) = selected_left {
            let filtered: Vec<_> = vertices
                .into_iter()
                .filter(|v| self.is_valid_vertex(v))
                .collect();
            if self.selected_left != filtered {
                self.selected_left = filtered;
                changed = true;
            }
        }

        if let Some(vertices) = selected_right {
            let filtered: Vec<_> = vertices
                .into_iter()
                .filter(|v| self.is_valid_vertex(v))
                .collect();
            if self.selected_right != filtered {
                self.selected_right = filtered;
                changed = true;
            }
        }

        if let Some(vertices) = selected_top {
            let filtered: Vec<_> = vertices
                .into_iter()
                .filter(|v| self.is_valid_vertex(v))
                .collect();
            if self.selected_top != filtered {
                self.selected_top = filtered;
                changed = true;
            }
        }

        if let Some(vertices) = selected_bottom {
            let filtered: Vec<_> = vertices
                .into_iter()
                .filter(|v| self.is_valid_vertex(v))
                .collect();
            if self.selected_bottom != filtered {
                self.selected_bottom = filtered;
                changed = true;
            }
        }

        changed
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

    #[test]
    fn test_selection_state_tracking() {
        let mut state = GoBoardState::new(9, 9);

        // Initially no snapshot exists
        assert!(state.has_selection_changed());

        // Capture initial snapshot
        let snapshot = state.capture_selection_snapshot();
        assert!(!state.has_selection_changed()); // No change after capture

        // Make changes and test detection
        state.selected_vertices.push(Vertex::new(1, 1));
        assert!(state.has_selection_changed());

        // Capture new snapshot
        state.capture_selection_snapshot();
        assert!(!state.has_selection_changed());
    }

    #[test]
    fn test_efficient_selection_updates() {
        let mut state = GoBoardState::new(9, 9);

        // Test that unchanged updates return false
        let vertices = vec![Vertex::new(1, 1), Vertex::new(2, 2)];
        assert!(state.update_selected_vertices(vertices.clone())); // First time should change
        assert!(!state.update_selected_vertices(vertices)); // Second time should not change

        // Test that actual changes return true
        let new_vertices = vec![Vertex::new(3, 3)];
        assert!(state.update_selected_vertices(new_vertices));
    }

    #[test]
    fn test_directional_selection_updates() {
        let mut state = GoBoardState::new(9, 9);

        // Test updating multiple directional selections
        let changed = state.update_directional_selections(
            Some(vec![Vertex::new(1, 1)]),
            Some(vec![Vertex::new(2, 2)]),
            None,         // Don't update top
            Some(vec![]), // Clear bottom
        );

        assert!(changed);
        assert_eq!(state.selected_left.len(), 1);
        assert_eq!(state.selected_right.len(), 1);
        assert_eq!(state.selected_bottom.len(), 0);

        // Test that unchanged updates return false
        let changed = state.update_directional_selections(
            Some(vec![Vertex::new(1, 1)]), // Same as before
            Some(vec![Vertex::new(2, 2)]), // Same as before
            None,                          // Don't update
            None,                          // Don't update
        );

        assert!(!changed);
    }

    #[test]
    fn test_selection_snapshot_differs() {
        let state1 = GoBoardState::new(9, 9);
        let mut state2 = GoBoardState::new(9, 9);
        state2.selected_vertices.push(Vertex::new(1, 1));

        let snapshot1 = SelectionStateSnapshot::from_board_state(&state1);
        let snapshot2 = SelectionStateSnapshot::from_board_state(&state2);

        assert!(snapshot1.differs_from(&snapshot2));
        assert!(snapshot2.differs_from(&snapshot1));
        assert!(!snapshot1.differs_from(&snapshot1));
    }

    #[test]
    fn test_ghost_stone_operations() {
        let mut state = GoBoardState::new(9, 9);
        let vertex = Vertex::new(3, 3);
        let ghost_stone = GhostStone::new(1, GhostStoneType::Good);

        // Test initial state
        assert!(state.get_ghost_stone(&vertex).is_none());

        // Test setting ghost stone
        assert!(state.set_ghost_stone(&vertex, Some(ghost_stone.clone())));
        assert_eq!(state.get_ghost_stone(&vertex), Some(&ghost_stone));

        // Test clearing ghost stone
        assert!(state.set_ghost_stone(&vertex, None));
        assert!(state.get_ghost_stone(&vertex).is_none());

        // Test out of bounds
        let invalid_vertex = Vertex::new(10, 10);
        assert!(!state.set_ghost_stone(&invalid_vertex, Some(ghost_stone)));
        assert_eq!(state.get_ghost_stone(&invalid_vertex), None);
    }

    #[test]
    fn test_clear_ghost_stones() {
        let mut state = GoBoardState::new(9, 9);

        // Set some ghost stones
        state.set_ghost_stone(
            &Vertex::new(0, 0),
            Some(GhostStone::new(1, GhostStoneType::Good)),
        );
        state.set_ghost_stone(
            &Vertex::new(1, 1),
            Some(GhostStone::new(-1, GhostStoneType::Bad)),
        );

        // Clear all ghost stones
        state.clear_ghost_stones();
        assert!(state.get_ghost_stone(&Vertex::new(0, 0)).is_none());
        assert!(state.get_ghost_stone(&Vertex::new(1, 1)).is_none());
    }

    #[test]
    fn test_update_ghost_stones() {
        let mut state = GoBoardState::new(9, 9);

        // Test bulk updates
        let updates = vec![
            (
                Vertex::new(1, 1),
                Some(GhostStone::new(1, GhostStoneType::Good)),
            ),
            (
                Vertex::new(2, 2),
                Some(GhostStone::new(-1, GhostStoneType::Bad)),
            ),
            (Vertex::new(3, 3), None), // Should clear if exists
        ];

        // First update should return true (changes made)
        assert!(state.update_ghost_stones(&updates));

        // Verify the updates were applied
        assert!(state.get_ghost_stone(&Vertex::new(1, 1)).is_some());
        assert!(state.get_ghost_stone(&Vertex::new(2, 2)).is_some());
        assert!(state.get_ghost_stone(&Vertex::new(3, 3)).is_none());

        // Same updates should return false (no changes)
        assert!(!state.update_ghost_stones(&updates));
    }

    #[test]
    fn test_update_ghost_stone_map() {
        let mut state = GoBoardState::new(3, 3);

        // Create a test ghost stone map
        let ghost_map = vec![
            vec![Some(GhostStone::new(1, GhostStoneType::Good)), None, None],
            vec![None, Some(GhostStone::new(-1, GhostStoneType::Bad)), None],
            vec![
                None,
                None,
                Some(GhostStone::new(1, GhostStoneType::Interesting)),
            ],
        ];

        // First update should return true
        assert!(state.update_ghost_stone_map(&ghost_map));

        // Verify the map was updated
        assert!(state.get_ghost_stone(&Vertex::new(0, 0)).is_some());
        assert!(state.get_ghost_stone(&Vertex::new(1, 1)).is_some());
        assert!(state.get_ghost_stone(&Vertex::new(2, 2)).is_some());

        // Same map should return false
        assert!(!state.update_ghost_stone_map(&ghost_map));

        // Different map should return true
        let mut different_map = ghost_map.clone();
        different_map[0][0] = None;
        assert!(state.update_ghost_stone_map(&different_map));

        // Test size mismatch
        let wrong_size_map = vec![vec![None; 2]; 2];
        assert!(!state.update_ghost_stone_map(&wrong_size_map));
    }

    #[test]
    fn test_ghost_stone_resize() {
        let mut state = GoBoardState::new(3, 3);

        // Set some ghost stones
        state.set_ghost_stone(
            &Vertex::new(0, 0),
            Some(GhostStone::new(1, GhostStoneType::Good)),
        );
        state.set_ghost_stone(
            &Vertex::new(2, 2),
            Some(GhostStone::new(-1, GhostStoneType::Bad)),
        );

        // Resize to larger
        state.resize(5, 5);
        assert_eq!(state.dimensions(), (5, 5));
        assert!(state.get_ghost_stone(&Vertex::new(0, 0)).is_some()); // Preserved
        assert!(state.get_ghost_stone(&Vertex::new(2, 2)).is_some()); // Preserved

        // Resize to smaller
        state.resize(2, 2);
        assert_eq!(state.dimensions(), (2, 2));
        assert!(state.get_ghost_stone(&Vertex::new(0, 0)).is_some()); // Preserved
        assert!(state.get_ghost_stone(&Vertex::new(2, 2)).is_none()); // Lost due to resize
    }

    #[test]
    fn test_update_stones() {
        let mut state = GoBoardState::new(9, 9);

        // Test bulk stone updates
        let updates = vec![
            (Vertex::new(1, 1), 1),
            (Vertex::new(2, 2), -1),
            (Vertex::new(3, 3), 0), // Clear stone
        ];

        // First update should return true (changes made)
        assert!(state.update_stones(&updates));

        // Verify the updates were applied
        assert_eq!(state.get_sign(&Vertex::new(1, 1)), Some(1));
        assert_eq!(state.get_sign(&Vertex::new(2, 2)), Some(-1));
        assert_eq!(state.get_sign(&Vertex::new(3, 3)), Some(0));

        // Same updates should return false (no changes)
        assert!(!state.update_stones(&updates));

        // Test invalid sign values
        let invalid_updates = vec![
            (Vertex::new(4, 4), 2),   // Invalid sign
            (Vertex::new(10, 10), 1), // Out of bounds
        ];
        assert!(!state.update_stones(&invalid_updates));
        assert_eq!(state.get_sign(&Vertex::new(4, 4)), Some(0)); // Should remain unchanged
    }

    #[test]
    fn test_update_sign_map() {
        let mut state = GoBoardState::new(3, 3);

        // Create a test sign map
        let sign_map = vec![vec![1, 0, -1], vec![0, 1, 0], vec![-1, 0, 1]];

        // First update should return true
        assert!(state.update_sign_map(&sign_map));

        // Verify the map was updated
        assert_eq!(state.get_sign(&Vertex::new(0, 0)), Some(1));
        assert_eq!(state.get_sign(&Vertex::new(2, 0)), Some(-1));
        assert_eq!(state.get_sign(&Vertex::new(1, 1)), Some(1));

        // Same map should return false
        assert!(!state.update_sign_map(&sign_map));

        // Different map should return true
        let mut different_map = sign_map.clone();
        different_map[0][0] = 0;
        assert!(state.update_sign_map(&different_map));

        // Test size mismatch
        let wrong_size_map = vec![vec![0; 2]; 2];
        assert!(!state.update_sign_map(&wrong_size_map));

        // Test empty map
        let empty_map = vec![];
        assert!(!state.update_sign_map(&empty_map));
    }

    #[test]
    fn test_get_sign_map_differences() {
        let mut state = GoBoardState::new(3, 3);

        // Set initial state
        let initial_map = vec![vec![1, 0, 0], vec![0, -1, 0], vec![0, 0, 1]];
        state.sign_map = initial_map;

        // Create new map with changes
        let new_map = vec![
            vec![1, 1, 0],   // Changed at (1, 0)
            vec![0, -1, -1], // Changed at (2, 1)
            vec![-1, 0, 1],  // Changed at (0, 2)
        ];

        let differences = state.get_sign_map_differences(&new_map);
        assert_eq!(differences.len(), 3);
        assert!(differences.contains(&Vertex::new(1, 0)));
        assert!(differences.contains(&Vertex::new(2, 1)));
        assert!(differences.contains(&Vertex::new(0, 2)));

        // Test with identical maps
        let same_differences = state.get_sign_map_differences(&state.sign_map.clone());
        assert!(same_differences.is_empty());

        // Test size mismatch
        let wrong_size_map = vec![vec![0; 2]; 2];
        let size_mismatch_differences = state.get_sign_map_differences(&wrong_size_map);
        assert!(size_mismatch_differences.is_empty());
    }
}
