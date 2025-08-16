use crate::go_board::types::{BoardRange, Vertex};
use gpui::*;

/// Event emitted when a vertex is clicked
#[derive(Clone, Debug)]
pub struct VertexClickEvent {
    pub vertex: Vertex,
    pub coordinates: [usize; 2], // [x, y] coordinates as required by spec
}

impl VertexClickEvent {
    pub fn new(vertex: Vertex) -> Self {
        Self {
            coordinates: [vertex.x, vertex.y],
            vertex,
        }
    }
}

/// Event emitted when mouse enters a vertex area
#[derive(Clone, Debug)]
pub struct VertexMouseEnterEvent {
    pub vertex: Vertex,
    pub coordinates: [usize; 2],
}

impl VertexMouseEnterEvent {
    pub fn new(vertex: Vertex) -> Self {
        Self {
            coordinates: [vertex.x, vertex.y],
            vertex,
        }
    }
}

/// Event emitted when mouse leaves a vertex area
#[derive(Clone, Debug)]
pub struct VertexMouseLeaveEvent {
    pub vertex: Vertex,
    pub coordinates: [usize; 2],
}

impl VertexMouseLeaveEvent {
    pub fn new(vertex: Vertex) -> Self {
        Self {
            coordinates: [vertex.x, vertex.y],
            vertex,
        }
    }
}

/// Event emitted when mouse is pressed down on a vertex
#[derive(Clone, Debug)]
pub struct VertexMouseDownEvent {
    pub vertex: Vertex,
    pub coordinates: [usize; 2],
    pub button: MouseButton,
}

impl VertexMouseDownEvent {
    pub fn new(vertex: Vertex, button: MouseButton) -> Self {
        Self {
            coordinates: [vertex.x, vertex.y],
            vertex,
            button,
        }
    }
}

/// Event emitted when mouse is released on a vertex
#[derive(Clone, Debug)]
pub struct VertexMouseUpEvent {
    pub vertex: Vertex,
    pub coordinates: [usize; 2],
    pub button: MouseButton,
}

impl VertexMouseUpEvent {
    pub fn new(vertex: Vertex, button: MouseButton) -> Self {
        Self {
            coordinates: [vertex.x, vertex.y],
            vertex,
            button,
        }
    }
}

/// Event emitted when mouse moves over a vertex
#[derive(Clone, Debug)]
pub struct VertexMouseMoveEvent {
    pub vertex: Vertex,
    pub coordinates: [usize; 2],
}

impl VertexMouseMoveEvent {
    pub fn new(vertex: Vertex) -> Self {
        Self {
            coordinates: [vertex.x, vertex.y],
            vertex,
        }
    }
}

/// Comprehensive event handlers for vertex interactions
pub struct VertexEventHandlers {
    pub on_click: Option<std::rc::Rc<dyn Fn(VertexClickEvent) + 'static>>,
    pub on_mouse_down: Option<std::rc::Rc<dyn Fn(VertexMouseDownEvent) + 'static>>,
    pub on_mouse_up: Option<std::rc::Rc<dyn Fn(VertexMouseUpEvent) + 'static>>,
    pub on_mouse_move: Option<std::rc::Rc<dyn Fn(VertexMouseMoveEvent) + 'static>>,
}

impl Default for VertexEventHandlers {
    fn default() -> Self {
        Self {
            on_click: None,
            on_mouse_down: None,
            on_mouse_up: None,
            on_mouse_move: None,
        }
    }
}

impl VertexEventHandlers {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_click<F>(mut self, handler: F) -> Self
    where
        F: Fn(VertexClickEvent) + 'static,
    {
        self.on_click = Some(std::rc::Rc::new(handler));
        self
    }

    pub fn with_mouse_down<F>(mut self, handler: F) -> Self
    where
        F: Fn(VertexMouseDownEvent) + 'static,
    {
        self.on_mouse_down = Some(std::rc::Rc::new(handler));
        self
    }

    pub fn with_mouse_up<F>(mut self, handler: F) -> Self
    where
        F: Fn(VertexMouseUpEvent) + 'static,
    {
        self.on_mouse_up = Some(std::rc::Rc::new(handler));
        self
    }

    pub fn with_mouse_move<F>(mut self, handler: F) -> Self
    where
        F: Fn(VertexMouseMoveEvent) + 'static,
    {
        self.on_mouse_move = Some(std::rc::Rc::new(handler));
        self
    }
}

/// Single vertex button for handling interactions at board intersections
#[derive(Clone)]
pub struct VertexButton {
    vertex: Vertex,
    vertex_size: f32,
    board_range: BoardRange,
    busy: bool,
}

impl VertexButton {
    /// Creates a new vertex button
    pub fn new(vertex: Vertex, vertex_size: f32, board_range: BoardRange) -> Self {
        Self {
            vertex,
            vertex_size,
            board_range,
            busy: false,
        }
    }

    /// Sets the busy state (disables interactions)
    pub fn with_busy(mut self, busy: bool) -> Self {
        self.busy = busy;
        self
    }

    /// Calculates the pixel position of this vertex
    fn pixel_position(&self) -> (f32, f32) {
        let relative_x = (self.vertex.x - self.board_range.x.0) as f32;
        let relative_y = (self.vertex.y - self.board_range.y.0) as f32;
        (relative_x * self.vertex_size, relative_y * self.vertex_size)
    }

    /// Calculates the clickable area size (larger than vertex for easier clicking)
    fn click_area_size(&self) -> f32 {
        (self.vertex_size * 0.8).max(20.0) // At least 20px for touch devices
    }

    /// Renders the vertex button with comprehensive event handling
    pub fn render_with_handlers(self, handlers: VertexEventHandlers) -> impl IntoElement {
        let (pixel_x, pixel_y) = self.pixel_position();
        let click_size = self.click_area_size();
        let offset = click_size / 2.0;

        let mut button = div()
            .absolute()
            .left(px(pixel_x - offset))
            .top(px(pixel_y - offset))
            .w(px(click_size))
            .h(px(click_size))
            .cursor_pointer()
            // Make button invisible but still interactive
            .bg(rgba(0x00000000)); // Transparent background

        // Add event handlers if not busy
        if !self.busy {
            let vertex = self.vertex.clone();

            // Mouse down events
            if let Some(handler) = handlers.on_mouse_down {
                let vertex_clone = vertex.clone();
                button = button.on_mouse_down(MouseButton::Left, move |_event, _view, cx| {
                    let event = VertexMouseDownEvent::new(vertex_clone.clone(), MouseButton::Left);
                    handler(event);
                    cx.stop_propagation();
                });
            }

            // Mouse up events
            if let Some(handler) = handlers.on_mouse_up {
                let vertex_clone = vertex.clone();
                button = button.on_mouse_up(MouseButton::Left, move |_event, _view, cx| {
                    let event = VertexMouseUpEvent::new(vertex_clone.clone(), MouseButton::Left);
                    handler(event);
                    cx.stop_propagation();
                });
            }

            // Click events (for backward compatibility)
            if let Some(handler) = handlers.on_click {
                let vertex_clone = vertex.clone();
                button = button.on_mouse_down(MouseButton::Left, move |_event, _view, cx| {
                    let event = VertexClickEvent::new(vertex_clone.clone());
                    handler(event);
                    cx.stop_propagation();
                });
            }

            // Mouse move events
            if let Some(handler) = handlers.on_mouse_move {
                let vertex_clone = vertex.clone();
                button = button.on_mouse_move(move |_event, _view, _cx| {
                    let event = VertexMouseMoveEvent::new(vertex_clone.clone());
                    handler(event);
                });
            }
        } else {
            // When busy, disable pointer events
            button = button.cursor_default();
        }

        button
    }

    /// Renders the vertex button as an invisible clickable area (legacy method)
    pub fn render<F>(&self, on_click: F) -> impl IntoElement
    where
        F: Fn(VertexClickEvent) + 'static,
    {
        let handlers = VertexEventHandlers::new().with_click(on_click);
        self.clone().render_with_handlers(handlers)
    }
}

/// Component for managing all vertex interactions on the board
#[derive(Clone)]
pub struct VertexInteractions {
    board_range: BoardRange,
    vertex_size: f32,
    busy: bool,
}

impl VertexInteractions {
    /// Creates a new vertex interactions manager
    pub fn new(board_range: BoardRange, vertex_size: f32) -> Self {
        Self {
            board_range,
            vertex_size,
            busy: false,
        }
    }

    /// Sets the busy state for all vertex interactions
    pub fn with_busy(mut self, busy: bool) -> Self {
        self.busy = busy;
        self
    }

    /// Updates the board range
    pub fn set_board_range(&mut self, range: BoardRange) {
        self.board_range = range;
    }

    /// Updates the vertex size
    pub fn set_vertex_size(&mut self, size: f32) {
        self.vertex_size = size;
    }

    /// Updates the busy state
    pub fn set_busy(&mut self, busy: bool) {
        self.busy = busy;
    }

    /// Renders all vertex buttons with comprehensive event handling
    pub fn render_with_handlers(self, handlers: VertexEventHandlers) -> impl IntoElement {
        let (width, height) = self.visible_dimensions();
        let mut container = div().relative().w(px(width)).h(px(height));

        // Use Rc to share handlers across vertices
        use std::rc::Rc;
        let shared_handlers = Rc::new(handlers);

        for y in self.board_range.y.0..=self.board_range.y.1 {
            for x in self.board_range.x.0..=self.board_range.x.1 {
                let vertex = Vertex::new(x, y);
                let vertex_button =
                    VertexButton::new(vertex, self.vertex_size, self.board_range.clone())
                        .with_busy(self.busy);

                // Create handlers for this specific vertex
                let mut vertex_handlers = VertexEventHandlers::new();

                if let Some(ref handler) = shared_handlers.on_click {
                    let handler_clone = handler.clone();
                    vertex_handlers = vertex_handlers.with_click(move |event| handler_clone(event));
                }

                if let Some(ref handler) = shared_handlers.on_mouse_down {
                    let handler_clone = handler.clone();
                    vertex_handlers =
                        vertex_handlers.with_mouse_down(move |event| handler_clone(event));
                }

                if let Some(ref handler) = shared_handlers.on_mouse_up {
                    let handler_clone = handler.clone();
                    vertex_handlers =
                        vertex_handlers.with_mouse_up(move |event| handler_clone(event));
                }

                if let Some(ref handler) = shared_handlers.on_mouse_move {
                    let handler_clone = handler.clone();
                    vertex_handlers =
                        vertex_handlers.with_mouse_move(move |event| handler_clone(event));
                }

                container = container.child(vertex_button.render_with_handlers(vertex_handlers));
            }
        }

        container
    }

    /// Renders all vertex buttons as an overlay (legacy method)
    pub fn render<F>(&self, on_vertex_click: F) -> impl IntoElement
    where
        F: Fn(VertexClickEvent) + 'static + Clone,
    {
        let handlers = VertexEventHandlers::new().with_click(on_vertex_click);
        self.clone().render_with_handlers(handlers)
    }

    /// Calculates the visible dimensions
    fn visible_dimensions(&self) -> (f32, f32) {
        let width = (self.board_range.x.1 - self.board_range.x.0) as f32 * self.vertex_size;
        let height = (self.board_range.y.1 - self.board_range.y.0) as f32 * self.vertex_size;
        (width, height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_click_event_creation() {
        let vertex = Vertex::new(3, 5);
        let event = VertexClickEvent::new(vertex.clone());

        assert_eq!(event.vertex, vertex);
        assert_eq!(event.coordinates, [3, 5]);
    }

    #[test]
    fn test_vertex_button_creation() {
        let vertex = Vertex::new(2, 4);
        let board_range = BoardRange::new((0, 8), (0, 8));
        let button = VertexButton::new(vertex.clone(), 25.0, board_range.clone());

        assert_eq!(button.vertex, vertex);
        assert_eq!(button.vertex_size, 25.0);
        assert_eq!(button.board_range, board_range);
        assert!(!button.busy);
    }

    #[test]
    fn test_vertex_button_pixel_position() {
        let vertex = Vertex::new(2, 3);
        let board_range = BoardRange::new((0, 8), (0, 8));
        let button = VertexButton::new(vertex, 20.0, board_range);

        let (pixel_x, pixel_y) = button.pixel_position();
        assert_eq!(pixel_x, 40.0); // 2 * 20
        assert_eq!(pixel_y, 60.0); // 3 * 20
    }

    #[test]
    fn test_vertex_button_click_area_size() {
        let vertex = Vertex::new(0, 0);
        let board_range = BoardRange::new((0, 8), (0, 8));

        // Test with small vertex size
        let button_small = VertexButton::new(vertex.clone(), 15.0, board_range.clone());
        assert_eq!(button_small.click_area_size(), 20.0); // Minimum 20px

        // Test with large vertex size
        let button_large = VertexButton::new(vertex, 40.0, board_range);
        assert_eq!(button_large.click_area_size(), 32.0); // 40 * 0.8
    }

    #[test]
    fn test_vertex_button_busy_state() {
        let vertex = Vertex::new(1, 1);
        let board_range = BoardRange::new((0, 8), (0, 8));
        let button = VertexButton::new(vertex, 25.0, board_range).with_busy(true);

        assert!(button.busy);
    }

    #[test]
    fn test_vertex_interactions_creation() {
        let board_range = BoardRange::new((0, 8), (0, 8));
        let interactions = VertexInteractions::new(board_range.clone(), 25.0);

        assert_eq!(interactions.board_range, board_range);
        assert_eq!(interactions.vertex_size, 25.0);
        assert!(!interactions.busy);
    }

    #[test]
    fn test_vertex_interactions_visible_dimensions() {
        let board_range = BoardRange::new((2, 6), (1, 4)); // 5x4 area
        let interactions = VertexInteractions::new(board_range, 30.0);

        let (width, height) = interactions.visible_dimensions();
        assert_eq!(width, 120.0); // (6-2) * 30 = 4 * 30
        assert_eq!(height, 90.0); // (4-1) * 30 = 3 * 30
    }

    #[test]
    fn test_vertex_interactions_busy_state() {
        let board_range = BoardRange::new((0, 8), (0, 8));
        let interactions = VertexInteractions::new(board_range, 25.0).with_busy(true);

        assert!(interactions.busy);
    }

    #[test]
    fn test_vertex_mouse_down_event_creation() {
        let vertex = Vertex::new(1, 2);
        let event = VertexMouseDownEvent::new(vertex.clone(), MouseButton::Left);

        assert_eq!(event.vertex, vertex);
        assert_eq!(event.coordinates, [1, 2]);
        assert_eq!(event.button, MouseButton::Left);
    }

    #[test]
    fn test_vertex_mouse_up_event_creation() {
        let vertex = Vertex::new(3, 4);
        let event = VertexMouseUpEvent::new(vertex.clone(), MouseButton::Right);

        assert_eq!(event.vertex, vertex);
        assert_eq!(event.coordinates, [3, 4]);
        assert_eq!(event.button, MouseButton::Right);
    }

    #[test]
    fn test_vertex_mouse_move_event_creation() {
        let vertex = Vertex::new(5, 6);
        let event = VertexMouseMoveEvent::new(vertex.clone());

        assert_eq!(event.vertex, vertex);
        assert_eq!(event.coordinates, [5, 6]);
    }

    #[test]
    fn test_vertex_event_handlers_creation() {
        let handlers = VertexEventHandlers::new();

        assert!(handlers.on_click.is_none());
        assert!(handlers.on_mouse_down.is_none());
        assert!(handlers.on_mouse_up.is_none());
        assert!(handlers.on_mouse_move.is_none());
    }

    #[test]
    fn test_vertex_event_handlers_with_click() {
        let handlers = VertexEventHandlers::new().with_click(|_event| {});

        assert!(handlers.on_click.is_some());
        assert!(handlers.on_mouse_down.is_none());
        assert!(handlers.on_mouse_up.is_none());
        assert!(handlers.on_mouse_move.is_none());
    }

    #[test]
    fn test_vertex_event_handlers_with_multiple() {
        let handlers = VertexEventHandlers::new()
            .with_click(|_event| {})
            .with_mouse_down(|_event| {})
            .with_mouse_up(|_event| {})
            .with_mouse_move(|_event| {});

        assert!(handlers.on_click.is_some());
        assert!(handlers.on_mouse_down.is_some());
        assert!(handlers.on_mouse_up.is_some());
        assert!(handlers.on_mouse_move.is_some());
    }
}
