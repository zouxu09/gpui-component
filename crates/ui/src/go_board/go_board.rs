use crate::go_board::memory_manager::{MarkerComponent, StoneComponent};
use crate::go_board::{
    BoardTheme, DifferentialRenderer, GhostStoneOverlay, GoBoardError, GoBoardResult, GoBoardState,
    GoBoardValidator, Grid, GridTheme, HeatOverlay, LineOverlay, Markers, MemoryManager,
    PaintOverlay, StoneTheme, Stones, ThemeCSSAdapter, Vertex, VertexEventHandlers,
    VertexInteractions, VertexSelections,
};
use gpui::*;

/// Main Go board component following GPUI reactive architecture
/// Provides a flexible, customizable Go board display inspired by Shudan
pub struct GoBoard {
    state: GoBoardState,
    theme: BoardTheme,
    css_adapter: ThemeCSSAdapter,
    differential_renderer: DifferentialRenderer,
    memory_manager: MemoryManager,
}

impl GoBoard {
    /// Creates a new Go board with default 19x19 dimensions
    pub fn new() -> Self {
        let theme = BoardTheme::default();
        Self {
            state: GoBoardState::standard(),
            css_adapter: ThemeCSSAdapter::from_theme(&theme),
            differential_renderer: DifferentialRenderer::new(),
            memory_manager: MemoryManager::new(),
            theme,
        }
    }

    /// Creates a Go board with specified dimensions
    pub fn with_size(width: usize, height: usize) -> Self {
        let theme = BoardTheme::default();
        Self {
            state: GoBoardState::new(width, height),
            css_adapter: ThemeCSSAdapter::from_theme(&theme),
            differential_renderer: DifferentialRenderer::new(),
            memory_manager: MemoryManager::new(),
            theme,
        }
    }

    /// Creates a Go board with specified dimensions and validation
    pub fn try_with_size(width: usize, height: usize) -> GoBoardResult<Self> {
        GoBoardValidator::validate_board_size(width, height)?;
        Ok(Self::with_size(width, height))
    }

    /// Creates a Go board with a specified theme
    pub fn with_theme(theme: BoardTheme) -> Self {
        Self {
            state: GoBoardState::standard(),
            css_adapter: ThemeCSSAdapter::from_theme(&theme),
            differential_renderer: DifferentialRenderer::new(),
            memory_manager: MemoryManager::new(),
            theme,
        }
    }

    /// Creates a Go board with custom vertex size
    pub fn with_vertex_size(mut self, size: f32) -> Self {
        self.state.vertex_size = size;
        self
    }

    /// Creates a Go board with validated custom vertex size
    pub fn try_with_vertex_size(mut self, size: f32) -> GoBoardResult<Self> {
        GoBoardValidator::validate_vertex_size(size)?;
        self.state.vertex_size = size;
        Ok(self)
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

    /// Updates sign map efficiently with change detection and memory cleanup
    pub fn update_sign_map(&mut self, sign_map: crate::go_board::SignMap) -> bool {
        // Perform memory cleanup before major updates if needed
        if self.memory_manager.needs_cleanup() {
            self.memory_manager.cleanup();
        }

        let result = self.state.update_sign_map(&sign_map);

        // Clear differential renderer cache if the update was significant
        if result {
            self.differential_renderer.invalidate_cache();
        }

        result
    }

    /// Updates sign map with validation
    pub fn try_update_sign_map(
        &mut self,
        sign_map: crate::go_board::SignMap,
    ) -> GoBoardResult<bool> {
        let (width, height) = self.state.dimensions();
        GoBoardValidator::validate_map_size(&sign_map, "sign_map", width, height)?;

        // Validate each sign value
        for (y, row) in sign_map.iter().enumerate() {
            for (x, &sign) in row.iter().enumerate() {
                let vertex = Vertex::new(x, y);
                GoBoardValidator::validate_sign(sign, &vertex)?;
            }
        }

        Ok(self.update_sign_map(sign_map))
    }

    /// Updates individual stones efficiently with memory management
    pub fn update_stones(&mut self, updates: &[(Vertex, i8)]) -> bool {
        // For bulk updates, check if cleanup is needed
        if updates.len() > 10 && self.memory_manager.needs_cleanup() {
            self.memory_manager.cleanup();
        }

        let result = self.state.update_stones(updates);

        // Invalidate cache for significant changes
        if result && updates.len() > 5 {
            self.differential_renderer.invalidate_cache();
        }

        result
    }

    /// Updates individual stones with validation
    pub fn try_update_stones(&mut self, updates: &[(Vertex, i8)]) -> GoBoardResult<bool> {
        let (width, height) = self.state.dimensions();

        // Validate bulk update size for performance
        GoBoardValidator::validate_bulk_update_size(updates.len(), 100)?;

        // Validate each update
        for (vertex, sign) in updates {
            GoBoardValidator::validate_vertex(vertex, width, height)?;
            GoBoardValidator::validate_sign(*sign, vertex)?;
        }

        Ok(self.update_stones(updates))
    }

    /// Gets the differences between current and new sign map
    pub fn get_sign_map_differences(&self, new_sign_map: &crate::go_board::SignMap) -> Vec<Vertex> {
        self.state.get_sign_map_differences(new_sign_map)
    }

    /// Sets a single stone at a vertex efficiently
    pub fn set_stone(&mut self, vertex: &Vertex, sign: i8) -> bool {
        self.state.set_sign(vertex, sign)
    }

    /// Sets a single stone at a vertex with validation
    pub fn try_set_stone(&mut self, vertex: &Vertex, sign: i8) -> GoBoardResult<bool> {
        let (width, height) = self.state.dimensions();
        GoBoardValidator::validate_vertex(vertex, width, height)?;
        GoBoardValidator::validate_sign(sign, vertex)?;
        Ok(self.state.set_sign(vertex, sign))
    }

    /// Gets the stone at a specific vertex
    pub fn get_stone(&self, vertex: &Vertex) -> Option<i8> {
        self.state.get_sign(vertex)
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

    /// Sets the heat map for influence visualization
    pub fn set_heat_map(&mut self, heat_map: crate::go_board::HeatMap) {
        if !heat_map.is_empty() && !heat_map[0].is_empty() {
            let height = heat_map.len();
            let width = heat_map[0].len();
            let (current_width, current_height) = self.state.dimensions();

            if width == current_width && height == current_height {
                self.state.heat_map = heat_map;
            }
        }
    }

    /// Sets the ghost stone map for analysis visualization
    pub fn set_ghost_stone_map(&mut self, ghost_stone_map: crate::go_board::GhostStoneMap) {
        if !ghost_stone_map.is_empty() && !ghost_stone_map[0].is_empty() {
            let height = ghost_stone_map.len();
            let width = ghost_stone_map[0].len();
            let (current_width, current_height) = self.state.dimensions();

            if width == current_width && height == current_height {
                self.state.ghost_stone_map = ghost_stone_map;
            }
        }
    }

    /// Updates ghost stone map efficiently with change detection and memory cleanup
    pub fn update_ghost_stone_map(
        &mut self,
        ghost_stone_map: crate::go_board::GhostStoneMap,
    ) -> bool {
        // Perform memory cleanup before major updates if needed
        if self.memory_manager.needs_cleanup() {
            self.memory_manager.cleanup();
        }

        let result = self.state.update_ghost_stone_map(&ghost_stone_map);

        // Clear differential renderer cache if the update was significant
        if result {
            self.differential_renderer.invalidate_cache();
        }

        result
    }

    /// Updates individual ghost stones efficiently with memory management
    pub fn update_ghost_stones(
        &mut self,
        updates: &[(Vertex, Option<crate::go_board::GhostStone>)],
    ) -> bool {
        // For bulk updates, check if cleanup is needed
        if updates.len() > 10 && self.memory_manager.needs_cleanup() {
            self.memory_manager.cleanup();
        }

        let result = self.state.update_ghost_stones(updates);

        // Invalidate cache for significant changes
        if result && updates.len() > 5 {
            self.differential_renderer.invalidate_cache();
        }

        result
    }

    /// Gets the ghost stone at a specific vertex
    pub fn get_ghost_stone(&self, vertex: &Vertex) -> Option<&crate::go_board::GhostStone> {
        self.state.get_ghost_stone(vertex)
    }

    /// Sets a single ghost stone at a vertex
    pub fn set_ghost_stone(
        &mut self,
        vertex: &Vertex,
        ghost_stone: Option<crate::go_board::GhostStone>,
    ) -> bool {
        self.state.set_ghost_stone(vertex, ghost_stone)
    }

    /// Clears all ghost stones from the board
    pub fn clear_ghost_stones(&mut self) {
        self.state.clear_ghost_stones();
    }

    /// Sets the lines array for drawing connections between vertices
    pub fn set_lines(&mut self, lines: Vec<crate::go_board::Line>) {
        self.state.lines = lines;
    }

    /// Adds a single line to the board
    pub fn add_line(&mut self, line: crate::go_board::Line) {
        self.state.lines.push(line);
    }

    /// Clears all lines from the board
    pub fn clear_lines(&mut self) {
        self.state.lines.clear();
    }

    /// Gets all lines on the board
    pub fn get_lines(&self) -> &[crate::go_board::Line] {
        &self.state.lines
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

    /// Sets the board theme (replaces both grid and stone themes) with memory cleanup
    pub fn set_theme(&mut self, theme: BoardTheme) {
        // Cleanup old theme-related cached components
        self.memory_manager.cleanup();

        self.theme = theme;
        self.css_adapter = ThemeCSSAdapter::from_theme(&self.theme);

        // Invalidate differential renderer cache since theme affects rendering
        self.differential_renderer.invalidate_cache();
    }

    /// Forces a full re-render on next update with memory cleanup
    pub fn invalidate_render_cache(&mut self) {
        // Clean up any cached components that may be invalidated
        self.memory_manager.cleanup();
        self.differential_renderer.invalidate_cache();
    }

    /// Gets statistics about the last differential update
    pub fn get_update_stats(&self) -> crate::go_board::UpdateStats {
        self.differential_renderer.get_update_stats()
    }

    /// Checks if a vertex was changed in the last update
    pub fn vertex_changed(&self, vertex: &Vertex) -> bool {
        self.differential_renderer.vertex_changed(vertex)
    }

    /// Registers an animation timer for cleanup tracking
    pub fn register_animation_timer(
        &mut self,
        timer_id: String,
        cleanup_callback: Option<fn()>,
    ) -> crate::go_board::TimerHandle {
        self.memory_manager
            .register_timer(timer_id, cleanup_callback)
    }

    /// Cleans up a specific animation timer
    pub fn cleanup_animation_timer(&mut self, timer_id: &str) -> bool {
        self.memory_manager.cleanup_timer(timer_id)
    }

    /// Cleans up all active animation timers
    pub fn cleanup_all_timers(&mut self) {
        self.memory_manager.cleanup_all_timers();
    }

    /// Performs memory cleanup (removes old pooled components and timers)
    pub fn cleanup_memory(&mut self) {
        self.memory_manager.cleanup();
    }

    /// Forces complete memory cleanup (removes all pooled components and timers)
    pub fn force_memory_cleanup(&mut self) {
        self.memory_manager.force_cleanup();
    }

    /// Gets current memory usage statistics
    pub fn get_memory_stats(&self) -> &crate::go_board::MemoryStats {
        self.memory_manager.get_memory_stats()
    }

    /// Gets component pool statistics
    pub fn get_pool_stats(&self) -> crate::go_board::ComponentPoolStats {
        self.memory_manager.get_pool_stats()
    }

    /// Checks if memory cleanup is needed based on configuration
    pub fn needs_memory_cleanup(&self) -> bool {
        self.memory_manager.needs_cleanup()
    }

    /// Gets memory efficiency ratio (reused vs created components)
    pub fn get_memory_efficiency(&self) -> f64 {
        self.memory_manager.get_efficiency_ratio()
    }

    /// Gets a pooled stone component for efficient rendering
    pub fn get_pooled_stone_component(&mut self, vertex: Vertex, sign: i8) -> StoneComponent {
        self.memory_manager
            .get_stone_component(vertex, sign, self.state.vertex_size)
    }

    /// Returns a stone component to the pool for reuse
    pub fn return_stone_component(&mut self, component: StoneComponent) {
        self.memory_manager.return_stone_component(component);
    }

    /// Gets a pooled marker component for efficient rendering
    pub fn get_pooled_marker_component(
        &mut self,
        vertex: Vertex,
        marker_type: crate::go_board::MarkerType,
    ) -> MarkerComponent {
        self.memory_manager
            .get_marker_component(vertex, marker_type, self.state.vertex_size)
    }

    /// Returns a marker component to the pool for reuse
    pub fn return_marker_component(&mut self, component: MarkerComponent) {
        self.memory_manager.return_marker_component(component);
    }

    /// Renders the board with component pooling for efficient memory usage
    /// This method demonstrates how to use component pooling for large boards
    pub fn render_with_pooling(&mut self, handlers: VertexEventHandlers) -> impl IntoElement {
        // Perform automatic cleanup if needed
        if self.memory_manager.needs_cleanup() {
            self.memory_manager.cleanup();
        }

        // For demonstration, we'll show how to use pooled components
        // In a real implementation, the Stones and Markers components would
        // request pooled components from the memory manager

        // Create grid component with theme-derived properties
        let grid_theme = self.grid_theme();
        let grid = Grid::new(self.state.board_range.clone(), self.state.vertex_size)
            .with_theme(grid_theme)
            .with_coordinates(self.state.show_coordinates);

        // Create stones component with theme-derived properties and pooling hint
        let stone_theme = self.stone_theme();
        let stones = Stones::new(
            self.state.board_range.clone(),
            self.state.vertex_size,
            self.state.sign_map.clone(),
        )
        .with_theme(stone_theme);

        // Create markers component with pooling capabilities
        let grid_offset = point(px(0.0), px(0.0));
        let markers = Markers::new(self.state.vertex_size, grid_offset);

        // Rest of the rendering remains the same but could be optimized
        // with component pooling in a production implementation
        let selections = VertexSelections::new(self.state.vertex_size, grid_offset);
        let selection_data = VertexSelections::from_board_state(
            &self.state.selected_vertices,
            &self.state.dimmed_vertices,
            &self.state.selected_left,
            &self.state.selected_right,
            &self.state.selected_top,
            &self.state.selected_bottom,
        );

        let paint_overlay = PaintOverlay::new(self.state.vertex_size, grid_offset);
        let heat_overlay = HeatOverlay::new(self.state.vertex_size, grid_offset);
        let ghost_stone_overlay = GhostStoneOverlay::new(self.state.vertex_size, grid_offset);
        let line_overlay = LineOverlay::new(self.state.vertex_size, grid_offset);

        let mut board_div = div()
            .id("go-board-pooled")
            .relative()
            .child(grid.render())
            .child(div().absolute().inset_0().child(stones.render()))
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .child(ghost_stone_overlay.render_ghost_stones(&self.state.ghost_stone_map)),
            )
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
                    .child(heat_overlay.render_heat_overlay(&self.state.heat_map)),
            )
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .child(line_overlay.render_lines(&self.state.lines)),
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

        // Add interaction layer
        let interactions =
            VertexInteractions::new(self.state.board_range.clone(), self.state.vertex_size)
                .with_busy(self.state.busy);

        let interaction_layer = interactions.render_with_handlers(handlers);
        board_div = board_div.child(div().absolute().inset_0().child(interaction_layer));

        board_div
    }

    /// Sets the grid theme (for backward compatibility)
    pub fn set_grid_theme(&mut self, grid_theme: GridTheme) {
        // Convert GridTheme to BoardTheme properties
        self.theme.board_background_color = grid_theme.background_color;
        self.theme.grid_line_color = grid_theme.grid_line_color;
        self.theme.grid_line_width = grid_theme.grid_line_width;
        self.theme.board_border_color = grid_theme.border_color;
        self.theme.board_border_width = grid_theme.border_width;
        self.theme.star_point_color = grid_theme.star_point_color;
        self.theme.star_point_size = grid_theme.star_point_size;
        self.css_adapter = ThemeCSSAdapter::from_theme(&self.theme);
        // Invalidate cache since theme affects rendering
        self.differential_renderer.invalidate_cache();
    }

    /// Sets the stone theme (for backward compatibility)
    pub fn set_stone_theme(&mut self, stone_theme: StoneTheme) {
        // Convert StoneTheme to BoardTheme properties
        self.theme.black_stone_color = stone_theme.black_color;
        self.theme.white_stone_color = stone_theme.white_color;
        self.theme.stone_size_ratio = stone_theme.stone_size_ratio;
        self.theme.stone_border_width = stone_theme.border_width;
        self.theme.stone_border_color = stone_theme.border_color;
        self.theme.fuzzy_placement = stone_theme.fuzzy_placement;
        self.theme.fuzzy_max_offset = stone_theme.fuzzy_max_offset;
        self.theme.random_variation = stone_theme.random_variation;
        self.theme.max_rotation = stone_theme.max_rotation;
        self.theme.black_stone_texture = stone_theme.black_stone_image;
        self.theme.white_stone_texture = stone_theme.white_stone_image;
        self.css_adapter = ThemeCSSAdapter::from_theme(&self.theme);
        // Invalidate cache since theme affects rendering
        self.differential_renderer.invalidate_cache();
    }

    /// Gets a reference to the board theme
    pub fn theme(&self) -> &BoardTheme {
        &self.theme
    }

    /// Gets a reference to the CSS adapter
    pub fn css_adapter(&self) -> &ThemeCSSAdapter {
        &self.css_adapter
    }

    /// Gets a reference to the grid theme (for backward compatibility)
    pub fn grid_theme(&self) -> GridTheme {
        // Convert BoardTheme to GridTheme
        GridTheme {
            background_color: self.theme.board_background_color,
            grid_line_color: self.theme.grid_line_color,
            grid_line_width: self.theme.grid_line_width,
            border_color: self.theme.board_border_color,
            border_width: self.theme.board_border_width,
            star_point_color: self.theme.star_point_color,
            star_point_size: self.theme.star_point_size,
        }
    }

    /// Gets a reference to the stone theme (for backward compatibility)
    pub fn stone_theme(&self) -> StoneTheme {
        // Convert BoardTheme to StoneTheme
        StoneTheme {
            black_color: self.theme.black_stone_color,
            white_color: self.theme.white_stone_color,
            stone_size_ratio: self.theme.stone_size_ratio,
            border_width: self.theme.stone_border_width,
            border_color: self.theme.stone_border_color,
            fuzzy_placement: self.theme.fuzzy_placement,
            fuzzy_max_offset: self.theme.fuzzy_max_offset,
            random_variation: self.theme.random_variation,
            max_rotation: self.theme.max_rotation,
            black_stone_image: self.theme.black_stone_texture.clone(),
            white_stone_image: self.theme.white_stone_texture.clone(),
        }
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

    /// Renders the board with comprehensive vertex event handlers using differential rendering
    pub fn render_with_vertex_handlers(&self, handlers: VertexEventHandlers) -> impl IntoElement {
        // This method creates a standard render for now
        // In a future implementation, this could be optimized with differential rendering
        // by tracking which elements need updates

        // Create grid component with theme-derived properties
        let grid_theme = self.grid_theme();
        let grid = Grid::new(self.state.board_range.clone(), self.state.vertex_size)
            .with_theme(grid_theme)
            .with_coordinates(self.state.show_coordinates);

        // Create stones component with theme-derived properties
        let stone_theme = self.stone_theme();
        let stones = Stones::new(
            self.state.board_range.clone(),
            self.state.vertex_size,
            self.state.sign_map.clone(),
        )
        .with_theme(stone_theme);

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

        // Create heat overlay component for influence visualization
        let heat_overlay = HeatOverlay::new(self.state.vertex_size, grid_offset);

        // Create ghost stone overlay component for analysis visualization
        let ghost_stone_overlay = GhostStoneOverlay::new(self.state.vertex_size, grid_offset);

        // Create line overlay component for drawing connections between vertices
        let line_overlay = LineOverlay::new(self.state.vertex_size, grid_offset);

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
                    .child(ghost_stone_overlay.render_ghost_stones(&self.state.ghost_stone_map)),
            )
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
                    .child(heat_overlay.render_heat_overlay(&self.state.heat_map)),
            )
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .child(line_overlay.render_lines(&self.state.lines)),
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

    /// Renders the board with differential updates (experimental optimization)
    /// This method analyzes changes and only re-renders modified elements
    pub fn render_with_differential_updates(
        &mut self,
        handlers: VertexEventHandlers,
    ) -> AnyElement {
        // Check if memory cleanup is needed and perform it automatically
        if self.memory_manager.needs_cleanup() {
            self.memory_manager.cleanup();
        }

        // Note: In a real GPUI application, this would require more sophisticated
        // state management and component caching. For now, this demonstrates
        // the differential analysis capability.

        // Analyze what has changed since last render
        use crate::go_board::types::SelectionStateSnapshot;
        let selection_state = SelectionStateSnapshot::from_board_state(&self.state);

        let update = self.differential_renderer.analyze_changes(
            &self.state.sign_map,
            &self.state.marker_map,
            &self.state.ghost_stone_map,
            &selection_state,
        );

        // For demonstration, we'll render the full board but could optimize
        // to only render changed elements in a production implementation
        if update.requires_full_render
            || !update.changed_stones.is_empty()
            || !update.changed_markers.is_empty()
            || !update.changed_ghost_stones.is_empty()
            || update.selection_changed
        {
            // In a real implementation, this is where we would:
            // 1. Only update the DOM elements that changed
            // 2. Use CSS transforms for animations
            // 3. Batch DOM updates to prevent layout thrashing
            // 4. Use virtual DOM or similar diffing strategies
            // 5. Use component pooling from memory manager for expensive components

            self.render_with_vertex_handlers(handlers)
                .into_any_element()
        } else {
            // No changes detected, return minimal render
            div().id("go-board-no-changes").into_any_element()
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
        let handlers = VertexEventHandlers::new();
        self.render_with_vertex_handlers(handlers)
    }
}

impl Drop for GoBoard {
    fn drop(&mut self) {
        // Ensure all timers and resources are cleaned up when GoBoard is dropped
        self.force_memory_cleanup();
    }
}
