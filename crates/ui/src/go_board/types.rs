/// Core data structures following Shudan's proven map-based approach
pub type SignMap = Vec<Vec<i8>>; // -1: white, 0: empty, 1: black
pub type MarkerMap = Vec<Vec<Option<Marker>>>;
pub type GhostStoneMap = Vec<Vec<Option<GhostStone>>>;
pub type HeatMap = Vec<Vec<Option<HeatData>>>;
pub type PaintMap = Vec<Vec<Option<PaintType>>>; // Changed to use PaintType enum

/// Paint types for territory visualization
#[derive(Clone, Debug, PartialEq)]
pub enum PaintType {
    Fill { opacity: f32 },
    Border { width: f32, color: String },
    Pattern { name: String, opacity: f32 },
}

/// Represents a position on the Go board using zero-based coordinates
/// [0, 0] denotes the upper left position of the board
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Vertex {
    pub x: usize,
    pub y: usize,
}

impl Vertex {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

/// Defines the visible range of the board for partial board display
#[derive(Clone, Debug, PartialEq)]
pub struct BoardRange {
    pub x: (usize, usize), // rangeX equivalent
    pub y: (usize, usize), // rangeY equivalent
}

impl BoardRange {
    pub fn new(x_range: (usize, usize), y_range: (usize, usize)) -> Self {
        Self {
            x: x_range,
            y: y_range,
        }
    }

    /// Creates a full board range for the given dimensions
    pub fn full(width: usize, height: usize) -> Self {
        Self {
            x: (0, width.saturating_sub(1)),
            y: (0, height.saturating_sub(1)),
        }
    }

    /// Returns the width of the visible range
    pub fn width(&self) -> usize {
        self.x.1.saturating_sub(self.x.0) + 1
    }

    /// Returns the height of the visible range
    pub fn height(&self) -> usize {
        self.y.1.saturating_sub(self.y.0) + 1
    }
}

/// Marker types supported by the Go board widget
#[derive(Clone, Debug, PartialEq)]
pub enum MarkerType {
    Circle,
    Cross,
    Triangle,
    Square,
    Point,
    Loader,
    Label(String),
}

/// Marker annotation on the board
#[derive(Clone, Debug, PartialEq)]
pub struct Marker {
    pub marker_type: MarkerType,
    pub label: Option<String>,       // For tooltips and text markers
    pub color: Option<String>,       // Color for the marker (CSS-style)
    pub size: f32,                   // Size multiplier relative to vertex size
    pub z_index: i32,                // Z-order for layering overlapping markers
    pub style_class: Option<String>, // Custom CSS class for styling
}

impl Marker {
    pub fn new(marker_type: MarkerType) -> Self {
        Self {
            marker_type,
            label: None,
            color: None,
            size: 1.0,
            z_index: 0,
            style_class: None,
        }
    }

    pub fn with_label(marker_type: MarkerType, label: String) -> Self {
        Self {
            marker_type,
            label: Some(label),
            color: None,
            size: 1.0,
            z_index: 0,
            style_class: None,
        }
    }

    pub fn with_color(mut self, color: String) -> Self {
        self.color = Some(color);
        self
    }

    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn with_z_index(mut self, z_index: i32) -> Self {
        self.z_index = z_index;
        self
    }

    pub fn with_style_class(mut self, style_class: String) -> Self {
        self.style_class = Some(style_class);
        self
    }
}

/// Ghost stone types for analysis visualization
#[derive(Clone, Debug, PartialEq)]
pub enum GhostStoneType {
    Good,
    Interesting,
    Doubtful,
    Bad,
}

/// Ghost stone for analysis and visualization
#[derive(Clone, Debug, PartialEq)]
pub struct GhostStone {
    pub sign: i8, // -1: white, 0: empty, 1: black
    pub stone_type: GhostStoneType,
    pub ghost_type: Option<String>, // Additional type identifier for analysis
    pub faint: bool,
}

impl GhostStone {
    pub fn new(sign: i8, stone_type: GhostStoneType) -> Self {
        Self {
            sign,
            stone_type,
            ghost_type: None,
            faint: false,
        }
    }

    pub fn with_ghost_type(mut self, ghost_type: String) -> Self {
        self.ghost_type = Some(ghost_type);
        self
    }

    pub fn faint(mut self) -> Self {
        self.faint = true;
        self
    }
}

/// Heat map data for influence visualization
#[derive(Clone, Debug, PartialEq)]
pub struct HeatData {
    pub strength: u8, // 0-9
    pub text: Option<String>,
}

impl HeatData {
    pub fn new(strength: u8) -> Self {
        Self {
            strength: strength.min(9), // Clamp to 0-9 range
            text: None,
        }
    }

    pub fn with_text(strength: u8, text: String) -> Self {
        Self {
            strength: strength.min(9),
            text: Some(text),
        }
    }
}

/// Line types for drawing connections between vertices
#[derive(Clone, Debug, PartialEq)]
pub enum LineType {
    Line,
    Arrow,
}

/// Line connecting two vertices
#[derive(Clone, Debug, PartialEq)]
pub struct Line {
    pub v1: Vertex,
    pub v2: Vertex,
    pub line_type: LineType,
}

impl Line {
    pub fn new(v1: Vertex, v2: Vertex, line_type: LineType) -> Self {
        Self { v1, v2, line_type }
    }

    pub fn line(v1: Vertex, v2: Vertex) -> Self {
        Self::new(v1, v2, LineType::Line)
    }

    pub fn arrow(v1: Vertex, v2: Vertex) -> Self {
        Self::new(v1, v2, LineType::Arrow)
    }
}

/// Selection types for directional vertex highlighting
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SelectionDirection {
    None,
    Left,
    Right,
    Top,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// Selection state for a vertex with visual configuration
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VertexSelection {
    pub vertex: Vertex,
    pub direction: SelectionDirection,
    pub opacity_percent: u8, // Changed to u8 percentage (0-100) for Hash compatibility
}

impl VertexSelection {
    pub fn new(vertex: Vertex) -> Self {
        Self {
            vertex,
            direction: SelectionDirection::None,
            opacity_percent: 100,
        }
    }

    pub fn with_direction(vertex: Vertex, direction: SelectionDirection) -> Self {
        Self {
            vertex,
            direction,
            opacity_percent: 100,
        }
    }

    pub fn dimmed(vertex: Vertex, opacity: f32) -> Self {
        Self {
            vertex,
            direction: SelectionDirection::None,
            opacity_percent: (opacity.clamp(0.0, 1.0) * 100.0) as u8,
        }
    }

    /// Gets the opacity as a floating-point value (0.0 to 1.0)
    pub fn opacity(&self) -> f32 {
        self.opacity_percent as f32 / 100.0
    }

    /// Sets the opacity from a floating-point value (0.0 to 1.0)
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity_percent = (opacity.clamp(0.0, 1.0) * 100.0) as u8;
        self
    }
}

/// Snapshot of selection state for efficient differential updates
#[derive(Clone, Debug, PartialEq)]
pub struct SelectionStateSnapshot {
    pub selected_vertices: Vec<Vertex>,
    pub dimmed_vertices: Vec<Vertex>,
    pub selected_left: Vec<Vertex>,
    pub selected_right: Vec<Vertex>,
    pub selected_top: Vec<Vertex>,
    pub selected_bottom: Vec<Vertex>,
}

impl SelectionStateSnapshot {
    pub fn new(
        selected_vertices: Vec<Vertex>,
        dimmed_vertices: Vec<Vertex>,
        selected_left: Vec<Vertex>,
        selected_right: Vec<Vertex>,
        selected_top: Vec<Vertex>,
        selected_bottom: Vec<Vertex>,
    ) -> Self {
        Self {
            selected_vertices,
            dimmed_vertices,
            selected_left,
            selected_right,
            selected_top,
            selected_bottom,
        }
    }

    /// Creates a snapshot from current board state
    pub fn from_board_state(state: &crate::go_board::GoBoardState) -> Self {
        Self {
            selected_vertices: state.selected_vertices.clone(),
            dimmed_vertices: state.dimmed_vertices.clone(),
            selected_left: state.selected_left.clone(),
            selected_right: state.selected_right.clone(),
            selected_top: state.selected_top.clone(),
            selected_bottom: state.selected_bottom.clone(),
        }
    }

    /// Checks if this snapshot differs from another
    pub fn differs_from(&self, other: &SelectionStateSnapshot) -> bool {
        self.selected_vertices != other.selected_vertices
            || self.dimmed_vertices != other.dimmed_vertices
            || self.selected_left != other.selected_left
            || self.selected_right != other.selected_right
            || self.selected_top != other.selected_top
            || self.selected_bottom != other.selected_bottom
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_creation() {
        let vertex = Vertex::new(3, 5);
        assert_eq!(vertex.x, 3);
        assert_eq!(vertex.y, 5);
    }

    #[test]
    fn test_board_range_full() {
        let range = BoardRange::full(19, 19);
        assert_eq!(range.x, (0, 18));
        assert_eq!(range.y, (0, 18));
        assert_eq!(range.width(), 19);
        assert_eq!(range.height(), 19);
    }

    #[test]
    fn test_board_range_partial() {
        let range = BoardRange::new((3, 10), (5, 12));
        assert_eq!(range.width(), 8);
        assert_eq!(range.height(), 8);
    }

    #[test]
    fn test_marker_creation() {
        let marker = Marker::new(MarkerType::Circle);
        assert_eq!(marker.marker_type, MarkerType::Circle);
        assert_eq!(marker.label, None);

        let labeled_marker =
            Marker::with_label(MarkerType::Label("A".to_string()), "Point A".to_string());
        assert!(matches!(labeled_marker.marker_type, MarkerType::Label(_)));
        assert_eq!(labeled_marker.label, Some("Point A".to_string()));
    }

    #[test]
    fn test_ghost_stone_creation() {
        let ghost = GhostStone::new(1, GhostStoneType::Good);
        assert_eq!(ghost.sign, 1);
        assert_eq!(ghost.stone_type, GhostStoneType::Good);
        assert!(!ghost.faint);

        let faint_ghost = GhostStone::new(-1, GhostStoneType::Doubtful).faint();
        assert!(faint_ghost.faint);
    }

    #[test]
    fn test_heat_data_creation() {
        let heat = HeatData::new(5);
        assert_eq!(heat.strength, 5);
        assert_eq!(heat.text, None);

        let heat_with_text = HeatData::with_text(8, "Strong".to_string());
        assert_eq!(heat_with_text.strength, 8);
        assert_eq!(heat_with_text.text, Some("Strong".to_string()));

        // Test clamping
        let clamped_heat = HeatData::new(15);
        assert_eq!(clamped_heat.strength, 9);
    }

    #[test]
    fn test_line_creation() {
        let v1 = Vertex::new(0, 0);
        let v2 = Vertex::new(5, 5);

        let line = Line::line(v1.clone(), v2.clone());
        assert_eq!(line.line_type, LineType::Line);

        let arrow = Line::arrow(v1, v2);
        assert_eq!(arrow.line_type, LineType::Arrow);
    }

    #[test]
    fn test_vertex_selection_creation() {
        let vertex = Vertex::new(3, 4);

        // Test basic selection
        let selection = VertexSelection::new(vertex.clone());
        assert_eq!(selection.vertex, vertex);
        assert_eq!(selection.direction, SelectionDirection::None);
        assert_eq!(selection.opacity(), 1.0);

        // Test directional selection
        let dir_selection =
            VertexSelection::with_direction(vertex.clone(), SelectionDirection::Left);
        assert_eq!(dir_selection.direction, SelectionDirection::Left);
        assert_eq!(dir_selection.opacity(), 1.0);

        // Test dimmed selection
        let dimmed_selection = VertexSelection::dimmed(vertex.clone(), 0.5);
        assert_eq!(dimmed_selection.opacity(), 0.5);
        assert_eq!(dimmed_selection.direction, SelectionDirection::None);

        // Test opacity clamping
        let clamped_high = VertexSelection::dimmed(vertex.clone(), 1.5);
        assert_eq!(clamped_high.opacity(), 1.0);

        let clamped_low = VertexSelection::dimmed(vertex, -0.5);
        assert_eq!(clamped_low.opacity(), 0.0);
    }

    #[test]
    fn test_selection_direction_enum() {
        let directions = vec![
            SelectionDirection::None,
            SelectionDirection::Left,
            SelectionDirection::Right,
            SelectionDirection::Top,
            SelectionDirection::Bottom,
            SelectionDirection::TopLeft,
            SelectionDirection::TopRight,
            SelectionDirection::BottomLeft,
            SelectionDirection::BottomRight,
        ];

        // Test that all directions can be cloned and compared
        for direction in directions {
            let cloned = direction.clone();
            assert_eq!(direction, cloned);
        }
    }
}
