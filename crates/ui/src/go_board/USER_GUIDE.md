# Go Board Widget User Guide

A comprehensive Go board widget for GPUI applications with themes, assets, and interactive features.

## Table of Contents

- [Quick Start](#quick-start)
- [Core Concepts](#core-concepts)
- [Basic Usage](#basic-usage)
- [Theming](#theming)
- [Interactive Features](#interactive-features)
- [Advanced Features](#advanced-features)
- [API Reference](#api-reference)
- [Examples](#examples)
- [Troubleshooting](#troubleshooting)

## Quick Start

### Basic Board Creation

```rust
use gpui_component::go_board::{Board, BoardView, BLACK, WHITE};

// Create a standard 19x19 board
let board = Board::new();

// Create a board with custom size
let board = Board::with_size(9, 9);

// Display the board
let board_view = BoardView::new(board);
```

### Adding Stones

```rust
let board = Board::new()
    .stone(Pos::new(3, 3), BLACK)    // Black stone at (3, 3)
    .stone(Pos::new(15, 15), WHITE); // White stone at (15, 15)
```

### Interactive Board

```rust
let board_view = BoardView::new(Board::new())
    .on_click(|event| {
        println!("Clicked at {:?}", event.pos);
    })
    .on_hover(|pos| {
        if let Some(p) = pos {
            println!("Hovering over {:?}", p);
        }
    });
```

## Core Concepts

### Position System

Positions use a grid coordinate system where:
- `(0, 0)` is the top-left corner
- `x` increases from left to right
- `y` increases from top to bottom
- Coordinates are `usize` values

```rust
use gpui_component::go_board::Pos;

let pos = Pos::new(3, 3); // 4th column, 4th row (0-indexed)
```

### Stone Types

```rust
use gpui_component::go_board::{Stone, BLACK, WHITE, EMPTY};

// Stone constants
let black_stone: Stone = BLACK;  // Value: 1
let white_stone: Stone = WHITE;  // Value: -1
let empty: Stone = EMPTY;        // Value: 0
```

### Board Sizes

Standard Go board sizes are supported:
- **9×9**: Small board for beginners
- **13×13**: Medium board for intermediate players
- **19×19**: Standard full-size board

## Basic Usage

### Creating Boards

```rust
// Standard 19x19 board
let board = Board::new();

// Custom size board
let board = Board::with_size(13, 13);

// Board with custom vertex size
let board = Board::new().vertex_size(30.0);

// Board with coordinates disabled
let board = Board::new().coordinates(false);
```

### Adding Content

```rust
let board = Board::new()
    // Add stones
    .stone(Pos::new(3, 3), BLACK)
    .stone(Pos::new(15, 15), WHITE)

    // Add markers
    .marker(Pos::new(9, 9), Marker::circle())
    .marker(Pos::new(4, 4), Marker::cross())

    // Add ghost stones
    .ghost(Pos::new(5, 5), Ghost::good(BLACK))
    .ghost(Pos::new(6, 6), Ghost::bad(WHITE))

    // Add selections
    .select(Pos::new(7, 7))
    .last_move(Pos::new(8, 8));
```

### Displaying Boards

```rust
// Basic display
let board_view = BoardView::new(board);

// With coordinates
let board_view = BoardView::new(board).coordinates(true);

// With custom size
let board_view = BoardView::new(board)
    .vertex_size(25.0);
```

## Theming

### Predefined Themes

```rust
use gpui_component::go_board::Theme;

// Default theme (wooden board)
let board = Board::new().theme(Theme::default());

// Dark theme
let board = Board::new().theme(Theme::dark());

// Minimalist theme
let board = Board::new().theme(Theme::minimal());

// High contrast theme (accessibility)
let board = Board::new().theme(Theme::high_contrast());
```

### Asset-Based Themes

```rust
// Use built-in assets
let board = Board::new().theme(Theme::with_assets());

// Custom theme with assets
let theme = Theme::default()
    .with_board_background("custom/board.jpg")
    .with_black_stone_asset("custom/black.png")
    .with_white_stone_asset("custom/white.png");

let board = Board::new().theme(theme);
```

### Custom Theme Builder

```rust
let custom_theme = Theme::default()
    .with_background(rgb(0x8B7355))      // Darker wood
    .with_border(rgb(0x654321))          // Dark brown border
    .with_grid_lines(rgb(0x2c2c2c))      // Dark gray lines
    .with_black_stone(rgb(0x000000))     // Pure black
    .with_white_stone(rgb(0xffffff))     // Pure white
    .with_border_width(3.0)              // Thicker border
    .with_grid_width(2.0)                // Thicker grid lines
    .with_stone_size(0.9)                // Larger stones
    .with_coord_size(18.0);              // Larger coordinates
```

## Interactive Features

### Click Handling

```rust
let board_view = BoardView::new(Board::new())
    .on_click(|event| {
        match event.mouse_button {
            Some(MouseButton::Left) => {
                println!("Left click at {:?}", event.pos);
                // Handle black stone placement
            }
            Some(MouseButton::Right) => {
                println!("Right click at {:?}", event.pos);
                // Handle white stone placement
            }
            _ => {}
        }
    });
```

### Hover Effects

```rust
let board_view = BoardView::new(Board::new())
    .on_hover(|pos| {
        if let Some(p) = pos {
            println!("Hovering over {:?}", p);
            // Show tooltip or highlight
        } else {
            // Mouse left the board
        }
    });
```

### Keyboard Navigation

```rust
let board_view = BoardView::new(Board::new())
    .on_key(|event| {
        match event.keystroke.key {
            Key::ArrowUp => Some(NavEvent::MoveFocus(Pos::new(0, -1))),
            Key::ArrowDown => Some(NavEvent::MoveFocus(Pos::new(0, 1))),
            Key::ArrowLeft => Some(NavEvent::MoveFocus(Pos::new(-1, 0))),
            Key::ArrowRight => Some(NavEvent::MoveFocus(Pos::new(1, 0))),
            Key::Enter => Some(NavEvent::Select(current_focus)),
            _ => None,
        }
    });
```

## Advanced Features

### Markers

```rust
use gpui_component::go_board::Marker;

let board = Board::new()
    .marker(Pos::new(3, 3), Marker::circle())
    .marker(Pos::new(4, 4), Marker::cross())
    .marker(Pos::new(5, 5), Marker::triangle())
    .marker(Pos::new(6, 6), Marker::square())
    .marker(Pos::new(7, 7), Marker::dot())
    .marker(Pos::new(8, 8), Marker::label("A"))
    .marker(Pos::new(9, 9), Marker::circle().with_color(rgb(0xff0000)));
```

### Ghost Stones

```rust
use gpui_component::go_board::Ghost;

let board = Board::new()
    .ghost(Pos::new(3, 3), Ghost::good(BLACK))      // Good move (green)
    .ghost(Pos::new(4, 4), Ghost::bad(WHITE))       // Bad move (red)
    .ghost(Pos::new(5, 5), Ghost::neutral(BLACK))   // Neutral move
    .ghost(Pos::new(6, 6), Ghost::good(WHITE).with_alpha(0.8));
```

### Heat Maps

```rust
use gpui_component::go_board::Heat;

let board = Board::new()
    .heat(Pos::new(3, 3), Heat::new(9).with_label("★"))
    .heat(Pos::new(4, 4), Heat::new(7))
    .heat(Pos::new(5, 5), Heat::new(5).with_label("5"))
    .heat(Pos::new(6, 6), Heat::new(3))
    .heat(Pos::new(7, 7), Heat::new(1));
```

### Lines and Arrows

```rust
use gpui_component::go_board::Line;

let board = Board::new()
    .line(Line::line(Pos::new(3, 3), Pos::new(15, 15)))
    .line(Line::arrow(Pos::new(0, 0), Pos::new(9, 9)))
    .line(Line::line(Pos::new(0, 9), Pos::new(9, 0)).with_color(rgb(0xff0000)))
    .line(Line::arrow(Pos::new(9, 0), Pos::new(0, 9)).with_width(3.0));
```

**Enhanced Line Styles:**

```rust
// Simple connection lines (gray)
.line(Line::connection(Pos::new(3, 3), Pos::new(15, 15)))

// Analysis arrows (dark)
.line(Line::analysis_arrow(Pos::new(0, 0), Pos::new(9, 9)))

// Highlight lines (blue)
.line(Line::highlight_line(Pos::new(4, 4), Pos::new(12, 12)))

// Direction arrows (red)
.line(Line::direction_arrow(Pos::new(1, 1), Pos::new(1, 5)))

// Custom styling
.line(Line::arrow(Pos::new(5, 5), Pos::new(10, 10))
    .with_color(rgb(0x00ff00))
    .with_width(4.0))
```

**Features:**
- **Proper rotation**: Lines and arrows automatically rotate to match their direction
- **Real arrowheads**: Arrows include properly shaped triangular arrowheads for clear direction indication
- **Smooth line rendering**: Lines use multiple overlapping segments for smooth, accurate representation
- **Color coding**: Different line types use distinct colors for better visual distinction
- **Responsive width**: Line thickness scales appropriately with board size
- **High-quality geometry**: Arrowheads use sophisticated mathematical calculations for proper triangular shapes

### Selections

```rust
use gpui_component::go_board::Selection;

let board = Board::new()
    .select(Pos::new(3, 3))           // Selected vertex (blue)
    .select(Pos::new(4, 4))           // Another selected vertex
    .last_move(Pos::new(5, 5));       // Last move indicator (orange)
```

## Bounded Boards

### Auto-sizing Boards

```rust
use gpui_component::go_board::BoundedBoard;

// Board that fits within 200x200 pixels
let bounded = BoundedBoard::new(200.0, 200.0);
let board = bounded.into_inner();

// Board with custom size constraints
let bounded = BoundedBoard::with_size(19, 19, 300.0, 400.0)
    .vertex_size_limits(10.0, 30.0);
let board = bounded.into_inner();
```

## API Reference

### Board Methods

| Method | Description | Returns |
|--------|-------------|---------|
| `new()` | Create standard 19×19 board | `Board` |
| `with_size(w, h)` | Create board with custom dimensions | `Board` |
| `theme(t)` | Set board theme | `Board` |
| `vertex_size(s)` | Set vertex size in pixels | `Board` |
| `coordinates(show)` | Show/hide coordinate labels | `Board` |
| `stone(pos, stone)` | Place stone at position | `Board` |
| `marker(pos, marker)` | Place marker at position | `Board` |
| `ghost(pos, ghost)` | Place ghost stone at position | `Board` |
| `heat(pos, heat)` | Set heat value at position | `Board` |
| `select(pos)` | Select vertex | `Board` |
| `last_move(pos)` | Mark last move | `Board` |
| `line(line)` | Add line/arrow | `Board` |

### BoardView Methods

| Method | Description | Returns |
|--------|-------------|---------|
| `new(board)` | Create view from board | `BoardView` |
| `coordinates(show)` | Show/hide coordinates | `BoardView` |
| `on_click(handler)` | Set click handler | `BoardView` |
| `on_hover(handler)` | Set hover handler | `BoardView` |
| `on_key(handler)` | Set keyboard handler | `BoardView` |

### Theme Methods

| Method | Description | Returns |
|--------|-------------|---------|
| `default()` | Default wooden theme | `Theme` |
| `dark()` | Dark theme | `Theme` |
| `minimal()` | Minimalist theme | `Theme` |
| `high_contrast()` | High contrast theme | `Theme` |
| `with_assets()` | Asset-based theme | `Theme` |
| `with_background(c)` | Set background color | `Theme` |
| `with_border(c)` | Set border color | `Theme` |
| `with_grid_lines(c)` | Set grid line color | `Theme` |
| `with_black_stone(c)` | Set black stone color | `Theme` |
| `with_white_stone(c)` | Set white stone color | `Theme` |

## Examples

### Complete Game Board

```rust
let game_board = Board::with_size(19, 19)
    .vertex_size(25.0)
    .theme(Theme::with_assets())
    .coordinates(true)
    .stone(Pos::new(3, 3), BLACK)
    .stone(Pos::new(15, 15), WHITE)
    .stone(Pos::new(9, 9), BLACK)
    .stone(Pos::new(12, 12), WHITE)
    .marker(Pos::new(9, 9), Marker::circle())
    .last_move(Pos::new(12, 12));

let board_view = BoardView::new(game_board)
    .on_click(|event| {
        // Handle stone placement
        println!("Placing stone at {:?}", event.pos);
    });
```

### Interactive Demo Board

```rust
let board = Board::with_size(9, 9)
    .vertex_size(35.0)
    .theme(Theme::dark())
    .coordinates(true)
    .stone(Pos::new(3, 3), BLACK)
    .stone(Pos::new(5, 5), WHITE)
    .ghost(Pos::new(4, 4), Ghost::good(BLACK))
    .marker(Pos::new(6, 6), Marker::cross().with_color(rgb(0xff0000)))
    .heat(Pos::new(4, 4), Heat::new(9).with_label("★"))
    .select(Pos::new(3, 3))
    .last_move(Pos::new(5, 5));

let interactive_view = BoardView::new(board)
    .on_click(|event| {
        match event.mouse_button {
            Some(MouseButton::Left) => println!("Black stone at {:?}", event.pos),
            Some(MouseButton::Right) => println!("White stone at {:?}", event.pos),
            _ => {}
        }
    })
    .on_hover(|pos| {
        if let Some(p) = pos {
            println!("Hovering over {:?}", p);
        }
    });
```

### Fully Functional Go Board Application

This example demonstrates how to create a complete, interactive Go board application with state management, stone placement, and visual updates.

#### Complete Application Structure

```rust
use gpui::*;
use gpui_component::{
    go_board::{Board, BoardView, Pos, Theme, BLACK, EMPTY, WHITE},
    v_flex, ActiveTheme,
};
use std::sync::{Arc, Mutex};
use story::Assets;

/// Fully functional Go board application with state management
pub struct GoBoardApp {
    board: Arc<Mutex<Board>>,
}

impl GoBoardApp {
    fn new() -> Self {
        // Create the initial board with some starting stones
        let board = Board::with_size(19, 19)
            .vertex_size(25.0)
            .theme(Theme::with_assets())
            .coordinates(true)
            .stone(Pos::new(3, 3), BLACK)    // Starting black stone
            .stone(Pos::new(15, 15), WHITE); // Starting white stone

        Self {
            board: Arc::new(Mutex::new(board)),
        }
    }
}

impl Render for GoBoardApp {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let board_arc = self.board.clone();

        v_flex()
            .size_full()
            .bg(cx.theme().background)
            .justify_center()
            .items_center()
            .gap_4()
            .child(
                v_flex()
                    .gap_2()
                    .child("Go Board Game")
                    .child("Left click: Black stone | Right click: White stone")
                    .child("Click on empty intersections to place stones"),
            )
            .child(cx.new(|_| {
                // Get the current board state for rendering
                let board = board_arc.lock().unwrap().clone();

                BoardView::new(board).on_click(move |event| {
                    let pos = event.pos;
                    let stone_type = match event.mouse_button {
                        Some(MouseButton::Left) => "BLACK",
                        Some(MouseButton::Right) => "WHITE",
                        _ => "UNKNOWN",
                    };

                    // Update the board state
                    if let Ok(mut board_guard) = board_arc.lock() {
                        let current_board = board_guard.clone();

                        // Check if position is already occupied
                        if current_board.stone_at(pos) != EMPTY {
                            println!("Position ({}, {}) is already occupied!", pos.x, pos.y);
                            return;
                        }

                        // Place the stone
                        let new_board = current_board
                            .stone(pos, if stone_type == "BLACK" { BLACK } else { WHITE });
                        *board_guard = new_board;

                        println!("Placed {} stone at ({}, {})", stone_type, pos.x, pos.y);
                    }
                })
            }))
    }
}

fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        gpui_component::init(cx);

        cx.open_window(Default::default(), |window, cx| {
            window.set_window_title("Go Board Game");
            cx.new(|_| GoBoardApp::new())
        })
        .unwrap();
    });
}
```

#### Key Implementation Details

**1. State Management with Arc<Mutex<Board>>**
```rust
pub struct GoBoardApp {
    board: Arc<Mutex<Board>>, // Thread-safe shared mutable state
}
```

**2. Initial Board Setup**
```rust
let board = Board::with_size(19, 19)
    .vertex_size(25.0)                    // Appropriate stone size
    .theme(Theme::with_assets())          // Use built-in assets
    .coordinates(true)                     // Show coordinate labels
    .stone(Pos::new(3, 3), BLACK)        // Starting position
    .stone(Pos::new(15, 15), WHITE);     // Starting position
```

**3. Event Handling with State Updates**
```rust
.on_click(move |event| {
    let pos = event.pos;
    let stone_type = match event.mouse_button {
        Some(MouseButton::Left) => "BLACK",
        Some(MouseButton::Right) => "WHITE",
        _ => "UNKNOWN",
    };

    // Thread-safe state update
    if let Ok(mut board_guard) = board_arc.lock() {
        let current_board = board_guard.clone();

        // Validation: check if position is empty
        if current_board.stone_at(pos) != EMPTY {
            println!("Position already occupied!");
            return;
        }

        // Update board state
        let new_board = current_board.stone(pos, stone_type);
        *board_guard = new_board;
    }
})
```

**4. UI Layout and Styling**
```rust
v_flex()
    .size_full()                           // Full window size
    .bg(cx.theme().background)            // Use system theme
    .justify_center()                      // Center content vertically
    .items_center()                        // Center content horizontally
    .gap_4()                              // Consistent spacing
    .child(instructions)                   // Game instructions
    .child(board_view)                     // Interactive board
```

#### Running the Application

```bash
# Run the complete Go board application
cargo run -p story --example asset_go_board
```

#### Features Implemented

- ✅ **Interactive Stone Placement**: Left-click for black, right-click for white
- ✅ **State Persistence**: Stones remain on the board after placement
- ✅ **Collision Detection**: Prevents placing stones on occupied positions
- ✅ **Visual Feedback**: Console output for stone placement
- ✅ **Responsive Layout**: Centered board with instructions
- ✅ **Asset Integration**: Uses built-in board and stone graphics
- ✅ **Coordinate System**: Shows standard Go notation (A-T, 1-19)
- ✅ **Theme Support**: Asset-based rendering with fallbacks

#### Extending the Application

**Add Game Logic**
```rust
// Track current player
let mut current_player = BLACK;

// In click handler
let new_board = current_board.stone(pos, current_player);
current_player = if current_player == BLACK { WHITE } else { BLACK };
```

**Add Move History**
```rust
// Track moves
let mut move_history = Vec::new();

// In click handler
move_history.push((pos, stone_type));
```