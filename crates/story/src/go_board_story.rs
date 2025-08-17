use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, InteractiveElement, IntoElement,
    ParentElement as _, Render, Styled as _, Window,
};

use gpui_component::{
    go_board::{
        BoardTheme, CornerPaint, DirectionalPaintMap, GhostStone, GhostStoneOverlay,
        GhostStoneType, GoBoard, GridTheme, HeatData, HeatOverlay, Line, LineOverlay, LineType,
        Marker, MarkerType, PaintOverlay, SelectionDirection, TextureThemeAdapter, TextureUtils,
        Vertex, VertexClickEvent, VertexEventHandlers, VertexMouseDownEvent, VertexMouseMoveEvent,
        VertexMouseUpEvent, VertexSelection,
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
    dark_theme_board: Entity<GoBoard>,
    minimalist_theme_board: Entity<GoBoard>,
    high_contrast_board: Entity<GoBoard>,
    textured_board: Entity<GoBoard>,
    stone_variation_board: Entity<GoBoard>,
    coordinate_board: Entity<GoBoard>,
    stone_board: Entity<GoBoard>,
    fuzzy_stone_board: Entity<GoBoard>,
    marker_board: Entity<GoBoard>,
    selection_board: Entity<GoBoard>,
    paint_overlay_board: Entity<GoBoard>,
    heat_overlay_board: Entity<GoBoard>,
    ghost_stone_board: Entity<GoBoard>,
    line_board: Entity<GoBoard>,
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
                // Create a custom theme using the new BoardTheme system
                let custom_theme = BoardTheme::default()
                    .with_board_background(gpui::rgb(0x8B7355)) // Darker wood
                    .with_grid_lines(gpui::rgb(0x2c2c2c), 1.5) // Dark gray lines, thicker
                    .with_stone_colors(gpui::rgb(0x000000), gpui::rgb(0xffffff)) // Pure B&W stones
                    .with_coordinates(gpui::rgb(0x654321), 12.0, 0.8); // Dark brown coordinates

                let mut board = GoBoard::with_size(9, 9).with_vertex_size(30.0);
                board.set_theme(custom_theme);
                board
            }),
            dark_theme_board: cx.new(|_| {
                let mut board = GoBoard::with_size(9, 9).with_vertex_size(30.0);
                board.set_theme(BoardTheme::dark());
                board
            }),
            minimalist_theme_board: cx.new(|_| {
                let mut board = GoBoard::with_size(9, 9).with_vertex_size(30.0);
                board.set_theme(BoardTheme::minimalist());
                board
            }),
            high_contrast_board: cx.new(|_| {
                let mut board = GoBoard::with_size(9, 9).with_vertex_size(30.0);
                board.set_theme(BoardTheme::high_contrast());
                board
            }),
            textured_board: cx.new(|_| {
                // Create a board with texture support demonstration
                let textured_theme = BoardTheme::default()
                    .with_board_texture("assets/wood_texture.png".to_string())
                    .with_stone_textures(
                        Some("assets/black_stone.png".to_string()),
                        Some("assets/white_stone.png".to_string()),
                    );

                let mut board = GoBoard::with_size(9, 9).with_vertex_size(30.0);
                board.set_theme(textured_theme);

                // Add some stones to demonstrate textured stones
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
            }),
            stone_variation_board: cx.new(|_| {
                // Create a board with random stone variations
                let variation_theme = BoardTheme::default()
                    .with_standard_stone_variations("assets/stone_variations")
                    .with_random_variation(true, 8.0); // Enable rotation variation too

                let mut board = GoBoard::with_size(9, 9).with_vertex_size(35.0);
                board.set_theme(variation_theme);

                // Add many stones to demonstrate variation
                let sign_map = vec![
                    vec![1, -1, 1, -1, 1, -1, 1, -1, 1],
                    vec![-1, 1, -1, 1, -1, 1, -1, 1, -1],
                    vec![1, -1, 1, -1, 1, -1, 1, -1, 1],
                    vec![-1, 1, -1, 1, -1, 1, -1, 1, -1],
                    vec![1, -1, 1, -1, 0, -1, 1, -1, 1],
                    vec![-1, 1, -1, 1, -1, 1, -1, 1, -1],
                    vec![1, -1, 1, -1, 1, -1, 1, -1, 1],
                    vec![-1, 1, -1, 1, -1, 1, -1, 1, -1],
                    vec![1, -1, 1, -1, 1, -1, 1, -1, 1],
                ];
                board.set_sign_map(sign_map);
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
            selection_board: cx.new(|_| {
                let mut board = GoBoard::with_size(9, 9).with_vertex_size(35.0);

                // Create some stones for context
                let sign_map = vec![
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 1, 0, -1, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 1, 0, 0, 0, 0],
                    vec![0, -1, 0, 0, 0, -1, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 1, 0, 0, -1, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                ];
                board.set_sign_map(sign_map);

                // Set up vertex selections to demonstrate different states
                use gpui_component::go_board::Vertex;

                // Selected vertices (highlighted in blue)
                let selected_vertices = vec![
                    Vertex::new(2, 2), // Normal selection
                    Vertex::new(6, 6), // Another selection
                ];
                board.set_selected_vertices(selected_vertices);

                // Dimmed vertices (reduced opacity)
                let dimmed_vertices = vec![
                    Vertex::new(0, 0), // Dimmed corner
                    Vertex::new(8, 0), // Dimmed corner
                    Vertex::new(0, 8), // Dimmed corner
                    Vertex::new(8, 8), // Dimmed corner
                    Vertex::new(4, 4), // Center dimmed
                ];
                board.set_dimmed_vertices(dimmed_vertices);

                // Directional selection indicators
                board.set_selected_left(vec![Vertex::new(1, 4)]); // Red left indicator
                board.set_selected_right(vec![Vertex::new(7, 4)]); // Green right indicator
                board.set_selected_top(vec![Vertex::new(4, 1)]); // Orange top indicator
                board.set_selected_bottom(vec![Vertex::new(4, 7)]); // Purple bottom indicator

                board
            }),
            paint_overlay_board: cx.new(|_| {
                let mut board = GoBoard::with_size(9, 9).with_vertex_size(35.0);

                // Create some stones for context
                let sign_map = vec![
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 1, 0, -1, 0, 0, 0, 0, 0],
                    vec![0, 0, 1, 0, -1, 0, 0, 0, 0],
                    vec![0, -1, 0, 1, 0, 0, 0, 0, 0],
                    vec![0, 0, -1, 0, 1, 0, 0, 0, 0],
                    vec![0, 0, 0, -1, 0, 1, 0, 0, 0],
                    vec![0, 0, 0, 0, -1, 0, 1, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                ];
                board.set_sign_map(sign_map);

                // Create paint map for territory analysis
                let paint_map = vec![
                    vec![0.8, 0.6, 0.4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                    vec![0.7, 0.0, 0.5, 0.0, -0.3, -0.5, -0.7, -0.8, -0.9],
                    vec![0.6, 0.4, 0.0, 0.2, 0.0, -0.4, -0.6, -0.7, -0.8],
                    vec![0.5, 0.0, 0.3, 0.0, -0.2, -0.4, -0.6, -0.7, -0.8],
                    vec![0.4, 0.3, 0.0, 0.1, 0.0, -0.3, -0.5, -0.6, -0.7],
                    vec![0.3, 0.2, 0.1, 0.0, -0.1, 0.0, -0.4, -0.5, -0.6],
                    vec![0.2, 0.1, 0.0, -0.1, 0.0, 0.2, 0.0, -0.3, -0.4],
                    vec![0.1, 0.0, -0.1, -0.2, -0.3, -0.4, -0.5, -0.6, -0.7],
                    vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                ];
                board.set_paint_map(paint_map);

                board
            }),
            heat_overlay_board: cx.new(|_| {
                let mut board = GoBoard::with_size(9, 9).with_vertex_size(35.0);

                // Create some stones for context
                let sign_map = vec![
                    vec![0, 0, 0, 1, 0, -1, 0, 0, 0],
                    vec![0, 1, 0, 0, 0, 0, 0, -1, 0],
                    vec![0, 0, 0, 1, 0, -1, 0, 0, 0],
                    vec![1, 0, 0, 0, 0, 0, 0, 0, -1],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![-1, 0, 0, 0, 0, 0, 0, 0, 1],
                    vec![0, 0, 0, -1, 0, 1, 0, 0, 0],
                    vec![0, -1, 0, 0, 0, 0, 0, 1, 0],
                    vec![0, 0, 0, -1, 0, 1, 0, 0, 0],
                ];
                board.set_sign_map(sign_map);

                // Create heat map for influence analysis
                let heat_map = vec![
                    vec![
                        Some(HeatData::with_text(2, "2".to_string())),
                        Some(HeatData::new(4)),
                        Some(HeatData::new(6)),
                        Some(HeatData::new(3)),
                        None,
                        Some(HeatData::new(3)),
                        Some(HeatData::new(6)),
                        Some(HeatData::new(4)),
                        Some(HeatData::with_text(2, "2".to_string())),
                    ],
                    vec![
                        Some(HeatData::new(3)),
                        None,
                        Some(HeatData::new(5)),
                        Some(HeatData::new(7)),
                        Some(HeatData::new(8)),
                        Some(HeatData::new(7)),
                        Some(HeatData::new(5)),
                        None,
                        Some(HeatData::new(3)),
                    ],
                    vec![
                        Some(HeatData::new(4)),
                        Some(HeatData::new(6)),
                        Some(HeatData::new(8)),
                        None,
                        Some(HeatData::with_text(9, "MAX".to_string())),
                        None,
                        Some(HeatData::new(8)),
                        Some(HeatData::new(6)),
                        Some(HeatData::new(4)),
                    ],
                    vec![
                        None,
                        Some(HeatData::new(7)),
                        Some(HeatData::with_text(9, "H".to_string())),
                        Some(HeatData::new(8)),
                        Some(HeatData::new(7)),
                        Some(HeatData::new(8)),
                        Some(HeatData::with_text(9, "H".to_string())),
                        Some(HeatData::new(7)),
                        None,
                    ],
                    vec![
                        Some(HeatData::new(5)),
                        Some(HeatData::new(8)),
                        Some(HeatData::new(7)),
                        Some(HeatData::new(6)),
                        Some(HeatData::new(5)),
                        Some(HeatData::new(6)),
                        Some(HeatData::new(7)),
                        Some(HeatData::new(8)),
                        Some(HeatData::new(5)),
                    ],
                    vec![
                        None,
                        Some(HeatData::new(7)),
                        Some(HeatData::with_text(9, "H".to_string())),
                        Some(HeatData::new(8)),
                        Some(HeatData::new(7)),
                        Some(HeatData::new(8)),
                        Some(HeatData::with_text(9, "H".to_string())),
                        Some(HeatData::new(7)),
                        None,
                    ],
                    vec![
                        Some(HeatData::new(4)),
                        Some(HeatData::new(6)),
                        Some(HeatData::new(8)),
                        None,
                        Some(HeatData::with_text(9, "MAX".to_string())),
                        None,
                        Some(HeatData::new(8)),
                        Some(HeatData::new(6)),
                        Some(HeatData::new(4)),
                    ],
                    vec![
                        Some(HeatData::new(3)),
                        None,
                        Some(HeatData::new(5)),
                        Some(HeatData::new(7)),
                        Some(HeatData::new(8)),
                        Some(HeatData::new(7)),
                        Some(HeatData::new(5)),
                        None,
                        Some(HeatData::new(3)),
                    ],
                    vec![
                        Some(HeatData::with_text(2, "2".to_string())),
                        Some(HeatData::new(4)),
                        Some(HeatData::new(6)),
                        Some(HeatData::new(3)),
                        None,
                        Some(HeatData::new(3)),
                        Some(HeatData::new(6)),
                        Some(HeatData::new(4)),
                        Some(HeatData::with_text(2, "2".to_string())),
                    ],
                ];
                board.set_heat_map(heat_map);

                board
            }),
            ghost_stone_board: cx.new(|_| {
                let mut board = GoBoard::with_size(9, 9).with_vertex_size(35.0);

                // Create some stones for context
                let sign_map = vec![
                    vec![0, 0, 0, 1, 0, -1, 0, 0, 0],
                    vec![0, 1, 0, 0, 0, 0, 0, -1, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![1, 0, 0, 0, 0, 0, 0, 0, -1],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![-1, 0, 0, 0, 0, 0, 0, 0, 1],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, -1, 0, 0, 0, 0, 0, 1, 0],
                    vec![0, 0, 0, -1, 0, 1, 0, 0, 0],
                ];
                board.set_sign_map(sign_map);

                // Create ghost stone map for analysis visualization
                let ghost_stone_map = vec![
                    vec![
                        Some(GhostStone::new(1, GhostStoneType::Good)),
                        Some(GhostStone::new(-1, GhostStoneType::Interesting)),
                        Some(GhostStone::new(1, GhostStoneType::Doubtful)),
                        None,
                        Some(GhostStone::new(-1, GhostStoneType::Good).faint()),
                        None,
                        Some(GhostStone::new(1, GhostStoneType::Bad)),
                        Some(GhostStone::new(-1, GhostStoneType::Interesting)),
                        Some(GhostStone::new(1, GhostStoneType::Good)),
                    ],
                    vec![
                        Some(GhostStone::new(-1, GhostStoneType::Doubtful)),
                        None,
                        Some(GhostStone::new(1, GhostStoneType::Good)),
                        Some(GhostStone::new(-1, GhostStoneType::Bad).faint()),
                        Some(GhostStone::new(1, GhostStoneType::Interesting)),
                        Some(GhostStone::new(-1, GhostStoneType::Doubtful)),
                        Some(GhostStone::new(1, GhostStoneType::Bad)),
                        None,
                        Some(GhostStone::new(-1, GhostStoneType::Good)),
                    ],
                    vec![
                        Some(GhostStone::new(1, GhostStoneType::Bad).faint()),
                        Some(GhostStone::new(-1, GhostStoneType::Good)),
                        Some(GhostStone::new(1, GhostStoneType::Interesting).faint()),
                        Some(GhostStone::new(-1, GhostStoneType::Doubtful)),
                        Some(GhostStone::new(1, GhostStoneType::Bad)),
                        Some(GhostStone::new(-1, GhostStoneType::Interesting)),
                        Some(GhostStone::new(1, GhostStoneType::Good).faint()),
                        Some(GhostStone::new(-1, GhostStoneType::Bad)),
                        Some(GhostStone::new(1, GhostStoneType::Doubtful)),
                    ],
                    vec![
                        None,
                        Some(GhostStone::new(-1, GhostStoneType::Interesting).faint()),
                        Some(GhostStone::new(1, GhostStoneType::Good)),
                        Some(GhostStone::new(-1, GhostStoneType::Bad)),
                        Some(GhostStone::new(1, GhostStoneType::Doubtful).faint()),
                        Some(GhostStone::new(-1, GhostStoneType::Good)),
                        Some(GhostStone::new(1, GhostStoneType::Interesting)),
                        Some(GhostStone::new(-1, GhostStoneType::Doubtful)),
                        None,
                    ],
                    vec![
                        Some(GhostStone::new(1, GhostStoneType::Good)),
                        Some(GhostStone::new(-1, GhostStoneType::Bad).faint()),
                        Some(GhostStone::new(1, GhostStoneType::Interesting)),
                        Some(GhostStone::new(-1, GhostStoneType::Doubtful)),
                        Some(GhostStone::new(1, GhostStoneType::Good).faint()),
                        Some(GhostStone::new(-1, GhostStoneType::Bad)),
                        Some(GhostStone::new(1, GhostStoneType::Interesting).faint()),
                        Some(GhostStone::new(-1, GhostStoneType::Doubtful)),
                        Some(GhostStone::new(1, GhostStoneType::Good)),
                    ],
                    vec![
                        None,
                        Some(GhostStone::new(-1, GhostStoneType::Doubtful)),
                        Some(GhostStone::new(1, GhostStoneType::Bad).faint()),
                        Some(GhostStone::new(-1, GhostStoneType::Good)),
                        Some(GhostStone::new(1, GhostStoneType::Interesting)),
                        Some(GhostStone::new(-1, GhostStoneType::Doubtful).faint()),
                        Some(GhostStone::new(1, GhostStoneType::Bad)),
                        Some(GhostStone::new(-1, GhostStoneType::Good)),
                        None,
                    ],
                    vec![
                        Some(GhostStone::new(1, GhostStoneType::Interesting).faint()),
                        Some(GhostStone::new(-1, GhostStoneType::Bad)),
                        Some(GhostStone::new(1, GhostStoneType::Good)),
                        Some(GhostStone::new(-1, GhostStoneType::Doubtful).faint()),
                        Some(GhostStone::new(1, GhostStoneType::Interesting)),
                        Some(GhostStone::new(-1, GhostStoneType::Bad)),
                        Some(GhostStone::new(1, GhostStoneType::Good).faint()),
                        Some(GhostStone::new(-1, GhostStoneType::Doubtful)),
                        Some(GhostStone::new(1, GhostStoneType::Bad)),
                    ],
                    vec![
                        Some(GhostStone::new(-1, GhostStoneType::Good)),
                        None,
                        Some(GhostStone::new(1, GhostStoneType::Doubtful).faint()),
                        Some(GhostStone::new(-1, GhostStoneType::Interesting)),
                        Some(GhostStone::new(1, GhostStoneType::Bad)),
                        Some(GhostStone::new(-1, GhostStoneType::Good).faint()),
                        Some(GhostStone::new(1, GhostStoneType::Doubtful)),
                        None,
                        Some(GhostStone::new(-1, GhostStoneType::Interesting)),
                    ],
                    vec![
                        Some(GhostStone::new(1, GhostStoneType::Bad).faint()),
                        Some(GhostStone::new(-1, GhostStoneType::Good)),
                        Some(GhostStone::new(1, GhostStoneType::Interesting)),
                        None,
                        Some(GhostStone::new(-1, GhostStoneType::Doubtful).faint()),
                        None,
                        Some(GhostStone::new(1, GhostStoneType::Bad)),
                        Some(GhostStone::new(-1, GhostStoneType::Good)),
                        Some(GhostStone::new(1, GhostStoneType::Interesting).faint()),
                    ],
                ];
                board.set_ghost_stone_map(ghost_stone_map);

                board
            }),
            line_board: cx.new(|_| {
                let mut board = GoBoard::with_size(9, 9).with_vertex_size(35.0);

                // Create some stones for context
                let sign_map = vec![
                    vec![0, 0, 0, 1, 0, -1, 0, 0, 0],
                    vec![0, 1, 0, 0, 0, 0, 0, -1, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![1, 0, 0, 0, 0, 0, 0, 0, -1],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![-1, 0, 0, 0, 0, 0, 0, 0, 1],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, -1, 0, 0, 0, 0, 0, 1, 0],
                    vec![0, 0, 0, -1, 0, 1, 0, 0, 0],
                ];
                board.set_sign_map(sign_map);

                // Create line demonstrations
                let lines = vec![
                    // Horizontal line connecting stones
                    Line::line(Vertex::new(3, 0), Vertex::new(5, 0)),
                    // Vertical line
                    Line::line(Vertex::new(1, 1), Vertex::new(1, 4)),
                    // Diagonal line
                    Line::line(Vertex::new(2, 2), Vertex::new(6, 6)),
                    // Arrows showing analysis directions
                    Line::arrow(Vertex::new(0, 3), Vertex::new(3, 6)),
                    Line::arrow(Vertex::new(8, 3), Vertex::new(5, 6)),
                    // Connection arrows between stones
                    Line::arrow(Vertex::new(3, 8), Vertex::new(5, 8)),
                    Line::arrow(Vertex::new(7, 7), Vertex::new(7, 4)),
                    // Analysis arrows
                    Line::arrow(Vertex::new(4, 4), Vertex::new(6, 2)),
                    Line::arrow(Vertex::new(4, 4), Vertex::new(2, 6)),
                    // Multiple line types demonstration
                    Line::line(Vertex::new(0, 7), Vertex::new(2, 7)), // Short horizontal
                    Line::line(Vertex::new(6, 1), Vertex::new(8, 1)), // Short horizontal
                    Line::arrow(Vertex::new(1, 0), Vertex::new(1, 2)), // Short vertical arrow
                    Line::arrow(Vertex::new(7, 8), Vertex::new(7, 6)), // Short vertical arrow
                ];
                board.set_lines(lines);

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
                section("Theming System").child(
                    v_flex()
                        .gap_4()
                        .child("BoardTheme provides comprehensive theming with CSS custom property support")
                        .child(
                            h_flex()
                                .gap_6()
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("Custom Theme (Builder Pattern)")
                                        .child(self.custom_theme_board.clone()),
                                )
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("Dark Theme")
                                        .child(self.dark_theme_board.clone()),
                                ),
                        )
                        .child(
                            h_flex()
                                .gap_6()
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("Minimalist Theme")
                                        .child(self.minimalist_theme_board.clone()),
                                )
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("High Contrast (Accessibility)")
                                        .child(self.high_contrast_board.clone()),
                                ),
                        ),
                ),
            )
            .child(
                section("Texture and Asset Support").child(
                    v_flex()
                        .gap_4()
                        .child("Advanced texture loading and stone variation system")
                        .child(
                            h_flex()
                                .gap_6()
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("Textured Board")
                                        .child("Board texture + custom stone images")
                                        .child(self.textured_board.clone()),
                                )
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("Stone Variations")
                                        .child("Random texture variations (random_0-4)")
                                        .child(self.stone_variation_board.clone()),
                                ),
                        ),
                ),
            )
            .child(
                section("Legacy Theme Support").child(
                    v_flex()
                        .gap_2()
                        .child("Backward compatibility with GridTheme and StoneTheme")
                        .child("(Same visual as Custom Theme above)")
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
                section("Vertex Selection").child(
                    v_flex()
                        .gap_2()
                        .child("9x9 Board with Vertex Selection and Directional Indicators")
                        .child("Blue circles: Selected vertices, Dimmed areas: Reduced opacity vertices")
                        .child("Red/Green/Orange/Purple: Directional selection indicators (left/right/top/bottom)")
                        .child(self.selection_board.clone()),
                ),
            )
            .child(
                section("Paint Overlay - Territory Analysis").child(
                    v_flex()
                        .gap_2()
                        .child("9x9 Board with Paint Map Overlay for Territory Visualization")
                        .child("Blue regions: Black territory (positive values), Gray regions: White territory (negative values)")
                        .child("Intensity varies with paint value strength (-1.0 to 1.0)")
                        .child(self.paint_overlay_board.clone()),
                ),
            )
            .child(
                section("Heat Map - Influence Visualization").child(
                    v_flex()
                        .gap_2()
                        .child("9x9 Board with Heat Map for Positional Influence Analysis")
                        .child("Color gradient: Blue (low influence) → Cyan → Yellow → Red (high influence)")
                        .child("Strength values 0-9 with optional text labels (hover to see effect)")
                        .child(self.heat_overlay_board.clone()),
                ),
            )
            .child(
                section("Ghost Stones - Analysis Visualization").child(
                    v_flex()
                        .gap_2()
                        .child("9x9 Board with Ghost Stones for Move Analysis")
                        .child("Green: Good moves, Blue: Interesting moves, Yellow: Doubtful moves, Red: Bad moves")
                        .child("Faint ghost stones have reduced opacity for subtle display")
                        .child(self.ghost_stone_board.clone()),
                ),
            )
            .child(
                section("Lines and Arrows - Connection Visualization").child(
                    v_flex()
                        .gap_2()
                        .child("9x9 Board with Lines and Arrows for Board Analysis")
                        .child("Gray lines: Simple connections, Dark arrows: Directional analysis")
                        .child("Demonstrates various line orientations and arrow directions")
                        .child(self.line_board.clone()),
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
                        .child("• BoardTheme system with CSS custom property generation")
                        .child("  - Supports --board-background-color, --grid-line-color, etc.")
                        .child("  - Theme builder pattern for easy customization")
                        .child("  - Predefined themes: default, dark, minimalist, high-contrast")
                        .child("  - Backward compatibility with GridTheme and StoneTheme")
                        .child("• Advanced texture and asset support")
                        .child("  - Board background textures with GPUI image rendering")
                        .child("  - Custom stone images with fallback to solid colors")
                        .child("  - Random stone variation textures (random_0 through random_4)")
                        .child("  - Deterministic variation placement for consistent appearance")
                        .child("  - Asset loading and caching system with error handling")
                        .child("• Responsive design with proper scaling")
                        .child("• Support for partial board ranges")
                        .child("• Comprehensive vertex interaction system")
                        .child("  - Click events for user interactions")
                        .child("  - Mouse down/up events for precise control")
                        .child("  - Mouse move events for hover feedback")
                        .child("  - Busy state support for disabling interactions")
                        .child("• Vertex selection and highlighting system")
                        .child("  - Selected vertices with visual highlighting")
                        .child("  - Dimmed vertices with opacity control")
                        .child("  - Directional selection indicators")
                        .child("  - Efficient selection state management")
                        .child("• Touch device support through pointer events")
                        .child("• Shudan-inspired architecture"),
                ),
            )
    }
}
