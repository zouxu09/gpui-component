#[cfg(test)]
mod tests {
    use crate::go_board::types::*;

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
    fn test_marker_creation() {
        let marker = Marker::new(MarkerType::Circle);
        assert_eq!(marker.marker_type, MarkerType::Circle);
        assert_eq!(marker.label, None);

        let labeled_marker =
            Marker::with_label(MarkerType::Label("A".to_string()), "Point A".to_string());
        assert!(matches!(labeled_marker.marker_type, MarkerType::Label(_)));
        assert_eq!(labeled_marker.label, Some("Point A".to_string()));
    }
}
