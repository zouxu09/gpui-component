# Go Board UI Widget Design Document

## Overview

This design document outlines the architecture for a high-performance Go board UI widget component in GPUI, inspired by the proven Shudan architecture. The component will provide a flexible, customizable, and performant Go board display system that supports advanced features like markers, overlays, animations, and comprehensive theming while maintaining clean separation between UI presentation and game logic.

The design follows GPUI's reactive architecture patterns and leverages Rust's type safety to create a robust component suitable for professional Go applications, educational tools, and game analysis software.

## Architecture

### Component Hierarchy

```
GoBoard (Main Component)
├── BoardBackground (Wood texture, styling)
├── Grid (Lines and coordinates)
│   ├── GridLines (Horizontal/vertical lines)
│   ├── StarPoints (Hoshi positions)
│   └── CoordinateLabels (A-T, 1-19 labels)
├── StoneLayer (Stone rendering)
│   ├── Stone (Individual stone with fuzzy positioning)
│   └── GhostStone (Analysis stones)
├── OverlayLayer (Visual overlays)
│   ├── PaintOverlay (Territory marking)
│   ├── HeatOverlay (Influence visualization)
│   └── SelectionOverlay (Vertex highlighting)
├── MarkerLayer (Annotations)
│   ├── Marker (Circles, squares, triangles, etc.)
│   └── LabelMarker (Text annotations)
├── LineLayer (Connections and arrows)
│   └── Line (Arrows and connection lines)
└── InteractionLayer (Event handling)
    └── VertexButton (Clickable areas)
```

### Data Model Architecture

Following Shudan's proven map-based approach:

```rust
// Core data structures matching Shudan's architecture
pub type SignMap = Vec<Vec<i8>>; // -1: white, 0: empty, 1: black
pub type MarkerMap = Vec<Vec<Option<Marker>>>;
pub type GhostStoneMap = Vec<Vec<Option<GhostStone>>>;
pub type HeatMap = Vec<Vec<Option<HeatData>>>;
pub type PaintMap = Vec<Vec<f32>>; // -1.0 to 1.0 for paint intensity

#[derive(Clone, Debug)]
pub struct Vertex {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Debug)]
pub struct BoardRange {
    pub x: (usize, usize), // rangeX equivalent
    pub y: (usize, usize), // rangeY equivalent
}
```

### State Management

The component uses a reactive state management system that efficiently handles updates:

```rust
pub struct GoBoardState {
    // Core board state
    pub sign_map: SignMap,
    pub marker_map: MarkerMap,
    pub ghost_stone_map: GhostStoneMap,
    pub heat_map: HeatMap,
    pub paint_map: PaintMap,

    // Visual state
    pub selected_vertices: Vec<Vertex>,
    pub dimmed_vertices: Vec<Vertex>,
    pub lines: Vec<Line>,

    // Animation state
    pub animated_vertices: Vec<Vertex>,
    pub animation_duration: Duration,

    // Configuration
    pub vertex_size: f32,
    pub board_range: BoardRange,
    pub show_coordinates: bool,
    pub fuzzy_stone_placement: bool,
    pub animate_stone_placement: bool,
    pub busy: bool,
}
```

## Components and Interfaces

### 1. GoBoard (Main Component)

The primary component that orchestrates all sub-components and manages state:

```rust
pub struct GoBoard {
    state: GoBoardState,
    theme: BoardTheme,
    event_handlers: EventHandlers,
}

impl GoBoard {
    pub fn new() -> Self;
    pub fn with_vertex_size(size: f32) -> Self;
    pub fn with_range(range: BoardRange) -> Self;
    pub fn with_bounded_size(max_width: f32, max_height: f32) -> Self;

    // State management
    pub fn set_sign_map(&mut self, map: SignMap);
    pub fn set_marker_map(&mut self, map: MarkerMap);
    pub fn set_selected_vertices(&mut self, vertices: Vec<Vertex>);

    // Event handling
    pub fn on_vertex_click<F>(&mut self, handler: F)
        where F: Fn(ClickEvent, Vertex) + 'static;
    pub fn on_vertex_hover<F>(&mut self, handler: F)
        where F: Fn(bool, Vertex) + 'static;
}
```

### 2. Grid Component

Handles the board grid, star points, and coordinate labels:

```rust
pub struct Grid {
    board_range: BoardRange,
    vertex_size: f32,
    show_coordinates: bool,
    coord_x_fn: Box<dyn Fn(usize) -> String>,
    coord_y_fn: Box<dyn Fn(usize) -> String>,
    theme: GridTheme,
}

impl Grid {
    fn render_grid_lines(&self) -> impl IntoElement;
    fn render_star_points(&self) -> impl IntoElement;
    fn render_coordinates(&self) -> impl IntoElement;
    fn calculate_hoshi_positions(&self) -> Vec<Vertex>;
}
```

### 3. Stone Components

Handles stone rendering with support for fuzzy positioning and animations:

```rust
pub struct Stone {
    position: Vertex,
    sign: i8, // -1, 0, 1
    shift: Option<(f32, f32)>, // Fuzzy positioning offset
    random_class: u8, // 0-4 for visual variation
    animated: bool,
}

pub struct GhostStone {
    position: Vertex,
    sign: i8,
    stone_type: GhostStoneType, // good, interesting, doubtful, bad
    faint: bool,
}

#[derive(Clone, Debug)]
pub enum GhostStoneType {
    Good,
    Interesting,
    Doubtful,
    Bad,
}
```

### 4. Marker System

Flexible marker system supporting all Shudan marker types:

```rust
pub struct Marker {
    position: Vertex,
    marker_type: MarkerType,
    label: Option<String>, // For tooltips and text markers
    style: MarkerStyle,
}

#[derive(Clone, Debug)]
pub enum MarkerType {
    Circle,
    Cross,
    Triangle,
    Square,
    Point,
    Loader,
    Label(String),
}

pub struct MarkerStyle {
    pub color: Color,
    pub stroke_width: f32,
    pub fill: Option<Color>,
    pub scale: f32,
}
```

### 5. Overlay System

Supports paint maps, heat maps, and selection overlays:

```rust
pub struct PaintOverlay {
    paint_map: PaintMap,
    directional_paint: DirectionalPaintMap, // For edge painting
}

pub struct HeatOverlay {
    heat_map: HeatMap,
}

#[derive(Clone, Debug)]
pub struct HeatData {
    pub strength: u8, // 0-9
    pub text: Option<String>,
}

pub struct DirectionalPaintMap {
    pub left: Vec<Vec<f32>>,
    pub right: Vec<Vec<f32>>,
    pub top: Vec<Vec<f32>>,
    pub bottom: Vec<Vec<f32>>,
    pub corners: Vec<Vec<CornerPaint>>,
}
```

### 6. Line and Arrow System

Supports drawing lines and arrows between vertices:

```rust
pub struct Line {
    pub v1: Vertex,
    pub v2: Vertex,
    pub line_type: LineType,
    pub style: LineStyle,
}

#[derive(Clone, Debug)]
pub enum LineType {
    Line,
    Arrow,
}

pub struct LineStyle {
    pub color: Color,
    pub width: f32,
    pub dash_pattern: Option<Vec<f32>>,
    pub arrow_size: f32,
}
```

### 7. Event System

Comprehensive event handling matching Shudan's capabilities:

```rust
pub struct EventHandlers {
    pub on_vertex_click: Option<Box<dyn Fn(ClickEvent, Vertex)>>,
    pub on_vertex_mouse_enter: Option<Box<dyn Fn(MouseMoveEvent, Vertex)>>,
    pub on_vertex_mouse_leave: Option<Box<dyn Fn(MouseMoveEvent, Vertex)>>,
    pub on_vertex_mouse_down: Option<Box<dyn Fn(MouseDownEvent, Vertex)>>,
    pub on_vertex_mouse_up: Option<Box<dyn Fn(MouseUpEvent, Vertex)>>,
    pub on_vertex_pointer_down: Option<Box<dyn Fn(PointerDownEvent, Vertex)>>,
    pub on_vertex_pointer_up: Option<Box<dyn Fn(PointerUpEvent, Vertex)>>,
    pub on_resized: Option<Box<dyn Fn(Size<Pixels>)>>,
}
```

## Data Models

### Theme System

Comprehensive theming system with CSS custom property equivalents:

```rust
pub struct BoardTheme {
    // Board appearance
    pub board_background_color: Color,
    pub board_foreground_color: Color,
    pub board_border_color: Color,
    pub board_border_width: f32,

    // Grid styling
    pub grid_line_color: Color,
    pub grid_line_width: f32,
    pub star_point_color: Color,
    pub star_point_size: f32,

    // Stone styling
    pub black_stone_color: Color,
    pub white_stone_color: Color,
    pub stone_shadow_enabled: bool,
    pub stone_shadow_color: Color,
    pub stone_shadow_offset: (f32, f32),

    // Coordinate styling
    pub coordinate_color: Color,
    pub coordinate_font_size: f32,
    pub coordinate_font_family: String,

    // Custom textures
    pub board_texture: Option<String>, // Background image path
    pub black_stone_texture: Option<String>,
    pub white_stone_texture: Option<String>,

    // Random stone variations
    pub enable_random_stone_variation: bool,
    pub stone_variation_textures: Vec<String>,
}

impl Default for BoardTheme {
    fn default() -> Self {
        // Shudan-inspired default theme
        Self {
            board_background_color: Color::from_hex("#ebb55b").unwrap(),
            board_foreground_color: Color::from_hex("#5e2e0c").unwrap(),
            board_border_color: Color::from_hex("#ca933a").unwrap(),
            board_border_width: 4.0,
            // ... other defaults
        }
    }
}
```

### Animation System

Stone placement animations with configurable duration:

```rust
pub struct AnimationState {
    pub animated_vertices: Vec<Vertex>,
    pub animation_start_time: Instant,
    pub animation_duration: Duration,
    pub animation_type: AnimationType,
}

#[derive(Clone, Debug)]
pub enum AnimationType {
    StonePlace,
    StoneRemove,
    MarkerFadeIn,
    MarkerFadeOut,
    OverlayTransition,
}

pub struct AnimationConfig {
    pub stone_placement_duration: Duration,
    pub marker_fade_duration: Duration,
    pub overlay_transition_duration: Duration,
    pub easing_function: EasingFunction,
}
```

## Error Handling

Comprehensive error handling for common integration issues:

```rust
#[derive(Debug, Clone)]
pub enum GoBoardError {
    InvalidBoardSize { width: usize, height: usize },
    InvalidVertexPosition { vertex: Vertex, board_size: (usize, usize) },
    InvalidRange { range: BoardRange, board_size: (usize, usize) },
    InvalidSignValue { value: i8, position: Vertex },
    InvalidMarkerType { marker_type: String },
    ThemeLoadError { theme_path: String, error: String },
    AnimationError { message: String },
}

impl std::fmt::Display for GoBoardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidBoardSize { width, height } => {
                write!(f, "Invalid board size: {}x{}. Board size must be between 1x1 and 25x25.", width, height)
            }
            Self::InvalidVertexPosition { vertex, board_size } => {
                write!(f, "Vertex ({}, {}) is outside board bounds ({}x{})",
                    vertex.x, vertex.y, board_size.0, board_size.1)
            }
            // ... other error messages
        }
    }
}
```

## Testing Strategy

### Unit Testing

- **Component Isolation**: Test each component (Grid, Stone, Marker, etc.) in isolation
- **State Management**: Verify state updates and reactive behavior
- **Theme Application**: Test theme switching and CSS property generation
- **Event Handling**: Mock event handlers and verify correct vertex coordinates
- **Data Validation**: Test error handling for invalid inputs

### Integration Testing

- **Component Interaction**: Test communication between Grid, Stone, and Marker layers
- **Performance Testing**: Benchmark rendering performance with large boards and many overlays
- **Animation Testing**: Verify smooth animations and proper cleanup
- **Memory Testing**: Ensure no memory leaks during repeated state updates

### Visual Testing

- **Snapshot Testing**: Compare rendered output against reference images
- **Theme Consistency**: Verify visual appearance across different themes
- **Responsive Testing**: Test scaling behavior at different sizes
- **Cross-platform Testing**: Ensure consistent appearance across platforms

### Property-Based Testing

- **Fuzzing**: Generate random board states and verify component stability
- **Invariant Testing**: Ensure component invariants hold across state changes
- **Performance Invariants**: Verify performance doesn't degrade with input size

## Performance Considerations

### Rendering Optimization

1. **Differential Updates**: Only re-render changed vertices when signMap updates
2. **Layer Separation**: Separate static (grid) from dynamic (stones, markers) content
3. **Efficient Z-ordering**: Use CSS transforms for layering instead of DOM manipulation
4. **Viewport Culling**: Only render visible portions for partial boards

### Memory Management

1. **Component Pooling**: Reuse stone and marker components for large boards
2. **Texture Sharing**: Share stone textures across multiple instances
3. **Animation Cleanup**: Properly clean up animation timers and handlers
4. **State Normalization**: Use normalized state structures for efficient updates

### Event Handling Optimization

1. **Event Delegation**: Use single event handler for all vertex interactions
2. **Debouncing**: Debounce rapid state changes to prevent excessive re-renders
3. **Lazy Evaluation**: Defer expensive calculations until actually needed
4. **Memoization**: Cache calculated positions and styles

## Architecture Decisions

### 1. Map-Based Data Model (Following Shudan)

**Decision**: Use Vec<Vec<T>> structure for all board data (signMap, markerMap, etc.)

**Rationale**:
- Proven architecture from Shudan
- Intuitive for developers familiar with 2D arrays
- Efficient memory layout for cache performance
- Easy integration with existing Go libraries

### 2. Component Layer Separation

**Decision**: Separate visual layers (stones, markers, overlays, lines)

**Rationale**:
- Independent rendering optimization for each layer
- Clean separation of concerns
- Easier testing and debugging
- Flexible z-index management

### 3. CSS-Based Theming

**Decision**: Use CSS custom properties for theming with Rust configuration

**Rationale**:
- Familiar to web developers
- Runtime theme switching without recompilation
- Leverage CSS for complex styling (gradients, shadows)
- Easy integration with design systems

### 4. Event-Driven Architecture

**Decision**: Comprehensive event system matching Shudan's approach

**Rationale**:
- Maximum flexibility for consuming applications
- Familiar API for developers using Shudan
- Support for both mouse and pointer events
- Enables complex interaction patterns

This design provides a solid foundation for implementing a professional-grade Go board widget that matches Shudan's capabilities while leveraging GPUI's performance advantages and Rust's type safety.