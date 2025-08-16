use crate::go_board::types::*;
use crate::go_board::{GoBoard, GridTheme};

#[test]
fn test_go_board_with_grid() {
    let mut board = GoBoard::new();

    // Test default grid theme
    assert_eq!(board.grid_theme().background_color, rgb(0xebb55b));
    assert_eq!(board.grid_theme().grid_line_color, rgb(0x000000));
    assert_eq!(board.grid_theme().grid_line_width, 1.0);

    // Test custom grid theme
    let custom_theme = GridTheme {
        background_color: rgb(0x123456),
        grid_line_color: rgb(0x654321),
        grid_line_width: 2.0,
        border_color: rgb(0xabcdef),
        border_width: 3.0,
    };

    board.set_grid_theme(custom_theme.clone());
    assert_eq!(
        board.grid_theme().background_color,
        custom_theme.background_color
    );
    assert_eq!(board.grid_theme().grid_line_width, 2.0);
}

#[test]
fn test_go_board_grid_integration() {
    let board = GoBoard::with_size(9, 9).with_vertex_size(30.0);

    // Verify board state affects grid rendering
    assert_eq!(board.state().dimensions(), (9, 9));
    assert_eq!(board.state().vertex_size, 30.0);

    // Test board pixel size calculation
    let size = board.board_pixel_size();
    assert_eq!(size.width, px(270.0)); // 9 * 30
    assert_eq!(size.height, px(270.0)); // 9 * 30
}

#[test]
fn test_go_board_partial_range() {
    let range = BoardRange::new((3, 12), (3, 12)); // 10x10 visible area
    let board = GoBoard::with_size(19, 19)
        .with_range(range)
        .with_vertex_size(20.0);

    // Verify partial board size
    let size = board.board_pixel_size();
    assert_eq!(size.width, px(200.0)); // 10 * 20
    assert_eq!(size.height, px(200.0)); // 10 * 20
}
