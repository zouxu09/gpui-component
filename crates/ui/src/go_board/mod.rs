// =============================================================================
// NEW SIMPLIFIED GO BOARD API
// =============================================================================
// This replaces the complex, fragmented previous system with a clean, ergonomic API

// Core types and data structures
pub mod core;

// Main board component with data management
pub mod board;

// Unified rendering system
pub mod render;

// View component with interactions
pub mod view;

// Re-export the main public API for easy usage
pub use board::{Board, BoundedBoard};
pub use view::{board_with_stones, bounded_board_view, demo_board_view, simple_board, BoardView};

// Re-export core types individually to avoid conflicts
pub use core::{
    BoardData, Ghost, GhostKind, Heat, Line, LineStyle, Marker, NavEvent, Pos, PosEvent, Range,
    Selection, SelectionStyle, Stone, Territory, Theme, BLACK, EMPTY, WHITE,
};

// Compatibility exports for deprecated modules
pub use error::{GoBoardResult, GoBoardValidator};
pub use state::GoBoardState;
pub use types::*;

// Re-export old components with new names for compatibility
pub use ghost_stone::GhostStoneOverlay;
pub use grid::Grid;
pub use grid::GridTheme;
pub use heat_overlay::HeatOverlay;
pub use interactions::VertexInteractions;
pub use line_overlay::LineOverlay;
pub use markers::Markers;
pub use paint_overlay::PaintOverlay;
pub use selection::VertexSelections;
pub use stones::{StoneTheme, Stones};

// =============================================================================
// COMPATIBILITY LAYER (for migration from old system)
// =============================================================================
// Keep the old module structure temporarily for backward compatibility
// TODO: Remove these after migration is complete

#[deprecated(note = "Use the new simplified API instead")]
pub mod bounded_go_board;

#[deprecated(note = "Use the new simplified API instead")]
pub mod coordinates;

#[deprecated(note = "Use the new simplified API instead")]
pub mod error;

#[deprecated(note = "Use the new simplified API instead")]
pub mod ghost_stone;

#[deprecated(note = "Use the new simplified API instead")]
pub mod go_board;

#[deprecated(note = "Use the new simplified API instead")]
pub mod grid;

#[deprecated(note = "Use the new simplified API instead")]
pub mod heat_overlay;

#[deprecated(note = "Use the new simplified API instead")]
pub mod interactions;

#[deprecated(note = "Use the new simplified API instead")]
pub mod keyboard_navigation;

#[deprecated(note = "Use the new simplified API instead")]
pub mod line_overlay;

#[deprecated(note = "Use the new simplified API instead")]
pub mod markers;

#[deprecated(note = "Use the new simplified API instead")]
pub mod paint_overlay;

#[deprecated(note = "Use the new simplified API instead")]
pub mod position_utils;

#[deprecated(note = "Use the new simplified API instead")]
pub mod selection;

#[deprecated(note = "Use the new simplified API instead")]
pub mod state;

#[deprecated(note = "Use the new simplified API instead")]
pub mod stones;

#[deprecated(note = "Use the new simplified API instead")]
pub mod theme;

#[deprecated(note = "Use the new simplified API instead")]
pub mod types;

// Keep the old types for compatibility (with deprecation warnings)
#[deprecated(note = "Use Pos instead")]
pub use types::Vertex;

#[deprecated(note = "Use Range instead")]
pub use types::BoardRange;

#[deprecated(note = "Use the new simplified Theme instead")]
pub use theme::BoardTheme;

#[deprecated(note = "Use the new simplified Board instead")]
pub use go_board::GoBoard;

#[deprecated(note = "Use the new simplified BoundedBoard instead")]
pub use bounded_go_board::BoundedGoBoard;

#[deprecated(note = "Use the new simplified interaction system instead")]
pub use interactions::{VertexClickEvent, VertexEventHandlers};

// =============================================================================
// CONVENIENCE RE-EXPORTS
// =============================================================================

/// Quick access to commonly used constants
pub mod constants {
    pub use crate::go_board::core::{BLACK, EMPTY, WHITE};
}

/// Quick access to theme presets
pub mod themes {
    pub use crate::go_board::core::Theme;

    pub fn default() -> Theme {
        Theme::default()
    }
    pub fn dark() -> Theme {
        Theme::dark()
    }
    pub fn minimal() -> Theme {
        Theme::minimal()
    }
    pub fn high_contrast() -> Theme {
        Theme::high_contrast()
    }
}

/// Quick access to marker creation - use crate::go_board::markers for legacy code
/// New code should use crate::go_board::core::Marker directly
pub mod marker_helpers {
    pub use crate::go_board::core::Marker;

    pub fn circle() -> Marker {
        Marker::circle()
    }
    pub fn cross() -> Marker {
        Marker::cross()
    }
    pub fn triangle() -> Marker {
        Marker::triangle()
    }
    pub fn square() -> Marker {
        Marker::square()
    }
    pub fn dot() -> Marker {
        Marker::dot()
    }
    pub fn label(text: impl Into<String>) -> Marker {
        Marker::label(text)
    }
}

/// Quick access to ghost stone creation
pub mod ghosts {
    pub use crate::go_board::core::{Ghost, GhostKind, Stone, BLACK, WHITE};

    pub fn good(stone: Stone) -> Ghost {
        Ghost::good(stone)
    }
    pub fn bad(stone: Stone) -> Ghost {
        Ghost::bad(stone)
    }
    pub fn neutral(stone: Stone) -> Ghost {
        Ghost::neutral(stone)
    }
}

/// Quick access to line creation
pub mod lines {
    pub use crate::go_board::core::{Line, Pos};

    pub fn line(from: Pos, to: Pos) -> Line {
        Line::line(from, to)
    }
    pub fn arrow(from: Pos, to: Pos) -> Line {
        Line::arrow(from, to)
    }
}

// =============================================================================
// EXAMPLE USAGE DOCUMENTATION
// =============================================================================

/// # Go Board Usage Examples
///
/// ## Basic Usage
///
/// ```rust
/// use crate::go_board::*;
///
/// // Create a simple board
/// let board = Board::new()
///     .stone(Pos::new(3, 3), BLACK)
///     .stone(Pos::new(15, 15), WHITE)
///     .marker(Pos::new(4, 4), markers::circle().with_color(rgb(0xff0000)));
///
/// // Create an interactive view
/// let view = BoardView::new(board)
///     .on_click(|event| {
///         println!("Clicked at {:?}", event.pos);
///     });
/// ```
///
/// ## Advanced Usage
///
/// ```rust
/// use crate::go_board::*;
///
/// // Create a complex board with analysis
/// let board = Board::new()
///     .theme(themes::dark())
///     .stones([
///         (Pos::new(3, 3), BLACK),
///         (Pos::new(4, 4), WHITE),
///         (Pos::new(5, 5), BLACK),
///     ])
///     .ghosts([
///         (Pos::new(6, 6), ghosts::good(WHITE)),
///         (Pos::new(7, 7), ghosts::bad(BLACK)),
///     ])
///     .lines([
///         lines::arrow(Pos::new(3, 3), Pos::new(6, 6)),
///     ])
///     .last_move(Pos::new(5, 5));
///
/// let view = BoardView::new(board)
///     .coordinates(true)
///     .on_click(|event| {
///         println!("Clicked at {}{}",
///             ('A' as u8 + event.pos.x as u8) as char,
///             event.pos.y + 1);
///     })
///     .on_hover(|pos| {
///         if let Some(p) = pos {
///             println!("Hovering over {:?}", p);
///         }
///     });
/// ```
///
/// ## Auto-sizing Board
///
/// ```rust
/// use crate::go_board::*;
///
/// // Create a board that automatically fits in the given space
/// let bounded = BoundedBoard::new(400.0, 400.0)
///     .update(|board| {
///         board.stone(Pos::new(9, 9), BLACK)
///     });
///
/// let view = BoardView::new(bounded.inner().clone());
/// ```
///
/// ## Loading from SGF-like format
///
/// ```rust
/// use crate::go_board::{board::from_position_string, *};
///
/// let board = from_position_string((19, 19), "B[dd],W[pp],B[pd]");
/// let view = BoardView::new(board);
/// ```

#[cfg(test)]
mod integration_tests {
    use super::*;
    use gpui::*;

    #[test]
    fn test_new_api_basic_usage() {
        let board = Board::new()
            .stone(Pos::new(3, 3), BLACK)
            .stone(Pos::new(15, 15), WHITE);

        assert_eq!(board.stone_at(Pos::new(3, 3)), BLACK);
        assert_eq!(board.stone_at(Pos::new(15, 15)), WHITE);
        assert_eq!(board.stone_at(Pos::new(0, 0)), EMPTY);
    }

    #[test]
    fn test_new_api_builder_pattern() {
        let board = Board::new()
            .clear_all()
            .stones([(Pos::new(3, 3), BLACK), (Pos::new(4, 4), WHITE)])
            .markers([
                (Pos::new(1, 1), Marker::circle()),
                (Pos::new(2, 2), Marker::triangle()),
            ])
            .theme(themes::dark());

        assert_eq!(board.stone_at(Pos::new(3, 3)), BLACK);
        assert!(board.marker_at(Pos::new(1, 1)).is_some());
    }

    #[test]
    fn test_board_view_creation() {
        let board = Board::new().stone(Pos::new(9, 9), BLACK);

        let view = BoardView::new(board).coordinates(false).on_click(|_| {});

        // Field is private but we can test functionality
        assert_eq!(view.board().stone_at(Pos::new(9, 9)), BLACK);
    }

    #[test]
    fn test_convenience_functions() {
        let _simple = simple_board(|_| {});
        let _demo = demo_board_view();
        let _bounded = bounded_board_view(300.0, 300.0);
    }

    #[test]
    fn test_helper_modules() {
        let marker = Marker::circle().with_color(rgb(0xff0000));
        let ghost = ghosts::good(BLACK).with_alpha(0.8);
        let line = lines::arrow(Pos::new(0, 0), Pos::new(1, 1));
        let theme = themes::dark();

        // Just test that they compile and create valid objects
        assert!(matches!(marker, Marker::Circle { .. }));
        assert_eq!(ghost.stone, BLACK);
        assert!(matches!(line.style, LineStyle::Arrow { .. }));
        // Theme tests would depend on actual implementation
    }

    #[test]
    fn test_constants() {
        assert_eq!(constants::EMPTY, 0);
        assert_eq!(constants::BLACK, 1);
        assert_eq!(constants::WHITE, -1);
    }
}
