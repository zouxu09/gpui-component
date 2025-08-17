/// Go Board UI Widget - Simple Usage Documentation
///
/// This document provides basic examples of how to use the Go board UI widget.
use gpui::rgb;
use gpui_component::go_board::{BoardTheme, GoBoard, Vertex};

/// Basic board creation example
pub fn create_basic_board() {
    // Standard 19x19 board
    let _board = GoBoard::new();

    // Custom sized board
    let _small_board = GoBoard::with_size(9, 9).with_vertex_size(30.0);
}

/// Stone placement example
pub fn place_stones() {
    let mut board = GoBoard::with_size(9, 9);

    // Create a simple pattern
    let sign_map = vec![
        vec![0, 0, 0, 1, 0, -1, 0, 0, 0],
        vec![0, 1, 0, 0, 0, 0, 0, -1, 0],
        vec![0, 0, 1, 0, 0, 0, -1, 0, 0],
        vec![1, 0, 0, 0, 0, 0, 0, 0, -1],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![-1, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![0, 0, -1, 0, 0, 0, 1, 0, 0],
        vec![0, -1, 0, 0, 0, 0, 0, 1, 0],
        vec![0, 0, 0, -1, 0, 1, 0, 0, 0],
    ];

    board.set_sign_map(sign_map);
}

/// Theming example
pub fn apply_theme() {
    let mut board = GoBoard::with_size(9, 9);

    // Use predefined themes
    board.set_theme(BoardTheme::dark());

    // Create custom theme
    let custom_theme = BoardTheme::default()
        .with_board_background(rgb(0x8B7355))
        .with_grid_lines(rgb(0x2c2c2c), 1.5)
        .with_stone_colors(rgb(0x000000), rgb(0xffffff));

    board.set_theme(custom_theme);
}

/// Stone manipulation example
pub fn manipulate_stones() {
    let mut board = GoBoard::with_size(9, 9);

    // Place individual stones
    board.set_stone(&Vertex::new(4, 4), 1); // Black stone at center
    board.set_stone(&Vertex::new(3, 3), -1); // White stone

    // Bulk updates
    let updates = vec![
        (Vertex::new(2, 2), 1),  // Black
        (Vertex::new(6, 6), -1), // White
        (Vertex::new(7, 7), 0),  // Remove stone
    ];
    board.update_stones(&updates);
}

/// Selection example
pub fn set_selections() {
    let mut board = GoBoard::with_size(9, 9);

    // Highlight specific vertices
    board.set_selected_vertices(vec![Vertex::new(4, 4), Vertex::new(3, 3)]);

    // Dim certain areas
    board.set_dimmed_vertices(vec![Vertex::new(0, 0), Vertex::new(8, 8)]);
}
