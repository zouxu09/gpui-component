pub mod bounded_go_board;
pub mod coordinates;
pub mod error;
pub mod ghost_stone;
pub mod go_board;
pub mod grid;
pub mod heat_overlay;
pub mod interactions;
pub mod keyboard_navigation;
pub mod line_overlay;
pub mod markers;
pub mod paint_overlay;
pub mod position_utils;
pub mod selection;
pub mod state;
pub mod stones;

pub mod theme;

pub mod types;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod grid_tests;

#[cfg(test)]
mod integration_tests;

#[cfg(test)]
mod error_validation_tests;

pub use bounded_go_board::BoundedGoBoard;
pub use coordinates::{default_coord_x, default_coord_y, CoordinateLabels, CoordinateTheme};

pub use error::{GoBoardError, GoBoardResult, GoBoardValidator};
pub use ghost_stone::{GhostStoneOverlay, GhostStoneRenderer, GhostStoneTheme};
pub use go_board::GoBoard;
pub use grid::{Grid, GridTheme};
pub use heat_overlay::{HeatOverlay, HeatOverlayRenderer};
pub use interactions::{
    VertexButton, VertexClickEvent, VertexEventHandlers, VertexInteractions, VertexMouseDownEvent,
    VertexMouseMoveEvent, VertexMouseUpEvent,
};
pub use keyboard_navigation::{
    AccessibleSelectionManager, KeyboardNavigation, NavigationAction, SelectionUpdate,
};
pub use line_overlay::{LineOverlay, LineRenderer, LineTheme};
pub use markers::{MarkerRenderer, Markers};

pub use paint_overlay::{
    CornerPaint, CornerPosition, DirectionalPaintMap, PaintDirection, PaintOverlay,
    PaintOverlayRenderer,
};
pub use position_utils::{PositionCalculator, PositionUtils};
pub use selection::{SelectionRenderer, VertexSelections};
pub use state::GoBoardState;
pub use stones::{Stone, StoneTheme, Stones};

pub use theme::BoardTheme;
pub use types::*;
