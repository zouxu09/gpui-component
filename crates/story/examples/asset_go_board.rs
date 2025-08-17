use gpui::*;
use gpui_component::{
    go_board::{BoardTheme, GoBoard, Vertex, VertexClickEvent, VertexEventHandlers},
    h_flex, v_flex, ActiveTheme,
};
use story::{create_new_window, init, Assets};

/// Standalone demo of Go board using the specified assets
/// Run with: cargo run --example asset_go_board
struct AssetGoBoardDemo {
    focus_handle: FocusHandle,
    board: Entity<GoBoard>,
}

impl AssetGoBoardDemo {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let board = cx.new(|_| {
            // Create a board using the specific assets mentioned in the request
            let asset_theme = BoardTheme::default()
                .with_board_texture("assets/icons/board.png".to_string())
                .with_stone_textures(
                    Some("assets/icons/black_stone.svg".to_string()),
                    Some("assets/icons/white_stone.svg".to_string()),
                );

            let mut board = GoBoard::with_size(9, 9).with_vertex_size(40.0);
            board.set_theme(asset_theme);
            board.set_show_coordinates(true);

            // Add a sample game pattern to demonstrate the stones
            let sign_map = vec![
                vec![0, 0, 0, 1, 0, -1, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0, -1, 0],
                vec![0, 0, 1, 0, 0, 0, -1, 0, 0],
                vec![1, 0, 0, 1, 0, -1, 0, 0, -1],
                vec![0, 0, 0, 0, 1, 0, 0, 0, 0],
                vec![-1, 0, 0, -1, 0, 1, 0, 0, 1],
                vec![0, 0, -1, 0, 0, 0, 1, 0, 0],
                vec![0, -1, 0, 0, 0, 0, 0, 1, 0],
                vec![0, 0, 0, -1, 0, 1, 0, 0, 0],
            ];
            board.set_sign_map(sign_map);

            board
        });

        Self {
            focus_handle: cx.focus_handle(),
            board,
        }
    }
}

impl Focusable for AssetGoBoardDemo {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for AssetGoBoardDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        v_flex()
            .size_full()
            .gap_4()
            .p_4()
            .bg(theme.background)
            .track_focus(&self.focus_handle)
            .child(
                h_flex().justify_center().child(
                    v_flex()
                        .gap_2()
                        .child("Go Board with Custom Assets")
                        .child("Uses assets/icons/board.png, black_stone.svg, white_stone.svg"),
                ),
            )
            .child(
                h_flex()
                    .justify_center()
                    .child(self.board.update(cx, |board, _| {
                        // Create a simple click handler that logs clicks
                        let handlers =
                            VertexEventHandlers::new().with_click(|event: VertexClickEvent| {
                                println!(
                                    "Asset Board Click: ({}, {}) - coordinate: {:?}",
                                    event.vertex.x, event.vertex.y, event.coordinates
                                );
                            });

                        board.render_with_vertex_handlers(handlers)
                    })),
            )
            .child(
                h_flex().justify_center().child(
                    v_flex()
                        .gap_2()
                        .child("Features Demonstrated:")
                        .child("• Board background using board.png")
                        .child("• Black stones using black_stone.svg")
                        .child("• White stones using white_stone.svg")
                        .child("• Coordinate labels for reference")
                        .child("• Click interactions (see console output)"),
                ),
            )
    }
}

fn main() {
    let app = gpui::Application::new().with_assets(Assets);

    app.run(move |cx| {
        init(cx);
        cx.activate(true);

        create_new_window(
            "Go Board Asset Demo",
            move |window, cx| cx.new(|cx| AssetGoBoardDemo::new(window, cx)),
            cx,
        );
    });
}
