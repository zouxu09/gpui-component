# Go Board UI Widget - Example Applications and Documentation

This directory contains comprehensive examples and documentation for the Go board UI widget system. The examples demonstrate how to use all features of the widget in real GPUI applications.

## 📁 Directory Structure

```
go_board_examples/
├── mod.rs                    # Module exports and quick start guide
├── demos.rs                  # Complete demo applications  
└── documentation.rs          # Detailed usage examples
```

## 🚀 Quick Start

The simplest way to use the Go board widget:

```rust
use gpui_component::go_board::GoBoard;

// Create a basic 9x9 board
let board = GoBoard::with_size(9, 9).with_vertex_size(30.0);

// Add some stones
let sign_map = vec![
    vec![0, 0, 0, 1, 0, -1, 0, 0, 0],  // -1=white, 1=black, 0=empty
    vec![0, 1, 0, 0, 0, 0, 0, -1, 0],
    // ... more rows
];
board.set_sign_map(sign_map);
```

## 📖 Demo Applications

### 1. BasicBoardDemo
Shows fundamental board setup and stone placement.

### 2. ThemingDemo  
Demonstrates the comprehensive theming system with multiple theme examples.

### 3. FeatureIntegrationDemo
Shows all major features working together: stones, markers, heat maps, paint overlays, ghost stones, lines, and selections.

### 4. InteractiveDemo
Comprehensive event handling with live feedback for clicks, hover, and mouse events.

### 5. PerformanceDemo
Demonstrates efficient updates and memory management for large boards.

### 6. ResponsiveDemo
Shows bounded sizing and responsive behavior with automatic scaling.

## 🎯 Key Features Demonstrated

### Stone Management
- signMap representation (-1: white, 0: empty, 1: black)
- Individual stone placement
- Bulk updates for efficiency
- Fuzzy positioning for natural appearance

### Visual Customization  
- BoardTheme system with predefined themes (dark, minimalist, high-contrast)
- Custom theme creation using builder pattern
- Texture and asset support
- CSS custom property generation

### Analysis Features
- **Markers**: Circles, triangles, labels, loaders with tooltips
- **Heat Maps**: Influence visualization with color gradients (0-9 scale)
- **Paint Overlays**: Territory analysis with opacity control
- **Ghost Stones**: Move analysis with type indicators (good, interesting, doubtful, bad)
- **Lines & Arrows**: Connection visualization between positions

### Interaction System
- Comprehensive event handling (click, hover, mouse down/up/move)
- Touch device support via pointer events
- Busy state for disabling interactions
- Vertex selection and highlighting

### Performance & Responsiveness
- Differential rendering for optimal performance
- Memory management and component pooling
- Bounded sizing with automatic vertex size calculation
- Partial board display for focused analysis

## 🛠️ Usage Patterns

### Basic Board Setup
```rust
let board = GoBoard::with_size(9, 9)
    .with_vertex_size(30.0);
```

### Custom Theming
```rust
let theme = BoardTheme::default()
    .with_board_background(rgb(0x8B7355))
    .with_grid_lines(rgb(0x2c2c2c), 1.5)
    .with_stone_colors(rgb(0x000000), rgb(0xffffff));
board.set_theme(theme);
```

### Event Handling
```rust
let handlers = VertexEventHandlers::new()
    .on_vertex_click(|vertex| {
        println!("Clicked: ({}, {})", vertex.x, vertex.y);
    });
board.render_with_vertex_handlers(handlers)
```

### Bounded Sizing
```rust
let bounded = BoundedGoBoard::with_size(9, 9, 200.0, 200.0);
// Automatically calculates optimal vertex size
```

## 🔧 Integration with GPUI

The widget follows GPUI best practices:

- **Reactive Architecture**: Uses Entity<T> for state management
- **Render Trait**: Implements proper GPUI rendering patterns  
- **Event System**: Integrates with GPUI's event handling
- **Styling**: Supports GPUI's styling system and CSS properties
- **Performance**: Uses GPUI's efficient rendering and update mechanisms

## 📚 Complete Documentation

See `documentation.rs` for detailed code examples of every feature:

- `basic_board_examples()` - Board creation and setup
- `stone_placement_examples()` - Stone management patterns
- `theming_examples()` - Theme customization
- `marker_examples()` - Annotation system usage
- `heat_map_examples()` - Influence visualization
- `paint_overlay_examples()` - Territory analysis
- `ghost_stone_examples()` - Move analysis
- `line_examples()` - Connection visualization
- `selection_examples()` - Vertex highlighting
- `event_handling_examples()` - User interaction
- `bounded_board_examples()` - Responsive sizing
- `performance_examples()` - Optimization techniques
- `complete_integration_example()` - Everything together

## 🎮 Running the Examples

To see the examples in action:

1. Run the story application: `cargo run --bin story`
2. Navigate to "Go Board" in the sidebar
3. Explore the different demo sections
4. Check the console for event handling output

## 📋 Requirements Met

This implementation satisfies all requirements from specification 16.2:

- ✅ **Example applications** demonstrating all widget features
- ✅ **Feature demos** showing signMap, markerMap, heatMap, and ghostStoneMap usage  
- ✅ **Custom theming** and event handling examples
- ✅ **Documentation examples** with proper GPUI integration patterns

The examples provide comprehensive coverage of the Go board widget's capabilities and serve as both documentation and reference implementations for developers using the widget.