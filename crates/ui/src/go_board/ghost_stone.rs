use crate::go_board::types::{GhostStone, GhostStoneMap, GhostStoneType, Vertex};
use gpui::{prelude::*, *};

/// Theme configuration for ghost stone appearance
#[derive(Clone, Debug)]
pub struct GhostStoneTheme {
    pub base_alpha: f32,         // Base transparency for normal ghost stones
    pub faint_alpha: f32,        // Additional transparency for faint ghost stones
    pub good_color: Hsla,        // Color for "good" moves
    pub interesting_color: Hsla, // Color for "interesting" moves
    pub doubtful_color: Hsla,    // Color for "doubtful" moves
    pub bad_color: Hsla,         // Color for "bad" moves
    pub border_width: f32,       // Border width for ghost stones
    pub size_reduction: f32,     // How much smaller than regular stones (0.0-1.0)
}

impl Default for GhostStoneTheme {
    fn default() -> Self {
        Self {
            base_alpha: 0.6,
            faint_alpha: 0.3,
            good_color: hsla(120.0 / 360.0, 0.8, 0.5, 1.0), // Green
            interesting_color: hsla(240.0 / 360.0, 0.8, 0.6, 1.0), // Blue
            doubtful_color: hsla(60.0 / 360.0, 0.8, 0.5, 1.0), // Yellow
            bad_color: hsla(0.0 / 360.0, 0.8, 0.5, 1.0),    // Red
            border_width: 2.0,
            size_reduction: 0.15, // Ghost stones are 15% smaller than regular stones
        }
    }
}

/// Ghost stone renderer for analysis and visualization features
/// Supports different ghost stone types with appropriate visual styling
#[derive(Clone)]
pub struct GhostStoneRenderer {
    pub vertex_size: f32,
    pub grid_offset: Point<Pixels>,
    pub theme: GhostStoneTheme,
}

impl GhostStoneRenderer {
    pub fn new(vertex_size: f32, grid_offset: Point<Pixels>) -> Self {
        Self {
            vertex_size,
            grid_offset,
            theme: GhostStoneTheme::default(),
        }
    }

    pub fn with_theme(mut self, theme: GhostStoneTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Renders a single ghost stone with the specified properties
    pub fn render_ghost_stone(
        &self,
        vertex: &Vertex,
        ghost_stone: &GhostStone,
    ) -> impl IntoElement {
        let position = self.calculate_ghost_stone_position(vertex);
        let size = self.calculate_ghost_stone_size();
        let (fill_color, border_color) = self.get_ghost_stone_colors(ghost_stone);

        div()
            .absolute()
            .left(position.x)
            .top(position.y)
            .w(px(size))
            .h(px(size))
            .rounded(px(size / 2.0)) // Make it circular
            .bg(fill_color)
            .border_4()
            .border_color(border_color)
            .flex()
            .items_center()
            .justify_center()
            .child(
                // Inner content can be used for additional visual cues
                div()
                    .w(px(size * 0.3))
                    .h(px(size * 0.3))
                    .rounded(px(size * 0.15))
                    .when(ghost_stone.sign == 1, |div| {
                        div.bg(gpui::black().alpha(0.8))
                    })
                    .when(ghost_stone.sign == -1, |div| {
                        div.bg(gpui::white().alpha(0.9))
                    }),
            )
    }

    /// Calculates the pixel position for ghost stone at the given vertex
    fn calculate_ghost_stone_position(&self, vertex: &Vertex) -> Point<Pixels> {
        let stone_size = self.calculate_ghost_stone_size();
        let offset = stone_size / 2.0;
        let x = self.grid_offset.x + px(vertex.x as f32 * self.vertex_size - offset);
        let y = self.grid_offset.y + px(vertex.y as f32 * self.vertex_size - offset);
        point(x, y)
    }

    /// Calculates the size of ghost stones (smaller than regular stones)
    fn calculate_ghost_stone_size(&self) -> f32 {
        self.vertex_size * (1.0 - self.theme.size_reduction)
    }

    /// Gets the appropriate colors for a ghost stone based on its type and properties
    fn get_ghost_stone_colors(&self, ghost_stone: &GhostStone) -> (Hsla, Hsla) {
        let base_color = match ghost_stone.stone_type {
            GhostStoneType::Good => self.theme.good_color,
            GhostStoneType::Interesting => self.theme.interesting_color,
            GhostStoneType::Doubtful => self.theme.doubtful_color,
            GhostStoneType::Bad => self.theme.bad_color,
        };

        // Adjust alpha based on faint property
        let alpha = if ghost_stone.faint {
            self.theme.faint_alpha
        } else {
            self.theme.base_alpha
        };

        let fill_color = base_color.alpha(alpha);
        let border_color = base_color
            .alpha(alpha + 0.2)
            .alpha((alpha + 0.2).min(1.0_f32));

        (fill_color, border_color)
    }

    /// Updates the renderer configuration
    pub fn update_config(&mut self, vertex_size: f32, grid_offset: Point<Pixels>) {
        self.vertex_size = vertex_size;
        self.grid_offset = grid_offset;
    }
}

/// Main ghost stone overlay component that manages and renders ghost stone visualization
#[derive(Clone)]
pub struct GhostStoneOverlay {
    renderer: GhostStoneRenderer,
}

impl GhostStoneOverlay {
    pub fn new(vertex_size: f32, grid_offset: Point<Pixels>) -> Self {
        Self {
            renderer: GhostStoneRenderer::new(vertex_size, grid_offset),
        }
    }

    pub fn with_theme(mut self, theme: GhostStoneTheme) -> Self {
        self.renderer = self.renderer.with_theme(theme);
        self
    }

    /// Renders the complete ghost stone overlay from ghost stone map data
    pub fn render_ghost_stones(&self, ghost_stone_map: &GhostStoneMap) -> impl IntoElement {
        let mut ghost_elements = Vec::new();

        // Render ghost stones
        for (y, row) in ghost_stone_map.iter().enumerate() {
            for (x, ghost_option) in row.iter().enumerate() {
                if let Some(ghost_stone) = ghost_option {
                    let vertex = Vertex::new(x, y);
                    ghost_elements.push(
                        self.renderer
                            .render_ghost_stone(&vertex, ghost_stone)
                            .into_any_element(),
                    );
                }
            }
        }

        div()
            .absolute()
            .top(px(0.0))
            .left(px(0.0))
            .w_full()
            .h_full()
            .children(ghost_elements)
    }

    /// Creates a ghost stone overlay from simple type and sign data for basic usage
    pub fn from_type_and_sign_maps(
        type_map: &[Vec<Option<GhostStoneType>>],
        sign_map: &[Vec<i8>],
        faint_map: Option<&[Vec<bool>]>,
    ) -> Vec<(Vertex, GhostStone)> {
        let mut ghost_data = Vec::new();

        for (y, (type_row, sign_row)) in type_map.iter().zip(sign_map.iter()).enumerate() {
            for (x, (type_option, &sign)) in type_row.iter().zip(sign_row.iter()).enumerate() {
                if let Some(ghost_type) = type_option {
                    if sign != 0 {
                        // Only create ghost stones for non-empty positions
                        let faint = faint_map
                            .and_then(|faint_map| faint_map.get(y))
                            .and_then(|faint_row| faint_row.get(x))
                            .copied()
                            .unwrap_or(false);

                        let ghost_stone = if faint {
                            GhostStone::new(sign, ghost_type.clone()).faint()
                        } else {
                            GhostStone::new(sign, ghost_type.clone())
                        };

                        ghost_data.push((Vertex::new(x, y), ghost_stone));
                    }
                }
            }
        }

        ghost_data
    }

    /// Creates a demonstration pattern showing all ghost stone types
    pub fn create_type_demonstration(width: usize, height: usize) -> GhostStoneMap {
        let mut ghost_map = vec![vec![None; width]; height];

        // Create a pattern demonstrating all ghost stone types
        let center_x = width / 2;
        let center_y = height / 2;

        if center_x >= 2 && center_y >= 2 && center_x + 2 < width && center_y + 2 < height {
            // Good moves (green) - top left
            ghost_map[center_y - 1][center_x - 1] = Some(GhostStone::new(1, GhostStoneType::Good));
            ghost_map[center_y - 2][center_x - 1] = Some(GhostStone::new(-1, GhostStoneType::Good));

            // Interesting moves (blue) - top right
            ghost_map[center_y - 1][center_x + 1] =
                Some(GhostStone::new(1, GhostStoneType::Interesting));
            ghost_map[center_y - 2][center_x + 1] =
                Some(GhostStone::new(-1, GhostStoneType::Interesting));

            // Doubtful moves (yellow) - bottom left
            ghost_map[center_y + 1][center_x - 1] =
                Some(GhostStone::new(1, GhostStoneType::Doubtful));
            ghost_map[center_y + 2][center_x - 1] =
                Some(GhostStone::new(-1, GhostStoneType::Doubtful));

            // Bad moves (red) - bottom right
            ghost_map[center_y + 1][center_x + 1] = Some(GhostStone::new(1, GhostStoneType::Bad));
            ghost_map[center_y + 2][center_x + 1] = Some(GhostStone::new(-1, GhostStoneType::Bad));

            // Faint ghost stones - center column
            ghost_map[center_y][center_x] = Some(GhostStone::new(1, GhostStoneType::Good).faint());
            ghost_map[center_y - 1][center_x] =
                Some(GhostStone::new(-1, GhostStoneType::Interesting).faint());
            ghost_map[center_y + 1][center_x] =
                Some(GhostStone::new(1, GhostStoneType::Doubtful).faint());
        }

        ghost_map
    }

    /// Efficiently renders only changed ghost stone areas for performance optimization
    pub fn render_ghost_updates(
        &self,
        updates: &[(Vertex, Option<GhostStone>)],
    ) -> impl IntoElement {
        let mut update_elements = Vec::new();

        for (vertex, ghost_option) in updates {
            if let Some(ghost_stone) = ghost_option {
                update_elements.push(self.renderer.render_ghost_stone(vertex, ghost_stone));
            }
            // Skip rendering None updates (they're clearing operations)
        }

        div()
            .absolute()
            .top(px(0.0))
            .left(px(0.0))
            .w_full()
            .h_full()
            .children(update_elements)
    }

    /// Updates renderer configuration
    pub fn update_renderer(&mut self, vertex_size: f32, grid_offset: Point<Pixels>) {
        self.renderer.update_config(vertex_size, grid_offset);
    }

    /// Updates the theme for all ghost stones
    pub fn update_theme(&mut self, theme: GhostStoneTheme) {
        self.renderer.theme = theme;
    }

    /// Gets a reference to the current renderer
    pub fn renderer(&self) -> &GhostStoneRenderer {
        &self.renderer
    }

    /// Gets a mutable reference to the current renderer
    pub fn renderer_mut(&mut self) -> &mut GhostStoneRenderer {
        &mut self.renderer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ghost_stone_theme_default() {
        let theme = GhostStoneTheme::default();
        assert_eq!(theme.base_alpha, 0.6);
        assert_eq!(theme.faint_alpha, 0.3);
        assert_eq!(theme.border_width, 2.0);
        assert_eq!(theme.size_reduction, 0.15);
    }

    #[test]
    fn test_ghost_stone_renderer_creation() {
        let renderer = GhostStoneRenderer::new(24.0, point(px(10.0), px(10.0)));
        assert_eq!(renderer.vertex_size, 24.0);
        assert_eq!(renderer.grid_offset.x, px(10.0));
        assert_eq!(renderer.grid_offset.y, px(10.0));
    }

    #[test]
    fn test_ghost_stone_size_calculation() {
        let renderer = GhostStoneRenderer::new(24.0, point(px(0.0), px(0.0)));
        let size = renderer.calculate_ghost_stone_size();
        assert_eq!(size, 24.0 * (1.0 - 0.15)); // Default size reduction is 15%
    }

    #[test]
    fn test_ghost_stone_position_calculation() {
        let renderer = GhostStoneRenderer::new(24.0, point(px(10.0), px(10.0)));
        let vertex = Vertex::new(2, 3);
        let position = renderer.calculate_ghost_stone_position(&vertex);

        let expected_size = 24.0 * (1.0 - 0.15);
        let expected_offset = expected_size / 2.0;
        let expected_x = px(10.0 + 2.0 * 24.0 - expected_offset);
        let expected_y = px(10.0 + 3.0 * 24.0 - expected_offset);

        assert_eq!(position.x, expected_x);
        assert_eq!(position.y, expected_y);
    }

    #[test]
    fn test_ghost_stone_colors() {
        let renderer = GhostStoneRenderer::new(24.0, point(px(0.0), px(0.0)));

        // Test different types
        let good_ghost = GhostStone::new(1, GhostStoneType::Good);
        let (fill, border) = renderer.get_ghost_stone_colors(&good_ghost);
        assert_eq!(fill.a, 0.6); // Base alpha

        let faint_ghost = GhostStone::new(1, GhostStoneType::Good).faint();
        let (faint_fill, _) = renderer.get_ghost_stone_colors(&faint_ghost);
        assert_eq!(faint_fill.a, 0.3); // Faint alpha

        // Test all types have different hues
        let interesting_ghost = GhostStone::new(1, GhostStoneType::Interesting);
        let doubtful_ghost = GhostStone::new(1, GhostStoneType::Doubtful);
        let bad_ghost = GhostStone::new(1, GhostStoneType::Bad);

        let (good_color, _) = renderer.get_ghost_stone_colors(&good_ghost);
        let (interesting_color, _) = renderer.get_ghost_stone_colors(&interesting_ghost);
        let (doubtful_color, _) = renderer.get_ghost_stone_colors(&doubtful_ghost);
        let (bad_color, _) = renderer.get_ghost_stone_colors(&bad_ghost);

        // All should have different hues
        assert_ne!(good_color.h, interesting_color.h);
        assert_ne!(good_color.h, doubtful_color.h);
        assert_ne!(good_color.h, bad_color.h);
    }

    #[test]
    fn test_ghost_stone_overlay_creation() {
        let overlay = GhostStoneOverlay::new(24.0, point(px(5.0), px(5.0)));
        assert_eq!(overlay.renderer.vertex_size, 24.0);
    }

    #[test]
    fn test_type_and_sign_maps_conversion() {
        let type_map = vec![
            vec![Some(GhostStoneType::Good), None],
            vec![None, Some(GhostStoneType::Bad)],
        ];
        let sign_map = vec![vec![1, 0], vec![0, -1]];
        let faint_map = vec![vec![false, false], vec![false, true]];

        let ghost_data =
            GhostStoneOverlay::from_type_and_sign_maps(&type_map, &sign_map, Some(&faint_map));

        assert_eq!(ghost_data.len(), 2); // Only non-empty positions

        // Check that data is correctly assigned
        let good_entry = ghost_data
            .iter()
            .find(|(v, _)| v == &Vertex::new(0, 0))
            .unwrap();
        assert_eq!(good_entry.1.stone_type, GhostStoneType::Good);
        assert_eq!(good_entry.1.sign, 1);
        assert!(!good_entry.1.faint);

        let bad_entry = ghost_data
            .iter()
            .find(|(v, _)| v == &Vertex::new(1, 1))
            .unwrap();
        assert_eq!(bad_entry.1.stone_type, GhostStoneType::Bad);
        assert_eq!(bad_entry.1.sign, -1);
        assert!(bad_entry.1.faint);
    }

    #[test]
    fn test_ghost_stone_rendering() {
        let overlay = GhostStoneOverlay::new(24.0, point(px(0.0), px(0.0)));
        let ghost_map = vec![
            vec![Some(GhostStone::new(1, GhostStoneType::Good)), None],
            vec![None, Some(GhostStone::new(-1, GhostStoneType::Bad).faint())],
        ];

        let _element = overlay.render_ghost_stones(&ghost_map);
        // Should render without panicking
    }

    #[test]
    fn test_type_demonstration() {
        let ghost_map = GhostStoneOverlay::create_type_demonstration(9, 9);

        // Check that we have ghost stones of different types
        let has_good = ghost_map.iter().flatten().any(|cell| {
            cell.as_ref()
                .map(|g| g.stone_type == GhostStoneType::Good)
                .unwrap_or(false)
        });
        let has_bad = ghost_map.iter().flatten().any(|cell| {
            cell.as_ref()
                .map(|g| g.stone_type == GhostStoneType::Bad)
                .unwrap_or(false)
        });
        let has_faint = ghost_map
            .iter()
            .flatten()
            .any(|cell| cell.as_ref().map(|g| g.faint).unwrap_or(false));

        assert!(has_good);
        assert!(has_bad);
        assert!(has_faint);
    }

    #[test]
    fn test_ghost_update_rendering() {
        let overlay = GhostStoneOverlay::new(24.0, point(px(0.0), px(0.0)));
        let updates = vec![
            (
                Vertex::new(1, 1),
                Some(GhostStone::new(1, GhostStoneType::Good)),
            ),
            (Vertex::new(2, 2), None), // Should be skipped
            (
                Vertex::new(3, 3),
                Some(GhostStone::new(-1, GhostStoneType::Bad).faint()),
            ),
        ];

        let _element = overlay.render_ghost_updates(&updates);
        // Should render only non-None updates without panicking
    }

    #[test]
    fn test_renderer_update() {
        let mut overlay = GhostStoneOverlay::new(20.0, point(px(0.0), px(0.0)));
        overlay.update_renderer(30.0, point(px(5.0), px(5.0)));

        assert_eq!(overlay.renderer.vertex_size, 30.0);
        assert_eq!(overlay.renderer.grid_offset.x, px(5.0));
        assert_eq!(overlay.renderer.grid_offset.y, px(5.0));
    }

    #[test]
    fn test_theme_update() {
        let mut overlay = GhostStoneOverlay::new(24.0, point(px(0.0), px(0.0)));
        let custom_theme = GhostStoneTheme {
            base_alpha: 0.8,
            faint_alpha: 0.4,
            ..GhostStoneTheme::default()
        };

        overlay.update_theme(custom_theme.clone());
        assert_eq!(overlay.renderer.theme.base_alpha, 0.8);
        assert_eq!(overlay.renderer.theme.faint_alpha, 0.4);
    }

    #[test]
    fn test_comprehensive_ghost_rendering() {
        let overlay = GhostStoneOverlay::new(25.0, point(px(2.0), px(2.0)));

        // Create comprehensive test data
        let ghost_map = vec![
            vec![
                Some(GhostStone::new(1, GhostStoneType::Good)),
                Some(GhostStone::new(-1, GhostStoneType::Interesting)),
                None,
            ],
            vec![
                Some(GhostStone::new(1, GhostStoneType::Doubtful).faint()),
                None,
                Some(GhostStone::new(-1, GhostStoneType::Bad)),
            ],
            vec![
                None,
                Some(GhostStone::new(1, GhostStoneType::Good).faint()),
                Some(GhostStone::new(-1, GhostStoneType::Interesting).faint()),
            ],
        ];

        let _element = overlay.render_ghost_stones(&ghost_map);
        // Should handle complex ghost configuration without panicking
    }
}
