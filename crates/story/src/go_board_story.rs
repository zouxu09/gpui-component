use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, InteractiveElement, IntoElement,
    ParentElement as _, Render, Styled as _, Window,
};

use gpui_component::{
    go_board::{GoBoard, GridTheme},
    h_flex, v_flex, ActiveTheme,
};

use crate::{section, Story};

pub struct GoBoardStory {
    focus_handle: gpui::FocusHandle,
    board_19x19: Entity<GoBoard>,
    board_13x13: Entity<GoBoard>,
    board_9x9: Entity<GoBoard>,
    custom_theme_board: Entity<GoBoard>,
}

impl GoBoardStory {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            board_19x19: cx.new(|_| GoBoard::new()),
            board_13x13: cx.new(|_| GoBoard::with_size(13, 13)),
            board_9x9: cx.new(|_| GoBoard::with_size(9, 9)),
            custom_theme_board: cx.new(|_| {
                let mut board = GoBoard::with_size(9, 9).with_vertex_size(30.0);

                // Custom theme with darker colors
                let custom_theme = GridTheme {
                    background_color: gpui::rgb(0x8B7355), // Darker wood
                    grid_line_color: gpui::rgb(0x2c2c2c),  // Dark gray lines
                    grid_line_width: 1.5,
                    border_color: gpui::rgb(0x654321), // Dark brown border
                    border_width: 3.0,
                    star_point_color: gpui::rgb(0x2c2c2c), // Dark gray star points
                    star_point_size: 8.0,                  // Slightly larger star points
                };

                board.set_grid_theme(custom_theme);
                board
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
                    h_flex()
                        .gap_6()
                        .child(
                            v_flex()
                                .gap_2()
                                .child("19x19 Board (Standard)")
                                .child(self.board_19x19.clone()),
                        )
                        .child(
                            v_flex()
                                .gap_2()
                                .child("13x13 Board")
                                .child(self.board_13x13.clone()),
                        )
                        .child(
                            v_flex()
                                .gap_2()
                                .child("9x9 Board")
                                .child(self.board_9x9.clone()),
                        ),
                ),
            )
            .child(
                section("Custom Theme").child(
                    v_flex()
                        .gap_2()
                        .child("9x9 Board with Custom Theme")
                        .child(self.custom_theme_board.clone()),
                ),
            )
            .child(
                section("Board Information").child(
                    v_flex()
                        .gap_2()
                        .child("Features:")
                        .child("• Grid-based layout with proper line positioning")
                        .child("• Star points (hoshi) for standard board sizes")
                        .child("• Configurable board sizes (9x9, 13x13, 19x19)")
                        .child("• Custom themes with colors and styling")
                        .child("• Responsive design with proper scaling")
                        .child("• Support for partial board ranges")
                        .child("• Shudan-inspired architecture"),
                ),
            )
    }
}
