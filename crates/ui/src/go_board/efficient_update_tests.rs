use crate::go_board::types::*;
use crate::go_board::*;

#[cfg(test)]
mod differential_rendering_tests {
    use super::*;

    #[test]
    fn test_go_board_efficient_sign_map_updates() {
        let mut board = GoBoard::new();

        // Create test sign map
        let sign_map = vec![vec![1, 0, -1], vec![0, 1, 0], vec![-1, 0, 1]];

        // First update should return true (changes made)
        assert!(board.update_sign_map(sign_map.clone()));

        // Verify stones were placed
        assert_eq!(board.get_stone(&Vertex::new(0, 0)), Some(1));
        assert_eq!(board.get_stone(&Vertex::new(2, 0)), Some(-1));
        assert_eq!(board.get_stone(&Vertex::new(1, 1)), Some(1));

        // Same update should return false (no changes)
        assert!(!board.update_sign_map(sign_map));
    }

    #[test]
    fn test_go_board_individual_stone_updates() {
        let mut board = GoBoard::new();

        // Test bulk stone updates
        let updates = vec![
            (Vertex::new(1, 1), 1),
            (Vertex::new(2, 2), -1),
            (Vertex::new(3, 3), 0),
        ];

        assert!(board.update_stones(&updates));

        // Verify individual stones
        assert_eq!(board.get_stone(&Vertex::new(1, 1)), Some(1));
        assert_eq!(board.get_stone(&Vertex::new(2, 2)), Some(-1));
        assert_eq!(board.get_stone(&Vertex::new(3, 3)), Some(0));

        // Same updates should return false
        assert!(!board.update_stones(&updates));
    }

    #[test]
    fn test_go_board_single_stone_operations() {
        let mut board = GoBoard::new();
        let vertex = Vertex::new(5, 5);

        // Test setting stones
        assert!(board.set_stone(&vertex, 1));
        assert_eq!(board.get_stone(&vertex), Some(1));

        // Test changing stone
        assert!(board.set_stone(&vertex, -1));
        assert_eq!(board.get_stone(&vertex), Some(-1));

        // Test clearing stone
        assert!(board.set_stone(&vertex, 0));
        assert_eq!(board.get_stone(&vertex), Some(0));

        // Test invalid sign
        assert!(!board.set_stone(&vertex, 2));
        assert_eq!(board.get_stone(&vertex), Some(0)); // Should remain unchanged
    }

    #[test]
    fn test_go_board_sign_map_differences() {
        let board = GoBoard::new();

        // Create a sign map with differences
        let new_sign_map = vec![
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1],
            vec![0; 19],
            vec![0; 19],
            vec![0; 19],
            vec![0; 19],
            vec![0; 19],
            vec![0; 19],
            vec![0; 19],
            vec![0; 19],
            vec![0; 19],
            vec![0; 19],
            vec![0; 19],
            vec![0; 19],
            vec![0; 19],
            vec![0; 19],
            vec![0; 19],
            vec![0; 19],
            vec![0; 19],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1],
        ];

        let differences = board.get_sign_map_differences(&new_sign_map);

        // Should detect the four corner stones
        assert_eq!(differences.len(), 4);
        assert!(differences.contains(&Vertex::new(0, 0)));
        assert!(differences.contains(&Vertex::new(18, 0)));
        assert!(differences.contains(&Vertex::new(0, 18)));
        assert!(differences.contains(&Vertex::new(18, 18)));
    }

    #[test]
    fn test_differential_renderer_cache_invalidation() {
        let mut board = GoBoard::new();

        // Set some initial state
        board.set_stone(&Vertex::new(9, 9), 1);

        // Get initial stats (should have no cached changes)
        let stats = board.get_update_stats();
        assert_eq!(stats.changed_stones, 0);

        // Invalidate cache
        board.invalidate_render_cache();

        // Verify cache was cleared
        let stats_after = board.get_update_stats();
        assert_eq!(stats_after.changed_stones, 0);
    }

    #[test]
    fn test_vertex_changed_tracking() {
        let board = GoBoard::new();
        let vertex = Vertex::new(5, 5);

        // Initially no vertices should be marked as changed
        assert!(!board.vertex_changed(&vertex));
    }

    #[test]
    fn test_theme_changes_invalidate_cache() {
        let mut board = GoBoard::new();

        // Set a theme - this should invalidate the differential renderer cache
        let custom_theme = BoardTheme::dark();
        board.set_theme(custom_theme);

        // Cache should be invalidated after theme change
        let stats = board.get_update_stats();
        assert_eq!(stats.changed_stones, 0); // No cached changes after invalidation

        // Test grid theme changes
        let grid_theme = GridTheme::default();
        board.set_grid_theme(grid_theme);

        // Test stone theme changes
        let stone_theme = StoneTheme::default();
        board.set_stone_theme(stone_theme);
    }

    #[test]
    fn test_performance_with_large_updates() {
        let mut board = GoBoard::with_size(19, 19);

        // Create a large number of stone updates
        let mut updates = Vec::new();
        for x in 0..19 {
            for y in 0..19 {
                if (x + y) % 2 == 0 {
                    updates.push((Vertex::new(x, y), if x % 2 == 0 { 1 } else { -1 }));
                }
            }
        }

        // This should be efficient even with many updates
        assert!(board.update_stones(&updates));

        // Same updates should be detected as no change
        assert!(!board.update_stones(&updates));

        // Verify some stones were placed
        assert_eq!(board.get_stone(&Vertex::new(0, 0)), Some(1));
        assert_eq!(board.get_stone(&Vertex::new(1, 1)), Some(-1));
    }

    #[test]
    fn test_bounded_board_efficient_updates() {
        let mut bounded = BoundedGoBoard::with_size(9, 9, 270.0, 270.0);

        // Test efficient updates on bounded board
        let sign_map = vec![
            vec![1, 0, 0, 0, 0, 0, 0, 0, -1],
            vec![0, 1, 0, 0, 0, 0, 0, -1, 0],
            vec![0, 0, 1, 0, 0, 0, -1, 0, 0],
            vec![0, 0, 0, 1, 0, -1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, -1, 0, 1, 0, 0, 0],
            vec![0, 0, -1, 0, 0, 0, 1, 0, 0],
            vec![0, -1, 0, 0, 0, 0, 0, 1, 0],
            vec![-1, 0, 0, 0, 0, 0, 0, 0, 1],
        ];

        bounded.set_sign_map(sign_map);

        // Verify stones were placed correctly
        assert_eq!(bounded.board().state().sign_map[0][0], 1);
        assert_eq!(bounded.board().state().sign_map[0][8], -1);
        assert_eq!(bounded.board().state().sign_map[8][0], -1);
        assert_eq!(bounded.board().state().sign_map[8][8], 1);
    }
}
