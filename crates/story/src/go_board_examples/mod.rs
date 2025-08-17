/// Go Board Examples Module
///
/// This module provides working example applications and demos for the Go board UI widget.
pub mod simple_demos;
pub mod simple_docs;

// Re-export working demo components
pub use simple_demos::{SimpleDemo, ThemeDemo};

// Re-export documentation examples
pub use simple_docs::{
    apply_theme, create_basic_board, manipulate_stones, place_stones, set_selections,
};

/// Quick start guide for new users
pub const QUICK_START_GUIDE: &str = r#"
# Go Board UI Widget - Quick Start Guide

## 1. Basic Setup
```rust
use gpui_component::go_board::GoBoard;

// Create a standard 19x19 board
let board = GoBoard::new();

// Or create custom size
let small_board = GoBoard::with_size(9, 9).with_vertex_size(30.0);
```

## 2. Add Stones
```rust
// Simple stone placement
board.set_stone(&Vertex::new(4, 4), 1);  // Black stone

// Full board state
let sign_map = vec![
    vec![0, 0, 1, 0, -1, 0, 0, 0, 0],  // -1=white, 1=black, 0=empty
    // ... more rows
];
board.set_sign_map(sign_map);
```

## 3. Customize Appearance
```rust
use gpui_component::go_board::BoardTheme;

let theme = BoardTheme::dark();  // Or ::minimalist(), ::high_contrast()
board.set_theme(theme);
```

## 4. Add Interactions
```rust
use gpui_component::go_board::VertexEventHandlers;

let handlers = VertexEventHandlers::new()
    .on_vertex_click(|vertex| {
        println!("Clicked: ({}, {})", vertex.x, vertex.y);
    });

board.render_with_vertex_handlers(handlers)
```

## 5. Add Analysis Features
```rust
// Markers for annotations
let mut marker_map = vec![vec![None; 9]; 9];
marker_map[4][4] = Some(MarkerType::Label("A".to_string()));

// Heat map for influence
let mut heat_map = vec![vec![0; 9]; 9];
heat_map[4][4] = 9;  // High influence

// Ghost stones for analysis
let mut ghost_map = vec![vec![None; 9]; 9];
ghost_map[3][3] = Some(GhostStone::new(1, "good"));
```

See the demo applications for complete examples!
"#;

/// Feature overview for documentation
pub const FEATURE_OVERVIEW: &str = r#"
# Go Board UI Widget - Feature Overview

## Core Features
- ✅ Grid-based Go board rendering with proper line positioning
- ✅ Stone placement with signMap support (-1: white, 0: empty, 1: black)
- ✅ Multiple board sizes (9x9, 13x13, 19x19, custom)
- ✅ Coordinate labeling with standard Go notation
- ✅ Comprehensive theming system with CSS custom properties

## Visual Features
- ✅ Markers and annotations (circles, triangles, labels, etc.)
- ✅ Heat maps for influence visualization
- ✅ Paint overlays for territory analysis
- ✅ Ghost stones for move analysis
- ✅ Lines and arrows for board analysis
- ✅ Vertex selection and highlighting

## Interaction Features
- ✅ Complete event handling (click, hover, mouse down/up)
- ✅ Touch device support
- ✅ Busy state for disabling interactions
- ✅ Keyboard navigation support

## Advanced Features
- ✅ Fuzzy stone placement for natural appearance
- ✅ Random visual variation with deterministic positioning
- ✅ Bounded sizing with automatic scaling
- ✅ Partial board display with range support
- ✅ Differential rendering for performance
- ✅ Memory management and cleanup
- ✅ Texture and asset support

## Performance Features
- ✅ Efficient bulk update operations
- ✅ Change detection and differential rendering
- ✅ Component pooling for memory efficiency
- ✅ Performance monitoring and statistics
"#;
