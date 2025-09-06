use crate::go_board::{board::Board, core::*, render::Renderer};
use gpui::{
    div, hsla, point, px, rgb, Bounds, Context, InteractiveElement, IntoElement, KeyDownEvent,
    Modifiers, MouseButton, ParentElement, Pixels, Point, Render, Styled, Window,
};
use std::rc::Rc;

/// View component that combines Board with Renderer and handles interactions
pub struct BoardView {
    board: Board,
    renderer: Renderer,
    show_coordinates: bool,
    focus: Option<Pos>,
    on_click: Option<Rc<dyn Fn(PosEvent)>>,
    on_hover: Option<Rc<dyn Fn(Option<Pos>)>>,
    on_key: Option<Rc<dyn Fn(KeyDownEvent) -> Option<NavEvent>>>,
}

impl BoardView {
    /// Create a new BoardView with automatic sizing
    pub fn new(board: Board) -> Self {
        let theme = board.theme.clone();
        let vertex_size = Self::calculate_default_vertex_size(board.dimensions());

        Self {
            board,
            renderer: Renderer::new(vertex_size, theme),
            show_coordinates: false,
            focus: None,
            on_click: None,
            on_hover: None,
            on_key: None,
        }
    }

    /// Create a BoardView with custom sizing constraints
    pub fn with_size(board: Board, max_width: f32, max_height: f32) -> Self {
        let theme = board.theme.clone();
        let vertex_size =
            Self::calculate_vertex_size_for_container(board.dimensions(), max_width, max_height);

        Self {
            board,
            renderer: Renderer::new(vertex_size, theme),
            show_coordinates: false,
            focus: None,
            on_click: None,
            on_hover: None,
            on_key: None,
        }
    }

    pub fn coordinates(mut self, show: bool) -> Self {
        self.show_coordinates = show;
        self
    }

    /// Calculate a default vertex size based on board dimensions
    fn calculate_default_vertex_size((width, height): (usize, usize)) -> f32 {
        // Use a reasonable default size that scales with board size
        // For 19x19 boards, use 20px, for smaller boards use proportionally larger sizes
        let base_size = 20.0; // Reduced from 24px to be more conservative
        let scale_factor = 19.0 / (width.max(height) as f32);
        base_size * scale_factor.max(0.5).min(1.5) // Reduced max from 2.0 to 1.5
    }

    /// Calculate vertex size to fit within container dimensions
    fn calculate_vertex_size_for_container(
        (width, height): (usize, usize),
        max_width: f32,
        max_height: f32,
    ) -> f32 {
        let available_width = max_width - 40.0; // Account for padding
        let available_height = max_height - 40.0; // Account for padding
        let board_width = width as f32;
        let board_height = height as f32;

        // Calculate vertex size to fit within container
        let vertex_size_x = available_width / (board_width + 1.0); // +1 for edge spacing
        let vertex_size_y = available_height / (board_height + 1.0); // +1 for edge spacing
        vertex_size_x.min(vertex_size_y).max(8.0).min(50.0) // Clamp between 8-50px
    }

    pub fn focus(mut self, pos: Option<Pos>) -> Self {
        self.focus = pos;
        self
    }

    pub fn on_click<F>(mut self, handler: F) -> Self
    where
        F: Fn(PosEvent) + 'static,
    {
        self.on_click = Some(Rc::new(handler));
        self
    }

    pub fn on_hover<F>(mut self, handler: F) -> Self
    where
        F: Fn(Option<Pos>) + 'static,
    {
        self.on_hover = Some(Rc::new(handler));
        self
    }

    pub fn on_key<F>(mut self, handler: F) -> Self
    where
        F: Fn(KeyDownEvent) -> Option<NavEvent> + 'static,
    {
        self.on_key = Some(Rc::new(handler));
        self
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    /// Update the board data and sync the renderer
    pub fn update_board<F>(mut self, f: F) -> Self
    where
        F: FnOnce(Board) -> Board,
    {
        self.board = f(self.board);
        self.sync_renderer();
        self
    }

    fn sync_renderer(&mut self) {
        let vertex_size = Self::calculate_default_vertex_size(self.board.dimensions());
        self.renderer = Renderer::new(vertex_size, self.board.theme.clone());
    }

    /// Add a stone to the board
    pub fn stone(mut self, pos: Pos, stone: Stone) -> Self {
        self.board.data_mut().set_stone(pos, stone);
        self
    }

    /// Add a marker to the board
    pub fn marker(mut self, pos: Pos, marker: Marker) -> Self {
        self.board.data_mut().set_marker(pos, Some(marker));
        self
    }

    /// Add a ghost stone to the board
    pub fn ghost(mut self, pos: Pos, ghost: Ghost) -> Self {
        self.board.data_mut().set_ghost(pos, Some(ghost));
        self
    }

    /// Select a position on the board
    pub fn select(mut self, pos: Pos) -> Self {
        self.board
            .data_mut()
            .set_selection(pos, Some(Selection::selected(pos)));
        self
    }

    /// Mark the last move
    pub fn last_move(mut self, pos: Pos) -> Self {
        self.board
            .data_mut()
            .set_selection(pos, Some(Selection::last_move(pos)));
        self
    }

    /// Clear all selections
    pub fn clear_selections(mut self) -> Self {
        self.board.data_mut().clear_selections();
        self
    }

    /// Add a line to the board
    pub fn line(mut self, line: Line) -> Self {
        self.board.data_mut().add_line(line);
        self
    }

    pub fn set_focus(&mut self, pos: Option<Pos>) {
        self.focus = pos;
    }

    pub fn move_focus(&mut self, dx: i32, dy: i32) -> Option<Pos> {
        if let Some(current) = self.focus {
            let new_x = current.x as i32 + dx;
            let new_y = current.y as i32 + dy;

            // Check if new position would be out of bounds
            if new_x < 0 || new_y < 0 {
                return None;
            }

            let new_pos = Pos::new(new_x as usize, new_y as usize);

            if self.board.data().is_valid_pos(new_pos) {
                self.focus = Some(new_pos);
                Some(new_pos)
            } else {
                None
            }
        } else {
            let (width, height) = self.board.dimensions();
            let start_pos = Pos::new(width / 2, height / 2);
            self.focus = Some(start_pos);
            Some(start_pos)
        }
    }

    pub fn handle_key_input(&mut self, event: &KeyDownEvent) -> Option<NavEvent> {
        let nav_event = match event.keystroke.key.as_str() {
            "ArrowLeft" => self.move_focus(-1, 0).map(NavEvent::MoveFocus),
            "ArrowRight" => self.move_focus(1, 0).map(NavEvent::MoveFocus),
            "ArrowUp" => self.move_focus(0, -1).map(NavEvent::MoveFocus),
            "ArrowDown" => self.move_focus(0, 1).map(NavEvent::MoveFocus),
            "Enter" | "Space" => self.focus.map(NavEvent::Select),
            "Escape" => Some(NavEvent::ClearSelection),
            _ => None,
        };

        if let Some(ref handler) = self.on_key {
            handler(event.clone()).or(nav_event)
        } else {
            nav_event
        }
    }

    pub fn pos_from_mouse(
        &self,
        mouse_pos: Point<Pixels>,
        container_bounds: Bounds<Pixels>,
    ) -> Option<Pos> {
        let vertex_size = Self::calculate_default_vertex_size(self.board.dimensions());
        let offset = if self.show_coordinates {
            let spacing = crate::go_board::render::ResponsiveSpacing::for_vertex_size(vertex_size);
            let effective_coord_size = self.board.theme.coord_size.max(spacing.min_coord_size);
            let margin = effective_coord_size + spacing.coord_margin_padding;
            point(px(margin), px(margin))
        } else {
            point(px(0.0), px(0.0))
        };

        let relative_mouse = Point::new(
            mouse_pos.x - container_bounds.origin.x,
            mouse_pos.y - container_bounds.origin.y,
        );

        self.board
            .pos_from_pixel(relative_mouse, offset, vertex_size)
    }
}

impl Render for BoardView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(focus_pos) = self.focus {
            self.board.data_mut().set_selection(
                focus_pos,
                Some(Selection::selected(focus_pos).with_color(rgb(0x80ff80))),
            );
        }

        // Use the vertex size from the renderer (calculated in constructor)
        let vertex_size = self.renderer.vertex_size();

        // Calculate board size
        let base_size = self.board.pixel_size(vertex_size);
        let (total_width, total_height) = if self.show_coordinates {
            let spacing = crate::go_board::render::ResponsiveSpacing::for_vertex_size(vertex_size);
            let effective_coord_size = self.board.theme.coord_size.max(spacing.min_coord_size);
            let margin = effective_coord_size + spacing.coord_margin_padding;
            (
                base_size.width.0 + 2.0 * margin,
                base_size.height.0 + 2.0 * margin,
            )
        } else {
            (base_size.width.0, base_size.height.0)
        };

        // Create container with calculated size
        let container = div()
            .id("go-board-view")
            .relative()
            .bg(self.board.theme.background)
            .w(px(total_width))
            .h(px(total_height));

        // Render the board content first
        let container = container.child({
            let renderer = Renderer::new(vertex_size, self.board.theme.clone())
                .with_coordinates(self.show_coordinates);
            renderer.render(self.board.data(), self.show_coordinates)
        });

        // Render interactions layer last to ensure it's on top
        let container = container.child(self.render_interactions(cx, vertex_size));

        container
            .key_context("go-board")
            .on_key_down(cx.listener(|view, event, _cx, _phase| {
                if let Some(nav_event) = view.handle_key_input(event) {
                    match nav_event {
                        NavEvent::MoveFocus(pos) => {
                            println!("Focus moved to {:?}", pos);
                        }
                        NavEvent::Select(pos) => {
                            // Call the on_click callback for keyboard selection
                            if let Some(ref handler) = view.on_click {
                                let event = PosEvent::with_mouse_button(
                                    pos,
                                    Modifiers::default(),
                                    MouseButton::Left,
                                );
                                handler(event);
                            }
                        }
                        NavEvent::ClearSelection => {
                            view.board.data_mut().clear_selections();
                        }
                    }
                }
            }))
    }
}

impl BoardView {
    fn render_interactions(&self, _cx: &mut Context<Self>, vertex_size: f32) -> impl IntoElement {
        let range = self.board.visible_range();

        let offset = if self.show_coordinates {
            let spacing = crate::go_board::render::ResponsiveSpacing::for_vertex_size(vertex_size);
            let effective_coord_size = self.board.theme.coord_size.max(spacing.min_coord_size);
            let margin = effective_coord_size + spacing.coord_margin_padding;
            point(px(margin), px(margin))
        } else {
            point(px(0.0), px(0.0))
        };

        let mut interactions = div().absolute().inset_0();

        for y in range.y.0..=range.y.1 {
            for x in range.x.0..=range.x.1 {
                let pos = Pos::new(x, y);
                let pixel_pos = self.board.pixel_from_pos(pos, offset, vertex_size);
                let button_size = vertex_size * 1.0; // Normal hit area size

                let mut button = div()
                    .absolute()
                    .left(pixel_pos.x - px(button_size / 2.0))
                    .top(pixel_pos.y - px(button_size / 2.0))
                    .w(px(button_size))
                    .h(px(button_size))
                    // Ensure the hit area participates in hit-testing
                    .bg(hsla(0.0, 0.0, 0.0, 0.001))
                    .id(("board_pos", x * 1000 + y))
                    .cursor_pointer();

                // Add click handlers if on_click is set
                if let Some(ref on_click) = self.on_click {
                    let on_click_left = on_click.clone();
                    let on_click_right = on_click.clone();
                    button = button
                        .on_mouse_down(MouseButton::Left, move |_, _window, _cx| {
                            let event = PosEvent::with_mouse_button(
                                pos,
                                Modifiers::default(),
                                MouseButton::Left,
                            );
                            (on_click_left)(event);
                        })
                        .on_mouse_down(MouseButton::Right, move |_, _window, _cx| {
                            let event = PosEvent::with_mouse_button(
                                pos,
                                Modifiers::default(),
                                MouseButton::Right,
                            );
                            (on_click_right)(event);
                        });
                }

                // Add hover handlers if on_hover is set
                if let Some(ref on_hover) = self.on_hover {
                    let on_hover = on_hover.clone();
                    button = button.on_mouse_move(move |_event, _window, _cx| {
                        (on_hover)(Some(pos));
                    });
                }

                interactions = interactions.child(button);
            }
        }

        interactions
    }
}

/// Create a simple interactive board
pub fn interactive_board<F>(click_handler: F) -> BoardView
where
    F: Fn(PosEvent) + 'static,
{
    BoardView::new(Board::new()).on_click(click_handler)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_view_creation() {
        let board = Board::new();
        let view = BoardView::new(board);
        assert_eq!(view.board().dimensions(), (19, 19));
        assert!(!view.show_coordinates); // Default is false
    }

    #[test]
    fn test_focus_movement() {
        let mut view = BoardView::new(Board::new());

        assert_eq!(view.focus, None);

        let pos = view.move_focus(0, 0);
        assert!(pos.is_some());
        assert_eq!(view.focus, pos);

        view.set_focus(Some(Pos::new(5, 5)));
        let new_pos = view.move_focus(1, 0).unwrap();
        assert_eq!(new_pos, Pos::new(6, 5));
    }

    #[test]
    fn test_keyboard_handling() {
        let mut view = BoardView::new(Board::new());
        view.set_focus(Some(Pos::new(5, 5)));

        // Test focus movement directly instead of constructing KeyDownEvent
        let new_pos = view.move_focus(1, 0);
        assert!(matches!(new_pos, Some(_)));
        assert_eq!(view.focus, Some(Pos::new(6, 5)));
    }

    #[test]
    fn test_builder_pattern() {
        let view = BoardView::new(Board::new())
            .coordinates(false)
            .stone(Pos::new(4, 4), BLACK)
            .marker(Pos::new(2, 2), Marker::circle())
            .on_click(|_| {});

        assert!(!view.show_coordinates);
        assert_eq!(view.board().stone_at(Pos::new(4, 4)), BLACK);
        assert!(view.board().marker_at(Pos::new(2, 2)).is_some());
    }

    #[test]
    fn test_factory_functions() {
        let _interactive = interactive_board(|_| {});
    }

    // =============================================================================
    // ENHANCED EVENT HANDLING TESTS - TDD Approach
    // =============================================================================

    #[test]
    fn test_focus_boundary_handling() {
        let mut view = BoardView::new(Board::new());

        // Test focus at board edges
        view.set_focus(Some(Pos::new(0, 0)));

        // Try to move focus beyond left edge
        let left_result = view.move_focus(-1, 0);
        assert!(left_result.is_none());
        assert_eq!(view.focus, Some(Pos::new(0, 0)));

        // Try to move focus beyond top edge
        let top_result = view.move_focus(0, -1);
        assert!(top_result.is_none());
        assert_eq!(view.focus, Some(Pos::new(0, 0)));

        // Test focus at right edge
        view.set_focus(Some(Pos::new(18, 18)));
        let right_result = view.move_focus(1, 0);
        assert!(right_result.is_none());
        assert_eq!(view.focus, Some(Pos::new(18, 18)));
    }

    #[test]
    fn test_coordinate_toggle_behavior() {
        let mut view = BoardView::new(Board::new());

        // Initially coordinates are off
        assert!(!view.show_coordinates);

        // Enable coordinates
        view = view.coordinates(true);
        assert!(view.show_coordinates);

        // Disable coordinates
        view = view.coordinates(false);
        assert!(!view.show_coordinates);
    }

    #[test]
    fn test_board_update_chain() {
        let view = BoardView::new(Board::new())
            .stone(Pos::new(3, 3), BLACK)
            .stone(Pos::new(4, 4), WHITE)
            .marker(Pos::new(5, 5), Marker::circle())
            .ghost(Pos::new(6, 6), Ghost::good(BLACK))
            .select(Pos::new(3, 3))
            .last_move(Pos::new(4, 4));

        let board = view.board();

        // Verify all updates were applied
        assert_eq!(board.stone_at(Pos::new(3, 3)), BLACK);
        assert_eq!(board.stone_at(Pos::new(4, 4)), WHITE);
        assert!(board.marker_at(Pos::new(5, 5)).is_some());
        assert!(board.data().ghosts.contains_key(&Pos::new(6, 6)));
        assert!(board.data().selections.contains_key(&Pos::new(3, 3)));
        assert!(board.data().selections.contains_key(&Pos::new(4, 4)));
    }

    #[test]
    fn test_empty_board_rendering() {
        let view = BoardView::new(Board::new());
        let board = view.board();

        // Empty board should have no stones, markers, etc.
        assert_eq!(board.data().stones.len(), 0);
        assert_eq!(board.data().markers.len(), 0);
        assert_eq!(board.data().ghosts.len(), 0);
        assert_eq!(board.data().selections.len(), 0);
        assert_eq!(board.data().lines.len(), 0);

        // But should have valid dimensions
        assert_eq!(board.dimensions(), (19, 19));
        assert_eq!(board.visible_range().width(), 19);
        assert_eq!(board.visible_range().height(), 19);
    }
}
