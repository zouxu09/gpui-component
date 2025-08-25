use gpui::*;
use std::collections::HashMap;

// Core types for the Go board - simplified and consolidated
// This replaces the scattered type definitions across multiple files

// =============================================================================
// CORE POSITION TYPES
// =============================================================================

/// A position on the Go board
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

/// Range of positions visible on the board (like Shudan's rangeX/rangeY)
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Range {
    pub x: (usize, usize),
    pub y: (usize, usize),
}

impl Range {
    pub fn new(x_range: (usize, usize), y_range: (usize, usize)) -> Self {
        Self {
            x: x_range,
            y: y_range,
        }
    }

    pub fn full(width: usize, height: usize) -> Self {
        Self::new((0, width.saturating_sub(1)), (0, height.saturating_sub(1)))
    }

    pub fn width(&self) -> usize {
        self.x.1.saturating_sub(self.x.0) + 1
    }

    pub fn height(&self) -> usize {
        self.y.1.saturating_sub(self.y.0) + 1
    }

    pub fn contains(&self, pos: Pos) -> bool {
        pos.x >= self.x.0 && pos.x <= self.x.1 && pos.y >= self.y.0 && pos.y <= self.y.1
    }
}

// =============================================================================
// BOARD CONTENT TYPES
// =============================================================================

/// Stone color/player (-1: white, 0: empty, 1: black)
pub type Stone = i8;

pub const EMPTY: Stone = 0;
pub const BLACK: Stone = 1;
pub const WHITE: Stone = -1;

/// Marker types for board annotations
#[derive(Debug, Clone, PartialEq)]
pub enum Marker {
    Circle { color: Hsla },
    Cross { color: Hsla },
    Triangle { color: Hsla },
    Square { color: Hsla },
    Dot { color: Hsla },
    Label { text: String, color: Hsla },
}

impl Marker {
    pub fn circle() -> Self {
        Self::Circle {
            color: rgb(0x000000).into(),
        }
    }
    pub fn cross() -> Self {
        Self::Cross {
            color: rgb(0x000000).into(),
        }
    }
    pub fn triangle() -> Self {
        Self::Triangle {
            color: rgb(0x000000).into(),
        }
    }
    pub fn square() -> Self {
        Self::Square {
            color: rgb(0x000000).into(),
        }
    }
    pub fn dot() -> Self {
        Self::Dot {
            color: rgb(0x000000).into(),
        }
    }
    pub fn label(text: impl Into<String>) -> Self {
        Self::Label {
            text: text.into(),
            color: rgb(0x000000).into(),
        }
    }

    pub fn with_color(mut self, color: Hsla) -> Self {
        match &mut self {
            Self::Circle { color: c }
            | Self::Cross { color: c }
            | Self::Triangle { color: c }
            | Self::Square { color: c }
            | Self::Dot { color: c }
            | Self::Label { color: c, .. } => *c = color,
        }
        self
    }
}

/// Ghost stones for move analysis
#[derive(Debug, Clone, PartialEq)]
pub struct Ghost {
    pub stone: Stone,
    pub kind: GhostKind,
    pub alpha: f32, // 0.0 - 1.0
}

#[derive(Debug, Clone, PartialEq)]
pub enum GhostKind {
    Good,    // Green tint
    Bad,     // Red tint
    Neutral, // No tint
}

impl Ghost {
    pub fn good(stone: Stone) -> Self {
        Self {
            stone,
            kind: GhostKind::Good,
            alpha: 0.6,
        }
    }

    pub fn bad(stone: Stone) -> Self {
        Self {
            stone,
            kind: GhostKind::Bad,
            alpha: 0.6,
        }
    }

    pub fn neutral(stone: Stone) -> Self {
        Self {
            stone,
            kind: GhostKind::Neutral,
            alpha: 0.6,
        }
    }

    pub fn with_alpha(mut self, alpha: f32) -> Self {
        self.alpha = alpha.clamp(0.0, 1.0);
        self
    }
}

/// Heat/influence visualization
#[derive(Debug, Clone, PartialEq)]
pub struct Heat {
    pub strength: u8, // 0-9
    pub label: Option<String>,
}

impl Heat {
    pub fn new(strength: u8) -> Self {
        Self {
            strength: strength.min(9),
            label: None,
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

/// Territory/area marking
#[derive(Debug, Clone, PartialEq)]
pub struct Territory {
    pub owner: Stone, // BLACK, WHITE, or EMPTY for neutral
    pub alpha: f32,   // 0.0 - 1.0
}

impl Territory {
    pub fn black(alpha: f32) -> Self {
        Self {
            owner: BLACK,
            alpha: alpha.clamp(0.0, 1.0),
        }
    }

    pub fn white(alpha: f32) -> Self {
        Self {
            owner: WHITE,
            alpha: alpha.clamp(0.0, 1.0),
        }
    }

    pub fn neutral(alpha: f32) -> Self {
        Self {
            owner: EMPTY,
            alpha: alpha.clamp(0.0, 1.0),
        }
    }
}

/// Lines and arrows for analysis
#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub from: Pos,
    pub to: Pos,
    pub style: LineStyle,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineStyle {
    Line { color: Rgba, width: f32 },
    Arrow { color: Rgba, width: f32 },
}

impl Line {
    pub fn line(from: Pos, to: Pos) -> Self {
        Self {
            from,
            to,
            style: LineStyle::Line {
                color: rgb(0x000000),
                width: 2.0,
            },
        }
    }

    pub fn arrow(from: Pos, to: Pos) -> Self {
        Self {
            from,
            to,
            style: LineStyle::Arrow {
                color: rgb(0x000000),
                width: 2.0,
            },
        }
    }

    pub fn with_color(mut self, color: Rgba) -> Self {
        match &mut self.style {
            LineStyle::Line { color: c, .. } | LineStyle::Arrow { color: c, .. } => *c = color,
        }
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        match &mut self.style {
            LineStyle::Line { width: w, .. } | LineStyle::Arrow { width: w, .. } => *w = width,
        }
        self
    }
}

/// Selection highlighting
#[derive(Debug, Clone, PartialEq)]
pub struct Selection {
    pub pos: Pos,
    pub style: SelectionStyle,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SelectionStyle {
    Selected { color: Rgba },
    Dimmed { alpha: f32 },
    LastMove { color: Rgba },
}

impl Selection {
    pub fn selected(pos: Pos) -> Self {
        Self {
            pos,
            style: SelectionStyle::Selected {
                color: rgb(0x0080ff),
            },
        }
    }

    pub fn dimmed(pos: Pos, alpha: f32) -> Self {
        Self {
            pos,
            style: SelectionStyle::Dimmed {
                alpha: alpha.clamp(0.0, 1.0),
            },
        }
    }

    pub fn last_move(pos: Pos) -> Self {
        Self {
            pos,
            style: SelectionStyle::LastMove {
                color: rgb(0xff8000),
            },
        }
    }

    pub fn with_color(mut self, color: Rgba) -> Self {
        match &mut self.style {
            SelectionStyle::Selected { color: c } | SelectionStyle::LastMove { color: c } => {
                *c = color
            }
            _ => {}
        }
        self
    }
}

// =============================================================================
// BOARD DATA CONTAINERS
// =============================================================================

/// Simplified board data using sparse HashMap storage for efficiency
#[derive(Debug, Clone, Default)]
pub struct BoardData {
    // Core game state
    pub stones: HashMap<Pos, Stone>,

    // Visual annotations
    pub markers: HashMap<Pos, Marker>,
    pub ghosts: HashMap<Pos, Ghost>,
    pub heat: HashMap<Pos, Heat>,
    pub territory: HashMap<Pos, Territory>,
    pub selections: HashMap<Pos, Selection>,

    // Lines and overlays
    pub lines: Vec<Line>,

    // Board dimensions
    pub size: (usize, usize), // (width, height)
    pub range: Range,         // Visible area
}

impl BoardData {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            size: (width, height),
            range: Range::full(width, height),
            ..Default::default()
        }
    }

    pub fn standard() -> Self {
        Self::new(19, 19)
    }

    // Stone operations
    pub fn set_stone(&mut self, pos: Pos, stone: Stone) {
        if self.is_valid_pos(pos) {
            if stone == EMPTY {
                self.stones.remove(&pos);
            } else {
                self.stones.insert(pos, stone);
            }
        }
    }

    pub fn get_stone(&self, pos: Pos) -> Stone {
        self.stones.get(&pos).copied().unwrap_or(EMPTY)
    }

    pub fn clear_stones(&mut self) {
        self.stones.clear();
    }

    // Marker operations
    pub fn set_marker(&mut self, pos: Pos, marker: Option<Marker>) {
        if self.is_valid_pos(pos) {
            match marker {
                Some(m) => {
                    self.markers.insert(pos, m);
                }
                None => {
                    self.markers.remove(&pos);
                }
            }
        }
    }

    pub fn get_marker(&self, pos: Pos) -> Option<&Marker> {
        self.markers.get(&pos)
    }

    pub fn clear_markers(&mut self) {
        self.markers.clear();
    }

    // Ghost stone operations
    pub fn set_ghost(&mut self, pos: Pos, ghost: Option<Ghost>) {
        if self.is_valid_pos(pos) {
            match ghost {
                Some(g) => {
                    self.ghosts.insert(pos, g);
                }
                None => {
                    self.ghosts.remove(&pos);
                }
            }
        }
    }

    pub fn clear_ghosts(&mut self) {
        self.ghosts.clear();
    }

    // Heat operations
    pub fn set_heat(&mut self, pos: Pos, heat: Option<Heat>) {
        if self.is_valid_pos(pos) {
            match heat {
                Some(h) => {
                    self.heat.insert(pos, h);
                }
                None => {
                    self.heat.remove(&pos);
                }
            }
        }
    }

    pub fn clear_heat(&mut self) {
        self.heat.clear();
    }

    // Territory operations
    pub fn set_territory(&mut self, pos: Pos, territory: Option<Territory>) {
        if self.is_valid_pos(pos) {
            match territory {
                Some(t) => {
                    self.territory.insert(pos, t);
                }
                None => {
                    self.territory.remove(&pos);
                }
            }
        }
    }

    pub fn clear_territory(&mut self) {
        self.territory.clear();
    }

    // Selection operations
    pub fn set_selection(&mut self, pos: Pos, selection: Option<Selection>) {
        if self.is_valid_pos(pos) {
            match selection {
                Some(s) => {
                    self.selections.insert(pos, s);
                }
                None => {
                    self.selections.remove(&pos);
                }
            }
        }
    }

    pub fn clear_selections(&mut self) {
        self.selections.clear();
    }

    // Line operations
    pub fn add_line(&mut self, line: Line) {
        self.lines.push(line);
    }

    pub fn clear_lines(&mut self) {
        self.lines.clear();
    }

    // Utility functions
    pub fn is_valid_pos(&self, pos: Pos) -> bool {
        pos.x < self.size.0 && pos.y < self.size.1
    }

    pub fn set_range(&mut self, range: Range) {
        self.range = range;
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.size = (width, height);
        self.range = Range::full(width, height);

        // Remove any positions that are now out of bounds
        let new_size = (width, height);
        self.stones
            .retain(|&pos, _| pos.x < new_size.0 && pos.y < new_size.1);
        self.markers
            .retain(|&pos, _| pos.x < new_size.0 && pos.y < new_size.1);
        self.ghosts
            .retain(|&pos, _| pos.x < new_size.0 && pos.y < new_size.1);
        self.heat
            .retain(|&pos, _| pos.x < new_size.0 && pos.y < new_size.1);
        self.territory
            .retain(|&pos, _| pos.x < new_size.0 && pos.y < new_size.1);
        self.selections
            .retain(|&pos, _| pos.x < new_size.0 && pos.y < new_size.1);
    }
}

// =============================================================================
// EVENT TYPES
// =============================================================================

/// Mouse events on board positions
#[derive(Debug, Clone)]
pub struct PosEvent {
    pub pos: Pos,
    pub modifiers: Modifiers,
}

impl PosEvent {
    pub fn new(pos: Pos, modifiers: Modifiers) -> Self {
        Self { pos, modifiers }
    }
}

/// Keyboard navigation events
#[derive(Debug, Clone)]
pub enum NavEvent {
    MoveFocus(Pos),
    Select(Pos),
    ClearSelection,
}

// =============================================================================
// THEME SIMPLIFICATION
// =============================================================================

/// Unified theme for the entire Go board
#[derive(Debug, Clone)]
pub struct Theme {
    // Board appearance
    pub background: Hsla,
    pub border: Hsla,
    pub border_width: f32,

    // Grid
    pub grid_lines: Hsla,
    pub grid_width: f32,
    pub star_points: Hsla,
    pub star_size: f32,

    // Stones
    pub black_stone: Hsla,
    pub white_stone: Hsla,
    pub stone_size: f32, // Ratio of vertex size (0.0-1.0)

    // Coordinates
    pub coordinates: Hsla,
    pub coord_size: f32,

    // Effects
    pub stone_shadow: bool,
    pub fuzzy_stones: bool,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: rgb(0xebb55b).into(), // Traditional wood color
            border: rgb(0xca933a).into(),
            border_width: 2.0,

            grid_lines: rgb(0x000000).into(),
            grid_width: 1.0,
            star_points: rgb(0x000000).into(),
            star_size: 6.0,

            black_stone: rgb(0x000000).into(),
            white_stone: rgb(0xffffff).into(),
            stone_size: 0.85,

            coordinates: rgb(0x000000).into(),
            coord_size: 16.0,

            stone_shadow: true,
            fuzzy_stones: false,
        }
    }
}

impl Theme {
    /// Dark theme variant
    pub fn dark() -> Self {
        Self {
            background: rgb(0x2d2d2d).into(),
            border: rgb(0x404040).into(),
            grid_lines: rgb(0x808080).into(),
            coordinates: rgb(0xcccccc).into(),
            ..Default::default()
        }
    }

    /// High contrast theme
    pub fn high_contrast() -> Self {
        Self {
            background: rgb(0xffffff).into(),
            border: rgb(0x000000).into(),
            border_width: 3.0,
            grid_lines: rgb(0x000000).into(),
            grid_width: 2.0,
            coordinates: rgb(0x000000).into(),
            coord_size: 14.0,
            ..Default::default()
        }
    }

    /// Minimal theme
    pub fn minimal() -> Self {
        Self {
            background: rgb(0xf8f8f8).into(),
            border: rgb(0xe0e0e0).into(),
            border_width: 1.0,
            grid_lines: rgb(0x666666).into(),
            grid_width: 0.5,
            star_points: rgb(0x666666).into(),
            star_size: 4.0,
            coordinates: rgb(0x666666).into(),
            coord_size: 10.0,
            stone_shadow: false,
            ..Default::default()
        }
    }

    /// Builder methods for theme customization
    pub fn with_board_background(mut self, color: Hsla) -> Self {
        self.background = color;
        self
    }

    pub fn with_stone_colors(mut self, black: Hsla, white: Hsla) -> Self {
        self.black_stone = black;
        self.white_stone = white;
        self
    }

    pub fn with_grid_lines(mut self, color: Hsla, width: f32) -> Self {
        self.grid_lines = color;
        self.grid_width = width;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pos_creation() {
        let pos = Pos::new(3, 4);
        assert_eq!(pos.x, 3);
        assert_eq!(pos.y, 4);
    }

    #[test]
    fn test_range_functionality() {
        let range = Range::new((2, 8), (1, 7));
        assert_eq!(range.width(), 7);
        assert_eq!(range.height(), 7);
        assert!(range.contains(Pos::new(5, 4)));
        assert!(!range.contains(Pos::new(0, 0)));
    }

    #[test]
    fn test_board_data_operations() {
        let mut board = BoardData::new(9, 9);

        // Test stone operations
        board.set_stone(Pos::new(4, 4), BLACK);
        assert_eq!(board.get_stone(Pos::new(4, 4)), BLACK);
        assert_eq!(board.get_stone(Pos::new(0, 0)), EMPTY);

        // Test marker operations
        board.set_marker(Pos::new(2, 2), Some(Marker::circle()));
        assert!(board.get_marker(Pos::new(2, 2)).is_some());

        // Test clear operations
        board.clear_stones();
        assert_eq!(board.get_stone(Pos::new(4, 4)), EMPTY);
    }

    #[test]
    fn test_marker_builder() {
        let marker = Marker::circle().with_color(rgb(0xff0000));
        if let Marker::Circle { color } = marker {
            assert_eq!(color, rgb(0xff0000));
        } else {
            panic!("Expected circle marker");
        }
    }

    #[test]
    fn test_ghost_stone_builder() {
        let ghost = Ghost::good(BLACK).with_alpha(0.8);
        assert_eq!(ghost.stone, BLACK);
        assert_eq!(ghost.kind, GhostKind::Good);
        assert_eq!(ghost.alpha, 0.8);
    }

    #[test]
    fn test_board_resize() {
        let mut board = BoardData::new(9, 9);
        board.set_stone(Pos::new(8, 8), BLACK);
        board.set_stone(Pos::new(10, 10), WHITE); // Out of bounds

        board.resize(5, 5);
        assert_eq!(board.size, (5, 5));
        assert_eq!(board.get_stone(Pos::new(8, 8)), EMPTY); // Removed due to resize
    }
}
