use crate::go_board::types::*;
use std::collections::HashSet;

/// Differential rendering system for efficient updates
/// Tracks changes between renders to minimize DOM manipulation
#[derive(Clone, Debug, Default)]
pub struct DifferentialRenderer {
    /// Previous state snapshots for change detection
    previous_sign_map: Option<SignMap>,
    previous_marker_map: Option<MarkerMap>,
    previous_ghost_stone_map: Option<GhostStoneMap>,
    previous_selection_state: Option<SelectionStateSnapshot>,

    /// Cached change sets from last update
    changed_stone_vertices: HashSet<Vertex>,
    changed_marker_vertices: HashSet<Vertex>,
    changed_ghost_stone_vertices: HashSet<Vertex>,
    selection_changed: bool,
}

/// Result of differential analysis containing only changed elements
#[derive(Clone, Debug, Default)]
pub struct DifferentialUpdate {
    pub changed_stones: Vec<Vertex>,
    pub changed_markers: Vec<Vertex>,
    pub changed_ghost_stones: Vec<Vertex>,
    pub selection_changed: bool,
    pub requires_full_render: bool,
}

impl DifferentialRenderer {
    /// Creates a new differential renderer
    pub fn new() -> Self {
        Self::default()
    }

    /// Analyzes changes between current and previous state
    pub fn analyze_changes(
        &mut self,
        current_sign_map: &SignMap,
        current_marker_map: &MarkerMap,
        current_ghost_stone_map: &GhostStoneMap,
        current_selection_state: &SelectionStateSnapshot,
    ) -> DifferentialUpdate {
        let mut update = DifferentialUpdate::default();

        // Analyze stone changes
        if let Some(ref prev_sign_map) = self.previous_sign_map {
            update.changed_stones = self.get_sign_map_differences(prev_sign_map, current_sign_map);
        } else {
            update.requires_full_render = true;
            update.changed_stones = self.get_all_non_empty_vertices(current_sign_map);
        }

        // Analyze marker changes
        if let Some(ref prev_marker_map) = self.previous_marker_map {
            update.changed_markers =
                self.get_marker_map_differences(prev_marker_map, current_marker_map);
        } else {
            update.requires_full_render = true;
            update.changed_markers = self.get_all_marker_vertices(current_marker_map);
        }

        // Analyze ghost stone changes
        if let Some(ref prev_ghost_map) = self.previous_ghost_stone_map {
            update.changed_ghost_stones =
                self.get_ghost_stone_map_differences(prev_ghost_map, current_ghost_stone_map);
        } else {
            update.requires_full_render = true;
            update.changed_ghost_stones =
                self.get_all_ghost_stone_vertices(current_ghost_stone_map);
        }

        // Analyze selection changes
        if let Some(ref prev_selection) = self.previous_selection_state {
            update.selection_changed = current_selection_state.differs_from(prev_selection);
        } else {
            update.selection_changed = true;
        }

        // Update cached state
        self.previous_sign_map = Some(current_sign_map.clone());
        self.previous_marker_map = Some(current_marker_map.clone());
        self.previous_ghost_stone_map = Some(current_ghost_stone_map.clone());
        self.previous_selection_state = Some(current_selection_state.clone());

        // Cache change sets for performance
        self.changed_stone_vertices = update.changed_stones.iter().cloned().collect();
        self.changed_marker_vertices = update.changed_markers.iter().cloned().collect();
        self.changed_ghost_stone_vertices = update.changed_ghost_stones.iter().cloned().collect();
        self.selection_changed = update.selection_changed;

        update
    }

    /// Forces a full render on next update (useful after theme changes)
    pub fn invalidate_cache(&mut self) {
        self.previous_sign_map = None;
        self.previous_marker_map = None;
        self.previous_ghost_stone_map = None;
        self.previous_selection_state = None;
        self.changed_stone_vertices.clear();
        self.changed_marker_vertices.clear();
        self.changed_ghost_stone_vertices.clear();
        self.selection_changed = false;
    }

    /// Gets vertices where signs differ between two maps
    fn get_sign_map_differences(&self, old_map: &SignMap, new_map: &SignMap) -> Vec<Vertex> {
        let mut differences = Vec::new();

        // Check dimensions match
        if old_map.len() != new_map.len()
            || (old_map.len() > 0 && new_map.len() > 0 && old_map[0].len() != new_map[0].len())
        {
            return self.get_all_non_empty_vertices(new_map);
        }

        for (y, (old_row, new_row)) in old_map.iter().zip(new_map.iter()).enumerate() {
            for (x, (old_sign, new_sign)) in old_row.iter().zip(new_row.iter()).enumerate() {
                if old_sign != new_sign {
                    differences.push(Vertex::new(x, y));
                }
            }
        }

        differences
    }

    /// Gets vertices where markers differ between two maps
    fn get_marker_map_differences(&self, old_map: &MarkerMap, new_map: &MarkerMap) -> Vec<Vertex> {
        let mut differences = Vec::new();

        // Check dimensions match
        if old_map.len() != new_map.len()
            || (old_map.len() > 0 && new_map.len() > 0 && old_map[0].len() != new_map[0].len())
        {
            return self.get_all_marker_vertices(new_map);
        }

        for (y, (old_row, new_row)) in old_map.iter().zip(new_map.iter()).enumerate() {
            for (x, (old_marker, new_marker)) in old_row.iter().zip(new_row.iter()).enumerate() {
                let different = match (old_marker, new_marker) {
                    (None, None) => false,
                    (Some(_), None) | (None, Some(_)) => true,
                    (Some(a), Some(b)) => a != b,
                };

                if different {
                    differences.push(Vertex::new(x, y));
                }
            }
        }

        differences
    }

    /// Gets vertices where ghost stones differ between two maps
    fn get_ghost_stone_map_differences(
        &self,
        old_map: &GhostStoneMap,
        new_map: &GhostStoneMap,
    ) -> Vec<Vertex> {
        let mut differences = Vec::new();

        // Check dimensions match
        if old_map.len() != new_map.len()
            || (old_map.len() > 0 && new_map.len() > 0 && old_map[0].len() != new_map[0].len())
        {
            return self.get_all_ghost_stone_vertices(new_map);
        }

        for (y, (old_row, new_row)) in old_map.iter().zip(new_map.iter()).enumerate() {
            for (x, (old_ghost, new_ghost)) in old_row.iter().zip(new_row.iter()).enumerate() {
                let different = match (old_ghost, new_ghost) {
                    (None, None) => false,
                    (Some(_), None) | (None, Some(_)) => true,
                    (Some(a), Some(b)) => a != b,
                };

                if different {
                    differences.push(Vertex::new(x, y));
                }
            }
        }

        differences
    }

    /// Gets all vertices with non-empty signs
    fn get_all_non_empty_vertices(&self, sign_map: &SignMap) -> Vec<Vertex> {
        let mut vertices = Vec::new();

        for (y, row) in sign_map.iter().enumerate() {
            for (x, sign) in row.iter().enumerate() {
                if *sign != 0 {
                    vertices.push(Vertex::new(x, y));
                }
            }
        }

        vertices
    }

    /// Gets all vertices with markers
    fn get_all_marker_vertices(&self, marker_map: &MarkerMap) -> Vec<Vertex> {
        let mut vertices = Vec::new();

        for (y, row) in marker_map.iter().enumerate() {
            for (x, marker) in row.iter().enumerate() {
                if marker.is_some() {
                    vertices.push(Vertex::new(x, y));
                }
            }
        }

        vertices
    }

    /// Gets all vertices with ghost stones
    fn get_all_ghost_stone_vertices(&self, ghost_map: &GhostStoneMap) -> Vec<Vertex> {
        let mut vertices = Vec::new();

        for (y, row) in ghost_map.iter().enumerate() {
            for (x, ghost) in row.iter().enumerate() {
                if ghost.is_some() {
                    vertices.push(Vertex::new(x, y));
                }
            }
        }

        vertices
    }

    /// Checks if a vertex has changed in the last update
    pub fn vertex_changed(&self, vertex: &Vertex) -> bool {
        self.changed_stone_vertices.contains(vertex)
            || self.changed_marker_vertices.contains(vertex)
            || self.changed_ghost_stone_vertices.contains(vertex)
    }

    /// Gets statistics about the last update
    pub fn get_update_stats(&self) -> UpdateStats {
        UpdateStats {
            changed_stones: self.changed_stone_vertices.len(),
            changed_markers: self.changed_marker_vertices.len(),
            changed_ghost_stones: self.changed_ghost_stone_vertices.len(),
            selection_changed: self.selection_changed,
        }
    }
}

/// Statistics about a differential update
#[derive(Clone, Debug)]
pub struct UpdateStats {
    pub changed_stones: usize,
    pub changed_markers: usize,
    pub changed_ghost_stones: usize,
    pub selection_changed: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_differential_renderer_creation() {
        let renderer = DifferentialRenderer::new();
        assert!(renderer.previous_sign_map.is_none());
        assert!(renderer.previous_marker_map.is_none());
        assert!(renderer.changed_stone_vertices.is_empty());
    }

    #[test]
    fn test_sign_map_differences() {
        let renderer = DifferentialRenderer::new();

        let old_map = vec![vec![0, 1, 0], vec![-1, 0, 1], vec![0, 0, 0]];

        let new_map = vec![
            vec![0, 1, -1], // Changed at (2, 0)
            vec![-1, 1, 1], // Changed at (1, 1)
            vec![0, 0, 0],  // No changes
        ];

        let differences = renderer.get_sign_map_differences(&old_map, &new_map);
        assert_eq!(differences.len(), 2);
        assert!(differences.contains(&Vertex::new(2, 0)));
        assert!(differences.contains(&Vertex::new(1, 1)));
    }

    #[test]
    fn test_full_render_required() {
        let mut renderer = DifferentialRenderer::new();

        let sign_map = vec![vec![1, 0, -1]];
        let marker_map = vec![vec![None, None, None]];
        let ghost_map = vec![vec![None, None, None]];
        let selection_state =
            SelectionStateSnapshot::new(vec![], vec![], vec![], vec![], vec![], vec![]);

        let update = renderer.analyze_changes(&sign_map, &marker_map, &ghost_map, &selection_state);

        // First analysis should require full render
        assert!(update.requires_full_render);
        assert_eq!(update.changed_stones.len(), 2); // Two non-empty stones
    }

    #[test]
    fn test_incremental_updates() {
        let mut renderer = DifferentialRenderer::new();

        let initial_sign_map = vec![vec![1, 0, -1]];
        let marker_map = vec![vec![None, None, None]];
        let ghost_map = vec![vec![None, None, None]];
        let selection_state =
            SelectionStateSnapshot::new(vec![], vec![], vec![], vec![], vec![], vec![]);

        // First update
        renderer.analyze_changes(&initial_sign_map, &marker_map, &ghost_map, &selection_state);

        // Second update with changes
        let updated_sign_map = vec![vec![1, 1, -1]]; // Added stone at (1, 0)
        let update =
            renderer.analyze_changes(&updated_sign_map, &marker_map, &ghost_map, &selection_state);

        assert!(!update.requires_full_render);
        assert_eq!(update.changed_stones.len(), 1);
        assert!(update.changed_stones.contains(&Vertex::new(1, 0)));
    }

    #[test]
    fn test_cache_invalidation() {
        let mut renderer = DifferentialRenderer::new();

        let sign_map = vec![vec![1, 0, -1]];
        let marker_map = vec![vec![None, None, None]];
        let ghost_map = vec![vec![None, None, None]];
        let selection_state =
            SelectionStateSnapshot::new(vec![], vec![], vec![], vec![], vec![], vec![]);

        // Populate cache
        renderer.analyze_changes(&sign_map, &marker_map, &ghost_map, &selection_state);
        assert!(renderer.previous_sign_map.is_some());

        // Invalidate cache
        renderer.invalidate_cache();
        assert!(renderer.previous_sign_map.is_none());
        assert!(renderer.changed_stone_vertices.is_empty());
    }

    #[test]
    fn test_marker_differences() {
        let renderer = DifferentialRenderer::new();

        let old_markers = vec![
            vec![Some(Marker::new(MarkerType::Circle)), None, None],
            vec![None, None, Some(Marker::new(MarkerType::Cross))],
        ];

        let new_markers = vec![
            vec![
                Some(Marker::new(MarkerType::Circle)),
                Some(Marker::new(MarkerType::Square)),
                None,
            ],
            vec![None, None, None], // Removed cross marker
        ];

        let differences = renderer.get_marker_map_differences(&old_markers, &new_markers);
        assert_eq!(differences.len(), 2);
        assert!(differences.contains(&Vertex::new(1, 0))); // Added square
        assert!(differences.contains(&Vertex::new(2, 1))); // Removed cross
    }

    #[test]
    fn test_update_stats() {
        let mut renderer = DifferentialRenderer::new();

        // Populate with some changes
        renderer.changed_stone_vertices.insert(Vertex::new(0, 0));
        renderer.changed_stone_vertices.insert(Vertex::new(1, 1));
        renderer.changed_marker_vertices.insert(Vertex::new(2, 2));
        renderer.selection_changed = true;

        let stats = renderer.get_update_stats();
        assert_eq!(stats.changed_stones, 2);
        assert_eq!(stats.changed_markers, 1);
        assert_eq!(stats.changed_ghost_stones, 0);
        assert!(stats.selection_changed);
    }

    #[test]
    fn test_vertex_changed() {
        let mut renderer = DifferentialRenderer::new();

        renderer.changed_stone_vertices.insert(Vertex::new(1, 1));
        renderer.changed_marker_vertices.insert(Vertex::new(2, 2));

        assert!(renderer.vertex_changed(&Vertex::new(1, 1)));
        assert!(renderer.vertex_changed(&Vertex::new(2, 2)));
        assert!(!renderer.vertex_changed(&Vertex::new(3, 3)));
    }
}
