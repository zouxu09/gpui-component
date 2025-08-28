use gpui::*;
use gpui_component::{
    go_board::{Board, BoardView, Ghost, Pos, PosEvent, Theme, BLACK, WHITE},
    h_flex, v_flex, ActiveTheme,
};
use story::Assets;

/// Example demonstrating Go board with asset-based rendering and click handling
///
/// Run with: cargo run -p story --example asset_go_board
/// Left click places black stones, right click places white stones
pub struct AssetGoBoardExample;

impl AssetGoBoardExample {
    fn view(_window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|_| Self)
    }

    fn handle_board_click(event: PosEvent) {
        let pos = event.pos;

        // For this example, we'll just print the click information
        // In a real application, you would need to have access to mutable state
        let stone_type = match event.mouse_button {
            Some(MouseButton::Left) => "BLACK",
            Some(MouseButton::Right) => "WHITE",
            _ => "UNKNOWN",
        };

        println!(
            "Clicked at position ({}, {}) with {} mouse button -> Place {} stone",
            pos.x,
            pos.y,
            event
                .mouse_button
                .map(|b| format!("{:?}", b))
                .unwrap_or_else(|| "None".to_string()),
            stone_type
        );
    }
}

impl Render for AssetGoBoardExample {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_8()
            .p_8()
            .size_full()
            .bg(cx.theme().background)
            .child(
                v_flex()
                    .gap_4()
                    .child("Click Instructions:")
                    .child("• Left click: Place black stone")
                    .child("• Right click: Place white stone")
                    .child("• Cannot place on existing stones"),
            )
            .child(
                h_flex()
                    .gap_8()
                    .child(
                        v_flex()
                            .gap_4()
                            .child("Default Theme (Interactive)")
                            .child(cx.new(|_| {
                                let board = Board::with_size(9, 9)
                                    .vertex_size(30.0)
                                    .stone(Pos::new(2, 2), BLACK)
                                    .stone(Pos::new(6, 6), WHITE)
                                    .stone(Pos::new(4, 4), BLACK)
                                    .stone(Pos::new(3, 5), WHITE)
                                    .ghost(Pos::new(5, 5), Ghost::good(BLACK))
                                    .ghost(Pos::new(1, 1), Ghost::bad(WHITE))
                                    .last_move(Pos::new(4, 4));
                                BoardView::new(board)
                                    .coordinates(true)
                                    .on_click(Self::handle_board_click)
                            })),
                    )
                    .child(
                        v_flex()
                            .gap_4()
                            .child("Asset Theme (Interactive)")
                            .child(cx.new(|_| {
                                let board = Board::with_size(9, 9)
                                    .vertex_size(30.0)
                                    .theme(Theme::with_assets())
                                    .stone(Pos::new(2, 2), BLACK)
                                    .stone(Pos::new(6, 6), WHITE)
                                    .stone(Pos::new(4, 4), BLACK)
                                    .stone(Pos::new(3, 5), WHITE)
                                    .ghost(Pos::new(5, 5), Ghost::good(BLACK))
                                    .ghost(Pos::new(1, 1), Ghost::bad(WHITE))
                                    .last_move(Pos::new(4, 4));
                                BoardView::new(board)
                                    .coordinates(true)
                                    .on_click(Self::handle_board_click)
                            })),
                    ),
            )
            .child(
                v_flex()
                    .gap_4()
                    .child("Custom Asset Paths (Interactive)")
                    .child(cx.new(|_| {
                        let board = Board::with_size(9, 9)
                            .vertex_size(30.0)
                            .theme(
                                Theme::default()
                                    .with_board_background("icons/board.png")
                                    .with_black_stone_asset("icons/black_stone.svg")
                                    .with_white_stone_asset("icons/white_stone.svg"),
                            )
                            .stone(Pos::new(2, 2), BLACK)
                            .stone(Pos::new(6, 6), WHITE)
                            .stone(Pos::new(4, 4), BLACK)
                            .stone(Pos::new(3, 5), WHITE)
                            .ghost(Pos::new(5, 5), Ghost::good(BLACK))
                            .ghost(Pos::new(1, 1), Ghost::bad(WHITE))
                            .ghost(Pos::new(7, 3), Ghost::neutral(BLACK))
                            .last_move(Pos::new(4, 4));
                        BoardView::new(board)
                            .coordinates(true)
                            .on_click(Self::handle_board_click)
                    })),
            )
    }
}

fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        gpui_component::init(cx);

        cx.open_window(Default::default(), |window, cx| {
            window.set_window_title("Asset Go Board Example");
            AssetGoBoardExample::view(window, cx)
        })
        .unwrap();
    });
}
