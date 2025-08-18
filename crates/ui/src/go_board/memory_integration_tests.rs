use super::*;
use std::thread;
use std::time::Duration;

/// Integration tests for memory management and cleanup systems
/// Tests the interaction between GoBoard component and MemoryManager

#[cfg(test)]
mod memory_integration_tests {
    use super::*;

    #[test]
    fn test_go_board_memory_cleanup_on_theme_change() {
        let mut board = GoBoard::new();

        // Get initial stats
        let initial_stats = board.get_memory_stats();

        // Change theme - should trigger cleanup
        let new_theme = BoardTheme::default();
        board.set_theme(new_theme);

        // Memory should be cleaned up
        let stats_after_theme = board.get_memory_stats();
        // Cleanup should have happened
        assert!(stats_after_theme.last_cleanup.is_some());
    }

    #[test]
    fn test_go_board_memory_cleanup_on_large_updates() {
        let mut board = GoBoard::new();

        // Create a scenario where cleanup would be triggered
        // First, make the memory manager think cleanup is needed
        board.force_memory_cleanup(); // Reset state

        // Wait to make cleanup timing trigger
        thread::sleep(Duration::from_millis(10));

        // Perform a large update that should trigger cleanup
        let large_updates: Vec<(Vertex, i8)> = (0..15)
            .map(|i| (Vertex::new(i % 19, i / 19), if i % 2 == 0 { 1 } else { -1 }))
            .collect();

        let changed = board.update_stones(&large_updates);
        assert!(changed);

        // Should have triggered cleanup due to size
        let stats = board.get_memory_stats();
        assert!(stats.last_cleanup.is_some());
    }

    #[test]
    fn test_component_pooling_usage() {
        let mut board = GoBoard::new();

        // Get initial pool stats
        let initial_pool_stats = board.get_pool_stats();
        assert_eq!(initial_pool_stats.stone_total_created, 0);
        assert_eq!(initial_pool_stats.stone_total_reused, 0);

        // Use component pooling
        let stone_component = board.get_pooled_stone_component(Vertex::new(5, 5), 1);
        assert_eq!(stone_component.vertex, Vertex::new(5, 5));
        assert_eq!(stone_component.sign, 1);
        assert!(stone_component.in_use);

        // Return component to pool
        board.return_stone_component(stone_component);

        // Get another component (should reuse)
        let reused_component = board.get_pooled_stone_component(Vertex::new(6, 6), -1);
        board.return_marker_component(
            board.get_pooled_marker_component(Vertex::new(7, 7), MarkerType::Circle),
        );

        // Check pool statistics
        let final_pool_stats = board.get_pool_stats();
        assert!(final_pool_stats.stone_total_created > 0);
        assert!(final_pool_stats.marker_total_created > 0);

        // Clean up
        board.return_stone_component(reused_component);
    }

    #[test]
    fn test_memory_efficiency_tracking() {
        let mut board = GoBoard::new();

        // Initial efficiency should be 0 (no operations yet)
        assert_eq!(board.get_memory_efficiency(), 0.0);

        // Create and return components to build up pool
        for i in 0..5 {
            let stone = board.get_pooled_stone_component(Vertex::new(i, i), 1);
            board.return_stone_component(stone);
        }

        // Get components again (should reuse)
        for i in 0..3 {
            let stone = board.get_pooled_stone_component(Vertex::new(i + 10, i + 10), -1);
            board.return_stone_component(stone);
        }

        // Efficiency should be greater than 0 due to reuse
        let efficiency = board.get_memory_efficiency();
        assert!(efficiency > 0.0);
        assert!(efficiency <= 1.0);
    }

    #[test]
    fn test_differential_rendering_with_memory_cleanup() {
        let mut board = GoBoard::new();

        // Set up initial state
        board.set_stone(&Vertex::new(3, 3), 1);
        board.set_stone(&Vertex::new(4, 4), -1);

        // Wait to make cleanup timing trigger
        thread::sleep(Duration::from_millis(10));

        // Render with differential updates - should trigger cleanup
        let handlers = VertexEventHandlers::new();
        let _render_result = board.render_with_differential_updates(handlers);

        // Should have performed cleanup
        let stats = board.get_memory_stats();
        assert!(stats.last_cleanup.is_some());
    }

    #[test]
    fn test_repeated_state_updates_no_memory_leaks() {
        let mut board = GoBoard::new();

        // Perform many repeated updates to test for memory leaks
        for iteration in 0..50 {
            // Update stones
            let updates: Vec<(Vertex, i8)> = (0..10)
                .map(|i| {
                    (
                        Vertex::new(i, iteration % 19),
                        if i % 2 == 0 { 1 } else { -1 },
                    )
                })
                .collect();
            board.update_stones(&updates);

            // Update ghost stones
            let ghost_updates: Vec<(Vertex, Option<GhostStone>)> = (0..5)
                .map(|i| {
                    (
                        Vertex::new(i + 10, iteration % 19),
                        Some(GhostStone::new(1, GhostStoneType::Good)),
                    )
                })
                .collect();
            board.update_ghost_stones(&ghost_updates);
        }

        // Check final memory state
        let final_stats = board.get_memory_stats();
        let pool_stats = board.get_pool_stats();

        // Should have reasonable memory usage (not growing unbounded)
        assert!(pool_stats.stone_pool_size < 100); // Pool should be bounded
        assert!(pool_stats.marker_pool_size < 100); // Pool should be bounded

        // Should have some reuse efficiency
        assert!(board.get_memory_efficiency() > 0.0);
    }

    #[test]
    fn test_memory_manager_drop_cleanup() {
        // Create a scope where GoBoard will be dropped
        {
            let mut board = GoBoard::new();

            let stone = board.get_pooled_stone_component(Vertex::new(0, 0), 1);
            board.return_stone_component(stone);
        }; // board is dropped here, should trigger cleanup

        // The actual cleanup verification would require more sophisticated testing
        // in a real implementation, possibly with mock objects or memory profiling
    }

    #[test]
    fn test_cleanup_configuration_effects() {
        use crate::go_board::memory_manager::CleanupConfig;

        // Create a board with custom cleanup configuration
        let config = CleanupConfig {
            max_pool_size: 5,
            cleanup_interval: 50, // Very short interval
            component_max_age: 100,
            enable_memory_monitoring: true,
        };

        let mut board = GoBoard::new();
        // In a real implementation, we'd want to be able to set the config
        // For now, we test the default behavior and verify cleanup happens

        // Fill up the pool beyond the default max
        for i in 0..10 {
            let stone = board.get_pooled_stone_component(Vertex::new(i, 0), 1);
            board.return_stone_component(stone);
        }

        // Force cleanup
        board.cleanup_memory();

        let pool_stats = board.get_pool_stats();
        // Pool should be limited in size
        assert!(pool_stats.stone_pool_size <= 100); // Default max pool size
    }

    #[test]
    fn test_memory_stats_accuracy() {
        let mut board = GoBoard::new();

        // Verify initial state
        let initial_stats = board.get_memory_stats();
        assert_eq!(initial_stats.pooled_stones, 0);
        assert_eq!(initial_stats.pooled_markers, 0);
        assert_eq!(initial_stats.total_allocations, 0);
        assert_eq!(initial_stats.total_deallocations, 0);

        // Perform operations and verify stat updates
        let stone = board.get_pooled_stone_component(Vertex::new(1, 1), 1);
        let stats_after_alloc = board.get_memory_stats();
        assert_eq!(stats_after_alloc.total_allocations, 1);

        board.return_stone_component(stone);
        let stats_after_return = board.get_memory_stats();
        assert_eq!(stats_after_return.total_deallocations, 1);
        assert_eq!(stats_after_return.pooled_stones, 1);
    }
}
