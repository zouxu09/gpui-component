use gpui::*;
use gpui_component::{
    button::Button,
    go_board::{Board, BoardView, Pos, Theme, BLACK, EMPTY, WHITE},
    h_flex, v_flex, ActiveTheme,
};
use std::sync::{Arc, Mutex};
use story::Assets;

/// Example demonstrating Go board with asset-based rendering, click handling, and resize functionality
///
/// Run with: cargo run -p story --example asset_go_board
/// Left click places black stones, right click places white stones
/// Use resize controls to change the board container size and see automatic vertex size adjustment
pub struct AssetGoBoardExample {
    board: Arc<Mutex<Board>>,
    container_width: f32,
    container_height: f32,
}

impl AssetGoBoardExample {
    fn new() -> Self {
        // Create the initial board
        let board = Board::with_size(19, 19)
            .theme(Theme::with_assets())
            .coordinates(true)
            .stone(Pos::new(3, 3), BLACK)
            .stone(Pos::new(15, 15), WHITE);

        Self {
            board: Arc::new(Mutex::new(board)),
            container_width: 500.0,
            container_height: 500.0,
        }
    }

    fn resize_container(&mut self, width: f32, height: f32) {
        self.container_width = width.max(200.0).min(800.0); // Clamp between 200-800px
        self.container_height = height.max(200.0).min(800.0); // Clamp between 200-800px
    }
}

impl Render for AssetGoBoardExample {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let board_arc = self.board.clone();
        let container_width = self.container_width;
        let container_height = self.container_height;

        v_flex()
            .size_full()
            .bg(cx.theme().background)
            .p_4()
            .gap_4()
            .child(
                v_flex()
                    .gap_2()
                    .child("Go Board Example with Resize Support")
                    .child("Left click: Black stone | Right click: White stone")
                    .child("Click on empty intersections to place stones")
                    .child(format!(
                        "Container: {:.0}×{:.0}px",
                        container_width, container_height
                    )),
            )
            .child(
                h_flex()
                    .gap_4()
                    .items_center()
                    .child("Resize Controls:")
                    .child(
                        h_flex()
                            .gap_2()
                            .items_center()
                            .child("Width:")
                            .child(Button::new("Small").on_click(cx.listener(
                                |this, _, _window, cx| {
                                    this.resize_container(300.0, this.container_height);
                                    cx.notify();
                                },
                            )))
                            .child(Button::new("Medium").on_click(cx.listener(
                                |this, _, _window, cx| {
                                    this.resize_container(500.0, this.container_height);
                                    cx.notify();
                                },
                            )))
                            .child(Button::new("Large").on_click(cx.listener(
                                |this, _, _window, cx| {
                                    this.resize_container(700.0, this.container_height);
                                    cx.notify();
                                },
                            ))),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .items_center()
                            .child("Height:")
                            .child(Button::new("Small").on_click(cx.listener(
                                |this, _, _window, cx| {
                                    this.resize_container(this.container_width, 300.0);
                                    cx.notify();
                                },
                            )))
                            .child(Button::new("Medium").on_click(cx.listener(
                                |this, _, _window, cx| {
                                    this.resize_container(this.container_width, 500.0);
                                    cx.notify();
                                },
                            )))
                            .child(Button::new("Large").on_click(cx.listener(
                                |this, _, _window, cx| {
                                    this.resize_container(this.container_width, 700.0);
                                    cx.notify();
                                },
                            ))),
                    ),
            )
            .child(
                v_flex()
                    .gap_2()
                    .child("Board Container (resize to see automatic vertex size adjustment)")
                    .child(
                        v_flex()
                            .w(px(container_width))
                            .h(px(container_height))
                            .border_1()
                            .border_color(gpui::rgb(0x666666))
                            .bg(gpui::rgb(0xf8f8f8))
                            .justify_center()
                            .items_center()
                            .child(cx.new(move |_| {
                                // Get the current board state
                                let board = board_arc.lock().unwrap().clone();

                                BoardView::with_size(board, container_width, container_height)
                                    .on_click(move |event| {
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
                                                println!(
                                                    "Position ({}, {}) is already occupied!",
                                                    pos.x, pos.y
                                                );
                                                return;
                                            }

                                            // Place the stone
                                            let new_board = current_board.stone(
                                                pos,
                                                if stone_type == "BLACK" { BLACK } else { WHITE },
                                            );
                                            *board_guard = new_board;

                                            println!(
                                                "Placed {} stone at ({}, {})",
                                                stone_type, pos.x, pos.y
                                            );
                                        }
                                    })
                            })),
                    ),
            )
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
