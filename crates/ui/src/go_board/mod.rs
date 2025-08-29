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

pub use board::{Board, BoundedBoard};
pub use render::Renderer;
pub use view::BoardView;

pub use core::{
    BoardData, Ghost, GhostKind, Heat, Line, LineStyle, Marker, NavEvent, Pos, PosEvent, Range,
    Selection, SelectionStyle, Stone, Territory, Theme, BLACK, EMPTY, WHITE,
};

// =============================================================================
// CONVENIENCE MODULES
// =============================================================================

pub mod constants {
    pub use crate::go_board::core::{BLACK, EMPTY, WHITE};
}

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

    pub fn with_assets() -> Theme {
        Theme::with_assets()
    }
}

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

pub mod lines {
    pub use crate::go_board::core::{Line, Pos};

    pub fn line(from: Pos, to: Pos) -> Line {
        Line::line(from, to)
    }

    pub fn arrow(from: Pos, to: Pos) -> Line {
        Line::arrow(from, to)
    }

    pub fn connection(from: Pos, to: Pos) -> Line {
        Line::connection(from, to)
    }

    pub fn analysis_arrow(from: Pos, to: Pos) -> Line {
        Line::analysis_arrow(from, to)
    }

    pub fn highlight_line(from: Pos, to: Pos) -> Line {
        Line::highlight_line(from, to)
    }

    pub fn direction_arrow(from: Pos, to: Pos) -> Line {
        Line::direction_arrow(from, to)
    }
}

pub mod factory {
    use crate::go_board::{Board, BoardView, Pos, BLACK, WHITE};

    pub fn empty_board() -> Board {
        Board::new()
    }

    pub fn teaching_board() -> Board {
        Board::with_size(9, 9).vertex_size(30.0)
    }

    pub fn demo_board() -> Board {
        Board::new()
            .stone(Pos::new(3, 3), BLACK)
            .stone(Pos::new(15, 15), WHITE)
            .stone(Pos::new(9, 9), BLACK)
            .last_move(Pos::new(9, 9))
    }

    pub fn simple_board_view() -> BoardView {
        BoardView::new(empty_board()).on_click(|event| {
            println!("Clicked at position {:?}", event.pos);
        })
    }
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
/// ## Auto-sizing Board
///
/// ```rust
/// use crate::go_board::*;
///
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
    use crate::go_board::view::{bounded_board_view, demo_board_view, simple_board};
    use gpui::{
        div, point, px, rgb, App, Context, Entity, InteractiveElement, IntoElement, Modifiers,
        MouseButton, ParentElement, Pixels, Point, Render, Styled, Window,
    };

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
        let marker = Marker::circle().with_color(rgb(0xff0000).into());
        let ghost = ghosts::good(BLACK).with_alpha(0.8);
        let line = lines::arrow(Pos::new(0, 0), Pos::new(1, 1));
        let theme = themes::dark();

        assert!(matches!(marker, Marker::Circle { .. }));
        assert_eq!(ghost.stone, BLACK);
        assert!(matches!(line.style, LineStyle::Arrow { .. }));
    }

    #[test]
    fn test_constants() {
        assert_eq!(constants::EMPTY, 0);
        assert_eq!(constants::BLACK, 1);
        assert_eq!(constants::WHITE, -1);
    }
}
