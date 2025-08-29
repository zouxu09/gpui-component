use gpui::{rgb, Hsla, Modifiers, MouseButton, Rgba, SharedString};
use std::collections::HashMap;

// =============================================================================
// CORE TYPES - Simplified and consolidated
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

/// Range of positions visible on the board
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
// BOARD CONTENT TYPES - Simplified
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
    pub alpha: f32,
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
    pub owner: Stone,
    pub alpha: f32,
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
                color: rgb(0x808080), // Gray for simple lines
                width: 2.0,
            },
        }
    }

    pub fn arrow(from: Pos, to: Pos) -> Self {
        Self {
            from,
            to,
            style: LineStyle::Arrow {
                color: rgb(0x404040), // Dark gray/black for arrows
                width: 2.5,
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

    pub fn with_style(mut self, style: LineStyle) -> Self {
        self.style = style;
        self
    }

    // Convenience methods for common line types
    pub fn connection(from: Pos, to: Pos) -> Self {
        Self::line(from, to).with_color(rgb(0x808080)) // Gray connection lines
    }

    pub fn analysis_arrow(from: Pos, to: Pos) -> Self {
        Self::arrow(from, to).with_color(rgb(0x404040)) // Dark analysis arrows
    }

    pub fn highlight_line(from: Pos, to: Pos) -> Self {
        Self::line(from, to).with_color(rgb(0x0066cc)).with_width(3.0) // Blue highlight lines
    }

    pub fn direction_arrow(from: Pos, to: Pos) -> Self {
        Self::arrow(from, to).with_color(rgb(0xcc3300)).with_width(2.5) // Red direction arrows
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
// BOARD DATA - Simplified storage
// =============================================================================

/// Board data using sparse HashMap storage
#[derive(Debug, Clone, Default)]
pub struct BoardData {
    pub stones: HashMap<Pos, Stone>,
    pub markers: HashMap<Pos, Marker>,
    pub ghosts: HashMap<Pos, Ghost>,
    pub heat: HashMap<Pos, Heat>,
    pub territory: HashMap<Pos, Territory>,
    pub selections: HashMap<Pos, Selection>,
    pub lines: Vec<Line>,
    pub size: (usize, usize),
    pub range: Range,
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
                Some(m) => self.markers.insert(pos, m),
                None => self.markers.remove(&pos),
            };
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
                Some(g) => self.ghosts.insert(pos, g),
                None => self.ghosts.remove(&pos),
            };
        }
    }

    pub fn clear_ghosts(&mut self) {
        self.ghosts.clear();
    }

    // Heat operations
    pub fn set_heat(&mut self, pos: Pos, heat: Option<Heat>) {
        if self.is_valid_pos(pos) {
            match heat {
                Some(h) => self.heat.insert(pos, h),
                None => self.heat.remove(&pos),
            };
        }
    }

    pub fn clear_heat(&mut self) {
        self.heat.clear();
    }

    // Territory operations
    pub fn set_territory(&mut self, pos: Pos, territory: Option<Territory>) {
        if self.is_valid_pos(pos) {
            match territory {
                Some(t) => self.territory.insert(pos, t),
                None => self.territory.remove(&pos),
            };
        }
    }

    pub fn clear_territory(&mut self) {
        self.territory.clear();
    }

    // Selection operations
    pub fn set_selection(&mut self, pos: Pos, selection: Option<Selection>) {
        if self.is_valid_pos(pos) {
            match selection {
                Some(s) => self.selections.insert(pos, s),
                None => self.selections.remove(&pos),
            };
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

        // Remove out-of-bounds positions
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
// EVENT TYPES - Simplified
// =============================================================================

/// Mouse events on board positions
#[derive(Debug, Clone)]
pub struct PosEvent {
    pub pos: Pos,
    pub modifiers: Modifiers,
    pub mouse_button: Option<MouseButton>,
}

impl PosEvent {
    pub fn new(pos: Pos, modifiers: Modifiers) -> Self {
        Self {
            pos,
            modifiers,
            mouse_button: None,
        }
    }

    pub fn with_mouse_button(pos: Pos, modifiers: Modifiers, button: MouseButton) -> Self {
        Self {
            pos,
            modifiers,
            mouse_button: Some(button),
        }
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
// THEME - Simplified
// =============================================================================

/// Unified theme for the Go board
#[derive(Debug, Clone)]
pub struct Theme {
    pub background: Hsla,
    pub border: Hsla,
    pub border_width: f32,
    pub grid_lines: Hsla,
    pub grid_width: f32,
    pub star_points: Hsla,
    pub star_size: f32,
    pub black_stone: Hsla,
    pub white_stone: Hsla,
    pub stone_size: f32,
    pub coordinates: Hsla,
    pub coord_size: f32,
    // Asset support
    pub board_background_path: Option<SharedString>,
    pub black_stone_path: Option<SharedString>,
    pub white_stone_path: Option<SharedString>,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: rgb(0xebb55b).into(),
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
            board_background_path: None,
            black_stone_path: None,
            white_stone_path: None,
        }
    }
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            background: rgb(0x2d2d2d).into(),
            border: rgb(0x404040).into(),
            grid_lines: rgb(0x808080).into(),
            coordinates: rgb(0xcccccc).into(),
            ..Default::default()
        }
    }

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
            ..Default::default()
        }
    }

    pub fn with_assets() -> Self {
        Self {
            board_background_path: Some("icons/board.png".into()),
            black_stone_path: Some("icons/black_stone.svg".into()),
            white_stone_path: Some("icons/white_stone.svg".into()),
            ..Default::default()
        }
    }

    pub fn with_board_background(mut self, path: impl Into<SharedString>) -> Self {
        self.board_background_path = Some(path.into());
        self
    }

    pub fn with_black_stone_asset(mut self, path: impl Into<SharedString>) -> Self {
        self.black_stone_path = Some(path.into());
        self
    }

    pub fn with_white_stone_asset(mut self, path: impl Into<SharedString>) -> Self {
        self.white_stone_path = Some(path.into());
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

        board.set_stone(Pos::new(4, 4), BLACK);
        assert_eq!(board.get_stone(Pos::new(4, 4)), BLACK);
        assert_eq!(board.get_stone(Pos::new(0, 0)), EMPTY);

        board.set_marker(Pos::new(2, 2), Some(Marker::circle()));
        assert!(board.get_marker(Pos::new(2, 2)).is_some());

        board.clear_stones();
        assert_eq!(board.get_stone(Pos::new(4, 4)), EMPTY);
    }

    #[test]
    fn test_marker_builder() {
        let marker = Marker::circle().with_color(rgb(0xff0000).into());
        if let Marker::Circle { color } = marker {
            assert_eq!(color, rgb(0xff0000).into());
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
        board.set_stone(Pos::new(10, 10), WHITE);

        board.resize(5, 5);
        assert_eq!(board.size, (5, 5));
        assert_eq!(board.get_stone(Pos::new(8, 8)), EMPTY);
    }

    // =============================================================================
    // THEME SYSTEM TESTS - TDD Approach
    // =============================================================================

    #[test]
    fn test_theme_default_values() {
        let theme = Theme::default();

        // Verify default colors are set
        assert_eq!(theme.background, rgb(0xebb55b).into());
        assert_eq!(theme.border, rgb(0xca933a).into());
        assert_eq!(theme.black_stone, rgb(0x000000).into());
        assert_eq!(theme.white_stone, rgb(0xffffff).into());

        // Verify default dimensions
        assert_eq!(theme.border_width, 2.0);
        assert_eq!(theme.grid_width, 1.0);
        assert_eq!(theme.stone_size, 0.85);
        assert_eq!(theme.coord_size, 16.0);

        // Verify no assets by default
        assert!(theme.board_background_path.is_none());
        assert!(theme.black_stone_path.is_none());
        assert!(theme.white_stone_path.is_none());
    }

    #[test]
    fn test_theme_variants() {
        let dark_theme = Theme::dark();
        let high_contrast_theme = Theme::high_contrast();
        let minimal_theme = Theme::minimal();

        // Dark theme should override specific colors
        assert_eq!(dark_theme.background, rgb(0x2d2d2d).into());
        assert_eq!(dark_theme.border, rgb(0x404040).into());
        assert_eq!(dark_theme.grid_lines, rgb(0x808080).into());
        assert_eq!(dark_theme.coordinates, rgb(0xcccccc).into());

        // High contrast should have stronger borders
        assert_eq!(high_contrast_theme.border_width, 3.0);
        assert_eq!(high_contrast_theme.grid_width, 2.0);
        assert_eq!(high_contrast_theme.coord_size, 14.0);

        // Minimal should have subtle styling
        assert_eq!(minimal_theme.border_width, 1.0);
        assert_eq!(minimal_theme.grid_width, 0.5);
        assert_eq!(minimal_theme.star_size, 4.0);
        assert_eq!(minimal_theme.coord_size, 10.0);
    }

    #[test]
    fn test_theme_asset_loading() {
        let theme = Theme::with_assets();

        // Should have asset paths set
        assert_eq!(
            *theme.board_background_path.as_deref().unwrap(),
            gpui::ArcCow::Borrowed("icons/board.png")
        );
        assert_eq!(
            *theme.black_stone_path.as_deref().unwrap(),
            gpui::ArcCow::Borrowed("icons/black_stone.svg")
        );
        assert_eq!(
            *theme.white_stone_path.as_deref().unwrap(),
            gpui::ArcCow::Borrowed("icons/white_stone.svg")
        );

        // Other properties should remain default
        assert_eq!(theme.background, rgb(0xebb55b).into());
        assert_eq!(theme.border_width, 2.0);
    }

    #[test]
    fn test_theme_builder_pattern() {
        let theme = Theme::default()
            .with_board_background("custom/board.jpg")
            .with_black_stone_asset("custom/black.png")
            .with_white_stone_asset("custom/white.png");

        assert_eq!(
            *theme.board_background_path.as_deref().unwrap(),
            gpui::ArcCow::Borrowed("custom/board.jpg")
        );
        assert_eq!(
            *theme.black_stone_path.as_deref().unwrap(),
            gpui::ArcCow::Borrowed("custom/black.png")
        );
        assert_eq!(
            *theme.white_stone_path.as_deref().unwrap(),
            gpui::ArcCow::Borrowed("custom/white.png")
        );
    }

    #[test]
    fn test_theme_color_consistency() {
        let theme = Theme::default();

        // Colors should be valid Hsla values
        assert!(theme.background.h >= 0.0 && theme.background.h <= 1.0);
        assert!(theme.background.s >= 0.0 && theme.background.s <= 1.0);
        assert!(theme.background.l >= 0.0 && theme.background.l <= 1.0);
        assert!(theme.background.a >= 0.0 && theme.background.a <= 1.0);

        // Stone colors should be distinct
        assert_ne!(theme.black_stone, theme.white_stone);

        // Grid lines should be visible
        assert!(theme.grid_lines.l < 0.5 || theme.grid_lines.l > 0.5);
    }
}
