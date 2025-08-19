/// Go Board UI Widget - Usage Documentation and Examples
///
/// This document provides comprehensive examples of how to use the Go board UI widget
/// in GPUI applications. Each section demonstrates a specific feature with complete
/// code examples and explanations.
use gpui::rgb;
use gpui_component::go_board::{
    BoardTheme, BoundedGoBoard, GhostStone, GhostStoneType, GoBoard, HeatData, Line, LineType,
    Marker, MarkerType, Vertex, VertexEventHandlers,
};

/// # Basic Board Setup
///
/// The simplest way to create a Go board widget:
///
/// ```rust
/// use gpui_component::go_board::GoBoard;
///
/// // Create a standard 19x19 board
/// let board = GoBoard::new();
///
/// // Create custom size boards
/// let small_board = GoBoard::with_size(9, 9);
/// let medium_board = GoBoard::with_size(13, 13);
///
/// // Set vertex size for scaling
/// let scaled_board = GoBoard::with_size(9, 9).with_vertex_size(30.0);
/// ```
pub fn basic_board_examples() {
    // Example 1: Standard 19x19 board
    let _standard_board = GoBoard::new();

    // Example 2: Custom sized board with scaling
    let _custom_board = GoBoard::with_size(9, 9).with_vertex_size(25.0);

    // Example 3: Board with coordinate labels
    let mut _coordinate_board = GoBoard::with_size(13, 13);
    // board.set_show_coordinates(true);
}

/// # Stone Placement and Sign Maps
///
/// The signMap is the primary way to place stones on the board:
/// - -1 = White stone
/// - 0 = Empty intersection
/// - 1 = Black stone
///
/// ```rust
/// // Create a 9x9 sign map
/// let sign_map = vec![
///     vec![0, 0, 0, 1, 0, -1, 0, 0, 0],  // Row 0: Black at (3,0), White at (5,0)
///     vec![0, 1, 0, 0, 0, 0, 0, -1, 0],  // Row 1: Black at (1,1), White at (7,1)
///     // ... more rows
/// ];
/// board.set_sign_map(sign_map);
///
/// // Individual stone placement
/// board.set_stone(&Vertex::new(4, 4), 1);  // Black stone at center
///
/// // Bulk updates for efficiency
/// let updates = vec![
///     (Vertex::new(3, 3), 1),   // Black
///     (Vertex::new(5, 5), -1),  // White
///     (Vertex::new(7, 7), 0),   // Remove stone
/// ];
/// board.update_stones(&updates);
/// ```
pub fn stone_placement_examples() {
    let mut board = GoBoard::with_size(9, 9);

    // Example 1: Full sign map
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

    // Example 2: Individual stone placement
    board.set_stone(&Vertex::new(4, 4), 1); // Black at center

    // Example 3: Bulk updates
    let updates = vec![
        (Vertex::new(2, 2), 1),
        (Vertex::new(6, 6), -1),
        (Vertex::new(2, 6), 1),
        (Vertex::new(6, 2), -1),
    ];
    board.update_stones(&updates);
}

/// # Comprehensive Theming System
///
/// The BoardTheme system provides extensive customization options:
///
/// ```rust
/// use gpui_component::go_board::BoardTheme;
///
/// // Use predefined themes
/// board.set_theme(BoardTheme::dark());
/// board.set_theme(BoardTheme::minimalist());
/// board.set_theme(BoardTheme::high_contrast());
///
/// // Create custom theme using builder pattern
/// let custom_theme = BoardTheme::default()
///     .with_board_background(rgb(0x8B7355))          // Wood color
///     .with_grid_lines(rgb(0x2c2c2c), 1.5)           // Dark gray, thick lines
///     .with_stone_colors(rgb(0x000000), rgb(0xffffff)) // Pure black/white
///     .with_coordinates(rgb(0x654321), 12.0, 0.8)    // Brown coordinates
///     .with_selection_style(rgb(0x268bd2), 0.4);     // Blue selection
///
/// board.set_theme(custom_theme);
/// ```
pub fn theming_examples() {
    let mut board = GoBoard::with_size(9, 9);

    // Example 1: Predefined themes
    board.set_theme(BoardTheme::dark());
    // board.set_theme(BoardTheme::minimalist());
    // board.set_theme(BoardTheme::high_contrast());

    // Example 2: Custom theme with builder pattern
    let custom_theme = BoardTheme::default()
        .with_board_background(rgb(0x2d3748)) // Slate blue
        .with_grid_lines(rgb(0x4a5568), 1.5) // Medium gray
        .with_stone_colors(rgb(0x1a202c), rgb(0xf7fafc)); // Dark vs light

    board.set_theme(custom_theme);

    // Example 3: Theme with texture support
    let textured_theme = BoardTheme::default()
        // Use default theme (no external assets)
        .with_random_variation(false, 0.0);

    board.set_theme(textured_theme);
}

/// # Marker System for Annotations
///
/// Markers provide visual annotations on the board:
///
/// ```rust
/// // Create marker map
/// let mut marker_map = vec![vec![None; 9]; 9];
///
/// // Basic marker types
/// marker_map[3][3] = Some(MarkerType::Circle);
/// marker_map[3][4] = Some(MarkerType::Cross);
/// marker_map[3][5] = Some(MarkerType::Triangle);
/// marker_map[3][6] = Some(MarkerType::Square);
/// marker_map[3][7] = Some(MarkerType::Point);
///
/// // Label markers
/// marker_map[4][3] = Some(MarkerType::Label("A".to_string()));
/// marker_map[4][4] = Some(MarkerType::Label("1".to_string()));
/// marker_map[4][5] = Some(MarkerType::Label("★".to_string()));
///
/// // Loader/spinner markers
/// marker_map[5][3] = Some(MarkerType::Loader);
///
/// board.set_marker_map(marker_map);
/// ```
pub fn marker_examples() {
    let mut board = GoBoard::with_size(9, 9);

    // Create comprehensive marker demonstration
    let mut marker_map = vec![vec![None; 9]; 9];

    // Row 1: Basic shapes
    marker_map[1][1] = Some(MarkerType::Circle);
    marker_map[1][2] = Some(MarkerType::Cross);
    marker_map[1][3] = Some(MarkerType::Triangle);
    marker_map[1][4] = Some(MarkerType::Square);
    marker_map[1][5] = Some(MarkerType::Point);

    // Row 2: Label markers
    marker_map[2][1] = Some(MarkerType::Label("A".to_string()));
    marker_map[2][2] = Some(MarkerType::Label("B".to_string()));
    marker_map[2][3] = Some(MarkerType::Label("1".to_string()));
    marker_map[2][4] = Some(MarkerType::Label("2".to_string()));
    marker_map[2][5] = Some(MarkerType::Label("★".to_string()));

    // Row 3: Special markers
    marker_map[3][1] = Some(MarkerType::Loader);

    board.set_marker_map(marker_map);
}

/// # Heat Maps for Influence Visualization
///
/// Heat maps display positional strength with color gradients:
///
/// ```rust
/// // Create heat map with values 0-9
/// let mut heat_map = vec![vec![0; 9]; 9];
///
/// // High influence areas
/// heat_map[3][3] = 9;  // Maximum influence
/// heat_map[4][4] = 8;  // Very high
/// heat_map[5][5] = 7;  // High
///
/// // Medium influence
/// heat_map[2][2] = 5;
/// heat_map[6][6] = 4;
///
/// // Low influence
/// heat_map[1][1] = 2;
/// heat_map[7][7] = 1;
///
/// board.set_heat_map(heat_map);
/// ```
pub fn heat_map_examples() {
    let mut board = GoBoard::with_size(9, 9);

    // Create influence-based heat map
    let mut heat_map = vec![vec![0; 9]; 9];

    // Center has highest influence
    heat_map[4][4] = 9;

    // Surrounding areas have decreasing influence
    for distance in 1..4 {
        let influence = 9 - distance * 2;
        for dx in -distance..=distance {
            for dy in -distance..=distance {
                let x = (4_i32 + dx) as usize;
                let y = (4_i32 + dy) as usize;
                if x < 9 && y < 9 && heat_map[y][x] == 0 {
                    heat_map[y][x] = influence.max(0) as u8;
                }
            }
        }
    }

    board.set_heat_map(heat_map);
}

/// # Paint Overlays for Territory Visualization
///
/// Paint overlays show territory control with colored regions:
///
/// ```rust
/// // Create paint map with values from -1.0 to 1.0
/// let mut paint_map = vec![vec![None; 9]; 9];
///
/// // Black territory (positive values)
/// for i in 0..4 {
///     for j in 0..4 {
///         paint_map[i][j] = Some(PaintType::Fill { opacity: 0.3 });
///     }
/// }
///
/// // White territory (negative values)
/// for i in 5..9 {
///     for j in 5..9 {
///         paint_map[i][j] = Some(PaintType::Fill { opacity: 0.3 });
///     }
/// }
///
/// board.set_paint_map(paint_map);
/// ```
pub fn paint_overlay_examples() {
    let mut board = GoBoard::with_size(9, 9);

    let mut paint_map = vec![vec![None; 9]; 9];

    // Upper-left: Black territory
    for i in 0..4 {
        for j in 0..4 {
            paint_map[i][j] = Some(gpui_component::go_board::PaintType::Fill { opacity: 0.4 });
        }
    }

    // Lower-right: White territory
    for i in 5..9 {
        for j in 5..9 {
            paint_map[i][j] = Some(gpui_component::go_board::PaintType::Fill { opacity: 0.4 });
        }
    }

    board.set_paint_map(paint_map);
}

/// # Ghost Stones for Move Analysis
///
/// Ghost stones show potential moves with different analysis types:
///
/// ```rust
/// // Create ghost stone map
/// let mut ghost_map = vec![vec![None; 9]; 9];
///
/// // Different analysis types
/// ghost_map[3][3] = Some(GhostStone::new(1, \"good\"));        // Good black move
/// ghost_map[5][5] = Some(GhostStone::new(-1, \"interesting\")); // Interesting white move
/// ghost_map[4][4] = Some(GhostStone::new(1, \"doubtful\"));    // Doubtful black move
/// ghost_map[6][6] = Some(GhostStone::new(-1, \"bad\"));        // Bad white move
///
/// // Faint ghost stones for subtle display
/// ghost_map[2][2] = Some(GhostStone::new(1, \"good\").faint());
///
/// board.set_ghost_stone_map(ghost_map);
/// ```
pub fn ghost_stone_examples() {
    let mut board = GoBoard::with_size(9, 9);

    let mut ghost_map = vec![vec![None; 9]; 9];

    // Analysis moves with different types
    ghost_map[2][2] = Some(GhostStone {
        sign: 1,
        ghost_type: Some("good".to_string()),
        faint: false,
    });

    ghost_map[6][6] = Some(GhostStone {
        sign: -1,
        ghost_type: Some("interesting".to_string()),
        faint: true,
    });

    ghost_map[2][6] = Some(GhostStone {
        sign: 1,
        ghost_type: Some("doubtful".to_string()),
        faint: false,
    });

    ghost_map[6][2] = Some(GhostStone {
        sign: -1,
        ghost_type: Some("bad".to_string()),
        faint: false,
    });

    board.set_ghost_stone_map(ghost_map);
}

/// # Lines and Arrows for Analysis
///
/// Lines show connections and analysis paths:
///
/// ```rust
/// // Create lines for board analysis
/// let lines = vec![
///     // Simple line connection
///     Line::line(Vertex::new(3, 3), Vertex::new(5, 5)),
///
///     // Directional arrow
///     Line::arrow(Vertex::new(1, 1), Vertex::new(7, 7)),
///
///     // Analysis arrows
///     Line::arrow(Vertex::new(4, 1), Vertex::new(4, 7)),  // Vertical
///     Line::arrow(Vertex::new(1, 4), Vertex::new(7, 4)),  // Horizontal
/// ];
///
/// board.set_lines(lines);
/// ```
pub fn line_examples() {
    let mut board = GoBoard::with_size(9, 9);

    let lines = vec![
        // Diagonal connection
        Line {
            v1: Vertex::new(2, 2),
            v2: Vertex::new(6, 6),
            line_type: "line".to_string(),
        },
        // Analysis arrows
        Line {
            v1: Vertex::new(4, 1),
            v2: Vertex::new(4, 7),
            line_type: "arrow".to_string(),
        },
        Line {
            v1: Vertex::new(1, 4),
            v2: Vertex::new(7, 4),
            line_type: "arrow".to_string(),
        },
    ];

    board.set_lines(lines);
}

/// # Vertex Selection and Highlighting
///
/// Manage vertex selection states for UI feedback:
///
/// ```rust
/// // Selected vertices (highlighted)
/// let selected = vec![
///     Vertex::new(4, 4),  // Center
///     Vertex::new(3, 3),  // Upper-left of center
///     Vertex::new(5, 5),  // Lower-right of center
/// ];
/// board.set_selected_vertices(selected);
///
/// // Dimmed vertices (reduced opacity)
/// let dimmed = vec![
///     Vertex::new(0, 0),  // Corners
///     Vertex::new(8, 0),
///     Vertex::new(0, 8),
///     Vertex::new(8, 8),
/// ];
/// board.set_dimmed_vertices(dimmed);
///
/// // Directional selection indicators
/// board.set_selected_left(vec![Vertex::new(2, 4)]);   // Red left arrow
/// board.set_selected_right(vec![Vertex::new(6, 4)]);  // Green right arrow
/// board.set_selected_top(vec![Vertex::new(4, 2)]);    // Orange top arrow
/// board.set_selected_bottom(vec![Vertex::new(4, 6)]); // Purple bottom arrow
/// ```
pub fn selection_examples() {
    let mut board = GoBoard::with_size(9, 9);

    // Primary selections
    board.set_selected_vertices(vec![
        Vertex::new(4, 4),
        Vertex::new(3, 3),
        Vertex::new(5, 5),
    ]);

    // Dimmed areas
    board.set_dimmed_vertices(vec![
        Vertex::new(0, 0),
        Vertex::new(8, 0),
        Vertex::new(0, 8),
        Vertex::new(8, 8),
    ]);

    // Directional indicators
    board.set_selected_left(vec![Vertex::new(2, 4)]);
    board.set_selected_right(vec![Vertex::new(6, 4)]);
    board.set_selected_top(vec![Vertex::new(4, 2)]);
    board.set_selected_bottom(vec![Vertex::new(4, 6)]);
}

/// # Event Handling System
///
/// Comprehensive event handling for user interactions:
///
/// ```rust
/// // Create event handlers
/// let handlers = VertexEventHandlers::new()
///     .on_vertex_click(|vertex| {
///         println!(\"Clicked vertex ({}, {})\", vertex.x, vertex.y);
///         // Handle stone placement logic here
///     })
///     .on_vertex_mouse_enter(|vertex| {
///         println!(\"Mouse entered vertex ({}, {})\", vertex.x, vertex.y);
///         // Show hover effects
///     })
///     .on_vertex_mouse_leave(|vertex| {
///         println!(\"Mouse left vertex ({}, {})\", vertex.x, vertex.y);
///         // Clear hover effects
///     });
///
/// // Render board with event handlers
/// board.render_with_vertex_handlers(handlers)
/// ```
pub fn event_handling_examples() {
    let board = GoBoard::with_size(9, 9);

    let _handlers = VertexEventHandlers::new()
        .on_vertex_click(|vertex| {
            println!("Clicked: ({}, {})", vertex.x, vertex.y);
        })
        .on_vertex_mouse_enter(|vertex| {
            println!("Hover enter: ({}, {})", vertex.x, vertex.y);
        })
        .on_vertex_mouse_leave(|vertex| {
            println!("Hover leave: ({}, {})", vertex.x, vertex.y);
        });

    // In your render method:
    // board.render_with_vertex_handlers(handlers)
}

/// # Bounded and Responsive Boards
///
/// BoundedGoBoard automatically scales to fit within size constraints:
///
/// ```rust
/// // Create bounded board that fits in 200x200 pixels
/// let bounded = BoundedGoBoard::with_size(9, 9, 200.0, 200.0);
///
/// // Board with size limits
/// let constrained = BoundedGoBoard::with_size(19, 19, 300.0, 400.0)
///     .with_vertex_size_limits(8.0, 20.0);  // Min 8px, max 20px per vertex
///
/// // Partial board display
/// let partial = BoundedGoBoard::with_size(5, 5, 200.0, 200.0);  // Show 5x5 area
/// ```
pub fn bounded_board_examples() {
    // Small board in constrained space
    let _small_bounded = BoundedGoBoard::with_size(9, 9, 150.0, 150.0);

    // Large board with size constraints
    let _large_bounded =
        BoundedGoBoard::with_size(19, 19, 400.0, 300.0).with_vertex_size_limits(5.0, 15.0);

    // Partial board for detailed analysis
    let _partial_bounded = BoundedGoBoard::with_size(7, 7, 250.0, 250.0);
}

/// # Performance Optimization
///
/// Efficient update methods for large boards and frequent changes:
///
/// ```rust
/// // Bulk stone updates (more efficient than individual calls)
/// let updates = vec![
///     (Vertex::new(3, 3), 1),
///     (Vertex::new(5, 5), -1),
///     (Vertex::new(7, 7), 0),  // Remove stone
/// ];
/// board.update_stones(&updates);
///
/// // Differential sign map updates (only changed positions)
/// let new_map = create_updated_sign_map();
/// let changed = board.update_sign_map(new_map);
///
/// // Memory management
/// if board.needs_memory_cleanup() {
///     board.cleanup_memory();
/// }
///
/// // Performance statistics
/// let stats = board.get_update_stats();
/// let efficiency = board.get_memory_efficiency();
/// ```
pub fn performance_examples() {
    let mut board = GoBoard::with_size(19, 19);

    // Efficient bulk updates
    let updates = vec![
        (Vertex::new(3, 3), 1),
        (Vertex::new(15, 15), -1),
        (Vertex::new(9, 9), 1),
    ];
    board.update_stones(&updates);

    // Check if cleanup needed
    if board.needs_memory_cleanup() {
        board.cleanup_memory();
    }

    // Get performance metrics
    let _stats = board.get_update_stats();
    let _efficiency = board.get_memory_efficiency();
}

/// # Complete Integration Example
///
/// A comprehensive example showing all features working together:
pub fn complete_integration_example() {
    // Create and configure board
    let mut board = GoBoard::with_size(9, 9).with_vertex_size(35.0);

    // Set custom theme
    let theme = BoardTheme::default()
        .with_board_background(rgb(0xf4d03f))
        .with_grid_lines(rgb(0x2c3e50), 1.5)
        .with_stone_colors(rgb(0x1a1a1a), rgb(0xf8f9fa));
    board.set_theme(theme);

    // Add stones
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

    // Add markers
    let mut marker_map = vec![vec![None; 9]; 9];
    marker_map[4][4] = Some(MarkerType::Label("★".to_string()));
    marker_map[2][2] = Some(MarkerType::Circle);
    marker_map[6][6] = Some(MarkerType::Triangle);
    board.set_marker_map(marker_map);

    // Add heat map
    let mut heat_map = vec![vec![0; 9]; 9];
    heat_map[4][4] = 9;
    heat_map[3][3] = 7;
    heat_map[5][5] = 7;
    board.set_heat_map(heat_map);

    // Add selections
    board.set_selected_vertices(vec![Vertex::new(4, 4)]);
    board.set_dimmed_vertices(vec![Vertex::new(0, 0), Vertex::new(8, 8)]);

    // Add lines
    let lines = vec![Line {
        v1: Vertex::new(3, 0),
        v2: Vertex::new(5, 8),
        line_type: "arrow".to_string(),
    }];
    board.set_lines(lines);

    // The board is now fully configured with all features
}
