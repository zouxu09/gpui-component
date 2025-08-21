use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, InteractiveElement, IntoElement,
    ParentElement as _, Render, Styled as _, Window,
};

use gpui_component::{
    go_board::{
        BoardTheme, BoundedGoBoard, CornerPaint, DirectionalPaintMap, GhostStone,
        GhostStoneOverlay, GhostStoneType, GoBoard, GridTheme, HeatData, HeatOverlay, Line,
        LineOverlay, LineType, Marker, MarkerType, PaintOverlay, PaintType, SelectionDirection,
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
    asset_board: Entity<GoBoard>,
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
    interactive_asset_board: Entity<GoBoard>,
    bounded_small_board: Entity<BoundedGoBoard>,
    bounded_medium_board: Entity<BoundedGoBoard>,
    bounded_large_board: Entity<BoundedGoBoard>,
    bounded_constrained_board: Entity<BoundedGoBoard>,
    partial_board_center: Entity<BoundedGoBoard>,
    partial_board_corner: Entity<BoundedGoBoard>,
    partial_board_edge: Entity<BoundedGoBoard>,
    efficient_update_demo: Entity<GoBoard>,
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
                // Create a board theme using color-only rendering (no external assets)
                let textured_theme = BoardTheme::default();

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
            asset_board: cx.new(|_| {
                // Asset demo disabled to avoid missing embedded resources; use default theme
                let asset_theme = BoardTheme::default();
                let mut board = GoBoard::with_size(9, 9).with_vertex_size(35.0);
                board.set_theme(asset_theme);

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
            }),
            stone_variation_board: cx.new(|_| {
                // Stone variation demo disabled (no external variation textures)
                let variation_theme = BoardTheme::default();

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
                let mut board = GoBoard::with_size(9, 9).with_vertex_size(25.0);
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
                    vec![
                        Some(PaintType::Fill { opacity: 0.8 }),
                        Some(PaintType::Fill { opacity: 0.6 }),
                        Some(PaintType::Fill { opacity: 0.4 }),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    ],
                    vec![
                        Some(PaintType::Fill { opacity: 0.7 }),
                        None,
                        Some(PaintType::Fill { opacity: 0.5 }),
                        None,
                        Some(PaintType::Fill { opacity: 0.3 }),
                        Some(PaintType::Fill { opacity: 0.5 }),
                        Some(PaintType::Fill { opacity: 0.7 }),
                        Some(PaintType::Fill { opacity: 0.8 }),
                        Some(PaintType::Fill { opacity: 0.9 }),
                    ],
                    vec![
                        Some(PaintType::Fill { opacity: 0.6 }),
                        Some(PaintType::Fill { opacity: 0.4 }),
                        None,
                        Some(PaintType::Fill { opacity: 0.2 }),
                        None,
                        Some(PaintType::Fill { opacity: 0.4 }),
                        Some(PaintType::Fill { opacity: 0.6 }),
                        Some(PaintType::Fill { opacity: 0.7 }),
                        Some(PaintType::Fill { opacity: 0.8 }),
                    ],
                    vec![
                        Some(PaintType::Fill { opacity: 0.5 }),
                        None,
                        Some(PaintType::Fill { opacity: 0.3 }),
                        None,
                        Some(PaintType::Fill { opacity: 0.2 }),
                        Some(PaintType::Fill { opacity: 0.4 }),
                        Some(PaintType::Fill { opacity: 0.6 }),
                        Some(PaintType::Fill { opacity: 0.7 }),
                        Some(PaintType::Fill { opacity: 0.8 }),
                    ],
                    vec![
                        Some(PaintType::Fill { opacity: 0.4 }),
                        Some(PaintType::Fill { opacity: 0.3 }),
                        None,
                        Some(PaintType::Fill { opacity: 0.1 }),
                        None,
                        Some(PaintType::Fill { opacity: 0.3 }),
                        Some(PaintType::Fill { opacity: 0.5 }),
                        Some(PaintType::Fill { opacity: 0.6 }),
                        Some(PaintType::Fill { opacity: 0.7 }),
                    ],
                    vec![
                        Some(PaintType::Fill { opacity: 0.3 }),
                        Some(PaintType::Fill { opacity: 0.2 }),
                        Some(PaintType::Fill { opacity: 0.1 }),
                        None,
                        Some(PaintType::Fill { opacity: 0.1 }),
                        None,
                        Some(PaintType::Fill { opacity: 0.4 }),
                        Some(PaintType::Fill { opacity: 0.5 }),
                        Some(PaintType::Fill { opacity: 0.6 }),
                    ],
                    vec![
                        Some(PaintType::Fill { opacity: 0.2 }),
                        Some(PaintType::Fill { opacity: 0.1 }),
                        None,
                        Some(PaintType::Fill { opacity: 0.1 }),
                        None,
                        Some(PaintType::Fill { opacity: 0.2 }),
                        None,
                        Some(PaintType::Fill { opacity: 0.3 }),
                        Some(PaintType::Fill { opacity: 0.4 }),
                    ],
                    vec![
                        Some(PaintType::Fill { opacity: 0.1 }),
                        None,
                        Some(PaintType::Fill { opacity: 0.1 }),
                        Some(PaintType::Fill { opacity: 0.2 }),
                        Some(PaintType::Fill { opacity: 0.3 }),
                        Some(PaintType::Fill { opacity: 0.4 }),
                        Some(PaintType::Fill { opacity: 0.5 }),
                        Some(PaintType::Fill { opacity: 0.6 }),
                        Some(PaintType::Fill { opacity: 0.7 }),
                    ],
                    vec![None, None, None, None, None, None, None, None, None],
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
            interactive_asset_board: cx.new(|_| {
                // Create an interactive board using the specific assets
                let asset_theme = BoardTheme::default();

                let mut board = GoBoard::with_size(9, 9).with_vertex_size(40.0);
                board.set_theme(asset_theme);
                board
            }),
            bounded_small_board: cx.new(|_| {
                // Small bounded board - 9x9 in 150x150 space
                let mut bounded = BoundedGoBoard::with_size(9, 9, 150.0, 150.0);
                let sign_map = vec![
                    vec![0, 1, 0, 0, 0, 0, 0, -1, 0],
                    vec![1, 0, 1, 0, 0, 0, -1, 0, -1],
                    vec![0, 1, 0, 0, 0, 0, 0, -1, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 1, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 1, 0, 0, 0, 0, 0, -1, 0],
                    vec![1, 0, 1, 0, 0, 0, -1, 0, -1],
                    vec![0, 1, 0, 0, 0, 0, 0, -1, 0],
                ];
                bounded.set_sign_map(sign_map);
                bounded
            }),
            bounded_medium_board: cx.new(|_| {
                // Medium bounded board - 13x13 in 250x250 space
                let mut bounded = BoundedGoBoard::with_size(13, 13, 250.0, 250.0);
                bounded.set_show_coordinates(true);
                let sign_map = vec![
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 1, 0, 0, 0, 0, 0, -1, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, -1, 0, 0, 0, 0, 0, 1, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                ];
                bounded.set_sign_map(sign_map);
                bounded
            }),
            bounded_large_board: cx.new(|_| {
                // Large bounded board - 19x19 in 380x380 space
                let mut bounded = BoundedGoBoard::with_size(19, 19, 380.0, 380.0);
                let sign_map = vec![vec![0; 19]; 19]; // Empty 19x19 board
                bounded.set_sign_map(sign_map);
                bounded
            }),
            bounded_constrained_board: cx.new(|_| {
                // Constrained aspect ratio - 19x19 in 200x400 space (height-constrained)
                let mut bounded = BoundedGoBoard::with_size(19, 19, 200.0, 400.0)
                    .with_vertex_size_limits(5.0, 25.0);

                // Add some stones to show the scaling effect
                let sign_map = vec![
                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, -1],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1],
                ];
                bounded.set_sign_map(sign_map);
                bounded
            }),
            partial_board_center: cx.new(|_| {
                // Partial board showing center area of a 19x19 board
                let mut bounded = BoundedGoBoard::with_size(5, 5, 200.0, 200.0); // 5x5 visible area

                bounded.set_show_coordinates(true);

                // Create a 5x5 sign map for just the visible area (coordinates 0-4)
                let sign_map = vec![
                    vec![0, 0, 0, 0, 0],
                    vec![0, -1, 0, 1, 0],
                    vec![0, 0, 1, 0, 0],
                    vec![0, 1, 0, -1, 0],
                    vec![0, 0, 0, 0, 0],
                ];

                bounded.set_sign_map(sign_map);
                bounded
            }),
            partial_board_corner: cx.new(|_| {
                // Partial board showing corner area
                let mut bounded = BoundedGoBoard::with_size(7, 7, 200.0, 200.0); // 7x7 visible area

                bounded.set_show_coordinates(true);

                // Create a 7x7 sign map showing typical corner pattern (coordinates 0-6)
                let sign_map = vec![
                    vec![0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 1, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, -1, 0, 0],
                    vec![0, 0, 0, -1, 0, 1, 0],
                    vec![0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0],
                ];

                bounded.set_sign_map(sign_map);
                bounded
            }),
            partial_board_edge: cx.new(|_| {
                // Partial board showing side edge
                let mut bounded = BoundedGoBoard::with_size(19, 3, 300.0, 150.0); // 19x3 slice

                bounded.set_show_coordinates(true);

                // Create a 19x3 sign map for edge play (coordinates 0-18 x 0-2)
                let sign_map = vec![
                    vec![0, 0, 1, 0, 0, -1, 0, 0, 1, 0, 0, -1, 0, 0, 1, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 1, 0, -1, 0, 1, 0, -1, 0, 1, 0, -1, 0, 1, 0, 0, 0, 0, 0],
                ];

                bounded.set_sign_map(sign_map);
                bounded
            }),
            // Demonstration of efficient differential updates
            efficient_update_demo: cx.new(|_| {
                let mut board = GoBoard::with_size(9, 9).with_vertex_size(30.0);

                // Demonstrate efficient bulk updates
                let initial_stones = vec![
                    (Vertex::new(2, 2), 1),
                    (Vertex::new(6, 2), -1),
                    (Vertex::new(2, 6), -1),
                    (Vertex::new(6, 6), 1),
                    (Vertex::new(4, 4), 1), // Center stone
                ];

                board.update_stones(&initial_stones);
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
                                        .child("Asset Board")
                                        .child("Using default theme (assets disabled)")
                                        .child(self.asset_board.clone()),
                                ),
                        )
                        .child(
                            h_flex()
                                .gap_6()
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("Stone Variations")
                                        .child("Stone Variations (disabled - using default theme)")
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
                section("Bounded Go Board - Responsive Sizing").child(
                    v_flex()
                        .gap_4()
                        .child("BoundedGoBoard automatically calculates vertex size to fit within constraints")
                        .child(
                            h_flex()
                                .gap_6()
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("Small (9x9 in 150x150)")
                                        .child(format!("Vertex size: {:.1}px", self.bounded_small_board.read(cx).vertex_size()))
                                        .child(self.bounded_small_board.clone()),
                                )
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("Medium (13x13 in 250x250)")
                                        .child(format!("Vertex size: {:.1}px", self.bounded_medium_board.read(cx).vertex_size()))
                                        .child(self.bounded_medium_board.clone()),
                                ),
                        )
                        .child(
                            h_flex()
                                .gap_6()
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("Large (19x19 in 380x380)")
                                        .child(format!("Vertex size: {:.1}px", self.bounded_large_board.read(cx).vertex_size()))
                                        .child(self.bounded_large_board.clone()),
                                )
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("Constrained (19x19 in 200x400)")
                                        .child(format!("Vertex size: {:.1}px", self.bounded_constrained_board.read(cx).vertex_size()))
                                        .child("Width-constrained with size limits (5-25px)")
                                        .child(self.bounded_constrained_board.clone()),
                                ),
                        ),
                ),
            )
            .child(
                section("Partial Board Display - Range Support").child(
                    v_flex()
                        .gap_4()
                        .child("Demonstrates partial board display with proper stone alignment")
                        .child(
                            h_flex()
                                .gap_6()
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("Center Area (5x5)")
                                        .child("Typical center fighting pattern")
                                        .child(format!("Vertex size: {:.1}px", self.partial_board_center.read(cx).vertex_size()))
                                        .child(self.partial_board_center.clone()),
                                )
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("Corner Area (7x7)")
                                        .child("Corner opening pattern")
                                        .child(format!("Vertex size: {:.1}px", self.partial_board_corner.read(cx).vertex_size()))
                                        .child(self.partial_board_corner.clone()),
                                )
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("Edge Slice (19x3)")
                                        .child("Side edge patterns")
                                        .child(format!("Vertex size: {:.1}px", self.partial_board_edge.read(cx).vertex_size()))
                                        .child(self.partial_board_edge.clone()),
                                ),
                        ),
                ),
            )
            .child(
                section("Interactive Board").child(
                    v_flex()
                        .gap_4()
                        .child("9x9 Board with Comprehensive Event Handling")
                        .child("Try different mouse interactions: click, mouse down/up, move")
                        .child(
                            h_flex()
                                .gap_6()
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("Standard Interactive Board")
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
                                )
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child("Asset Board with Stone Placement")
                                        .child("Click to place stones alternating black/white")
                                        .child(self.interactive_asset_board.update(cx, |board, cx| {
                                            // Static variable to track current player (alternating black/white)
                                            // In a real app, this would be proper state management
                                            let handlers = VertexEventHandlers::new()
                                                .with_click(|event: VertexClickEvent| {
                                                    println!(
                                                        "Asset Board Click: ({}, {}) - placing stone",
                                                        event.vertex.x, event.vertex.y
                                                    );
                                                    // Note: In a real implementation, we'd need mutable access to the board
                                                    // to actually place stones. This demonstrates the event handling.
                                                });

                                            board.render_with_vertex_handlers(handlers)
                                        })),
                                ),
                        ),
                ),
            )
            .child(
                section("Efficient Update System - Differential Rendering").child(
                    v_flex()
                        .gap_4()
                        .child("Demonstrates efficient signMap updates with change detection and differential rendering")
                        .child(
                            v_flex()
                                .gap_2()
                                .child("Efficient Updates Demo")
                                .child("Uses update_stones() and update_sign_map() methods for optimal performance")
                                .child("Only re-renders elements that have actually changed")
                                .child(self.efficient_update_demo.clone()),
                        ),
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
                        .child("  - Color-only board and stones (no external assets)")
                        .child("  - Deterministic variation placement for consistent appearance")
                        .child("  - Asset loading and caching system with error handling")
                        .child("• Bounded sizing and responsive behavior")
                        .child("  - BoundedGoBoard component with automatic vertex size calculation")
                        .child("  - maxWidth/maxHeight constraints with proportional scaling")
                        .child("  - Configurable vertex size limits (min/max bounds)")
                        .child("  - Width and height constraint detection")
                        .child("  - Support for extreme aspect ratios and small displays")
                        .child("• Partial board display with range support")
                        .child("  - Shudan-style rangeX and rangeY parameter support")
                        .child("  - Efficient rendering that only processes visible board areas")
                        .child("  - Automatic coordinate label updates for partial boards")
                        .child("  - Support for arbitrary board sections (corners, edges, center)")
                        .child("  - Dynamic range updates with automatic vertex size recalculation")
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
                        .child("• Shudan-inspired architecture")
                        .child("• Efficient differential rendering system")
                        .child("  - Change detection for signMap, markerMap, and ghostStoneMap updates")
                        .child("  - Only re-renders elements that have actually changed")
                        .child("  - Optimized for large boards and frequent updates")
                        .child("  - Built-in performance monitoring and statistics")
                        .child("  - Memory-efficient state tracking and caching"),
                ),
            )
    }
}
