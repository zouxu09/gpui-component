/// Simple Go Board Demos
///
/// This module contains working demo applications that showcase the Go board widget.
use gpui::{
    div, App, AppContext, Context, Entity, FocusHandle, Focusable, IntoElement, ParentElement,
    Render, Styled, Window,
};

use gpui_component::{
    go_board::{BoardTheme, GoBoard, Vertex},
    v_flex, ActiveTheme,
};

/// Basic demo showing stone placement
pub struct SimpleDemo {
    focus_handle: FocusHandle,
    board: Entity<GoBoard>,
}

impl SimpleDemo {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let board = cx.new(|_| {
            let mut board = GoBoard::with_size(9, 9).with_vertex_size(30.0);

            // Simple pattern
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
            board
        });

        Self {
            focus_handle: cx.focus_handle(),
            board,
        }
    }
}

impl Render for SimpleDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_3()
            .child("Simple Go Board Demo")
            .child("9x9 board with basic stone placement")
            .child(self.board.clone())
    }
}

impl Focusable for SimpleDemo {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

/// Demo showing different themes
pub struct ThemeDemo {
    focus_handle: FocusHandle,
    default_board: Entity<GoBoard>,
    dark_board: Entity<GoBoard>,
}

impl ThemeDemo {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        // Sample stones for both boards
        let sample_stones = vec![
            vec![0, 0, 1, 0, -1, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0, 0, -1, 0],
            vec![1, 0, 0, 0, 0, 0, 0, 0, -1],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![-1, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![0, -1, 0, 0, 0, 0, 0, 1, 0],
            vec![0, 0, -1, 0, 0, 0, 1, 0, 0],
        ];

        Self {
            focus_handle: cx.focus_handle(),
            default_board: cx.new(|_| {
                let mut board = GoBoard::with_size(9, 9).with_vertex_size(25.0);
                board.set_sign_map(sample_stones.clone());
                board
            }),
            dark_board: cx.new(|_| {
                let mut board = GoBoard::with_size(9, 9).with_vertex_size(25.0);
                board.set_theme(BoardTheme::dark());
                board.set_sign_map(sample_stones);
                board
            }),
        }
    }
}

impl Render for ThemeDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        v_flex()
            .gap_4()
            .bg(theme.background)
            .child("Theme Demo")
            .child("Comparing default and dark themes")
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(
                        v_flex()
                            .gap_2()
                            .child("Default Theme")
                            .child(self.default_board.clone()),
                    )
                    .child(
                        v_flex()
                            .gap_2()
                            .child("Dark Theme")
                            .child(self.dark_board.clone()),
                    ),
            )
    }
}

impl Focusable for ThemeDemo {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
