/// Comprehensive Go Board UI Widget Demo Applications
/// 
/// This module contains focused demo applications that demonstrate specific
/// features of the Go board UI widget system. Each demo is self-contained
/// and showcases a particular aspect of the widget.

use gpui::{
    div, px, rgb, App, AppContext, Context, Entity, FocusHandle, Focusable, IntoElement, ParentElement,
    Render, Styled, Window,
};

use gpui_component::{
    go_board::{
        BoardTheme, BoundedGoBoard, GhostStone, GoBoard, Line, MarkerType, Vertex,
        VertexEventHandlers,
    },
    h_flex, v_flex, ActiveTheme,
};

/// Demo 1: Basic Go Board Setup and Stone Placement
/// Demonstrates the fundamental usage of the Go board widget
pub struct BasicBoardDemo {
    focus_handle: FocusHandle,
    board: Entity<GoBoard>,
}

impl BasicBoardDemo {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let board = cx.new(|_| {
            let mut board = GoBoard::with_size(9, 9).with_vertex_size(30.0);

            // Basic game pattern demonstration
            let sign_map = vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, -1, 0, 0, 0],  // Black vs White stones
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
        });

        Self {
            focus_handle: cx.focus_handle(),
            board,
        }
    }
}

impl Render for BasicBoardDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .child("Basic Go Board Demo")
            .child("9x9 board with simple stone placement")
            .child("signMap: -1 = white stones, 1 = black stones, 0 = empty")
            .child(self.board.clone())
    }
}

/// Demo 2: Advanced Theming System
/// Shows the complete theming capabilities with multiple theme examples
pub struct ThemingDemo {
    focus_handle: FocusHandle,
    default_board: Entity<GoBoard>,
    dark_board: Entity<GoBoard>,
    custom_board: Entity<GoBoard>,
    minimalist_board: Entity<GoBoard>,
}

impl ThemingDemo {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        // Sample game state for all boards
        let sample_sign_map = vec![
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

        Self {
            focus_handle: cx.focus_handle(),
            default_board: cx.new(|_| {
                let mut board = GoBoard::with_size(9, 9).with_vertex_size(25.0);
                board.set_sign_map(sample_sign_map.clone());
                board
            }),
            dark_board: cx.new(|_| {
                let mut board = GoBoard::with_size(9, 9).with_vertex_size(25.0);
                board.set_theme(BoardTheme::dark());
                board.set_sign_map(sample_sign_map.clone());
                board
            }),
            custom_board: cx.new(|_| {
                let custom_theme = BoardTheme::default()
                    .with_board_background(rgb(0x2d3748))  // Dark slate
                    .with_grid_lines(rgb(0x4a5568), 1.5)   // Medium gray, thicker lines
                    .with_stone_colors(rgb(0x1a202c), rgb(0xf7fafc))  // Very dark vs very light
                    .with_selection_style(rgb(0x38b2ac), 0.5);  // Teal selection

                let mut board = GoBoard::with_size(9, 9).with_vertex_size(25.0);
                board.set_theme(custom_theme);
                board.set_sign_map(sample_sign_map.clone());
                board
            }),
            minimalist_board: cx.new(|_| {
                let mut board = GoBoard::with_size(9, 9).with_vertex_size(25.0);
                board.set_theme(BoardTheme::minimalist());
                board.set_sign_map(sample_sign_map);
                board
            }),
        }
    }
}

impl Render for ThemingDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_4()
            .child("Advanced Theming Demo")
            .child("BoardTheme system with predefined and custom themes")
            .child(
                h_flex()
                    .gap_4()
                    .child(
                        v_flex()
                            .gap_2()
                            .child("Default Theme")
                            .child(self.default_board.clone())
                    )
                    .child(
                        v_flex()
                            .gap_2()
                            .child("Dark Theme")
                            .child(self.dark_board.clone())
                    )
            )
            .child(
                h_flex()
                    .gap_4()
                    .child(
                        v_flex()
                            .gap_2()
                            .child("Custom Theme")
                            .child("Builder pattern with custom colors")
                            .child(self.custom_board.clone())
                    )
                    .child(
                        v_flex()
                            .gap_2()
                            .child("Minimalist Theme")
                            .child(self.minimalist_board.clone())
                    )
            )
    }
}

/// Demo 3: Comprehensive Feature Integration
/// Demonstrates all major features working together in one example
pub struct FeatureIntegrationDemo {
    focus_handle: FocusHandle,
    board: Entity<GoBoard>,
}

impl FeatureIntegrationDemo {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let board = cx.new(|_| {
            let mut board = GoBoard::with_size(9, 9).with_vertex_size(35.0);

            // Stones for context
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

            // Markers for important positions
            let mut marker_map = vec![vec![None; 9]; 9];
            marker_map[3][3] = Some(MarkerType::Circle);
            marker_map[5][5] = Some(MarkerType::Triangle);
            marker_map[4][4] = Some(MarkerType::Label("★".to_string()));
            board.set_marker_map(marker_map);

            // Heat map for influence
            let mut heat_map = vec![vec![0; 9]; 9];
            heat_map[3][3] = 7;
            heat_map[4][4] = 9;
            heat_map[5][5] = 8;
            heat_map[2][2] = 5;
            heat_map[6][6] = 6;
            board.set_heat_map(heat_map);

            // Paint overlay for territory
            let mut paint_map = vec![vec![None; 9]; 9];
            for i in 0..4 {
                for j in 0..4 {
                    paint_map[i][j] = Some(gpui_component::go_board::PaintType::Fill { opacity: 0.3 });
                }
            }
            for i in 5..9 {
                for j in 5..9 {
                    paint_map[i][j] = Some(gpui_component::go_board::PaintType::Fill { opacity: 0.3 });
                }
            }
            board.set_paint_map(paint_map);

            // Ghost stones for analysis
            let mut ghost_map = vec![vec![None; 9]; 9];
            ghost_map[2][6] = Some(GhostStone {
                sign: 1,
                ghost_type: Some("good".to_string()),
                faint: false,
            });
            ghost_map[6][2] = Some(GhostStone {
                sign: -1,
                ghost_type: Some("interesting".to_string()),
                faint: true,
            });
            board.set_ghost_stone_map(ghost_map);

            // Lines connecting important positions
            let lines = vec![
                Line {
                    v1: Vertex::new(3, 0),
                    v2: Vertex::new(5, 8),
                    line_type: "arrow".to_string(),
                },
                Line {
                    v1: Vertex::new(0, 3),
                    v2: Vertex::new(8, 5),
                    line_type: "line".to_string(),
                },
            ];
            board.set_lines(lines);

            // Selection states
            board.set_selected_vertices(vec![Vertex::new(4, 4)]);
            board.set_dimmed_vertices(vec![Vertex::new(0, 0), Vertex::new(8, 8)]);

            board
        });

        Self {
            focus_handle: cx.focus_handle(),
            board,
        }
    }
}

impl Render for FeatureIntegrationDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_3()
            .child("Comprehensive Feature Integration Demo")
            .child("All major features combined in one board:")
            .child("• Stones: Black and white pieces")
            .child("• Markers: Circle, triangle, and star label")
            .child("• Heat map: Influence visualization (colored backgrounds)")
            .child("• Paint overlay: Territory regions (blue areas)")
            .child("• Ghost stones: Analysis moves (semi-transparent)")
            .child("• Lines: Connection arrows between positions")
            .child("• Selection: Highlighted and dimmed vertices")
            .child(self.board.clone())
    }
}

/// Demo 4: Interactive Event Handling
/// Shows comprehensive event handling with live feedback
pub struct InteractiveDemo {
    focus_handle: FocusHandle,
    board: Entity<GoBoard>,
    last_event: String,
}

impl InteractiveDemo {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let board = cx.new(|_| {
            let mut board = GoBoard::with_size(9, 9).with_vertex_size(40.0);
            board.set_show_coordinates(true);
            board
        });

        Self {
            focus_handle: cx.focus_handle(),
            board,
            last_event: "No interactions yet".to_string(),
        }
    }

    pub fn update_event(&mut self, event: String) {
        self.last_event = event;
    }
}

impl Render for InteractiveDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_3()
            .child("Interactive Event Handling Demo")
            .child("Try clicking, mouse down/up, and moving over the board")
            .child(format!("Last event: {}", self.last_event))
            .child(
                self.board.update(cx, |board, _| {
                    let handlers = VertexEventHandlers::new()
                        .on_vertex_click(|vertex| {
                            println!("Click at ({}, {})", vertex.x, vertex.y);
                        })
                        .on_vertex_mouse_enter(|vertex| {
                            println!("Mouse enter at ({}, {})", vertex.x, vertex.y);
                        })
                        .on_vertex_mouse_leave(|vertex| {
                            println!("Mouse leave at ({}, {})", vertex.x, vertex.y);
                        });

                    board.render_with_vertex_handlers(handlers)
                })
            )
    }
}

/// Demo 5: Performance and Memory Management
/// Demonstrates efficient updates and memory management features
pub struct PerformanceDemo {
    focus_handle: FocusHandle,
    board: Entity<GoBoard>,
    update_count: usize,
}

impl PerformanceDemo {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let board = cx.new(|_| {
            GoBoard::with_size(19, 19).with_vertex_size(20.0)
        });

        Self {
            focus_handle: cx.focus_handle(),
            board,
            update_count: 0,
        }
    }

    pub fn add_random_stones(&mut self, cx: &mut Context<Self>) {
        self.board.update(cx, |board, _| {
            // Simulate efficient bulk stone updates
            let updates = vec![
                (Vertex::new(self.update_count % 19, (self.update_count * 2) % 19), 1),
                (Vertex::new((self.update_count * 3) % 19, (self.update_count * 4) % 19), -1),
                (Vertex::new((self.update_count * 5) % 19, (self.update_count * 6) % 19), 1),
            ];
            
            board.update_stones(&updates);
            self.update_count += 1;
        });
    }
}

impl Render for PerformanceDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let memory_stats = self.board.read(cx).get_memory_stats();
        let update_stats = self.board.read(cx).get_update_stats();

        v_flex()
            .gap_3()
            .child("Performance and Memory Management Demo")
            .child("Large 19x19 board with efficient update system")
            .child(format!("Updates performed: {}", self.update_count))
            .child(format!("Memory efficiency: {:.1}%", 
                self.board.read(cx).get_memory_efficiency() * 100.0))
            .child(format!("Update stats: {:?}", update_stats))
            .child("Click to add random stones using bulk updates")
            .child(
                self.board.clone()
                    .on_click(cx, |demo, _event, cx| {
                        demo.add_random_stones(cx);
                    })
            )
    }
}

/// Demo 6: Responsive and Bounded Boards
/// Shows different sizing strategies and responsive behavior
pub struct ResponsiveDemo {
    focus_handle: FocusHandle,
    small_bounded: Entity<BoundedGoBoard>,
    medium_bounded: Entity<BoundedGoBoard>,
    large_bounded: Entity<BoundedGoBoard>,
    constrained_bounded: Entity<BoundedGoBoard>,
}

impl ResponsiveDemo {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        // Sample stones for all boards
        let sample_stones = vec![
            vec![0, 0, 1, 0, 0, 0, -1, 0, 0],
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
            small_bounded: cx.new(|_| {
                let mut board = BoundedGoBoard::with_size(9, 9, 150.0, 150.0);
                board.set_sign_map(sample_stones.clone());
                board
            }),
            medium_bounded: cx.new(|_| {
                let mut board = BoundedGoBoard::with_size(9, 9, 250.0, 250.0);
                board.set_sign_map(sample_stones.clone());
                board.set_show_coordinates(true);
                board
            }),
            large_bounded: cx.new(|_| {
                let mut board = BoundedGoBoard::with_size(9, 9, 350.0, 350.0);
                board.set_sign_map(sample_stones.clone());
                board
            }),
            constrained_bounded: cx.new(|_| {
                let mut board = BoundedGoBoard::with_size(9, 9, 180.0, 400.0);
                board.set_sign_map(sample_stones);
                board
            }),
        }
    }
}

impl Render for ResponsiveDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_4()
            .child("Responsive and Bounded Boards Demo")
            .child("BoundedGoBoard automatically calculates optimal vertex size")
            .child(
                h_flex()
                    .gap_4()
                    .child(
                        v_flex()
                            .gap_2()
                            .child("Small (150x150)")
                            .child(format!("Vertex: {:.1}px", self.small_bounded.read(cx).vertex_size()))
                            .child(self.small_bounded.clone())
                    )
                    .child(
                        v_flex()
                            .gap_2()
                            .child("Medium (250x250)")
                            .child(format!("Vertex: {:.1}px", self.medium_bounded.read(cx).vertex_size()))
                            .child(self.medium_bounded.clone())
                    )
            )
            .child(
                h_flex()
                    .gap_4()
                    .child(
                        v_flex()
                            .gap_2()
                            .child("Large (350x350)")
                            .child(format!("Vertex: {:.1}px", self.large_bounded.read(cx).vertex_size()))
                            .child(self.large_bounded.clone())
                    )
                    .child(
                        v_flex()
                            .gap_2()
                            .child("Constrained (180x400)")
                            .child("Width-limited aspect ratio")
                            .child(format!("Vertex: {:.1}px", self.constrained_bounded.read(cx).vertex_size()))
                            .child(self.constrained_bounded.clone())
                    )
            )
    }
}

impl Focusable for BasicBoardDemo {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Focusable for ThemingDemo {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Focusable for FeatureIntegrationDemo {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Focusable for InteractiveDemo {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Focusable for PerformanceDemo {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Focusable for ResponsiveDemo {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}