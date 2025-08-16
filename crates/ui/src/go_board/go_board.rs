use crate::go_board::{
    GoBoardState, Grid, GridTheme, Markers, PaintOverlay, StoneTheme, Stones, Vertex,
    VertexEventHandlers, VertexInteractions, VertexSelections,
};
use gpui::*;

/// Main Go board component following GPUI reactive architecture
/// Provides a flexible, customizable Go board display inspired by Shudan
pub struct GoBoard {
    state: GoBoardState,
    grid_theme: GridTheme,
    stone_theme: StoneTheme,
}

impl GoBoard {
    /// Creates a new Go board with default 19x19 dimensions
    pub fn new() -> Self {
        Self {
            state: GoBoardState::standard(),
            grid_theme: GridTheme::default(),
            stone_theme: StoneTheme::default(),
        }
    }

    /// Creates a Go board with specified dimensions
    pub fn with_size(width: usize, height: usize) -> Self {
        Self {
            state: GoBoardState::new(width, height),
            grid_theme: GridTheme::default(),
            stone_theme: StoneTheme::default(),
        }
    }

    /// Creates a Go board with custom vertex size
    pub fn with_vertex_size(mut self, size: f32) -> Self {
        self.state.vertex_size = size;
        self
    }

    /// Creates a Go board with specified board range for partial display
    pub fn with_range(mut self, range: crate::go_board::BoardRange) -> Self {
        self.state.board_range = range;
        self
    }

    /// Creates a bounded Go board that fits within max dimensions
    pub fn with_bounded_size(mut self, max_width: f32, max_height: f32) -> Self {
        let (board_width, board_height) = self.state.dimensions();
        let max_vertex_size_x = max_width / board_width as f32;
        let max_vertex_size_y = max_height / board_height as f32;
        self.state.vertex_size = max_vertex_size_x.min(max_vertex_size_y).max(1.0);
        self
    }

    /// Gets a reference to the board state
    pub fn state(&self) -> &GoBoardState {
        &self.state
    }

    /// Gets a mutable reference to the board state
    pub fn state_mut(&mut self) -> &mut GoBoardState {
        &mut self.state
    }

    /// Sets the sign map (stone positions)
    pub fn set_sign_map(&mut self, sign_map: crate::go_board::SignMap) {
        if !sign_map.is_empty() && !sign_map[0].is_empty() {
            let height = sign_map.len();
            let width = sign_map[0].len();

            // Ensure all rows have the same length
            if sign_map.iter().all(|row| row.len() == width) {
                self.state.sign_map = sign_map;

                // Resize other maps to match if needed
                let (current_width, current_height) = self.state.dimensions();
                if current_width != width || current_height != height {
                    self.state.resize(width, height);
                    // Restore the sign map after resize
                    self.state.sign_map = self.state.sign_map.clone();
                }
            }
        }
    }

    /// Sets the marker map
    pub fn set_marker_map(&mut self, marker_map: crate::go_board::MarkerMap) {
        if !marker_map.is_empty() && !marker_map[0].is_empty() {
            let height = marker_map.len();
            let width = marker_map[0].len();
            let (current_width, current_height) = self.state.dimensions();

            if width == current_width && height == current_height {
                self.state.marker_map = marker_map;
            }
        }
    }

    /// Sets the selected vertices
    pub fn set_selected_vertices(&mut self, vertices: Vec<Vertex>) {
        self.state.selected_vertices = vertices
            .into_iter()
            .filter(|v| self.state.is_valid_vertex(v))
            .collect();
    }

    /// Sets the dimmed vertices
    pub fn set_dimmed_vertices(&mut self, vertices: Vec<Vertex>) {
        self.state.dimmed_vertices = vertices
            .into_iter()
            .filter(|v| self.state.is_valid_vertex(v))
            .collect();
    }

    /// Sets vertices with left-directional selection indicators
    pub fn set_selected_left(&mut self, vertices: Vec<Vertex>) {
        self.state.selected_left = vertices
            .into_iter()
            .filter(|v| self.state.is_valid_vertex(v))
            .collect();
    }

    /// Sets vertices with right-directional selection indicators
    pub fn set_selected_right(&mut self, vertices: Vec<Vertex>) {
        self.state.selected_right = vertices
            .into_iter()
            .filter(|v| self.state.is_valid_vertex(v))
            .collect();
    }

    /// Sets vertices with top-directional selection indicators
    pub fn set_selected_top(&mut self, vertices: Vec<Vertex>) {
        self.state.selected_top = vertices
            .into_iter()
            .filter(|v| self.state.is_valid_vertex(v))
            .collect();
    }

    /// Sets vertices with bottom-directional selection indicators
    pub fn set_selected_bottom(&mut self, vertices: Vec<Vertex>) {
        self.state.selected_bottom = vertices
            .into_iter()
            .filter(|v| self.state.is_valid_vertex(v))
            .collect();
    }

    /// Sets the paint map for territory visualization
    pub fn set_paint_map(&mut self, paint_map: crate::go_board::PaintMap) {
        if !paint_map.is_empty() && !paint_map[0].is_empty() {
            let height = paint_map.len();
            let width = paint_map[0].len();
            let (current_width, current_height) = self.state.dimensions();

            if width == current_width && height == current_height {
                self.state.paint_map = paint_map;
            }
        }
    }

    /// Sets the coordinate display visibility
    pub fn set_show_coordinates(&mut self, show: bool) {
        self.state.show_coordinates = show;
    }

    /// Sets the fuzzy stone placement mode
    pub fn set_fuzzy_stone_placement(&mut self, fuzzy: bool) {
        self.state.fuzzy_stone_placement = fuzzy;
    }

    /// Sets the stone placement animation mode
    pub fn set_animate_stone_placement(&mut self, animate: bool) {
        self.state.animate_stone_placement = animate;
    }

    /// Sets the busy state (disables interactions)
    pub fn set_busy(&mut self, busy: bool) {
        self.state.busy = busy;
    }

    /// Sets the grid theme
    pub fn set_grid_theme(&mut self, theme: GridTheme) {
        self.grid_theme = theme;
    }

    /// Sets the stone theme
    pub fn set_stone_theme(&mut self, theme: StoneTheme) {
        self.stone_theme = theme;
    }

    /// Gets a reference to the grid theme
    pub fn grid_theme(&self) -> &GridTheme {
        &self.grid_theme
    }

    /// Gets a reference to the stone theme
    pub fn stone_theme(&self) -> &StoneTheme {
        &self.stone_theme
    }

    /// Calculates the total board size in pixels
    pub fn board_pixel_size(&self) -> Size<Pixels> {
        let range_width = self.state.board_range.width() as f32;
        let range_height = self.state.board_range.height() as f32;

        Size {
            width: px(range_width * self.state.vertex_size),
            height: px(range_height * self.state.vertex_size),
        }
    }

    /// Renders the board with comprehensive vertex event handlers
    pub fn render_with_vertex_handlers(&self, handlers: VertexEventHandlers) -> impl IntoElement {
        // Create grid component with current state
        let grid = Grid::new(self.state.board_range.clone(), self.state.vertex_size)
            .with_theme(self.grid_theme.clone())
            .with_coordinates(self.state.show_coordinates);

        // Create stones component with current sign map
        let stones = Stones::new(
            self.state.board_range.clone(),
            self.state.vertex_size,
            self.state.sign_map.clone(),
        )
        .with_theme(self.stone_theme.clone());

        // Create markers component with current marker map
        let grid_offset = point(px(0.0), px(0.0)); // Will be adjusted based on actual grid positioning
        let markers = Markers::new(self.state.vertex_size, grid_offset);

        // Create selection component for highlighting selected and dimmed vertices
        let selections = VertexSelections::new(self.state.vertex_size, grid_offset);
        let selection_data = VertexSelections::from_board_state(
            &self.state.selected_vertices,
            &self.state.dimmed_vertices,
            &self.state.selected_left,
            &self.state.selected_right,
            &self.state.selected_top,
            &self.state.selected_bottom,
        );

        // Create paint overlay component for territory visualization
        let paint_overlay = PaintOverlay::new(self.state.vertex_size, grid_offset);

        // Create base board div with all layers
        let mut board_div = div()
            .id("go-board")
            .relative()
            .child(grid.render())
            .child(div().absolute().inset_0().child(stones.render()))
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .child(paint_overlay.render_paint_overlay(&self.state.paint_map, None)),
            )
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .child(markers.render_markers(&self.state.marker_map)),
            )
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .child(selections.render_selections(&selection_data)),
            );

        // Add interaction layer with comprehensive event handlers
        let interactions =
            VertexInteractions::new(self.state.board_range.clone(), self.state.vertex_size)
                .with_busy(self.state.busy);

        let interaction_layer = interactions.render_with_handlers(handlers);
        board_div = board_div.child(div().absolute().inset_0().child(interaction_layer));

        board_div
    }
}

impl Default for GoBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for GoBoard {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let handlers = VertexEventHandlers::new();
        self.render_with_vertex_handlers(handlers)
    }
}
