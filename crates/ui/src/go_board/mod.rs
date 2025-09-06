// =============================================================================
// GO BOARD MODULE - Simplified and consolidated
// =============================================================================

pub mod board;
pub mod core;
pub mod render;
pub mod view;

// =============================================================================
// MAIN EXPORTS
// =============================================================================

pub use board::Board;
pub use render::Renderer;
pub use view::BoardView;

pub use core::{
    BoardData, Ghost, Heat, Line, Marker, NavEvent, Pos, PosEvent, Range, Selection, Stone,
    Territory, Theme, BLACK, EMPTY, WHITE,
};

// =============================================================================
// SIMPLIFIED FACTORY FUNCTIONS
// =============================================================================

/// Create an empty board
pub fn empty_board() -> Board {
    Board::new()
}

/// Create a teaching board (9x9)
pub fn teaching_board() -> Board {
    Board::with_size(9, 9)
}

/// Create a simple interactive board view
pub fn interactive_board<F>(click_handler: F) -> BoardView
where
    F: Fn(PosEvent) + 'static,
{
    BoardView::new(Board::new()).on_click(click_handler)
}

// =============================================================================
// USAGE EXAMPLES
// =============================================================================

/// # Go Board Usage Examples
///
/// ## Basic Usage
///
/// ```rust
/// use crate::go_board::*;
///
/// let board = Board::new()
///     .stone(Pos::new(3, 3), BLACK)
///     .stone(Pos::new(15, 15), WHITE)
///     .marker(Pos::new(4, 4), markers::circle().with_color(rgb(0xff0000)));
///
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
/// ## Responsive Board
///
/// ```rust
/// use crate::go_board::*;
///
/// // Board that automatically calculates appropriate size
/// let responsive_view = BoardView::new(Board::new())
///     .stone(Pos::new(9, 9), BLACK);
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
    use crate::go_board::view::interactive_board;
    use gpui::rgb;

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
            .stone(Pos::new(3, 3), BLACK)
            .stone(Pos::new(4, 4), WHITE)
            .marker(Pos::new(1, 1), Marker::circle())
            .marker(Pos::new(2, 2), Marker::triangle())
            .theme(Theme::dark());

        assert_eq!(board.stone_at(Pos::new(3, 3)), BLACK);
        assert!(board.marker_at(Pos::new(1, 1)).is_some());
    }

    #[test]
    fn test_board_view_creation() {
        let board = Board::new().stone(Pos::new(9, 9), BLACK);
        let view = BoardView::new(board).coordinates(false).on_click(|_| {});
        assert_eq!(view.board().stone_at(Pos::new(9, 9)), BLACK);
    }

    #[test]
    fn test_convenience_functions() {
        let _simple = interactive_board(|_| {});
        let _empty = empty_board();
        let _teaching = teaching_board();
    }

    #[test]
    fn test_direct_api_usage() {
        let marker = Marker::circle().with_color(rgb(0xff0000).into());
        let ghost = Ghost::good(BLACK).with_alpha(0.8);
        let line = Line::arrow(Pos::new(0, 0), Pos::new(1, 1));

        assert!(matches!(marker, Marker::Circle { .. }));
        assert_eq!(ghost.stone, BLACK);
        assert!(line.is_arrow);
    }

    #[test]
    fn test_constants() {
        assert_eq!(EMPTY, 0);
        assert_eq!(BLACK, 1);
        assert_eq!(WHITE, -1);
    }
}
