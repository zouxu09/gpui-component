use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, InteractiveElement, IntoElement,
    ParentElement as _, Render, Styled as _, Window,
};

use gpui_component::{
    go_board::{
        GoBoard, GridTheme, Marker, MarkerType, VertexClickEvent, VertexEventHandlers,
        VertexMouseDownEvent, VertexMouseMoveEvent, VertexMouseUpEvent,
    },
    h_flex, v_flex, ActiveTheme,
};

use crate::{section, Story};

pub struct GoBoardStory {
    focus_handle: gpui::FocusHandle,
    board_19x19: Entity<GoBoard>,
    board_13x13: Entity<GoBoard>,
    board_9x9: Entity<GoBoard>,
    custom_theme_board: Entity<GoBoard>,
    coordinate_board: Entity<GoBoard>,
    stone_board: Entity<GoBoard>,
    fuzzy_stone_board: Entity<GoBoard>,
    marker_board: Entity<GoBoard>,
    interactive_board: Entity<GoBoard>,
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
            coordinate_board: cx.new(|_| {
                let mut board = GoBoard::with_size(13, 13).with_vertex_size(25.0);
                board.set_show_coordinates(true);
                board
            }),
            stone_board: cx.new(|_| {
                let mut board = GoBoard::with_size(9, 9).with_vertex_size(35.0);

                // Create a simple game pattern
                let sign_map = vec![
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 1, 0, -1, 0, 0, 0],
                    vec![0, 0, 1, 0, 0, 0, -1, 0, 0],
                    vec![0, 1, 0, 1, 0, -1, 0, -1, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, -1, 0, -1, 0, 1, 0, 1, 0],
                    vec![0, 0, -1, 0, 0, 0, 1, 0, 0],
                    vec![0, 0, 0, -1, 0, 1, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                ];
                board.set_sign_map(sign_map);
                board
            }),
            fuzzy_stone_board: cx.new(|_| {
                let mut board = GoBoard::with_size(9, 9).with_vertex_size(35.0);

                // Create the same pattern as stone_board
                let sign_map = vec![
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 1, 0, -1, 0, 0, 0],
                    vec![0, 0, 1, 0, 0, 0, -1, 0, 0],
                    vec![0, 1, 0, 1, 0, -1, 0, -1, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, -1, 0, -1, 0, 1, 0, 1, 0],
                    vec![0, 0, -1, 0, 0, 0, 1, 0, 0],
                    vec![0, 0, 0, -1, 0, 1, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                ];
                board.set_sign_map(sign_map);

                // Enable fuzzy positioning and visual variation
                use gpui_component::go_board::StoneTheme;
                let fuzzy_theme = StoneTheme {
                    fuzzy_placement: true,
                    fuzzy_max_offset: 3.0,
                    random_variation: true,
                    max_rotation: 8.0,
                    ..StoneTheme::default()
                };
                board.set_stone_theme(fuzzy_theme);
                board
            }),
            marker_board: cx.new(|_| {
                let mut board = GoBoard::with_size(9, 9).with_vertex_size(35.0);

                // Create a marker map demonstrating all marker types
                let mut marker_map = vec![vec![None; 9]; 9];

                // Row 1: Basic marker types
                marker_map[1][1] = Some(Marker::new(MarkerType::Circle));
                marker_map[1][2] = Some(Marker::new(MarkerType::Cross));
                marker_map[1][3] = Some(Marker::new(MarkerType::Triangle));
                marker_map[1][4] = Some(Marker::new(MarkerType::Square));
                marker_map[1][5] = Some(Marker::new(MarkerType::Point));

                // Row 2: Colored markers with tooltips
                marker_map[2][1] = Some(
                    Marker::with_label(
                        MarkerType::Circle,
                        "Red circle marker - hover for tooltip!".to_string(),
                    )
                    .with_color("red".to_string()),
                );
                marker_map[2][2] = Some(
                    Marker::with_label(MarkerType::Cross, "Blue cross with tooltip".to_string())
                        .with_color("blue".to_string()),
                );
                marker_map[2][3] = Some(
                    Marker::with_label(MarkerType::Triangle, "Green triangle marker".to_string())
                        .with_color("green".to_string()),
                );
                marker_map[2][4] = Some(
                    Marker::with_label(MarkerType::Square, "Important marker".to_string())
                        .with_color("#FF0000".to_string()),
                );
                marker_map[2][5] = Some(
                    Marker::with_label(MarkerType::Point, "Point of interest".to_string())
                        .with_color("#0000FF".to_string()),
                );

                // Row 3: Different sizes
                marker_map[3][1] = Some(Marker::new(MarkerType::Circle).with_size(0.8));
                marker_map[3][2] = Some(Marker::new(MarkerType::Cross).with_size(1.2));
                marker_map[3][3] = Some(Marker::new(MarkerType::Triangle).with_size(1.5));
                marker_map[3][4] = Some(Marker::new(MarkerType::Square).with_size(0.6));
                marker_map[3][5] = Some(Marker::new(MarkerType::Point).with_size(2.0));

                // Row 4: Label markers
                marker_map[4][1] = Some(Marker::new(MarkerType::Label("A".to_string())));
                marker_map[4][2] = Some(
                    Marker::new(MarkerType::Label("B".to_string())).with_color("red".to_string()),
                );
                marker_map[4][3] = Some(
                    Marker::new(MarkerType::Label("1".to_string())).with_color("blue".to_string()),
                );
                marker_map[4][4] =
                    Some(Marker::new(MarkerType::Label("2".to_string())).with_size(1.5));

                // Row 5: Loader markers
                marker_map[5][2] = Some(Marker::new(MarkerType::Loader));
                marker_map[5][3] =
                    Some(Marker::new(MarkerType::Loader).with_color("red".to_string()));
                marker_map[5][4] = Some(Marker::new(MarkerType::Loader).with_size(1.3));

                // Row 6: Z-index layering demonstration - overlapping markers
                // Background layer (z-index 1)
                marker_map[6][1] = Some(
                    Marker::new(MarkerType::Circle)
                        .with_color("blue".to_string())
                        .with_size(1.2)
                        .with_z_index(1)
                        .with_style_class("bg-layer".to_string()),
                );

                // Mid layer (z-index 5) - overlaps with background
                marker_map[6][2] = Some(
                    Marker::new(MarkerType::Square)
                        .with_color("green".to_string())
                        .with_size(1.0)
                        .with_z_index(5)
                        .with_style_class("mid-layer".to_string()),
                );

                // Foreground layer (z-index 10) - should appear on top
                marker_map[6][3] = Some(
                    Marker::new(MarkerType::Cross)
                        .with_color("red".to_string())
                        .with_size(0.8)
                        .with_z_index(10)
                        .with_style_class("fg-layer".to_string()),
                );

                board.set_marker_map(marker_map);
                board
            }),
            interactive_board: cx.new(|_| GoBoard::with_size(9, 9).with_vertex_size(40.0)),
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
                section("Coordinate Labels").child(
                    v_flex()
                        .gap_2()
                        .child("13x13 Board with Coordinate Labels")
                        .child(self.coordinate_board.clone()),
                ),
            )
            .child(
                section("Stone Rendering").child(
                    h_flex()
                        .gap_6()
                        .child(
                            v_flex()
                                .gap_2()
                                .child("9x9 Board with Basic Stones")
                                .child(self.stone_board.clone()),
                        )
                        .child(
                            v_flex()
                                .gap_2()
                                .child("9x9 Board with Fuzzy Positioning")
                                .child(self.fuzzy_stone_board.clone()),
                        ),
                ),
            )
            .child(
                section("Marker Types").child(
                    v_flex()
                        .gap_2()
                        .child("9x9 Board with Different Marker Types")
                        .child("Row 1: Basic shapes, Row 2: Colored markers with tooltips (hover to see), Row 3: Different sizes, Row 4: Labels, Row 5: Loaders, Row 6: Z-index layering")
                        .child(self.marker_board.clone()),
                ),
            )
            .child(
                section("Interactive Board").child(
                    v_flex()
                        .gap_2()
                        .child("9x9 Board with Comprehensive Event Handling")
                        .child("Try different mouse interactions: click, mouse down/up, move")
                        .child(self.interactive_board.update(cx, |board, _| {
                            let handlers = VertexEventHandlers::new()
                                .with_click(|event: VertexClickEvent| {
                                    println!(
                                        "Click: ({}, {}) - coordinates: {:?}",
                                        event.vertex.x, event.vertex.y, event.coordinates
                                    );
                                })
                                .with_mouse_down(|event: VertexMouseDownEvent| {
                                    println!(
                                        "Mouse Down: ({}, {}) - button: {:?}",
                                        event.vertex.x, event.vertex.y, event.button
                                    );
                                })
                                .with_mouse_up(|event: VertexMouseUpEvent| {
                                    println!(
                                        "Mouse Up: ({}, {}) - button: {:?}",
                                        event.vertex.x, event.vertex.y, event.button
                                    );
                                })
                                .with_mouse_move(|event: VertexMouseMoveEvent| {
                                    println!(
                                        "Mouse Move: ({}, {})",
                                        event.vertex.x, event.vertex.y
                                    );
                                });

                            board.render_with_vertex_handlers(handlers)
                        })),
                ),
            )
            .child(
                section("Board Information").child(
                    v_flex()
                        .gap_2()
                        .child("Features:")
                        .child("• Grid-based layout with proper line positioning")
                        .child("• Star points (hoshi) for standard board sizes")
                        .child("• Stone rendering with signMap support (-1: white, 1: black)")
                        .child("• Fuzzy stone placement for natural appearance")
                        .child("• Random visual variation with deterministic positioning")
                        .child("• Coordinate labels with standard Go notation (A-T, 1-19)")
                        .child("• Configurable board sizes (9x9, 13x13, 19x19)")
                        .child("• Custom themes with colors and styling")
                        .child("• Responsive design with proper scaling")
                        .child("• Support for partial board ranges")
                        .child("• Comprehensive vertex interaction system")
                        .child("  - Click events for user interactions")
                        .child("  - Mouse down/up events for precise control")
                        .child("  - Mouse move events for hover feedback")
                        .child("  - Busy state support for disabling interactions")
                        .child("• Touch device support through pointer events")
                        .child("• Shudan-inspired architecture"),
                ),
            )
    }
}
