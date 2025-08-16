use crate::go_board::{GoBoardState, Vertex};
use gpui::*;

/// Main Go board component following GPUI reactive architecture
/// Provides a flexible, customizable Go board display inspired by Shudan
pub struct GoBoard {
    state: GoBoardState,
}

impl GoBoard {
    /// Creates a new Go board with default 19x19 dimensions
    pub fn new() -> Self {
        Self {
            state: GoBoardState::standard(),
        }
    }

    /// Creates a Go board with specified dimensions
    pub fn with_size(width: usize, height: usize) -> Self {
        Self {
            state: GoBoardState::new(width, height),
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

    /// Calculates the total board size in pixels
    pub fn board_pixel_size(&self) -> Size<Pixels> {
        let range_width = self.state.board_range.width() as f32;
        let range_height = self.state.board_range.height() as f32;

        Size {
            width: px(range_width * self.state.vertex_size),
            height: px(range_height * self.state.vertex_size),
        }
    }
}

impl Default for GoBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for GoBoard {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let board_size = self.board_pixel_size();

        // Basic board container for now - will be expanded in future tasks
        div()
            .id("go-board")
            .w(board_size.width)
            .h(board_size.height)
            .bg(rgb(0xebb55b)) // Default wood color from Shudan theme
            .border_1()
            .border_color(rgb(0xca933a))
            .child(
                div()
                    .size_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(rgb(0x5e2e0c))
                    .child(format!(
                        "Go Board {}x{}",
                        self.state.width(),
                        self.state.height()
                    )),
            )
    }
}
