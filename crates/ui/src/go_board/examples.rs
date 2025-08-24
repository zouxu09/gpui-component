use crate::go_board::*;
use gpui::*;

/// Example usage of the new simplified Go board API
/// These examples show how much simpler the new API is compared to the old system

/// Example 1: Basic board with stones
pub fn basic_board_example() -> BoardView {
    // Old way (complex):
    // let mut board = GoBoard::new();
    // board.set_sign_map(...);
    // board.set_marker_map(...);
    // let handlers = VertexEventHandlers::new().with_click(...);
    // board.render_with_vertex_handlers(handlers)

    // New way (simple):
    BoardView::new(
        Board::new()
            .stone(Pos::new(3, 3), BLACK)
            .stone(Pos::new(15, 15), WHITE)
            .stone(Pos::new(9, 9), BLACK)
            .last_move(Pos::new(9, 9)),
    )
    .on_click(|event| {
        println!("Stone placed at {:?}", event.pos);
    })
}

/// Example 2: Analysis board with ghost stones and markers
pub fn analysis_board_example() -> BoardView {
    BoardView::new(
        Board::new()
            .theme(themes::dark())
            .stones([
                (Pos::new(3, 3), BLACK),
                (Pos::new(4, 4), WHITE),
                (Pos::new(5, 5), BLACK),
                (Pos::new(16, 16), WHITE),
            ])
            .ghosts([
                (Pos::new(3, 4), ghosts::good(WHITE).with_alpha(0.7)),
                (Pos::new(4, 3), ghosts::bad(BLACK).with_alpha(0.6)),
                (Pos::new(6, 6), ghosts::neutral(WHITE)),
            ])
            .markers([
                (Pos::new(3, 15), Marker::circle().with_color(rgb(0xff0000))),
                (
                    Pos::new(15, 3),
                    Marker::triangle().with_color(rgb(0x0000ff)),
                ),
                (
                    Pos::new(9, 9),
                    Marker::label("A").with_color(rgb(0x00ff00)),
                ),
            ])
            .lines([
                lines::arrow(Pos::new(3, 3), Pos::new(6, 6)).with_color(rgb(0xff8000)),
                lines::line(Pos::new(4, 4), Pos::new(16, 16)).with_color(rgb(0x8000ff)),
            ])
            .select(Pos::new(5, 5)),
    )
    .coordinates(true)
    .on_click(|event| {
        println!("Analysis click at {:?}", event.pos);
    })
    .on_hover(|pos| {
        if let Some(p) = pos {
            println!("Analyzing position {:?}", p);
        }
    })
}

/// Example 3: Small teaching board
pub fn teaching_board_example() -> BoardView {
    BoardView::new(
        Board::with_size(9, 9)
            .vertex_size(35.0)
            .theme(themes::high_contrast())
            .stones([
                (Pos::new(2, 2), BLACK),
                (Pos::new(6, 6), WHITE),
                (Pos::new(4, 4), BLACK),
            ])
            .markers([
                (Pos::new(2, 2), markers::label("1")),
                (Pos::new(6, 6), markers::label("2")),
                (Pos::new(4, 4), markers::label("3")),
            ])
            .ghosts([
                (Pos::new(3, 3), ghosts::good(WHITE)),
                (Pos::new(5, 5), ghosts::good(WHITE)),
            ]),
    )
    .on_click(|event| {
        let coord = format!("{}{}", (b'A' + event.pos.x as u8) as char, event.pos.y + 1);
        println!("Teaching board click at {}", coord);
    })
}

/// Example 4: Problem/puzzle board
pub fn puzzle_board_example() -> BoardView {
    BoardView::new(
        Board::with_size(13, 13)
            .range(Range::new((3, 9), (3, 9))) // Focus on center area
            .stones([
                (Pos::new(5, 5), BLACK),
                (Pos::new(6, 5), WHITE),
                (Pos::new(5, 6), WHITE),
                (Pos::new(7, 6), BLACK),
            ])
            .markers([(Pos::new(6, 6), Marker::circle().with_color(rgb(0xff0000)))])
            .ghosts([
                (Pos::new(6, 6), ghosts::good(BLACK).with_alpha(0.5)),
                (Pos::new(4, 5), ghosts::bad(BLACK).with_alpha(0.4)),
                (Pos::new(5, 4), ghosts::bad(BLACK).with_alpha(0.4)),
            ]),
    )
    .coordinates(false) // Cleaner for puzzles
    .on_click(|event| {
        println!("Puzzle attempt at {:?}", event.pos);
    })
}

/// Example 5: Auto-sizing board for responsive layout
pub fn responsive_board_example(container_width: f32, container_height: f32) -> BoardView {
    // Auto-calculates vertex size to fit in container
    let bounded = BoundedBoard::new(container_width, container_height)
        .vertex_size_limits(15.0, 40.0)
        .update(|board| {
            board
                .stones([
                    (Pos::new(3, 3), BLACK),
                    (Pos::new(15, 15), WHITE),
                    (Pos::new(9, 9), BLACK),
                ])
                .last_move(Pos::new(9, 9))
        });

    BoardView::new(bounded.inner().clone()).on_click(|event| {
        println!("Responsive board click at {:?}", event.pos);
    })
}

/// Example 6: Load board from SGF-like position
pub fn sgf_position_example() -> BoardView {
    // Much simpler than parsing full SGF
    let board = board::from_position_string((19, 19), "B[dd],W[pp],B[pd],W[dp],B[pj],W[nq]");

    BoardView::new(board).on_click(|event| {
        println!("SGF board click at {:?}", event.pos);
    })
}

/// Example 7: Interactive game board with keyboard navigation
pub fn game_board_example() -> BoardView {
    let mut current_player = BLACK;

    BoardView::new(Board::new())
        .focus(Some(Pos::new(9, 9))) // Start focus at center
        .on_click(move |event| {
            println!("Game move: {:?} plays at {:?}", current_player, event.pos);
            // In real app, you'd update the board state and switch players
            current_player = -current_player; // Switch between BLACK and WHITE
        })
        .on_key(|key_event| {
            // Custom keyboard handling for game controls
            match key_event.keystroke.key.as_str() {
                "p" => {
                    println!("Pass move");
                    None
                }
                "u" => {
                    println!("Undo move");
                    None
                }
                "r" => {
                    println!("Resign");
                    None
                }
                _ => None, // Let default navigation handle other keys
            }
        })
}

/// Example 8: Comparison of old vs new API complexity
pub mod api_comparison {
    use super::*;

    // Old API (complex setup):
    pub fn old_way_example() {
        // This would require:
        // 1. Creating GoBoardState
        // 2. Setting up theme components (GridTheme, StoneTheme, etc.)
        // 3. Managing multiple overlay components
        // 4. Complex event handler setup
        // 5. Manual coordinate calculations
        // 6. Separate rendering for each layer

        // Example of old complexity:
        /*
        let mut state = GoBoardState::new(19, 19);
        state.set_sign_map(sign_map);
        state.set_marker_map(marker_map);

        let grid_theme = GridTheme { ... };
        let stone_theme = StoneTheme { ... };
        let board_theme = BoardTheme { ... };

        let grid = Grid::new(board_range, vertex_size).with_theme(grid_theme);
        let stones = Stones::new(board_range, vertex_size, sign_map).with_theme(stone_theme);
        let markers = Markers::new(vertex_size, grid_offset);
        let ghost_overlay = GhostStoneOverlay::new(vertex_size, grid_offset);
        let heat_overlay = HeatOverlay::new(vertex_size, grid_offset);
        // ... more overlays

        let handlers = VertexEventHandlers::new()
            .with_click(|event| { ... })
            .with_mouse_down(|event| { ... });

        // Complex rendering setup...
        */
    }

    // New API (simple setup):
    pub fn new_way_example() -> BoardView {
        // One-liner with fluent API:
        BoardView::new(
            Board::new()
                .stone(Pos::new(3, 3), BLACK)
                .marker(Pos::new(4, 4), Marker::circle())
                .ghost(Pos::new(5, 5), ghosts::good(WHITE)),
        )
        .on_click(|event| println!("Click: {:?}", event.pos))
    }
}

/// Example 9: Performance comparison
pub mod performance_examples {
    use super::*;

    /// Efficient bulk operations with new API
    pub fn bulk_operations_example() -> BoardView {
        // Generate lots of stones efficiently
        let stones: Vec<(Pos, Stone)> = (0..19)
            .flat_map(|x| {
                (0..19).filter_map(move |y| {
                    if (x + y) % 3 == 0 {
                        Some((Pos::new(x, y), if (x + y) % 6 == 0 { BLACK } else { WHITE }))
                    } else {
                        None
                    }
                })
            })
            .collect();

        // Set all stones at once (much more efficient than the old way)
        BoardView::new(Board::new().stones(stones)).on_click(|event| {
            println!("Bulk board click at {:?}", event.pos);
        })
    }

    /// Memory-efficient sparse representation
    pub fn sparse_board_example() -> BoardView {
        // The new HashMap-based storage is much more memory efficient
        // for sparse boards (only occupied positions consume memory)

        let mut board = Board::new();

        // Add just a few stones - uses minimal memory
        board.data_mut().set_stone(Pos::new(3, 3), BLACK);
        board.data_mut().set_stone(Pos::new(15, 15), WHITE);
        // Empty positions consume no memory!

        BoardView::new(board)
    }
}

#[cfg(test)]
mod example_tests {
    use super::*;

    #[test]
    fn test_all_examples_compile() {
        // Just test that all examples compile without panicking
        let _ = basic_board_example();
        let _ = analysis_board_example();
        let _ = teaching_board_example();
        let _ = puzzle_board_example();
        let _ = responsive_board_example(400.0, 400.0);
        let _ = sgf_position_example();
        let _ = game_board_example();
        let _ = api_comparison::new_way_example();
        let _ = performance_examples::bulk_operations_example();
        let _ = performance_examples::sparse_board_example();
    }

    #[test]
    fn test_example_board_states() {
        let basic = basic_board_example();
        assert_eq!(basic.board().stone_at(Pos::new(3, 3)), BLACK);

        let teaching = teaching_board_example();
        assert_eq!(teaching.board().dimensions(), (9, 9));
        assert_eq!(teaching.board().stone_at(Pos::new(4, 4)), BLACK);
    }
}
