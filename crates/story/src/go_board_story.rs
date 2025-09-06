use gpui::{
    px, rgb, App, AppContext, Context, Entity, FocusHandle, Focusable, InteractiveElement,
    IntoElement, ParentElement as _, Render, Styled as _, Window,
};

use gpui_component::{
    go_board::{
        core::{Ghost, Heat, Line, Marker, Pos, Theme, BLACK, WHITE},
        Board, BoardView,
    },
    h_flex, v_flex, ActiveTheme,
};

use crate::{section, Story};

pub struct GoBoardStory {
    focus_handle: gpui::FocusHandle,
    board_19x19: Entity<BoardView>,
    board_13x13: Entity<BoardView>,
    board_9x9: Entity<BoardView>,
    custom_theme_board: Entity<BoardView>,
    dark_theme_board: Entity<BoardView>,
    minimalist_theme_board: Entity<BoardView>,
    high_contrast_board: Entity<BoardView>,
    textured_board: Entity<BoardView>,
    asset_board: Entity<BoardView>,
    stone_variation_board: Entity<BoardView>,
    coordinate_board: Entity<BoardView>,
    stone_board: Entity<BoardView>,
    fuzzy_stone_board: Entity<BoardView>,
    marker_board: Entity<BoardView>,
    selection_board: Entity<BoardView>,
    paint_overlay_board: Entity<BoardView>,
    heat_overlay_board: Entity<BoardView>,
    ghost_stone_board: Entity<BoardView>,
    line_board: Entity<BoardView>,
    interactive_board: Entity<BoardView>,
    interactive_asset_board: Entity<BoardView>,
    bounded_small_board: Entity<BoardView>,
    bounded_medium_board: Entity<BoardView>,
    bounded_large_board: Entity<BoardView>,
    bounded_constrained_board: Entity<BoardView>,
    partial_board_center: Entity<BoardView>,
    partial_board_corner: Entity<BoardView>,
    partial_board_edge: Entity<BoardView>,
    efficient_update_demo: Entity<BoardView>,
}

impl GoBoardStory {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            board_19x19: cx.new(|_| BoardView::new(Board::new()).coordinates(true)),
            board_13x13: cx.new(|_| BoardView::new(Board::with_size(13, 13)).coordinates(true)),
            board_9x9: cx.new(|_| BoardView::new(Board::with_size(9, 9)).coordinates(true)),
            custom_theme_board: cx.new(|_| {
                // Create a custom theme using the new Theme system
                let mut custom_theme = Theme::default();
                custom_theme.background = gpui::rgb(0x8B7355).into(); // Darker wood
                custom_theme.grid_lines = gpui::rgb(0x2c2c2c).into(); // Dark gray lines
                custom_theme.grid_width = 1.5; // Thicker lines
                custom_theme.black_stone = gpui::rgb(0x000000).into(); // Pure black stones
                custom_theme.white_stone = gpui::rgb(0xffffff).into(); // Pure white stones
                custom_theme.coordinates = gpui::rgb(0x654321).into(); // Dark brown coordinates
                custom_theme.coord_size = 12.0;

                BoardView::new(Board::with_size(9, 9).theme(custom_theme))
            }),
            dark_theme_board: cx.new(|_| {
                BoardView::new(Board::with_size(9, 9).theme(Theme::dark())).coordinates(true)
            }),
            minimalist_theme_board: cx.new(|_| {
                BoardView::new(Board::with_size(9, 9).theme(Theme::minimal())).coordinates(true)
            }),
            high_contrast_board: cx.new(|_| {
                BoardView::new(Board::with_size(9, 9).theme(Theme::high_contrast()))
                    .coordinates(true)
            }),
            textured_board: cx.new(|_| {
                // Create a board with default theme
                let board = Board::with_size(9, 9);

                // Add stones using the new API - manually setting each position
                let board = board
                    .stone(Pos::new(3, 0), BLACK)
                    .stone(Pos::new(5, 0), WHITE)
                    .stone(Pos::new(1, 1), BLACK)
                    .stone(Pos::new(7, 1), WHITE)
                    .stone(Pos::new(2, 2), BLACK)
                    .stone(Pos::new(6, 2), WHITE)
                    .stone(Pos::new(0, 3), BLACK)
                    .stone(Pos::new(8, 3), WHITE)
                    .stone(Pos::new(0, 5), WHITE)
                    .stone(Pos::new(8, 5), BLACK)
                    .stone(Pos::new(2, 6), WHITE)
                    .stone(Pos::new(6, 6), BLACK)
                    .stone(Pos::new(1, 7), WHITE)
                    .stone(Pos::new(7, 7), BLACK)
                    .stone(Pos::new(3, 8), WHITE)
                    .stone(Pos::new(5, 8), BLACK);

                BoardView::new(board).coordinates(true)
            }),
            asset_board: cx.new(|_| {
                // Use assets for board background and stones
                let board = Board::with_size(9, 9).theme(Theme::default());

                // Add a sample game pattern using the new API
                let board = board
                    .stone(Pos::new(3, 0), BLACK)
                    .stone(Pos::new(5, 0), WHITE)
                    .stone(Pos::new(1, 1), BLACK)
                    .stone(Pos::new(7, 1), WHITE)
                    .stone(Pos::new(2, 2), BLACK)
                    .stone(Pos::new(6, 2), WHITE)
                    .stone(Pos::new(0, 3), BLACK)
                    .stone(Pos::new(3, 3), BLACK)
                    .stone(Pos::new(5, 3), WHITE)
                    .stone(Pos::new(8, 3), WHITE)
                    .stone(Pos::new(4, 4), BLACK)
                    .stone(Pos::new(0, 5), WHITE)
                    .stone(Pos::new(3, 5), WHITE)
                    .stone(Pos::new(5, 5), BLACK)
                    .stone(Pos::new(8, 5), BLACK)
                    .stone(Pos::new(2, 6), WHITE)
                    .stone(Pos::new(6, 6), BLACK)
                    .stone(Pos::new(1, 7), WHITE)
                    .stone(Pos::new(7, 7), BLACK)
                    .stone(Pos::new(3, 8), WHITE)
                    .stone(Pos::new(5, 8), BLACK)
                    // Add some ghost stones to demonstrate asset tinting
                    .ghost(Pos::new(4, 3), Ghost::good(WHITE))
                    .ghost(Pos::new(4, 5), Ghost::bad(BLACK))
                    .ghost(Pos::new(7, 4), Ghost::neutral(WHITE))
                    .last_move(Pos::new(5, 8));

                BoardView::new(board)
            }),
            stone_variation_board: cx.new(|_| {
                // Stone variation demo disabled (no external variation textures)
                let board = Board::with_size(9, 9);

                // Add many stones to demonstrate variation using the new API
                // Create a checkerboard pattern manually
                let mut board = board;
                for y in 0..9 {
                    for x in 0..9 {
                        if (x + y) % 2 == 0 && (x != 4 || y != 4) {
                            // Skip center
                            board = board.stone(Pos::new(x, y), BLACK);
                        } else if (x + y) % 2 == 1 {
                            board = board.stone(Pos::new(x, y), WHITE);
                        }
                    }
                }
                BoardView::new(board)
            }),
            coordinate_board: cx.new(|_| {
                let board = Board::with_size(9, 9);
                // Add stones to demonstrate coordinates
                let board = board
                    .stone(Pos::new(3, 0), BLACK)
                    .stone(Pos::new(5, 0), WHITE)
                    .stone(Pos::new(1, 1), BLACK)
                    .stone(Pos::new(7, 1), WHITE)
                    .stone(Pos::new(2, 2), BLACK)
                    .stone(Pos::new(6, 2), WHITE)
                    .stone(Pos::new(0, 3), BLACK)
                    .stone(Pos::new(8, 3), WHITE)
                    .stone(Pos::new(0, 5), WHITE)
                    .stone(Pos::new(8, 5), BLACK)
                    .stone(Pos::new(2, 6), WHITE)
                    .stone(Pos::new(6, 6), BLACK)
                    .stone(Pos::new(1, 7), WHITE)
                    .stone(Pos::new(7, 7), BLACK)
                    .stone(Pos::new(3, 8), WHITE)
                    .stone(Pos::new(5, 8), BLACK);
                BoardView::new(board).coordinates(true)
            }),
            stone_board: cx.new(|_| {
                let board = Board::with_size(9, 9);

                // Create a simple game pattern using the new API
                let board = board
                    .stone(Pos::new(3, 1), BLACK)
                    .stone(Pos::new(5, 1), WHITE)
                    .stone(Pos::new(2, 2), BLACK)
                    .stone(Pos::new(6, 2), WHITE)
                    .stone(Pos::new(1, 3), BLACK)
                    .stone(Pos::new(3, 3), BLACK)
                    .stone(Pos::new(5, 3), WHITE)
                    .stone(Pos::new(7, 3), WHITE)
                    .stone(Pos::new(1, 5), WHITE)
                    .stone(Pos::new(3, 5), WHITE)
                    .stone(Pos::new(5, 5), BLACK)
                    .stone(Pos::new(7, 5), BLACK)
                    .stone(Pos::new(2, 6), WHITE)
                    .stone(Pos::new(6, 6), BLACK)
                    .stone(Pos::new(3, 7), WHITE)
                    .stone(Pos::new(5, 7), BLACK);
                BoardView::new(board).coordinates(true)
            }),
            fuzzy_stone_board: cx.new(|_| {
                let board = Board::with_size(9, 9);

                // Create the same pattern as stone_board using the new API
                let board = board
                    .stone(Pos::new(3, 1), BLACK)
                    .stone(Pos::new(5, 1), WHITE)
                    .stone(Pos::new(2, 2), BLACK)
                    .stone(Pos::new(6, 2), WHITE)
                    .stone(Pos::new(1, 3), BLACK)
                    .stone(Pos::new(3, 3), BLACK)
                    .stone(Pos::new(5, 3), WHITE)
                    .stone(Pos::new(7, 3), WHITE)
                    .stone(Pos::new(1, 5), WHITE)
                    .stone(Pos::new(3, 5), WHITE)
                    .stone(Pos::new(5, 5), BLACK)
                    .stone(Pos::new(7, 5), BLACK)
                    .stone(Pos::new(2, 6), WHITE)
                    .stone(Pos::new(6, 6), BLACK)
                    .stone(Pos::new(3, 7), WHITE)
                    .stone(Pos::new(5, 7), BLACK);

                // Note: Fuzzy positioning and visual variation features are not available
                // in the new simplified API. The new API focuses on core functionality.
                BoardView::new(board).coordinates(true)
            }),
            marker_board: cx.new(|_| {
                let board = Board::with_size(9, 9);

                // Add markers using the new API - manually setting each position
                let board = board
                    // Row 1: Basic marker types
                    .marker(Pos::new(1, 1), Marker::circle())
                    .marker(Pos::new(2, 1), Marker::cross())
                    .marker(Pos::new(3, 1), Marker::triangle())
                    .marker(Pos::new(4, 1), Marker::square())
                    .marker(Pos::new(5, 1), Marker::dot())
                    // Row 2: Colored markers
                    .marker(
                        Pos::new(1, 2),
                        Marker::circle().with_color(gpui::rgb(0xff0000).into()),
                    )
                    .marker(
                        Pos::new(2, 2),
                        Marker::triangle().with_color(gpui::rgb(0x0000ff).into()),
                    )
                    .marker(
                        Pos::new(3, 2),
                        Marker::triangle().with_color(gpui::rgb(0x00ff00).into()),
                    )
                    .marker(
                        Pos::new(4, 2),
                        Marker::square().with_color(gpui::rgb(0xff0000).into()),
                    )
                    .marker(
                        Pos::new(5, 2),
                        Marker::dot().with_color(gpui::rgb(0x0000ff).into()),
                    )
                    // Row 3: Different colors
                    .marker(
                        Pos::new(1, 3),
                        Marker::circle().with_color(gpui::rgb(0xff0000).into()),
                    )
                    .marker(
                        Pos::new(2, 3),
                        Marker::square().with_color(gpui::rgb(0x00ff00).into()),
                    )
                    .marker(
                        Pos::new(3, 3),
                        Marker::triangle().with_color(gpui::rgb(0x0000ff).into()),
                    )
                    .marker(
                        Pos::new(4, 3),
                        Marker::square().with_color(gpui::rgb(0xffff00).into()),
                    )
                    .marker(
                        Pos::new(5, 3),
                        Marker::dot().with_color(gpui::rgb(0xff00ff).into()),
                    )
                    // Row 4: Label markers
                    .marker(Pos::new(1, 4), Marker::label("A"))
                    .marker(
                        Pos::new(2, 4),
                        Marker::label("B").with_color(gpui::rgb(0xff0000).into()),
                    )
                    .marker(
                        Pos::new(3, 4),
                        Marker::label("1").with_color(gpui::rgb(0x0000ff).into()),
                    )
                    .marker(Pos::new(4, 4), Marker::label("2"))
                    // Row 5: Loader markers (animated dots)
                    .marker(
                        Pos::new(1, 5),
                        Marker::dot().with_color(gpui::rgb(0xff8000).into()),
                    )
                    .marker(
                        Pos::new(2, 5),
                        Marker::dot().with_color(gpui::rgb(0x8000ff).into()),
                    )
                    .marker(
                        Pos::new(3, 5),
                        Marker::dot().with_color(gpui::rgb(0x00ff80).into()),
                    )
                    .marker(
                        Pos::new(4, 5),
                        Marker::dot().with_color(gpui::rgb(0xff0080).into()),
                    )
                    // Row 6: Different colored markers (z-index not supported in new API)
                    .marker(
                        Pos::new(1, 6),
                        Marker::circle().with_color(gpui::rgb(0x0000ff).into()),
                    )
                    .marker(
                        Pos::new(2, 6),
                        Marker::square().with_color(gpui::rgb(0x00ff00).into()),
                    )
                    .marker(
                        Pos::new(3, 6),
                        Marker::cross().with_color(gpui::rgb(0xff0000).into()),
                    );

                BoardView::new(board).coordinates(true)
            }),
            selection_board: cx.new(|_| {
                let board = Board::with_size(9, 9);

                // Create some stones for context using the new API
                let board = board
                    .stone(Pos::new(1, 1), BLACK)
                    .stone(Pos::new(3, 1), WHITE)
                    .stone(Pos::new(4, 2), BLACK)
                    .stone(Pos::new(3, 3), WHITE)
                    .stone(Pos::new(5, 3), WHITE)
                    .stone(Pos::new(3, 5), BLACK)
                    .stone(Pos::new(6, 5), WHITE)
                    // Selected vertices (highlighted in blue)
                    .select(Pos::new(2, 2))
                    .select(Pos::new(6, 6))
                    // Dimmed vertices (reduced opacity) - Note: Dimming not directly supported in new API
                    // Directional selection indicators not supported in new API
                    .coordinates(true);

                BoardView::new(board).coordinates(true)
            }),
            paint_overlay_board: cx.new(|_| {
                let board = Board::with_size(9, 9);

                // Create some stones for context using the new API
                let board = board
                    .stone(Pos::new(1, 1), BLACK)
                    .stone(Pos::new(3, 1), WHITE)
                    .stone(Pos::new(2, 2), BLACK)
                    .stone(Pos::new(4, 2), WHITE)
                    .stone(Pos::new(3, 3), BLACK)
                    .stone(Pos::new(1, 3), WHITE)
                    .stone(Pos::new(4, 4), BLACK)
                    .stone(Pos::new(2, 4), WHITE)
                    .stone(Pos::new(3, 5), WHITE)
                    .stone(Pos::new(5, 5), BLACK)
                    .stone(Pos::new(4, 6), WHITE)
                    .stone(Pos::new(6, 6), BLACK)
                    // Note: Paint overlay functionality is not available in the new simplified API
                    // The new API focuses on core board functionality without advanced overlays
                    .coordinates(true);

                BoardView::new(board)
            }),
            heat_overlay_board: cx.new(|_| {
                let board = Board::with_size(9, 9);

                // Create some stones for context using the new API
                let board = board
                    .stone(Pos::new(3, 0), BLACK)
                    .stone(Pos::new(5, 0), WHITE)
                    .stone(Pos::new(1, 1), BLACK)
                    .stone(Pos::new(7, 1), WHITE)
                    .stone(Pos::new(3, 2), BLACK)
                    .stone(Pos::new(5, 2), WHITE)
                    .stone(Pos::new(0, 3), BLACK)
                    .stone(Pos::new(8, 3), WHITE)
                    .stone(Pos::new(0, 5), WHITE)
                    .stone(Pos::new(8, 5), BLACK)
                    .stone(Pos::new(3, 6), WHITE)
                    .stone(Pos::new(5, 6), BLACK)
                    .stone(Pos::new(1, 7), WHITE)
                    .stone(Pos::new(7, 7), BLACK)
                    .stone(Pos::new(3, 8), WHITE)
                    .stone(Pos::new(5, 8), BLACK)
                    // Add heat overlay to demonstrate influence visualization
                    .heat(Pos::new(0, 0), Heat::new(1).with_label("1"))
                    .heat(Pos::new(1, 0), Heat::new(2).with_label("2"))
                    .heat(Pos::new(2, 0), Heat::new(3).with_label("3"))
                    .heat(Pos::new(4, 0), Heat::new(4).with_label("4"))
                    .heat(Pos::new(6, 0), Heat::new(5).with_label("5"))
                    .heat(Pos::new(7, 0), Heat::new(6).with_label("6"))
                    .heat(Pos::new(8, 0), Heat::new(7).with_label("7"))
                    .heat(Pos::new(0, 1), Heat::new(8).with_label("8"))
                    .heat(Pos::new(2, 1), Heat::new(9).with_label("9"))
                    .heat(Pos::new(4, 1), Heat::new(6))
                    .heat(Pos::new(5, 1), Heat::new(4))
                    .heat(Pos::new(6, 1), Heat::new(3))
                    .heat(Pos::new(8, 1), Heat::new(1))
                    .heat(Pos::new(0, 2), Heat::new(7))
                    .heat(Pos::new(1, 2), Heat::new(5))
                    .heat(Pos::new(4, 2), Heat::new(2))
                    .heat(Pos::new(6, 2), Heat::new(8))
                    .heat(Pos::new(7, 2), Heat::new(4))
                    .heat(Pos::new(8, 2), Heat::new(2))
                    // Add a few more strategic positions
                    .heat(Pos::new(2, 4), Heat::new(6))
                    .heat(Pos::new(4, 4), Heat::new(9).with_label("★"))
                    .heat(Pos::new(6, 4), Heat::new(5))
                    .heat(Pos::new(1, 8), Heat::new(3))
                    .heat(Pos::new(2, 8), Heat::new(4))
                    .heat(Pos::new(4, 8), Heat::new(7))
                    .heat(Pos::new(6, 8), Heat::new(2))
                    .heat(Pos::new(7, 8), Heat::new(5))
                    .heat(Pos::new(8, 8), Heat::new(1))
                    .coordinates(true);

                BoardView::new(board)
            }),
            ghost_stone_board: cx.new(|_| {
                let board = Board::with_size(9, 9);

                // Create some stones for context using the new API
                let board = board
                    .stone(Pos::new(3, 0), BLACK)
                    .stone(Pos::new(5, 0), WHITE)
                    .stone(Pos::new(1, 1), BLACK)
                    .stone(Pos::new(7, 1), WHITE)
                    .stone(Pos::new(0, 3), BLACK)
                    .stone(Pos::new(8, 3), WHITE)
                    .stone(Pos::new(0, 5), WHITE)
                    .stone(Pos::new(8, 5), BLACK)
                    .stone(Pos::new(1, 7), WHITE)
                    .stone(Pos::new(7, 7), BLACK)
                    .stone(Pos::new(3, 8), WHITE)
                    .stone(Pos::new(5, 8), BLACK)
                    // Note: Ghost stone functionality is not available in the new simplified API
                    // The new API focuses on core board functionality without advanced overlays
                    .coordinates(true);

                BoardView::new(board)
            }),
            line_board: cx.new(|_| {
                let board = Board::with_size(9, 9);

                // Create some stones for context using the new API
                let board = board
                    .stone(Pos::new(3, 0), BLACK)
                    .stone(Pos::new(5, 0), WHITE)
                    .stone(Pos::new(1, 1), BLACK)
                    .stone(Pos::new(7, 1), WHITE)
                    .stone(Pos::new(0, 3), BLACK)
                    .stone(Pos::new(8, 3), WHITE)
                    .stone(Pos::new(0, 5), WHITE)
                    .stone(Pos::new(8, 5), BLACK)
                    .stone(Pos::new(1, 7), WHITE)
                    .stone(Pos::new(7, 7), BLACK)
                    .stone(Pos::new(3, 8), WHITE)
                    .stone(Pos::new(5, 8), BLACK);

                // Create line demonstrations
                let lines = vec![
                    // Simple connection lines (gray)
                    Line::line(Pos::new(3, 0), Pos::new(5, 0)), // Horizontal connection
                    Line::line(Pos::new(1, 1), Pos::new(1, 4)), // Vertical connection
                    Line::line(Pos::new(2, 2), Pos::new(6, 6)), // Diagonal connection
                    // Analysis arrows (dark)
                    Line::arrow(Pos::new(0, 3), Pos::new(3, 6)), // Analysis direction
                    Line::arrow(Pos::new(8, 3), Pos::new(5, 6)), // Analysis direction
                    // Connection arrows between stones
                    Line::arrow(Pos::new(3, 8), Pos::new(5, 8)), // Horizontal arrow
                    Line::arrow(Pos::new(7, 7), Pos::new(7, 4)), // Vertical arrow
                    // Strategic analysis arrows
                    Line::arrow(Pos::new(4, 4), Pos::new(6, 2)), // Up-right analysis
                    Line::arrow(Pos::new(4, 4), Pos::new(2, 6)), // Down-left analysis
                    // Multiple line types demonstration
                    Line::line(Pos::new(0, 7), Pos::new(2, 7)), // Short horizontal connection
                    Line::line(Pos::new(6, 1), Pos::new(8, 1)), // Short horizontal connection
                    Line::arrow(Pos::new(1, 0), Pos::new(1, 2)), // Short vertical arrow
                    Line::arrow(Pos::new(7, 8), Pos::new(7, 6)), // Short vertical arrow
                    // Highlight lines for special connections
                    Line::line(Pos::new(2, 3), Pos::new(6, 3))
                        .with_color(rgb(0x0066cc))
                        .with_width(3.0), // Blue highlight
                    Line::arrow(Pos::new(4, 1), Pos::new(4, 3))
                        .with_color(rgb(0xcc3300))
                        .with_width(2.5), // Red direction
                ];

                let mut board = board;
                for line in lines {
                    board = board.line(line);
                }

                BoardView::new(board)
            }),
            interactive_board: cx.new(|_| {
                let board = Board::with_size(9, 9);
                BoardView::new(board).coordinates(true)
            }),
            interactive_asset_board: cx.new(|_| {
                // Create an interactive board using assets for stones and background
                let board = Board::with_size(9, 9)
                    .theme(Theme::with_assets())
                    .stone(Pos::new(2, 2), BLACK)
                    .stone(Pos::new(6, 6), WHITE)
                    .stone(Pos::new(4, 4), BLACK)
                    .stone(Pos::new(3, 5), WHITE)
                    .ghost(Pos::new(5, 5), Ghost::good(BLACK))
                    .ghost(Pos::new(1, 1), Ghost::bad(WHITE))
                    .last_move(Pos::new(4, 4));
                BoardView::new(board).coordinates(true)
            }),
            bounded_small_board: cx.new(|_| {
                // Small bounded board - 9x9 in 150x150 space
                let mut board = Board::with_size(9, 9);
                let stones = vec![
                    (Pos::new(1, 0), BLACK),
                    (Pos::new(7, 0), WHITE),
                    (Pos::new(0, 1), BLACK),
                    (Pos::new(2, 1), BLACK),
                    (Pos::new(6, 1), WHITE),
                    (Pos::new(8, 1), WHITE),
                    (Pos::new(1, 2), BLACK),
                    (Pos::new(7, 2), WHITE),
                    (Pos::new(4, 4), BLACK),
                    (Pos::new(1, 6), BLACK),
                    (Pos::new(7, 6), WHITE),
                    (Pos::new(0, 7), BLACK),
                    (Pos::new(2, 7), BLACK),
                    (Pos::new(6, 7), WHITE),
                    (Pos::new(8, 7), WHITE),
                    (Pos::new(1, 8), BLACK),
                    (Pos::new(7, 8), WHITE),
                ];

                for (pos, stone) in stones {
                    board = board.stone(pos, stone);
                }

                BoardView::with_size(board, 150.0, 150.0)
            }),
            bounded_medium_board: cx.new(|_| {
                // Medium bounded board - 13x13 in 250x250 space
                let mut board = Board::with_size(13, 13);
                let stones = vec![
                    (Pos::new(3, 1), BLACK),
                    (Pos::new(9, 1), WHITE),
                    (Pos::new(1, 3), BLACK),
                    (Pos::new(11, 3), WHITE),
                    (Pos::new(6, 6), BLACK),
                    (Pos::new(1, 9), WHITE),
                    (Pos::new(11, 9), BLACK),
                    (Pos::new(3, 11), WHITE),
                    (Pos::new(9, 11), BLACK),
                ];

                for (pos, stone) in stones {
                    board = board.stone(pos, stone);
                }

                BoardView::with_size(board, 250.0, 250.0).coordinates(true)
            }),
            bounded_large_board: cx.new(|_| {
                // Large bounded board - 19x19 in 380x380 space
                BoardView::with_size(Board::with_size(19, 19), 380.0, 380.0)
            }),
            bounded_constrained_board: cx.new(|_| {
                // Constrained aspect ratio - 19x19 in 200x400 space (height-constrained)
                let mut board = Board::with_size(19, 19);

                // Add some stones to show the scaling effect
                let sign_map = vec![
                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, -1],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1],
                ];

                // Apply the sign map using the new API
                for (y, row) in sign_map.iter().enumerate() {
                    for (x, &sign) in row.iter().enumerate() {
                        if sign != 0 {
                            let stone = if sign > 0 { BLACK } else { WHITE };
                            board = board.stone(Pos::new(x, y), stone);
                        }
                    }
                }

                BoardView::with_size(board, 200.0, 400.0)
            }),
            partial_board_center: cx.new(|_| {
                // Partial board showing center area of a 19x19 board
                let mut board = Board::with_size(5, 5); // 5x5 visible area

                // Create a 5x5 sign map for just the visible area (coordinates 0-4)
                let sign_map = vec![
                    vec![0, 0, 0, 0, 0],
                    vec![0, -1, 0, 1, 0],
                    vec![0, 0, 1, 0, 0],
                    vec![0, 1, 0, -1, 0],
                    vec![0, 0, 0, 0, 0],
                ];

                // Apply the sign map using the new API
                for (y, row) in sign_map.iter().enumerate() {
                    for (x, &sign) in row.iter().enumerate() {
                        if sign != 0 {
                            let stone = if sign > 0 { BLACK } else { WHITE };
                            board = board.stone(Pos::new(x, y), stone);
                        }
                    }
                }

                BoardView::new(board)
            }),
            partial_board_corner: cx.new(|_| {
                // Partial board showing corner area
                let mut board = Board::with_size(7, 7); // 7x7 visible area

                // Create a 7x7 sign map showing typical corner pattern (coordinates 0-6)
                let sign_map = vec![
                    vec![0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 1, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, -1, 0, 0],
                    vec![0, 0, 0, -1, 0, 1, 0],
                    vec![0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0],
                ];

                // Apply the sign map using the new API
                for (y, row) in sign_map.iter().enumerate() {
                    for (x, &sign) in row.iter().enumerate() {
                        if sign != 0 {
                            let stone = if sign > 0 { BLACK } else { WHITE };
                            board = board.stone(Pos::new(x, y), stone);
                        }
                    }
                }

                BoardView::new(board)
            }),
            partial_board_edge: cx.new(|_| {
                // Partial board showing side edge
                let mut board = Board::with_size(19, 3); // 19x3 slice

                // Create a 19x3 sign map for edge play (coordinates 0-18 x 0-2)
                let sign_map = vec![
                    vec![0, 0, 1, 0, 0, -1, 0, 0, 1, 0, 0, -1, 0, 0, 1, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 1, 0, -1, 0, 1, 0, -1, 0, 1, 0, -1, 0, 1, 0, 0, 0, 0, 0],
                ];

                // Apply the sign map using the new API
                for (y, row) in sign_map.iter().enumerate() {
                    for (x, &sign) in row.iter().enumerate() {
                        if sign != 0 {
                            let stone = if sign > 0 { BLACK } else { WHITE };
                            board = board.stone(Pos::new(x, y), stone);
                        }
                    }
                }

                BoardView::new(board)
            }),
            // Demonstration of efficient differential updates
            efficient_update_demo: cx.new(|_| {
                let mut board = Board::with_size(9, 9);

                // Demonstrate efficient bulk updates
                let initial_stones = vec![
                    (Pos::new(2, 2), BLACK),
                    (Pos::new(6, 2), WHITE),
                    (Pos::new(2, 6), WHITE),
                    (Pos::new(6, 6), BLACK),
                    (Pos::new(4, 4), BLACK), // Center stone
                ];

                // Apply stones using the new API
                for (pos, stone) in initial_stones {
                    board = board.stone(pos, stone);
                }

                BoardView::new(board)
            }),
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn name(&self) -> &'static str {
        "Go Board"
    }
}

impl Focusable for GoBoardStory {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Story for GoBoardStory {
    fn title() -> &'static str {
        "Go Board"
    }

    fn description() -> &'static str {
        "A Go board widget component for displaying game boards with grid lines, various sizes, and custom themes."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render + Focusable> {
        Self::view(window, cx)
    }
}

impl Render for GoBoardStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        v_flex()
            .id("go-board-story")
            .size_full()
            .p_4()
            .gap_6()
            .bg(theme.background)
            .track_focus(&self.focus_handle)
            .child(
                section("Standard Boards").child(
                    v_flex()
                        .gap_6()
                        .child(
                            h_flex()
                                .gap_6()
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("19x19 Board (Standard)")
                                        .child(self.board_19x19.clone())
                                        .border_1()
                                        .border_color(gpui::rgb(0xcccccc))
                                        .p_2()
                                        .max_w(px(500.0)), // Add max width constraint
                                )
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("13x13 Board")
                                        .child(self.board_13x13.clone())
                                        .border_1()
                                        .border_color(gpui::rgb(0xcccccc))
                                        .p_2()
                                        .max_w(px(400.0)), // Add max width constraint
                                )
                        )
                        .child(
                            h_flex()
                                .justify_center()
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("9x9 Board")
                                        .child(self.board_9x9.clone())
                                        .border_1()
                                        .border_color(gpui::rgb(0xcccccc))
                                        .p_2()
                                        .max_w(px(300.0)), // Add max width constraint
                                )
                        ),
                ),
            )
            .child(
                section("Theming System").child(
                    v_flex()
                        .gap_4()
                        .child("BoardTheme provides comprehensive theming with CSS custom property support")
                        .child(
                            h_flex()
                                .gap_6()
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("Custom Theme (Builder Pattern)")
                                        .child(self.custom_theme_board.clone()),
                                )
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("Dark Theme")
                                        .child(self.dark_theme_board.clone()),
                                ),
                        )
                        .child(
                            h_flex()
                                .gap_6()
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("Minimalist Theme")
                                        .child(self.minimalist_theme_board.clone()),
                                )
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("High Contrast (Accessibility)")
                                        .child(self.high_contrast_board.clone()),
                                ),
                        ),
                ),
            )
            .child(
                section("Texture and Asset Support").child(
                    v_flex()
                        .gap_4()
                        .child("Advanced texture loading and stone variation system")
                        .child(
                            h_flex()
                                .gap_6()
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("Textured Board")
                                        .child("Board texture + custom stone images")
                                        .child(self.textured_board.clone()),
                                )
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("Asset Board")
                                        .child("Using assets for stones and board background")
                                        .child(self.asset_board.clone()),
                                ),
                        )
                        .child(
                            h_flex()
                                .gap_6()
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("Stone Variations")
                                        .child("Stone Variations (disabled - using default theme)")
                                        .child(self.stone_variation_board.clone()),
                                ),
                        ),
                ),
            )
            .child(
                section("Legacy Theme Support").child(
                    v_flex()
                        .gap_2()
                        .child("Backward compatibility with GridTheme and StoneTheme")
                        .child("(Same visual as Custom Theme above)")
                ),
            )
            .child(
                section("Coordinate Labels").child(
                    v_flex()
                        .gap_2()
                        .child("13x13 Board with Coordinate Labels")
                        .child(self.coordinate_board.clone()),
                ),
            )
            .child(
                section("Stone Rendering").child(
                    h_flex()
                        .gap_6()
                        .child(
                            v_flex()
                                .gap_2()
                                .child("9x9 Board with Basic Stones")
                                .child(self.stone_board.clone()),
                        )
                        .child(
                            v_flex()
                                .gap_2()
                                .child("9x9 Board with Fuzzy Positioning")
                                .child(self.fuzzy_stone_board.clone()),
                        ),
                ),
            )
            .child(
                section("Marker Types").child(
                    v_flex()
                        .gap_2()
                        .child("9x9 Board with Different Marker Types")
                        .child("Row 1: Basic shapes, Row 2: Colored markers, Row 3: Different colors, Row 4: Labels, Row 5: Loader dots, Row 6: Colored markers")
                        .child(self.marker_board.clone()),
                ),
            )
            .child(
                section("Vertex Selection").child(
                    v_flex()
                        .gap_2()
                        .child("9x9 Board with Vertex Selection and Directional Indicators")
                        .child("Blue circles: Selected vertices, Dimmed areas: Reduced opacity vertices")
                        .child("Red/Green/Orange/Purple: Directional selection indicators (left/right/top/bottom)")
                        .child(self.selection_board.clone()),
                ),
            )
            .child(
                section("Paint Overlay - Territory Analysis").child(
                    v_flex()
                        .gap_2()
                        .child("9x9 Board with Paint Map Overlay for Territory Visualization")
                        .child("Blue regions: Black territory (positive values), Gray regions: White territory (negative values)")
                        .child("Intensity varies with paint value strength (-1.0 to 1.0)")
                        .child(self.paint_overlay_board.clone()),
                ),
            )
            .child(
                section("Heat Map - Influence Visualization").child(
                    v_flex()
                        .gap_2()
                        .child("9x9 Board with Heat Map for Positional Influence Analysis")
                        .child("Color gradient: Blue (low influence) → Cyan → Yellow → Red (high influence)")
                        .child("Strength values 0-9 with optional text labels (hover to see effect)")
                        .child(self.heat_overlay_board.clone()),
                ),
            )
            .child(
                section("Ghost Stones - Analysis Visualization").child(
                    v_flex()
                        .gap_2()
                        .child("9x9 Board with Ghost Stones for Move Analysis")
                        .child("Green: Good moves, Blue: Interesting moves, Yellow: Doubtful moves, Red: Bad moves")
                        .child("Faint ghost stones have reduced opacity for subtle display")
                        .child(self.ghost_stone_board.clone()),
                ),
            )
            .child(
                section("Lines and Arrows - Connection Visualization").child(
                    v_flex()
                        .gap_2()
                        .child("9x9 Board with Lines and Arrows for Board Analysis")
                        .child("Gray lines: Simple connections between stones")
                        .child("Dark arrows: Directional analysis and strategic moves")
                        .child("Blue lines: Highlighted connections")
                        .child("Red arrows: Direction indicators")
                        .child("All lines and arrows are properly rotated and positioned")
                        .child(self.line_board.clone()),
                ),
            )
            .child(
                section("Bounded Go Board - Responsive Sizing").child(
                    v_flex()
                        .gap_4()
                        .child("BoundedGoBoard automatically calculates vertex size to fit within constraints")
                        .child(
                            h_flex()
                                .gap_6()
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("Small (9x9 in 150x150)")
                                        .child(format!("Vertex size: calculated automatically"))
                                        .child(self.bounded_small_board.clone()),
                                )
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("Medium (13x13 in 250x250)")
                                        .child(format!("Vertex size: calculated automatically"))
                                        .child(self.bounded_medium_board.clone()),
                                ),
                        )
                        .child(
                            h_flex()
                                .gap_6()
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("Large (19x19 in 380x380)")
                                        .child(format!("Vertex size: calculated automatically"))
                                        .child(self.bounded_large_board.clone()),
                                )
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("Constrained (19x19 in 200x400)")
                                        .child(format!("Vertex size: calculated automatically"))
                                        .child("Width-constrained with size limits (5-25px)")
                                        .child(self.bounded_constrained_board.clone()),
                                ),
                        ),
                ),
            )
            .child(
                section("Partial Board Display - Range Support").child(
                    v_flex()
                        .gap_4()
                        .child("Demonstrates partial board display with proper stone alignment")
                        .child(
                            h_flex()
                                .gap_6()
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("Center Area (5x5)")
                                        .child("Typical center fighting pattern")
                                        .child(format!("Vertex size: calculated automatically"))
                                        .child(self.partial_board_center.clone()),
                                )
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("Corner Area (7x7)")
                                        .child("Corner opening pattern")
                                        .child(format!("Vertex size: calculated automatically"))
                                        .child(self.partial_board_corner.clone()),
                                )
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("Edge Slice (19x3)")
                                        .child("Side edge patterns")
                                        .child(format!("Vertex size: calculated automatically"))
                                        .child(self.partial_board_edge.clone()),
                                ),
                        ),
                ),
            )
            .child(
                section("Interactive Board").child(
                    v_flex()
                        .gap_4()
                        .child("9x9 Board with Comprehensive Event Handling")
                        .child("Try different mouse interactions: click, mouse down/up, move")
                        .child(
                            h_flex()
                                .gap_6()
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("Standard Interactive Board")
                                        .child(self.interactive_board.clone()),
                                )
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("Asset Board with Stone Placement")
                                        .child("Click to place stones alternating black/white")
                                        .child(self.interactive_asset_board.clone()),
                                ),
                        ),
                ),
            )
            .child(
                section("Efficient Update System - Differential Rendering").child(
                    v_flex()
                        .gap_4()
                        .child("Demonstrates efficient signMap updates with change detection and differential rendering")
                        .child(
                            v_flex()
                                .gap_2()
                                .child("Efficient Updates Demo")
                                .child("Uses update_stones() and update_sign_map() methods for optimal performance")
                                .child("Only re-renders elements that have actually changed")
                                .child(self.efficient_update_demo.clone()),
                        ),
                ),
            )
            .child(
                section("Board Information").child(
                    v_flex()
                        .gap_2()
                        .child("Features:")
                        .child("• Grid-based layout with proper line positioning")
                        .child("• Star points (hoshi) for standard board sizes (9x9, 13x13, 19x19)")
                        .child("• Stone rendering with Stone type support (EMPTY: 0, BLACK: 1, WHITE: -1)")
                        .child("• Coordinate labels with standard Go notation (A-T, 1-19)")
                        .child("• Configurable board sizes (9x9, 13x13, 19x19)")
                        .child("• Custom themes with colors and styling")
                        .child("• Theme system with builder pattern for easy customization")
                        .child("  - Predefined themes: default, dark, minimalist, high-contrast")
                        .child("  - Asset-based themes with Theme::with_assets()")
                        .child("  - Custom color and dimension overrides")
                        .child("• Marker system for board annotations")
                        .child("  - Circle, Cross, Triangle, Square, Dot shapes")
                        .child("  - Text labels with custom colors")
                        .child("  - Color customization for all marker types")
                        .child("• Ghost stones for move analysis")
                        .child("  - Good moves (green tint), Bad moves (red tint), Neutral (no tint)")
                        .child("  - Configurable opacity levels")
                        .child("• Heat/influence visualization")
                        .child("  - Strength values 0-9 with color gradients")
                        .child("  - Optional text labels for strength values")
                        .child("• Lines and arrows for analysis")
                        .child("  - Simple lines and directional arrows")
                        .child("  - Customizable colors and widths")
                        .child("• Selection and highlighting system")
                        .child("  - Selected vertices with blue highlighting")
                        .child("  - Last move indicators with orange highlighting")
                        .child("  - Dimmed vertices with opacity control")
                        .child("• Bounded sizing and responsive behavior")
                        .child("  - BoardView component with automatic vertex size calculation")
                        .child("  - maxWidth/maxHeight constraints with proportional scaling")
                        .child("  - Configurable vertex size limits (min/max bounds)")
                        .child("  - Support for extreme aspect ratios and small displays")
                        .child("• Interactive event handling")
                        .child("  - Click events for user interactions")
                        .child("  - Mouse button detection (left/right click)")
                        .child("  - Hover events for feedback")
                        .child("  - Keyboard navigation support")
                        .child("• Efficient state management")
                        .child("  - Builder pattern for fluent API")
                        .child("  - Immutable updates with clone-on-write")
                        .child("  - Sparse HashMap storage for memory efficiency")
                        .child("• Asset integration")
                        .child("  - Built-in icon assets for board and stones")
                        .child("  - Theme-based asset loading")
                        .child("  - Fallback to color-based rendering"),
                ),
            )
    }
}
