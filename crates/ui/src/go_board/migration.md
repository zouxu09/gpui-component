# Go Board API Migration Guide

This guide helps migrate from the old complex Go board system to the new simplified API.

## Overview of Changes

### Before (Old API)
- **20+ files** with unclear boundaries
- **Complex state management** spread across multiple structs
- **Separate themes** for grid, stones, and overlays
- **Manual overlay coordination**
- **Verbose event handling**
- **Complex rendering pipeline**

### After (New API)
- **4 core files** with clear responsibilities
- **Unified state management** in a single `BoardData` struct
- **Single theme system** for everything
- **Automatic overlay coordination**
- **Simple event handling** with closures
- **Unified rendering system**

## File Structure Changes

| Old Files | New Files | Purpose |
|-----------|-----------|---------|
| `types.rs`, `state.rs`, `theme.rs` | `core.rs` | Core types and data structures |
| `go_board.rs`, `bounded_go_board.rs` | `board.rs` | Board component with data management |
| `grid.rs`, `stones.rs`, `markers.rs`, `*_overlay.rs` | `render.rs` | Unified rendering system |
| `interactions.rs`, `keyboard_navigation.rs`, `selection.rs` | `view.rs` | View component with interactions |

## API Migration Examples

### 1. Basic Board Creation

#### Old Way:
```rust
let mut board = GoBoard::new();
let mut state = GoBoardState::new(19, 19);
board.set_sign_map(sign_map);
board.set_theme(BoardTheme::default());
```

#### New Way:
```rust
let board = Board::new()
    .stone(Pos::new(3, 3), BLACK)
    .stone(Pos::new(15, 15), WHITE);
```

### 2. Adding Markers

#### Old Way:
```rust
let mut marker_map = vec![vec![None; 19]; 19];
marker_map[3][3] = Some(Marker::new(MarkerType::Circle));
board.set_marker_map(marker_map);
```

#### New Way:
```rust
let board = Board::new()
    .marker(Pos::new(3, 3), markers::circle());
```

### 3. Event Handling

#### Old Way:
```rust
let handlers = VertexEventHandlers::new()
    .with_click(|event| {
        println!("Clicked vertex: {}, {}", event.vertex.x, event.vertex.y);
    })
    .with_mouse_down(|event| { /* ... */ })
    .with_mouse_up(|event| { /* ... */ });

board.render_with_vertex_handlers(handlers)
```

#### New Way:
```rust
let view = BoardView::new(board)
    .on_click(|event| {
        println!("Clicked position: {:?}", event.pos);
    });
```

### 4. Theme Configuration

#### Old Way:
```rust
let grid_theme = GridTheme {
    background_color: rgb(0xebb55b),
    grid_line_color: rgb(0x000000),
    // ... many more fields
};

let stone_theme = StoneTheme {
    black_color: rgb(0x000000),
    white_color: rgb(0xffffff),
    // ... many more fields
};

let board_theme = BoardTheme {
    // ... combining grid and stone themes
};

board.set_theme(board_theme);
```

#### New Way:
```rust
let board = Board::new()
    .theme(themes::dark()); // Or themes::minimal(), themes::high_contrast()

// Or customize:
let custom_theme = Theme {
    background: rgb(0x2d2d2d),
    black_stone: rgb(0x000000),
    white_stone: rgb(0xffffff),
    ..themes::default()
};
let board = Board::new().theme(custom_theme);
```

### 5. Ghost Stones and Analysis

#### Old Way:
```rust
let mut ghost_map = vec![vec![None; 19]; 19];
ghost_map[3][3] = Some(GhostStone::new(1, GhostStoneType::Good));
board.set_ghost_stone_map(ghost_map);

let ghost_overlay = GhostStoneOverlay::new(vertex_size, grid_offset);
// Complex overlay management...
```

#### New Way:
```rust
let board = Board::new()
    .ghost(Pos::new(3, 3), ghosts::good(BLACK))
    .ghost(Pos::new(4, 4), ghosts::bad(WHITE).with_alpha(0.7));
```

### 6. Range/Partial Board Display

#### Old Way:
```rust
let range = BoardRange::new((3, 15), (3, 15));
board = board.with_range(range);
// Complex coordinate offset calculations...
```

#### New Way:
```rust
let board = Board::new()
    .range(Range::new((3, 15), (3, 15)));
```

### 7. Auto-sizing Boards

#### Old Way:
```rust
let mut bounded = BoundedGoBoard::new(max_width, max_height);
bounded.set_vertex_size_limits(min_size, max_size);
bounded.set_range_xy(range_x, range_y);
// Complex size calculations...
```

#### New Way:
```rust
let bounded = BoundedBoard::new(max_width, max_height)
    .vertex_size_limits(min_size, max_size)
    .update(|board| board.range(Range::new(range_x, range_y)));
```

## Type Mapping

| Old Type | New Type | Notes |
|----------|----------|-------|
| `Vertex` | `Pos` | Simpler name, same functionality |
| `BoardRange` | `Range` | Simplified interface |
| `SignMap` | `HashMap<Pos, Stone>` | Sparse storage, more efficient |
| `MarkerMap` | `HashMap<Pos, Marker>` | Sparse storage |
| `GhostStoneMap` | `HashMap<Pos, Ghost>` | Unified ghost stone handling |
| `VertexEventHandlers` | Closure functions | Much simpler event handling |
| `GridTheme + StoneTheme + BoardTheme` | `Theme` | Single unified theme |

## Migration Strategy

### Phase 1: Parallel Implementation
1. Keep old modules with `#[deprecated]` warnings
2. Implement new API alongside old one
3. Add compatibility layer for gradual migration

### Phase 2: Gradual Migration
1. Update examples to use new API
2. Update tests to use new API
3. Update documentation to recommend new API

### Phase 3: Full Migration
1. Remove old modules
2. Remove compatibility layer
3. Clean up remaining deprecated code

## Performance Improvements

### Memory Usage
- **Old**: Dense 2D arrays for everything (19×19×4 = 1444 bytes minimum per layer)
- **New**: Sparse HashMaps (only occupied positions use memory)

### Rendering Performance
- **Old**: Multiple separate render passes for each overlay
- **New**: Single unified render pass with optimal layering

### Code Complexity
- **Old**: ~2000 lines across 20+ files
- **New**: ~1000 lines across 4 files (50% reduction)

## Breaking Changes

### Removed Features
1. **Complex validation system**: Simplified to basic bounds checking
2. **Multiple theme types**: Unified into single `Theme` struct
3. **Separate overlay components**: Integrated into unified renderer
4. **Complex event system**: Simplified to closure-based handlers

### API Changes
1. **Coordinate system**: `Vertex { x, y }` → `Pos::new(x, y)`
2. **Builder pattern**: More consistent and fluent
3. **Event handling**: From trait objects to closures
4. **Theme system**: Single theme instead of multiple theme types

## Common Migration Patterns

### Pattern 1: Board State Updates
```rust
// Old
let mut board = GoBoard::new();
board.set_stone(&Vertex::new(3, 3), 1);
board.set_marker_map(marker_map);

// New
let board = Board::new()
    .stone(Pos::new(3, 3), BLACK)
    .marker(Pos::new(4, 4), markers::circle());
```

### Pattern 2: Event Handling
```rust
// Old
let handlers = VertexEventHandlers::new()
    .with_click(move |event| {
        // Complex event handling
    });

// New
let view = BoardView::new(board)
    .on_click(|event| {
        // Simple event handling
    });
```

### Pattern 3: Bulk Operations
```rust
// Old
for (vertex, sign) in updates {
    board.set_stone(&vertex, sign);
}

// New
let board = Board::new()
    .stones(updates.into_iter().map(|(v, s)| (Pos::new(v.x, v.y), s)));
```

## Benefits of Migration

1. **50% less code** to maintain
2. **Simpler mental model** - everything in one place
3. **Better performance** - sparse storage and unified rendering
4. **Easier testing** - fewer components to mock
5. **Better ergonomics** - fluent API with method chaining
6. **Future-proof** - cleaner architecture for future features

## Examples

See `examples.rs` for comprehensive examples showing:
- Basic board creation
- Analysis boards with ghost stones
- Teaching boards
- Puzzle boards
- Responsive boards
- Game boards with keyboard navigation
- Performance optimizations

The new API is designed to make common tasks simple while still supporting advanced use cases.
