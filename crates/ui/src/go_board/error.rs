use crate::go_board::types::Vertex;
use std::fmt;

/// Comprehensive error types for Go board component operations
/// Provides clear, actionable error messages for common integration issues
#[derive(Debug, Clone, PartialEq)]
pub enum GoBoardError {
    /// Invalid board dimensions
    InvalidBoardSize {
        width: usize,
        height: usize,
        message: String,
    },

    /// Vertex coordinates are out of bounds
    VertexOutOfBounds {
        vertex: Vertex,
        board_width: usize,
        board_height: usize,
    },

    /// Invalid sign value (must be -1, 0, or 1)
    InvalidSignValue { sign: i8, vertex: Vertex },

    /// Invalid board range parameters
    InvalidRange {
        start_x: usize,
        end_x: usize,
        start_y: usize,
        end_y: usize,
        board_width: usize,
        board_height: usize,
    },

    /// Invalid vertex size (must be positive)
    InvalidVertexSize { size: f32 },

    /// Map size mismatch (dimensions don't match board)
    MapSizeMismatch {
        map_type: String,
        expected_width: usize,
        expected_height: usize,
        actual_width: usize,
        actual_height: usize,
    },

    /// Invalid marker type or configuration
    InvalidMarker { vertex: Vertex, message: String },

    /// Invalid theme configuration
    InvalidTheme {
        field: String,
        value: String,
        message: String,
    },

    /// Generic validation error with custom message
    ValidationError {
        field: String,
        value: String,
        message: String,
    },
}

impl fmt::Display for GoBoardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GoBoardError::InvalidBoardSize {
                width,
                height,
                message,
            } => {
                write!(f, "Invalid board size {}x{}: {}. Supported sizes: 9x9, 13x13, 19x19, or custom sizes between 1x1 and 50x50.", width, height, message)
            }

            GoBoardError::VertexOutOfBounds {
                vertex,
                board_width,
                board_height,
            } => {
                write!(f, "Vertex ({}, {}) is out of bounds for {}x{} board. Valid coordinates: (0-{}, 0-{}).",
                    vertex.x, vertex.y, board_width, board_height, board_width - 1, board_height - 1)
            }

            GoBoardError::InvalidSignValue { sign, vertex } => {
                write!(f, "Invalid sign value {} at vertex ({}, {}). Valid values: -1 (white stone), 0 (empty), 1 (black stone).",
                    sign, vertex.x, vertex.y)
            }

            GoBoardError::InvalidRange {
                start_x,
                end_x,
                start_y,
                end_y,
                board_width,
                board_height,
            } => {
                write!(f, "Invalid range ({}:{}, {}:{}) for {}x{} board. Range coordinates must be within board bounds and start <= end.",
                    start_x, end_x, start_y, end_y, board_width, board_height)
            }

            GoBoardError::InvalidVertexSize { size } => {
                write!(f, "Invalid vertex size {}. Vertex size must be positive (recommended: 12.0-48.0 pixels).", size)
            }

            GoBoardError::MapSizeMismatch {
                map_type,
                expected_width,
                expected_height,
                actual_width,
                actual_height,
            } => {
                write!(f, "{} size mismatch: expected {}x{}, got {}x{}. All maps must match the board dimensions.",
                    map_type, expected_width, expected_height, actual_width, actual_height)
            }

            GoBoardError::InvalidMarker { vertex, message } => {
                write!(
                    f,
                    "Invalid marker at vertex ({}, {}): {}. Check marker type and configuration.",
                    vertex.x, vertex.y, message
                )
            }

            GoBoardError::InvalidTheme {
                field,
                value,
                message,
            } => {
                write!(f, "Invalid theme configuration for '{}' = '{}': {}. Check theme documentation for valid values.",
                    field, value, message)
            }

            GoBoardError::ValidationError {
                field,
                value,
                message,
            } => {
                write!(
                    f,
                    "Validation error for '{}' = '{}': {}.",
                    field, value, message
                )
            }
        }
    }
}

impl std::error::Error for GoBoardError {}

/// Result type for Go board operations
pub type GoBoardResult<T> = Result<T, GoBoardError>;

/// Validation utilities for Go board components
pub struct GoBoardValidator;

impl GoBoardValidator {
    /// Validates board dimensions
    pub fn validate_board_size(width: usize, height: usize) -> GoBoardResult<()> {
        if width == 0 || height == 0 {
            return Err(GoBoardError::InvalidBoardSize {
                width,
                height,
                message: "Board dimensions must be greater than 0".to_string(),
            });
        }

        if width > 50 || height > 50 {
            return Err(GoBoardError::InvalidBoardSize {
                width,
                height,
                message: "Board dimensions must not exceed 50x50 for performance reasons"
                    .to_string(),
            });
        }

        Ok(())
    }

    /// Validates vertex coordinates against board dimensions
    pub fn validate_vertex(
        vertex: &Vertex,
        board_width: usize,
        board_height: usize,
    ) -> GoBoardResult<()> {
        if vertex.x >= board_width || vertex.y >= board_height {
            return Err(GoBoardError::VertexOutOfBounds {
                vertex: *vertex,
                board_width,
                board_height,
            });
        }
        Ok(())
    }

    /// Validates sign value for stone placement
    pub fn validate_sign(sign: i8, vertex: &Vertex) -> GoBoardResult<()> {
        if !(-1..=1).contains(&sign) {
            return Err(GoBoardError::InvalidSignValue {
                sign,
                vertex: *vertex,
            });
        }
        Ok(())
    }

    /// Validates board range parameters
    pub fn validate_range(
        start_x: usize,
        end_x: usize,
        start_y: usize,
        end_y: usize,
        board_width: usize,
        board_height: usize,
    ) -> GoBoardResult<()> {
        if start_x > end_x || start_y > end_y {
            return Err(GoBoardError::InvalidRange {
                start_x,
                end_x,
                start_y,
                end_y,
                board_width,
                board_height,
            });
        }

        if end_x >= board_width || end_y >= board_height {
            return Err(GoBoardError::InvalidRange {
                start_x,
                end_x,
                start_y,
                end_y,
                board_width,
                board_height,
            });
        }

        Ok(())
    }

    /// Validates vertex size parameter
    pub fn validate_vertex_size(size: f32) -> GoBoardResult<()> {
        if size <= 0.0 {
            return Err(GoBoardError::InvalidVertexSize { size });
        }

        if size < 1.0 {
            return Err(GoBoardError::ValidationError {
                field: "vertex_size".to_string(),
                value: size.to_string(),
                message: "Vertex size below 1.0 may cause rendering issues".to_string(),
            });
        }

        if size > 100.0 {
            return Err(GoBoardError::ValidationError {
                field: "vertex_size".to_string(),
                value: size.to_string(),
                message: "Vertex size above 100.0 may cause performance issues".to_string(),
            });
        }

        Ok(())
    }

    /// Validates map dimensions against board size
    pub fn validate_map_size<T>(
        map: &[Vec<T>],
        map_type: &str,
        expected_width: usize,
        expected_height: usize,
    ) -> GoBoardResult<()> {
        if map.is_empty() {
            return Err(GoBoardError::MapSizeMismatch {
                map_type: map_type.to_string(),
                expected_width,
                expected_height,
                actual_width: 0,
                actual_height: 0,
            });
        }

        let actual_height = map.len();
        let actual_width = map[0].len();

        // Check height
        if actual_height != expected_height {
            return Err(GoBoardError::MapSizeMismatch {
                map_type: map_type.to_string(),
                expected_width,
                expected_height,
                actual_width,
                actual_height,
            });
        }

        // Check width consistency across all rows
        for (row_idx, row) in map.iter().enumerate() {
            if row.len() != expected_width {
                return Err(GoBoardError::ValidationError {
                    field: format!("{}_row_{}", map_type, row_idx),
                    value: row.len().to_string(),
                    message: format!(
                        "Inconsistent row width: expected {}, got {}",
                        expected_width,
                        row.len()
                    ),
                });
            }
        }

        Ok(())
    }

    /// Validates theme color values
    pub fn validate_color(color_name: &str, color_value: &str) -> GoBoardResult<()> {
        // Basic color validation (hex colors, named colors, rgb/rgba)
        if color_value.is_empty() {
            return Err(GoBoardError::InvalidTheme {
                field: color_name.to_string(),
                value: color_value.to_string(),
                message: "Color value cannot be empty".to_string(),
            });
        }

        // Check for hex colors
        if color_value.starts_with('#') {
            if color_value.len() != 4 && color_value.len() != 7 && color_value.len() != 9 {
                return Err(GoBoardError::InvalidTheme {
                    field: color_name.to_string(),
                    value: color_value.to_string(),
                    message: "Hex colors must be #RGB, #RRGGBB, or #RRGGBBAA format".to_string(),
                });
            }
        }

        Ok(())
    }

    /// Validates bulk updates for performance
    pub fn validate_bulk_update_size(update_count: usize, max_size: usize) -> GoBoardResult<()> {
        if update_count > max_size {
            return Err(GoBoardError::ValidationError {
                field: "bulk_update_size".to_string(),
                value: update_count.to_string(),
                message: format!(
                    "Bulk update size {} exceeds maximum {} for performance reasons",
                    update_count, max_size
                ),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_board_size() {
        // Valid sizes
        assert!(GoBoardValidator::validate_board_size(9, 9).is_ok());
        assert!(GoBoardValidator::validate_board_size(19, 19).is_ok());
        assert!(GoBoardValidator::validate_board_size(1, 1).is_ok());
        assert!(GoBoardValidator::validate_board_size(50, 50).is_ok());

        // Invalid sizes
        assert!(GoBoardValidator::validate_board_size(0, 9).is_err());
        assert!(GoBoardValidator::validate_board_size(9, 0).is_err());
        assert!(GoBoardValidator::validate_board_size(51, 19).is_err());
        assert!(GoBoardValidator::validate_board_size(19, 51).is_err());
    }

    #[test]
    fn test_validate_vertex() {
        // Valid vertices
        assert!(GoBoardValidator::validate_vertex(&Vertex::new(0, 0), 19, 19).is_ok());
        assert!(GoBoardValidator::validate_vertex(&Vertex::new(18, 18), 19, 19).is_ok());
        assert!(GoBoardValidator::validate_vertex(&Vertex::new(5, 5), 9, 9).is_ok());

        // Invalid vertices
        assert!(GoBoardValidator::validate_vertex(&Vertex::new(19, 18), 19, 19).is_err());
        assert!(GoBoardValidator::validate_vertex(&Vertex::new(18, 19), 19, 19).is_err());
        assert!(GoBoardValidator::validate_vertex(&Vertex::new(9, 5), 9, 9).is_err());
    }

    #[test]
    fn test_validate_sign() {
        let vertex = Vertex::new(5, 5);

        // Valid signs
        assert!(GoBoardValidator::validate_sign(-1, &vertex).is_ok());
        assert!(GoBoardValidator::validate_sign(0, &vertex).is_ok());
        assert!(GoBoardValidator::validate_sign(1, &vertex).is_ok());

        // Invalid signs
        assert!(GoBoardValidator::validate_sign(-2, &vertex).is_err());
        assert!(GoBoardValidator::validate_sign(2, &vertex).is_err());
        assert!(GoBoardValidator::validate_sign(5, &vertex).is_err());
    }

    #[test]
    fn test_validate_range() {
        // Valid ranges
        assert!(GoBoardValidator::validate_range(0, 18, 0, 18, 19, 19).is_ok());
        assert!(GoBoardValidator::validate_range(5, 15, 5, 15, 19, 19).is_ok());
        assert!(GoBoardValidator::validate_range(0, 0, 0, 0, 19, 19).is_ok());

        // Invalid ranges
        assert!(GoBoardValidator::validate_range(15, 5, 0, 18, 19, 19).is_err()); // start > end
        assert!(GoBoardValidator::validate_range(0, 18, 15, 5, 19, 19).is_err()); // start > end
        assert!(GoBoardValidator::validate_range(0, 19, 0, 18, 19, 19).is_err()); // out of bounds
        assert!(GoBoardValidator::validate_range(0, 18, 0, 19, 19, 19).is_err());
        // out of bounds
    }

    #[test]
    fn test_validate_vertex_size() {
        // Valid sizes
        assert!(GoBoardValidator::validate_vertex_size(20.0).is_ok());
        assert!(GoBoardValidator::validate_vertex_size(1.0).is_ok());
        assert!(GoBoardValidator::validate_vertex_size(100.0).is_ok());

        // Invalid sizes
        assert!(GoBoardValidator::validate_vertex_size(0.0).is_err());
        assert!(GoBoardValidator::validate_vertex_size(-5.0).is_err());

        // Warning sizes (should return error with guidance)
        assert!(GoBoardValidator::validate_vertex_size(0.5).is_err());
        assert!(GoBoardValidator::validate_vertex_size(150.0).is_err());
    }

    #[test]
    fn test_validate_map_size() {
        // Valid map
        let valid_map = vec![vec![0; 9]; 9];
        assert!(GoBoardValidator::validate_map_size(&valid_map, "sign_map", 9, 9).is_ok());

        // Empty map
        let empty_map: Vec<Vec<i8>> = vec![];
        assert!(GoBoardValidator::validate_map_size(&empty_map, "sign_map", 9, 9).is_err());

        // Wrong height
        let wrong_height_map = vec![vec![0; 9]; 8];
        assert!(GoBoardValidator::validate_map_size(&wrong_height_map, "sign_map", 9, 9).is_err());

        // Wrong width
        let wrong_width_map = vec![vec![0; 8]; 9];
        assert!(GoBoardValidator::validate_map_size(&wrong_width_map, "sign_map", 9, 9).is_err());

        // Inconsistent row widths
        let inconsistent_map = vec![vec![0; 9], vec![0; 8], vec![0; 9]];
        assert!(GoBoardValidator::validate_map_size(&inconsistent_map, "sign_map", 9, 9).is_err());
    }

    #[test]
    fn test_validate_color() {
        // Valid colors
        assert!(GoBoardValidator::validate_color("background", "#000000").is_ok());
        assert!(GoBoardValidator::validate_color("background", "#000").is_ok());
        assert!(GoBoardValidator::validate_color("background", "#00000000").is_ok());
        assert!(GoBoardValidator::validate_color("background", "red").is_ok());
        assert!(GoBoardValidator::validate_color("background", "rgb(255,0,0)").is_ok());

        // Invalid colors
        assert!(GoBoardValidator::validate_color("background", "").is_err());
        assert!(GoBoardValidator::validate_color("background", "#00").is_err());
        assert!(GoBoardValidator::validate_color("background", "#0000000").is_err());
    }

    #[test]
    fn test_validate_bulk_update_size() {
        // Valid update sizes
        assert!(GoBoardValidator::validate_bulk_update_size(10, 100).is_ok());
        assert!(GoBoardValidator::validate_bulk_update_size(50, 100).is_ok());
        assert!(GoBoardValidator::validate_bulk_update_size(100, 100).is_ok());

        // Invalid update sizes
        assert!(GoBoardValidator::validate_bulk_update_size(101, 100).is_err());
        assert!(GoBoardValidator::validate_bulk_update_size(200, 100).is_err());
    }

    #[test]
    fn test_error_display_messages() {
        let vertex_error = GoBoardError::VertexOutOfBounds {
            vertex: Vertex::new(20, 20),
            board_width: 19,
            board_height: 19,
        };

        let message = format!("{}", vertex_error);
        assert!(message.contains("(20, 20)"));
        assert!(message.contains("19x19"));
        assert!(message.contains("Valid coordinates"));

        let size_error = GoBoardError::InvalidBoardSize {
            width: 0,
            height: 9,
            message: "Width cannot be zero".to_string(),
        };

        let size_message = format!("{}", size_error);
        assert!(size_message.contains("Invalid board size"));
        assert!(size_message.contains("Supported sizes"));
    }
}
