use gpui::*;
use gpui_component::{
    go_board::{Board, BoardView, Pos, Theme, BLACK, EMPTY, WHITE},
    v_flex, ActiveTheme,
};
use std::sync::{Arc, Mutex};
use story::Assets;

/// Example demonstrating Go board with asset-based rendering and click handling
///
/// Run with: cargo run -p story --example asset_go_board
/// Left click places black stones, right click places white stones
pub struct AssetGoBoardExample {
    board: Arc<Mutex<Board>>,
}

impl AssetGoBoardExample {
    fn new() -> Self {
        // Create the initial board
        let board = Board::with_size(19, 19)
            .vertex_size(25.0)
            .theme(Theme::with_assets())
            .coordinates(true)
            .stone(Pos::new(3, 3), BLACK)
            .stone(Pos::new(15, 15), WHITE);

        Self {
            board: Arc::new(Mutex::new(board)),
        }
    }
}

impl Render for AssetGoBoardExample {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let board_arc = self.board.clone();

        v_flex()
            .size_full()
            .bg(cx.theme().background)
            .justify_center()
            .items_center()
            .gap_4()
            .child(
                v_flex()
                    .gap_2()
                    .child("Go Board Example")
                    .child("Left click: Black stone | Right click: White stone")
                    .child("Click on empty intersections to place stones"),
            )
            .child(cx.new(|_| {
                // Get the current board state
                let board = board_arc.lock().unwrap().clone();

                BoardView::new(board).on_click(move |event| {
                    let pos = event.pos;
                    let stone_type = match event.mouse_button {
                        Some(MouseButton::Left) => "BLACK",
                        Some(MouseButton::Right) => "WHITE",
                        _ => "UNKNOWN",
                    };

                    // Update the board state
                    if let Ok(mut board_guard) = board_arc.lock() {
                        let current_board = board_guard.clone();

                        // Check if position is already occupied
                        if current_board.stone_at(pos) != EMPTY {
                            println!("Position ({}, {}) is already occupied!", pos.x, pos.y);
                            return;
                        }

                        // Place the stone
                        let new_board = current_board
                            .stone(pos, if stone_type == "BLACK" { BLACK } else { WHITE });
                        *board_guard = new_board;

                        println!("Placed {} stone at ({}, {})", stone_type, pos.x, pos.y);
                    }
                })
            }))
    }
}

fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        gpui_component::init(cx);

        cx.open_window(Default::default(), |window, cx| {
            window.set_window_title("Asset Go Board Example");
            cx.new(|_| AssetGoBoardExample::new())
        })
        .unwrap();
    });
}
