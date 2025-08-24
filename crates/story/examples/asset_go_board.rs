use gpui::*;
use gpui_component::{
    go_board::{Board, BoardView, Pos, Theme, BLACK, WHITE},
    h_flex, v_flex, ActiveTheme,
};
use story::{create_new_window, init, Assets};

/// Standalone demo of Go board using the specified assets
/// Run with: cargo run --example asset_go_board
struct AssetGoBoardDemo {
    focus_handle: FocusHandle,
    board_view: Entity<BoardView>,
}

impl AssetGoBoardDemo {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        // Create a board using the default theme
        let mut board = Board::with_size(9, 9).vertex_size(40.0).theme(Theme::default());

        // Add a sample game pattern to demonstrate the stones
        let stones = [
            (Pos::new(3, 0), BLACK), (Pos::new(5, 0), WHITE),
            (Pos::new(1, 1), BLACK), (Pos::new(7, 1), WHITE),
            (Pos::new(2, 2), BLACK), (Pos::new(6, 2), WHITE),
            (Pos::new(0, 3), BLACK), (Pos::new(3, 3), BLACK), (Pos::new(5, 3), WHITE), (Pos::new(8, 3), WHITE),
            (Pos::new(4, 4), BLACK),
            (Pos::new(0, 5), WHITE), (Pos::new(3, 5), WHITE), (Pos::new(5, 5), BLACK), (Pos::new(8, 5), BLACK),
            (Pos::new(2, 6), WHITE), (Pos::new(6, 6), BLACK),
            (Pos::new(1, 7), WHITE), (Pos::new(7, 7), BLACK),
            (Pos::new(3, 8), WHITE), (Pos::new(5, 8), BLACK),
        ];
        
        board = board.stones(stones);

        let board_view = cx.new(|_| {
            BoardView::new(board)
                .coordinates(true)
                .on_click(|event| {
                    println!(
                        "Board Click: ({}, {})",
                        event.pos.x, event.pos.y
                    );
                })
        });

        Self {
            focus_handle: cx.focus_handle(),
            board_view,
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
                        .child("Go Board (fixed alignment)")
                        .child("Testing grid and vertex alignment"),
                ),
            )
            .child(
                h_flex()
                    .justify_center()
                    .child(self.board_view.clone()),
            )
            .child(
                h_flex().justify_center().child(
                    v_flex()
                        .gap_2()
                        .child("Features Demonstrated:")
                        .child("• Fixed grid line and vertex alignment")
                        .child("• Coordinate labels")
                        .child("• Properly aligned stones and markers")
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
