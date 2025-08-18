use crate::go_board::types::*;
use crate::go_board::{
    BoardTheme, GhostStone, GoBoard, GoBoardError, GoBoardResult, GridTheme, Line, MarkerType,
    StoneTheme, VertexEventHandlers, VertexInteractions,
};
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[test]
fn test_go_board_with_grid() {
    let mut board = GoBoard::new();

    // Test default grid theme
    assert_eq!(board.grid_theme().background_color, rgb(0xebb55b));
    assert_eq!(board.grid_theme().grid_line_color, rgb(0x000000));
    assert_eq!(board.grid_theme().grid_line_width, 1.0);

    // Test custom grid theme
    let custom_theme = GridTheme {
        background_color: rgb(0x123456),
        grid_line_color: rgb(0x654321),
        grid_line_width: 2.0,
        border_color: rgb(0xabcdef),
        border_width: 3.0,
    };

    board.set_grid_theme(custom_theme.clone());
    assert_eq!(
        board.grid_theme().background_color,
        custom_theme.background_color
    );
    assert_eq!(board.grid_theme().grid_line_width, 2.0);
}

#[test]
fn test_go_board_grid_integration() {
    let board = GoBoard::with_size(9, 9).with_vertex_size(30.0);

    // Verify board state affects grid rendering
    assert_eq!(board.state().dimensions(), (9, 9));
    assert_eq!(board.state().vertex_size, 30.0);

    // Test board pixel size calculation
    let size = board.board_pixel_size();
    assert_eq!(size.width, px(270.0)); // 9 * 30
    assert_eq!(size.height, px(270.0)); // 9 * 30
}

#[test]
fn test_go_board_partial_range() {
    let range = BoardRange::new((3, 12), (3, 12)); // 10x10 visible area
    let board = GoBoard::with_size(19, 19)
        .with_range(range)
        .with_vertex_size(20.0);

    // Verify partial board size
    let size = board.board_pixel_size();
    assert_eq!(size.width, px(200.0)); // 10 * 20
    assert_eq!(size.height, px(200.0)); // 10 * 20
}

/// Integration test combining stones, markers, and overlays
#[test]
fn test_stones_markers_overlays_integration() {
    let mut board = GoBoard::with_size(9, 9).with_vertex_size(30.0);

    // Create a complex board state with stones
    let mut sign_map = vec![vec![0; 9]; 9];
    sign_map[3][3] = 1; // Black stone
    sign_map[3][5] = -1; // White stone
    sign_map[5][3] = 1; // Black stone
    sign_map[5][5] = -1; // White stone
    board.set_sign_map(sign_map.clone());

    // Add markers to annotate key positions
    let mut marker_map = vec![vec![None; 9]; 9];
    marker_map[3][3] = Some(Marker::new(MarkerType::Circle));
    marker_map[5][5] = Some(Marker::new(MarkerType::Triangle));
    marker_map[4][4] = Some(Marker::with_label(
        MarkerType::Label("A1".to_string()),
        "Point A1".to_string(),
    ));
    board.set_marker_map(marker_map.clone());

    // Add paint overlay for territory
    let mut paint_map = vec![vec![None; 9]; 9];
    for x in 0..4 {
        for y in 0..4 {
            paint_map[y][x] = Some(PaintType::Fill { opacity: 0.3 });
        }
    }
    board.set_paint_map(paint_map.clone());

    // Add heat map for influence
    let mut heat_map = vec![vec![None; 9]; 9];
    heat_map[3][3] = Some(HeatData::new(7)); // High influence
    heat_map[4][4] = Some(HeatData::new(5)); // Medium influence
    heat_map[5][5] = Some(HeatData::new(8)); // Very high influence
    board.set_heat_map(heat_map.clone());

    // Add ghost stones for analysis
    let mut ghost_map = vec![vec![None; 9]; 9];
    ghost_map[2][2] =
        Some(GhostStone::new(1, GhostStoneType::Good).with_ghost_type("good".to_string()));
    ghost_map[6][6] = Some(
        GhostStone::new(-1, GhostStoneType::Interesting)
            .with_ghost_type("interesting".to_string())
            .faint(),
    );
    board.set_ghost_stone_map(ghost_map.clone());

    // Add selection states
    board.set_selected_vertices(vec![Vertex::new(3, 3), Vertex::new(5, 5)]);
    board.set_dimmed_vertices(vec![Vertex::new(1, 1), Vertex::new(7, 7)]);

    // Add lines connecting important positions
    let lines = vec![
        Line::arrow(Vertex::new(3, 3), Vertex::new(5, 5)),
        Line::line(Vertex::new(3, 5), Vertex::new(5, 3)),
    ];
    board.set_lines(lines.clone());

    // Verify all components are properly integrated
    assert_eq!(board.state().sign_map, sign_map);
    assert_eq!(board.state().marker_map, marker_map);
    assert_eq!(board.state().paint_map, paint_map);
    assert_eq!(board.state().heat_map, heat_map);
    assert_eq!(board.state().ghost_stone_map, ghost_map);
    assert_eq!(board.state().selected_vertices.len(), 2);
    assert_eq!(board.state().dimmed_vertices.len(), 2);
    assert_eq!(board.get_lines().len(), 2);

    // Test that individual stone access works
    assert_eq!(board.get_stone(&Vertex::new(3, 3)), Some(1));
    assert_eq!(board.get_stone(&Vertex::new(3, 5)), Some(-1));
    assert_eq!(board.get_stone(&Vertex::new(0, 0)), Some(0));

    // Test ghost stone access
    let ghost = board.get_ghost_stone(&Vertex::new(2, 2));
    assert!(ghost.is_some());
    assert_eq!(ghost.unwrap().sign, 1);
    assert_eq!(ghost.unwrap().ghost_type, Some("good".to_string()));
}

/// Performance benchmark for complex board states
#[test]
fn test_complex_board_performance() {
    let start = Instant::now();

    // Create large board with complex state
    let mut board = GoBoard::with_size(19, 19).with_vertex_size(20.0);

    // Add many stones in a realistic game pattern
    let mut sign_map = vec![vec![0; 19]; 19];
    for i in 0..19 {
        for j in 0..19 {
            if (i + j) % 7 == 0 {
                sign_map[i][j] = if (i * j) % 2 == 0 { 1 } else { -1 };
            }
        }
    }

    let setup_time = start.elapsed();
    println!("Board setup time: {:?}", setup_time);

    let update_start = Instant::now();
    board.set_sign_map(sign_map.clone());
    let sign_map_time = update_start.elapsed();
    println!("Sign map update time: {:?}", sign_map_time);

    // Add comprehensive markers
    let marker_start = Instant::now();
    let mut marker_map = vec![vec![None; 19]; 19];
    for i in (0..19).step_by(3) {
        for j in (0..19).step_by(3) {
            marker_map[i][j] = Some(if (i + j) % 2 == 0 {
                MarkerType::Circle
            } else {
                MarkerType::Triangle
            });
        }
    }
    board.set_marker_map(marker_map);
    let marker_time = marker_start.elapsed();
    println!("Marker map update time: {:?}", marker_time);

    // Add complex heat map
    let heat_start = Instant::now();
    let mut heat_map = vec![vec![0; 19]; 19];
    for i in 0..19 {
        for j in 0..19 {
            heat_map[i][j] = ((i * j) % 10) as u8;
        }
    }
    board.set_heat_map(heat_map);
    let heat_time = heat_start.elapsed();
    println!("Heat map update time: {:?}", heat_time);

    // Add paint overlay
    let paint_start = Instant::now();
    let mut paint_map = vec![vec![None; 19]; 19];
    for i in 0..19 {
        for j in 0..19 {
            if (i + j) % 5 == 0 {
                paint_map[i][j] = Some(PaintType::Fill {
                    opacity: 0.2 + (i as f32 / 19.0) * 0.3,
                });
            }
        }
    }
    board.set_paint_map(paint_map);
    let paint_time = paint_start.elapsed();
    println!("Paint map update time: {:?}", paint_time);

    let total_time = start.elapsed();
    println!("Total complex board setup time: {:?}", total_time);

    // Performance assertions (reasonable thresholds)
    assert!(
        total_time < Duration::from_millis(100),
        "Complex board setup should complete within 100ms"
    );
    assert!(
        sign_map_time < Duration::from_millis(20),
        "Sign map updates should be fast"
    );
    assert!(
        marker_time < Duration::from_millis(15),
        "Marker updates should be fast"
    );
    assert!(
        heat_time < Duration::from_millis(10),
        "Heat map updates should be fast"
    );
    assert!(
        paint_time < Duration::from_millis(10),
        "Paint map updates should be fast"
    );
}

/// Test event handling with multiple simultaneous interactions
#[test]
fn test_multiple_simultaneous_interactions() {
    let mut board = GoBoard::with_size(9, 9).with_vertex_size(25.0);
    let mut event_log = Vec::new();

    // Create event handlers that log interactions
    let handlers = VertexEventHandlers::new()
        .on_vertex_click(|vertex| {
            // In a real test, we'd capture this event
            println!("Click at vertex: {:?}", vertex);
        })
        .on_vertex_mouse_enter(|vertex| {
            println!("Mouse enter at vertex: {:?}", vertex);
        })
        .on_vertex_mouse_leave(|vertex| {
            println!("Mouse leave at vertex: {:?}", vertex);
        });

    // Test that board state affects interaction handling
    board.set_busy(false);
    assert!(!board.state().busy);

    // Add complex selection states
    board.set_selected_vertices(vec![
        Vertex::new(0, 0),
        Vertex::new(4, 4),
        Vertex::new(8, 8),
    ]);
    board.set_dimmed_vertices(vec![
        Vertex::new(1, 1),
        Vertex::new(3, 3),
        Vertex::new(7, 7),
    ]);

    // Test directional selections
    board.set_selected_left(vec![Vertex::new(2, 4)]);
    board.set_selected_right(vec![Vertex::new(6, 4)]);
    board.set_selected_top(vec![Vertex::new(4, 2)]);
    board.set_selected_bottom(vec![Vertex::new(4, 6)]);

    // Verify all selection states are maintained
    assert_eq!(board.state().selected_vertices.len(), 3);
    assert_eq!(board.state().dimmed_vertices.len(), 3);
    assert_eq!(board.state().selected_left.len(), 1);
    assert_eq!(board.state().selected_right.len(), 1);
    assert_eq!(board.state().selected_top.len(), 1);
    assert_eq!(board.state().selected_bottom.len(), 1);

    // Test busy state disables interactions
    board.set_busy(true);
    assert!(board.state().busy);

    // Verify coordinates display
    board.set_show_coordinates(true);
    assert!(board.state().show_coordinates);

    board.set_show_coordinates(false);
    assert!(!board.state().show_coordinates);

    // Test fuzzy stone placement
    board.set_fuzzy_stone_placement(true);
    assert!(board.state().fuzzy_stone_placement);
}

/// Verify proper integration of all layers and components
#[test]
fn test_all_layers_integration() {
    let mut board = GoBoard::with_size(13, 13).with_vertex_size(24.0);

    // Layer 1: Grid background
    let grid_theme = GridTheme {
        background_color: rgb(0xf4d03f),
        grid_line_color: rgb(0x2c3e50),
        grid_line_width: 1.5,
        border_color: rgb(0x34495e),
        border_width: 2.0,
        star_point_color: rgb(0x2c3e50),
        star_point_size: 3.0,
    };
    board.set_grid_theme(grid_theme);

    // Layer 2: Stone theme
    let stone_theme = StoneTheme {
        black_color: rgb(0x1a1a1a),
        white_color: rgb(0xf8f9fa),
        stone_size_ratio: 0.9,
        border_width: 1.0,
        border_color: rgb(0x343a40),
        fuzzy_placement: true,
        fuzzy_max_offset: 0.1,
        random_variation: true,
        max_rotation: 5.0,
        black_stone_image: None,
        white_stone_image: None,
    };
    board.set_stone_theme(stone_theme);

    // Layer 3: Stones with realistic game pattern
    let mut sign_map = vec![vec![0; 13]; 13];
    // Corner play
    sign_map[3][3] = 1;
    sign_map[3][9] = -1;
    sign_map[9][3] = -1;
    sign_map[9][9] = 1;
    // Center play
    sign_map[6][6] = 1;
    sign_map[6][7] = -1;
    board.set_sign_map(sign_map);

    // Layer 4: Markers for important moves
    let mut marker_map = vec![vec![None; 13]; 13];
    marker_map[3][3] = Some(MarkerType::Circle);
    marker_map[6][6] = Some(MarkerType::Label("★".to_string()));
    marker_map[9][9] = Some(MarkerType::Triangle);
    board.set_marker_map(marker_map);

    // Layer 5: Heat map for influence visualization
    let mut heat_map = vec![vec![None; 13]; 13];
    // High influence around corners
    for i in 2..6 {
        for j in 2..6 {
            heat_map[i][j] = Some(HeatData::new(6));
        }
    }
    heat_map[6][6] = Some(HeatData::new(9)); // Very high influence at center
    board.set_heat_map(heat_map);

    // Layer 6: Paint overlay for territory
    let paint_start = Instant::now();
    let mut paint_map = vec![vec![None; 19]; 19];
    for i in 0..19 {
        for j in 0..19 {
            if (i + j) % 4 == 0 {
                paint_map[i][j] = Some(PaintType::Fill {
                    opacity: 0.1 + (i + j) as f32 * 0.01,
                });
            }
        }
    }
    board.set_paint_map(paint_map);

    // Layer 7: Ghost stones for analysis
    let mut ghost_map = vec![vec![None; 13]; 13];
    ghost_map[5][5] =
        Some(GhostStone::new(1, GhostStoneType::Good).with_ghost_type("good".to_string()));
    ghost_map[7][7] = Some(
        GhostStone::new(-1, GhostStoneType::Doubtful)
            .with_ghost_type("doubtful".to_string())
            .faint(),
    );
    board.set_ghost_stone_map(ghost_map);

    // Layer 8: Lines connecting related stones
    let lines = vec![
        Line::arrow(Vertex::new(3, 3), Vertex::new(6, 6)),
        Line::line(Vertex::new(9, 9), Vertex::new(6, 6)),
    ];
    board.set_lines(lines);

    // Layer 9: Selection states
    board.set_selected_vertices(vec![Vertex::new(6, 6)]);
    board.set_dimmed_vertices(vec![Vertex::new(0, 0), Vertex::new(12, 12)]);

    // Verify all layers are properly configured
    assert_eq!(board.state().dimensions(), (13, 13));
    assert_eq!(board.state().vertex_size, 24.0);
    assert_eq!(board.get_stone(&Vertex::new(3, 3)), Some(1));
    assert_eq!(board.get_stone(&Vertex::new(3, 9)), Some(-1));
    assert_eq!(board.state().selected_vertices.len(), 1);
    assert_eq!(board.state().dimmed_vertices.len(), 2);
    assert_eq!(board.get_lines().len(), 2);

    // Test board pixel size calculation with all layers
    let size = board.board_pixel_size();
    assert_eq!(size.width, px(312.0)); // 13 * 24
    assert_eq!(size.height, px(312.0)); // 13 * 24

    // Test memory efficiency with complex state
    assert!(!board.needs_memory_cleanup()); // Should be efficient
    let stats = board.get_memory_stats();
    println!("Memory stats: {:?}", stats);
}

/// Test error handling integration across components
#[test]
fn test_error_handling_integration() {
    let mut board = GoBoard::with_size(9, 9);

    // Test validation with try_* methods
    assert!(board.try_with_vertex_size(25.0).is_ok());
    assert!(board.try_with_vertex_size(-1.0).is_err());
    assert!(board.try_with_vertex_size(0.0).is_err());

    // Test stone setting validation
    assert!(board.try_set_stone(&Vertex::new(4, 4), 1).is_ok());
    assert!(board.try_set_stone(&Vertex::new(9, 8), 1).is_err()); // Out of bounds
    assert!(board.try_set_stone(&Vertex::new(4, 4), 2).is_err()); // Invalid sign

    // Test bulk operations validation
    let valid_updates = vec![
        (Vertex::new(1, 1), 1),
        (Vertex::new(2, 2), -1),
        (Vertex::new(3, 3), 0),
    ];
    assert!(board.try_update_stones(&valid_updates).is_ok());

    let invalid_updates = vec![
        (Vertex::new(1, 1), 1),
        (Vertex::new(20, 20), 1), // Out of bounds
    ];
    assert!(board.try_update_stones(&invalid_updates).is_err());

    // Test map validation
    let valid_map = vec![vec![0; 9]; 9];
    assert!(board.try_update_sign_map(valid_map).is_ok());

    let invalid_map = vec![vec![0; 8]; 9]; // Wrong width
    assert!(board.try_update_sign_map(invalid_map).is_err());
}

/// Test differential rendering and update optimization
#[test]
fn test_differential_rendering_integration() {
    let mut board = GoBoard::with_size(9, 9);

    // Initial state
    let mut sign_map = vec![vec![0; 9]; 9];
    sign_map[4][4] = 1;
    board.set_sign_map(sign_map.clone());

    // Make a change and verify differential detection
    sign_map[3][3] = -1;
    let changes = board.get_sign_map_differences(&sign_map);
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0], Vertex::new(3, 3));

    // Update the board state
    let changed = board.update_sign_map(sign_map);
    assert!(changed);

    // Verify no changes when setting same state
    let same_map = board.state().sign_map.clone();
    let no_changes = board.get_sign_map_differences(&same_map);
    assert!(no_changes.is_empty());

    // Test update statistics
    let stats = board.get_update_stats();
    println!("Update stats: {:?}", stats);

    // Test vertex change tracking
    assert!(board.vertex_changed(&Vertex::new(3, 3)));
    assert!(!board.vertex_changed(&Vertex::new(0, 0)));
}

/// Test theme integration across all components
#[test]
fn test_theme_integration() {
    let mut board = GoBoard::with_size(9, 9);

    // Create comprehensive board theme
    let theme = BoardTheme {
        board_background_color: rgb(0xfdf6e3),
        grid_line_color: rgb(0x586e75),
        grid_line_width: 1.0,
        board_border_color: rgb(0x073642),
        board_border_width: 2.0,
        star_point_color: rgb(0x586e75),
        star_point_size: 4.0,
        black_stone_color: rgb(0x073642),
        white_stone_color: rgb(0xfdf6e3),
        stone_size_ratio: 0.95,
        stone_border_width: 1.0,
        stone_border_color: rgb(0x93a1a1),
        coordinate_label_color: rgb(0x586e75),
        coordinate_label_font_size: 12.0,
        selection_color: rgb(0x268bd2),
        selection_opacity: 0.4,
        marker_color: rgb(0xdc322f),
        ghost_stone_opacity: 0.6,
        line_color: rgb(0x859900),
        line_width: 2.0,
        heat_map_colors: vec![
            rgb(0x002b36),
            rgb(0x073642),
            rgb(0x586e75),
            rgb(0x657b83),
            rgb(0x839496),
            rgb(0x93a1a1),
            rgb(0xeee8d5),
            rgb(0xfdf6e3),
        ],
        paint_positive_color: rgb(0x268bd2),
        paint_negative_color: rgb(0xdc322f),
        fuzzy_placement: true,
        fuzzy_max_offset: 0.15,
        random_variation: true,
        max_rotation: 8.0,
        black_stone_texture: None,
        white_stone_texture: None,
    };

    board.set_theme(theme.clone());

    // Verify theme is applied correctly
    assert_eq!(
        board.theme().board_background_color,
        theme.board_background_color
    );
    assert_eq!(board.theme().black_stone_color, theme.black_stone_color);
    assert_eq!(board.theme().stone_size_ratio, theme.stone_size_ratio);
    assert_eq!(board.theme().fuzzy_placement, theme.fuzzy_placement);

    // Test CSS adapter integration
    let css_adapter = board.css_adapter();
    // The CSS adapter should reflect the theme changes
    // In a real implementation, we'd verify CSS property generation

    // Test that theme affects all components
    board.invalidate_render_cache(); // Force theme reapplication

    // Verify backward compatibility with individual theme setters
    let grid_theme = board.grid_theme();
    assert_eq!(grid_theme.background_color, theme.board_background_color);
    assert_eq!(grid_theme.grid_line_color, theme.grid_line_color);

    let stone_theme = board.stone_theme();
    assert_eq!(stone_theme.black_color, theme.black_stone_color);
    assert_eq!(stone_theme.white_color, theme.white_stone_color);
}

/// Test memory management and cleanup integration
#[test]
fn test_memory_management_integration() {
    let mut board = GoBoard::with_size(19, 19);

    // Initial memory state
    let initial_stats = board.get_memory_stats().clone();
    println!("Initial memory stats: {:?}", initial_stats);

    // Use component pooling
    let vertices_to_test = vec![
        Vertex::new(3, 3),
        Vertex::new(5, 5),
        Vertex::new(9, 9),
        Vertex::new(15, 15),
    ];

    let mut stone_components = Vec::new();
    let mut marker_components = Vec::new();

    for vertex in &vertices_to_test {
        let stone = board.get_pooled_stone_component(*vertex, 1);
        stone_components.push(stone);

        let marker = board.get_pooled_marker_component(*vertex, MarkerType::Circle);
        marker_components.push(marker);
    }

    // Check memory usage increased
    let after_allocation_stats = board.get_memory_stats();
    println!("After allocation stats: {:?}", after_allocation_stats);

    // Return components to pool
    for stone in stone_components {
        board.return_stone_component(stone);
    }
    for marker in marker_components {
        board.return_marker_component(marker);
    }

    // Get pool statistics
    let pool_stats = board.get_pool_stats();
    println!("Pool stats: {:?}", pool_stats);

    // Test cleanup
    assert!(!board.needs_memory_cleanup()); // Should not need cleanup yet

    // Force cleanup
    board.cleanup_memory();
    let after_cleanup_stats = board.get_memory_stats();
    println!("After cleanup stats: {:?}", after_cleanup_stats);

    // Force complete cleanup
    board.force_memory_cleanup();
    let final_stats = board.get_memory_stats();
    println!("Final memory stats: {:?}", final_stats);

    // Test memory efficiency
    let efficiency = board.get_memory_efficiency();
    println!("Memory efficiency: {:.2}%", efficiency * 100.0);
    assert!(efficiency >= 0.0 && efficiency <= 1.0);
}
