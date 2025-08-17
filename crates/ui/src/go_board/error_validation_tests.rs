use super::*;
use crate::go_board::{GoBoardError, GoBoardResult, GoBoardValidator};

/// Tests for comprehensive error handling and validation systems
/// Verifies error types, validation logic, and error message clarity

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[test]
    fn test_go_board_construction_validation() {
        // Valid board sizes should work
        assert!(GoBoard::try_with_size(9, 9).is_ok());
        assert!(GoBoard::try_with_size(19, 19).is_ok());
        assert!(GoBoard::try_with_size(1, 1).is_ok());
        assert!(GoBoard::try_with_size(50, 50).is_ok());

        // Invalid board sizes should return errors
        let zero_width_error = GoBoard::try_with_size(0, 9);
        assert!(zero_width_error.is_err());
        assert!(matches!(
            zero_width_error.unwrap_err(),
            GoBoardError::InvalidBoardSize { .. }
        ));

        let zero_height_error = GoBoard::try_with_size(9, 0);
        assert!(zero_height_error.is_err());

        let too_large_error = GoBoard::try_with_size(51, 19);
        assert!(too_large_error.is_err());
        assert!(matches!(
            too_large_error.unwrap_err(),
            GoBoardError::InvalidBoardSize { .. }
        ));
    }

    #[test]
    fn test_vertex_size_validation() {
        let board = GoBoard::new();

        // Valid vertex sizes
        assert!(board.try_with_vertex_size(20.0).is_ok());
        assert!(board.try_with_vertex_size(1.0).is_ok());
        assert!(board.try_with_vertex_size(100.0).is_ok());

        // Invalid vertex sizes
        let zero_size_error = board.clone().try_with_vertex_size(0.0);
        assert!(zero_size_error.is_err());
        assert!(matches!(
            zero_size_error.unwrap_err(),
            GoBoardError::InvalidVertexSize { .. }
        ));

        let negative_size_error = board.clone().try_with_vertex_size(-5.0);
        assert!(negative_size_error.is_err());

        // Warning sizes that should still error
        let tiny_size_error = board.clone().try_with_vertex_size(0.5);
        assert!(tiny_size_error.is_err());

        let huge_size_error = board.clone().try_with_vertex_size(150.0);
        assert!(huge_size_error.is_err());
    }

    #[test]
    fn test_stone_setting_validation() {
        let mut board = GoBoard::new();

        // Valid stone placements
        assert!(board.try_set_stone(&Vertex::new(0, 0), 1).is_ok());
        assert!(board.try_set_stone(&Vertex::new(18, 18), -1).is_ok());
        assert!(board.try_set_stone(&Vertex::new(9, 9), 0).is_ok());

        // Invalid vertex positions
        let out_of_bounds_error = board.try_set_stone(&Vertex::new(19, 18), 1);
        assert!(out_of_bounds_error.is_err());
        assert!(matches!(
            out_of_bounds_error.unwrap_err(),
            GoBoardError::VertexOutOfBounds { .. }
        ));

        let out_of_bounds_y_error = board.try_set_stone(&Vertex::new(18, 19), 1);
        assert!(out_of_bounds_y_error.is_err());

        // Invalid sign values
        let invalid_sign_error = board.try_set_stone(&Vertex::new(5, 5), 2);
        assert!(invalid_sign_error.is_err());
        assert!(matches!(
            invalid_sign_error.unwrap_err(),
            GoBoardError::InvalidSignValue { .. }
        ));

        let another_invalid_sign_error = board.try_set_stone(&Vertex::new(5, 5), -5);
        assert!(another_invalid_sign_error.is_err());
    }

    #[test]
    fn test_sign_map_validation() {
        let mut board = GoBoard::new();

        // Valid sign map
        let valid_map = vec![vec![0; 19]; 19];
        assert!(board.try_update_sign_map(valid_map).is_ok());

        // Valid sign map with stones
        let mut valid_stones_map = vec![vec![0; 19]; 19];
        valid_stones_map[5][5] = 1;
        valid_stones_map[6][6] = -1;
        assert!(board.try_update_sign_map(valid_stones_map).is_ok());

        // Invalid map dimensions - wrong height
        let wrong_height_map = vec![vec![0; 19]; 18];
        let height_error = board.try_update_sign_map(wrong_height_map);
        assert!(height_error.is_err());
        assert!(matches!(
            height_error.unwrap_err(),
            GoBoardError::MapSizeMismatch { .. }
        ));

        // Invalid map dimensions - wrong width
        let wrong_width_map = vec![vec![0; 18]; 19];
        let width_error = board.try_update_sign_map(wrong_width_map);
        assert!(width_error.is_err());

        // Invalid sign values in map
        let mut invalid_signs_map = vec![vec![0; 19]; 19];
        invalid_signs_map[3][3] = 5; // Invalid sign value
        let sign_error = board.try_update_sign_map(invalid_signs_map);
        assert!(sign_error.is_err());
        assert!(matches!(
            sign_error.unwrap_err(),
            GoBoardError::InvalidSignValue { .. }
        ));

        // Empty map
        let empty_map: Vec<Vec<i8>> = vec![];
        let empty_error = board.try_update_sign_map(empty_map);
        assert!(empty_error.is_err());
    }

    #[test]
    fn test_bulk_stone_updates_validation() {
        let mut board = GoBoard::new();

        // Valid bulk updates
        let valid_updates = vec![
            (Vertex::new(1, 1), 1),
            (Vertex::new(2, 2), -1),
            (Vertex::new(3, 3), 0),
        ];
        assert!(board.try_update_stones(&valid_updates).is_ok());

        // Empty updates should be valid
        let empty_updates = vec![];
        assert!(board.try_update_stones(&empty_updates).is_ok());

        // Updates with invalid vertices
        let invalid_vertex_updates = vec![
            (Vertex::new(1, 1), 1),
            (Vertex::new(19, 18), 1), // Out of bounds
        ];
        let vertex_error = board.try_update_stones(&invalid_vertex_updates);
        assert!(vertex_error.is_err());
        assert!(matches!(
            vertex_error.unwrap_err(),
            GoBoardError::VertexOutOfBounds { .. }
        ));

        // Updates with invalid signs
        let invalid_sign_updates = vec![
            (Vertex::new(1, 1), 1),
            (Vertex::new(2, 2), 3), // Invalid sign
        ];
        let sign_error = board.try_update_stones(&invalid_sign_updates);
        assert!(sign_error.is_err());
        assert!(matches!(
            sign_error.unwrap_err(),
            GoBoardError::InvalidSignValue { .. }
        ));

        // Too many updates (performance limit)
        let large_updates: Vec<(Vertex, i8)> =
            (0..101).map(|i| (Vertex::new(i % 19, i / 19), 1)).collect();
        let bulk_error = board.try_update_stones(&large_updates);
        assert!(bulk_error.is_err());
        assert!(matches!(
            bulk_error.unwrap_err(),
            GoBoardError::ValidationError { .. }
        ));
    }

    #[test]
    fn test_validator_board_size_edge_cases() {
        // Boundary cases
        assert!(GoBoardValidator::validate_board_size(1, 1).is_ok());
        assert!(GoBoardValidator::validate_board_size(50, 50).is_ok());

        // Just outside boundaries
        assert!(GoBoardValidator::validate_board_size(0, 1).is_err());
        assert!(GoBoardValidator::validate_board_size(1, 0).is_err());
        assert!(GoBoardValidator::validate_board_size(51, 1).is_err());
        assert!(GoBoardValidator::validate_board_size(1, 51).is_err());

        // Common Go board sizes
        assert!(GoBoardValidator::validate_board_size(9, 9).is_ok());
        assert!(GoBoardValidator::validate_board_size(13, 13).is_ok());
        assert!(GoBoardValidator::validate_board_size(19, 19).is_ok());
    }

    #[test]
    fn test_validator_range_validation() {
        // Valid ranges
        assert!(GoBoardValidator::validate_range(0, 18, 0, 18, 19, 19).is_ok());
        assert!(GoBoardValidator::validate_range(5, 15, 5, 15, 19, 19).is_ok());
        assert!(GoBoardValidator::validate_range(0, 0, 0, 0, 19, 19).is_ok());

        // Invalid ranges - start > end
        assert!(GoBoardValidator::validate_range(15, 5, 0, 18, 19, 19).is_err());
        assert!(GoBoardValidator::validate_range(0, 18, 15, 5, 19, 19).is_err());

        // Invalid ranges - out of bounds
        assert!(GoBoardValidator::validate_range(0, 19, 0, 18, 19, 19).is_err());
        assert!(GoBoardValidator::validate_range(0, 18, 0, 19, 19, 19).is_err());

        // Edge case - single point ranges
        assert!(GoBoardValidator::validate_range(5, 5, 10, 10, 19, 19).is_ok());
    }

    #[test]
    fn test_validator_color_validation() {
        // Valid hex colors
        assert!(GoBoardValidator::validate_color("background", "#000000").is_ok());
        assert!(GoBoardValidator::validate_color("background", "#fff").is_ok());
        assert!(GoBoardValidator::validate_color("background", "#12345678").is_ok());

        // Valid named colors
        assert!(GoBoardValidator::validate_color("background", "red").is_ok());
        assert!(GoBoardValidator::validate_color("background", "transparent").is_ok());

        // Valid function colors
        assert!(GoBoardValidator::validate_color("background", "rgb(255,0,0)").is_ok());
        assert!(GoBoardValidator::validate_color("background", "rgba(255,0,0,0.5)").is_ok());

        // Invalid colors
        assert!(GoBoardValidator::validate_color("background", "").is_err());
        assert!(GoBoardValidator::validate_color("background", "#00").is_err());
        assert!(GoBoardValidator::validate_color("background", "#0000000").is_err());
        assert!(GoBoardValidator::validate_color("background", "#123456789").is_err());
    }

    #[test]
    fn test_validator_animation_duration() {
        // Valid durations
        assert!(GoBoardValidator::validate_animation_duration(100).is_ok());
        assert!(GoBoardValidator::validate_animation_duration(1000).is_ok());
        assert!(GoBoardValidator::validate_animation_duration(5000).is_ok());
        assert!(GoBoardValidator::validate_animation_duration(10000).is_ok());

        // Invalid durations
        assert!(GoBoardValidator::validate_animation_duration(10001).is_err());
        assert!(GoBoardValidator::validate_animation_duration(20000).is_err());
    }

    #[test]
    fn test_validator_bulk_update_limits() {
        // Valid update sizes
        assert!(GoBoardValidator::validate_bulk_update_size(0, 100).is_ok());
        assert!(GoBoardValidator::validate_bulk_update_size(50, 100).is_ok());
        assert!(GoBoardValidator::validate_bulk_update_size(100, 100).is_ok());

        // Invalid update sizes
        assert!(GoBoardValidator::validate_bulk_update_size(101, 100).is_err());
        assert!(GoBoardValidator::validate_bulk_update_size(200, 100).is_err());

        // Different limits
        assert!(GoBoardValidator::validate_bulk_update_size(150, 200).is_ok());
        assert!(GoBoardValidator::validate_bulk_update_size(250, 200).is_err());
    }

    #[test]
    fn test_error_message_quality() {
        // Test that error messages contain helpful information
        let vertex_error = GoBoardError::VertexOutOfBounds {
            vertex: Vertex::new(20, 20),
            board_width: 19,
            board_height: 19,
        };

        let error_message = format!("{}", vertex_error);
        assert!(error_message.contains("(20, 20)"));
        assert!(error_message.contains("19x19"));
        assert!(error_message.contains("Valid coordinates"));
        assert!(error_message.contains("(0-18, 0-18)"));

        let size_error = GoBoardError::InvalidBoardSize {
            width: 0,
            height: 9,
            message: "Width cannot be zero".to_string(),
        };

        let size_message = format!("{}", size_error);
        assert!(size_message.contains("Invalid board size"));
        assert!(size_message.contains("0x9"));
        assert!(size_message.contains("Supported sizes"));
        assert!(size_message.contains("Width cannot be zero"));

        let sign_error = GoBoardError::InvalidSignValue {
            sign: 5,
            vertex: Vertex::new(3, 3),
        };

        let sign_message = format!("{}", sign_error);
        assert!(sign_message.contains("Invalid sign value 5"));
        assert!(sign_message.contains("(3, 3)"));
        assert!(sign_message.contains("Valid values: -1"));
        assert!(sign_message.contains("white stone"));
        assert!(sign_message.contains("black stone"));
    }

    #[test]
    fn test_error_chain_validation() {
        // Test that we can catch and handle multiple validation errors
        let mut board = GoBoard::new();

        // Create an invalid bulk update with multiple issues
        let problematic_updates = vec![
            (Vertex::new(1, 1), 1),   // Valid
            (Vertex::new(25, 25), 1), // Out of bounds
            (Vertex::new(2, 2), 10),  // Invalid sign
        ];

        // Should catch the first error (out of bounds)
        let result = board.try_update_stones(&problematic_updates);
        assert!(result.is_err());

        // Verify that validation stops at first error for performance
        match result.unwrap_err() {
            GoBoardError::VertexOutOfBounds { vertex, .. } => {
                assert_eq!(vertex, Vertex::new(25, 25));
            }
            _ => panic!("Expected VertexOutOfBounds error"),
        }
    }

    #[test]
    fn test_map_validation_edge_cases() {
        // Test with different board sizes
        for &size in &[9, 13, 19] {
            let valid_map = vec![vec![0; size]; size];
            assert!(
                GoBoardValidator::validate_map_size(&valid_map, "test_map", size, size).is_ok()
            );

            // Wrong dimensions
            let wrong_map = vec![vec![0; size + 1]; size];
            assert!(
                GoBoardValidator::validate_map_size(&wrong_map, "test_map", size, size).is_err()
            );
        }

        // Inconsistent row lengths
        let inconsistent_map = vec![
            vec![0, 0, 0],
            vec![0, 0], // Wrong length
            vec![0, 0, 0],
        ];
        let inconsistent_error =
            GoBoardValidator::validate_map_size(&inconsistent_map, "test_map", 3, 3);
        assert!(inconsistent_error.is_err());
        assert!(matches!(
            inconsistent_error.unwrap_err(),
            GoBoardError::ValidationError { .. }
        ));
    }

    #[test]
    fn test_performance_validation_limits() {
        let mut board = GoBoard::new();

        // Test that performance validation kicks in
        let max_allowed_updates = 100;

        // Just at the limit should work
        let at_limit_updates: Vec<(Vertex, i8)> = (0..max_allowed_updates)
            .map(|i| (Vertex::new(i % 19, i / 19), 1))
            .collect();
        assert!(board.try_update_stones(&at_limit_updates).is_ok());

        // Just over the limit should fail
        let over_limit_updates: Vec<(Vertex, i8)> = (0..max_allowed_updates + 1)
            .map(|i| (Vertex::new(i % 19, i / 19), 1))
            .collect();
        let limit_error = board.try_update_stones(&over_limit_updates);
        assert!(limit_error.is_err());
        assert!(matches!(
            limit_error.unwrap_err(),
            GoBoardError::ValidationError { .. }
        ));
    }

    #[test]
    fn test_error_types_are_clone_and_debug() {
        let error = GoBoardError::InvalidBoardSize {
            width: 0,
            height: 9,
            message: "Test".to_string(),
        };

        // Should be able to clone
        let _cloned = error.clone();

        // Should be able to debug print
        let _debug_str = format!("{:?}", error);

        // Should be able to display
        let _display_str = format!("{}", error);
    }
}
