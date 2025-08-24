use crate::go_board::core::*;
use gpui::*;

/// Unified renderer that handles all board elements in a single, coherent system
/// This replaces Grid, Stones, Markers, GhostStoneOverlay, HeatOverlay, etc.
pub struct Renderer {
    vertex_size: f32,
    theme: Theme,
    coord_offset: Point<Pixels>,
}

impl Renderer {
    pub fn new(vertex_size: f32, theme: Theme) -> Self {
        Self {
            vertex_size,
            theme,
            coord_offset: point(px(0.0), px(0.0)),
        }
    }

    pub fn with_coordinates(mut self, show: bool) -> Self {
        if show {
            let margin = self.theme.coord_size + 8.0;
            self.coord_offset = point(px(margin), px(margin));
        }
        self
    }

    /// Main render method - creates the complete board
    pub fn render(&self, data: &BoardData, show_coordinates: bool) -> impl IntoElement {
        let range = &data.range;
        let grid_width = range.width() as f32 * self.vertex_size;
        let grid_height = range.height() as f32 * self.vertex_size;

        if show_coordinates {
            self.render_with_coordinates(data, grid_width, grid_height)
        } else {
            self.render_board_only(data, grid_width, grid_height)
                .into_any_element()
        }
    }

    /// Render board without coordinates
    fn render_board_only(&self, data: &BoardData, width: f32, height: f32) -> impl IntoElement {
        let mut board = div()
            .relative()
            .w(px(width))
            .h(px(height))
            .bg(self.theme.background)
            .border_1()
            .border_color(self.theme.border)
            .overflow_hidden();

        // Add all layers in correct order
        board = board
            .child(self.render_grid(data))
            .child(self.render_territory(data))
            .child(self.render_heat(data))
            .child(self.render_stones(data))
            .child(self.render_ghosts(data))
            .child(self.render_lines(data))
            .child(self.render_markers(data))
            .child(self.render_selections(data));

        board
    }

    /// Render board with coordinates
    fn render_with_coordinates(
        &self,
        data: &BoardData,
        grid_width: f32,
        grid_height: f32,
    ) -> AnyElement {
        let margin = self.theme.coord_size + 8.0;
        let total_width = grid_width + 2.0 * margin;
        let total_height = grid_height + 2.0 * margin;

        let mut container = div().relative().w(px(total_width)).h(px(total_height));

        // Add coordinate labels
        container = container.child(self.render_coordinates(data, grid_width, grid_height, margin));

        // Add main board
        container = container.child(
            div()
                .absolute()
                .left(px(margin))
                .top(px(margin))
                .child(self.render_board_only(data, grid_width, grid_height)),
        );

        container.into_any_element()
    }

    /// Render interactive layer for mouse/keyboard events
    pub fn render_interactive(&self, data: &BoardData, show_coordinates: bool) -> impl IntoElement {
        let range = &data.range;
        let _grid_width = range.width() as f32 * self.vertex_size;
        let _grid_height = range.height() as f32 * self.vertex_size;

        let offset = if show_coordinates {
            let margin = self.theme.coord_size + 8.0;
            point(px(margin), px(margin))
        } else {
            point(px(0.0), px(0.0))
        };

        let mut interactive = div().absolute().inset_0(); // Above all visual elements

        // Create invisible buttons for each position
        for y in range.y.0..=range.y.1 {
            for x in range.x.0..=range.x.1 {
                let pos = Pos::new(x, y);
                let pixel_pos = self.pos_to_pixel(pos, range, offset);
                let button_size = self.vertex_size * 0.8; // Slightly smaller for better feel

                interactive = interactive.child(
                    div()
                        .absolute()
                        .left(pixel_pos.x - px(button_size / 2.0))
                        .top(pixel_pos.y - px(button_size / 2.0))
                        .w(px(button_size))
                        .h(px(button_size))
                        .id(("pos", x * 1000 + y)), // Add hover and click handlers here
                                                    // Note: In real implementation, these would be connected to the board's event handlers
                );
            }
        }

        interactive
    }

    // =============================================================================
    // INDIVIDUAL LAYER RENDERING
    // =============================================================================

    /// Render grid lines and star points
    fn render_grid(&self, data: &BoardData) -> impl IntoElement {
        let range = &data.range;
        let mut grid = div().absolute().inset_0();

        // Horizontal lines
        for y in range.y.0..=range.y.1 {
            let relative_y = (y - range.y.0) as f32;
            let pixel_y = relative_y * self.vertex_size + self.vertex_size / 2.0;

            grid = grid.child(
                div()
                    .absolute()
                    .left(px(self.vertex_size / 2.0))
                    .top(px(pixel_y - self.theme.grid_width / 2.0))
                    .w(px((range.width() - 1) as f32 * self.vertex_size))
                    .h(px(self.theme.grid_width))
                    .bg(self.theme.grid_lines),
            );
        }

        // Vertical lines
        for x in range.x.0..=range.x.1 {
            let relative_x = (x - range.x.0) as f32;
            let pixel_x = relative_x * self.vertex_size + self.vertex_size / 2.0;

            grid = grid.child(
                div()
                    .absolute()
                    .left(px(pixel_x - self.theme.grid_width / 2.0))
                    .top(px(self.vertex_size / 2.0))
                    .w(px(self.theme.grid_width))
                    .h(px((range.height() - 1) as f32 * self.vertex_size))
                    .bg(self.theme.grid_lines),
            );
        }

        // Star points
        for pos in self.calculate_star_points(data) {
            if range.contains(pos) {
                let pixel_pos = self.pos_to_pixel(pos, range, point(px(0.0), px(0.0)));
                grid = grid.child(
                    div()
                        .absolute()
                        .left(pixel_pos.x - px(self.theme.star_size / 2.0))
                        .top(pixel_pos.y - px(self.theme.star_size / 2.0))
                        .w(px(self.theme.star_size))
                        .h(px(self.theme.star_size))
                        .rounded_full()
                        .bg(self.theme.star_points),
                );
            }
        }

        grid
    }

    /// Render stones
    fn render_stones(&self, data: &BoardData) -> impl IntoElement {
        let mut stones = div().absolute().inset_0();
        let range = &data.range;

        for (&pos, &stone) in &data.stones {
            if range.contains(pos) && stone != EMPTY {
                let pixel_pos = self.pos_to_pixel(pos, range, point(px(0.0), px(0.0)));
                let stone_size = self.vertex_size * self.theme.stone_size;

                let color = match stone {
                    BLACK => self.theme.black_stone,
                    WHITE => self.theme.white_stone,
                    _ => continue,
                };

                stones = stones.child(
                    div()
                        .absolute()
                        .left(pixel_pos.x - px(stone_size / 2.0))
                        .top(pixel_pos.y - px(stone_size / 2.0))
                        .w(px(stone_size))
                        .h(px(stone_size))
                        .rounded_full()
                        .bg(color), // Shadow effect would be applied here if available
                                    // TODO: Add shadow when available in gpui
                );
            }
        }

        stones
    }

    /// Render markers
    fn render_markers(&self, data: &BoardData) -> impl IntoElement {
        let mut markers = div().absolute().inset_0();
        let range = &data.range;

        for (&pos, marker) in &data.markers {
            if range.contains(pos) {
                let pixel_pos = self.pos_to_pixel(pos, range, point(px(0.0), px(0.0)));
                let marker_size = self.vertex_size * 0.4;

                markers = markers.child(self.render_marker(marker, pixel_pos, marker_size));
            }
        }

        markers
    }

    /// Render individual marker
    fn render_marker(&self, marker: &Marker, pos: Point<Pixels>, size: f32) -> impl IntoElement {
        match marker {
            Marker::Circle { color } => div()
                .absolute()
                .left(pos.x - px(size / 2.0))
                .top(pos.y - px(size / 2.0))
                .w(px(size))
                .h(px(size))
                .rounded_full()
                .border_2()
                .border_color(*color),
            Marker::Cross { color } => div()
                .absolute()
                .left(pos.x - px(size / 2.0))
                .top(pos.y - px(size / 2.0))
                .w(px(size))
                .h(px(size))
                .flex()
                .items_center()
                .justify_center()
                .child(div().w(px(size * 0.8)).h(px(2.0)).bg(*color))
                .child(div().absolute().w(px(2.0)).h(px(size * 0.8)).bg(*color)),
            Marker::Triangle { color } => {
                // Simplified triangle using CSS borders
                div()
                    .absolute()
                    .left(pos.x - px(size / 2.0))
                    .top(pos.y - px(size / 2.0))
                    .w(px(size))
                    .h(px(size))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .w(px(0.0))
                            .h(px(0.0))
                            .border_l_8()
                            .border_r_8()
                            .border_b_8()
                            .border_color(*color),
                    )
            }
            Marker::Square { color } => div()
                .absolute()
                .left(pos.x - px(size / 2.0))
                .top(pos.y - px(size / 2.0))
                .w(px(size))
                .h(px(size))
                .border_2()
                .border_color(*color),
            Marker::Dot { color } => div()
                .absolute()
                .left(pos.x - px(size / 4.0))
                .top(pos.y - px(size / 4.0))
                .w(px(size / 2.0))
                .h(px(size / 2.0))
                .rounded_full()
                .bg(*color),
            Marker::Label { text, color } => div()
                .absolute()
                .left(pos.x - px(size / 2.0))
                .top(pos.y - px(size / 2.0))
                .w(px(size))
                .h(px(size))
                .flex()
                .items_center()
                .justify_center()
                .text_xs()
                .text_color(*color)
                .child(text.clone()),
        }
    }

    /// Render ghost stones
    fn render_ghosts(&self, data: &BoardData) -> impl IntoElement {
        let mut ghosts = div().absolute().inset_0();
        let range = &data.range;

        for (&pos, ghost) in &data.ghosts {
            if range.contains(pos) && ghost.stone != EMPTY {
                let pixel_pos = self.pos_to_pixel(pos, range, point(px(0.0), px(0.0)));
                let stone_size = self.vertex_size * self.theme.stone_size * 0.8; // Smaller than regular stones

                let base_color = match ghost.stone {
                    BLACK => self.theme.black_stone,
                    WHITE => self.theme.white_stone,
                    _ => continue,
                };

                let tinted_color = match ghost.kind {
                    GhostKind::Good => {
                        // Green tint
                        let mut hsla: Hsla = base_color.into();
                        hsla.h = 120.0 / 360.0; // Green hue
                        hsla.s = 0.6;
                        hsla.into()
                    }
                    GhostKind::Bad => {
                        // Red tint
                        let mut hsla: Hsla = base_color.into();
                        hsla.h = 0.0; // Red hue
                        hsla.s = 0.6;
                        hsla.into()
                    }
                    GhostKind::Neutral => base_color,
                };

                ghosts = ghosts.child(
                    div()
                        .absolute()
                        .left(pixel_pos.x - px(stone_size / 2.0))
                        .top(pixel_pos.y - px(stone_size / 2.0))
                        .w(px(stone_size))
                        .h(px(stone_size))
                        .rounded_full()
                        .bg(tinted_color)
                        .opacity(ghost.alpha),
                );
            }
        }

        ghosts
    }

    /// Render heat overlay
    fn render_heat(&self, data: &BoardData) -> impl IntoElement {
        let mut heat = div().absolute().inset_0();
        let range = &data.range;

        for (&pos, heat_data) in &data.heat {
            if range.contains(pos) && heat_data.strength > 0 {
                let pixel_pos = self.pos_to_pixel(pos, range, point(px(0.0), px(0.0)));
                let heat_size = self.vertex_size * 0.7;
                let color = self.strength_to_color(heat_data.strength);

                let mut heat_element = div()
                    .absolute()
                    .left(pixel_pos.x - px(heat_size / 2.0))
                    .top(pixel_pos.y - px(heat_size / 2.0))
                    .w(px(heat_size))
                    .h(px(heat_size))
                    .rounded(px(heat_size * 0.1))
                    .bg(color)
                    .flex()
                    .items_center()
                    .justify_center();

                if let Some(ref label) = heat_data.label {
                    heat_element = heat_element.child(
                        div()
                            .text_xs()
                            .text_color(if heat_data.strength > 5 {
                                gpui::white()
                            } else {
                                gpui::black()
                            })
                            .child(label.clone()),
                    );
                }

                heat = heat.child(heat_element);
            }
        }

        heat
    }

    /// Render territory overlay
    fn render_territory(&self, data: &BoardData) -> impl IntoElement {
        let mut territory = div().absolute().inset_0();
        let range = &data.range;

        for (&pos, territory_data) in &data.territory {
            if range.contains(pos) {
                let pixel_pos = self.pos_to_pixel(pos, range, point(px(0.0), px(0.0)));
                let territory_size = self.vertex_size;

                let color = match territory_data.owner {
                    BLACK => self.theme.black_stone,
                    WHITE => self.theme.white_stone,
                    _ => rgb(0x808080).into(), // Neutral
                };

                territory = territory.child(
                    div()
                        .absolute()
                        .left(pixel_pos.x - px(territory_size / 2.0))
                        .top(pixel_pos.y - px(territory_size / 2.0))
                        .w(px(territory_size))
                        .h(px(territory_size))
                        .bg(color)
                        .opacity(territory_data.alpha),
                );
            }
        }

        territory
    }

    /// Render selection highlights
    fn render_selections(&self, data: &BoardData) -> impl IntoElement {
        let mut selections = div().absolute().inset_0();
        let range = &data.range;

        for (&pos, selection) in &data.selections {
            if range.contains(pos) {
                let pixel_pos = self.pos_to_pixel(pos, range, point(px(0.0), px(0.0)));
                let selection_size = self.vertex_size * 0.9;

                let element = match &selection.style {
                    SelectionStyle::Selected { color } => div()
                        .w(px(selection_size))
                        .h(px(selection_size))
                        .rounded(px(selection_size * 0.1))
                        .border_2()
                        .border_color(*color),
                    SelectionStyle::Dimmed { alpha } => div()
                        .w(px(selection_size))
                        .h(px(selection_size))
                        .bg(rgb(0x000000))
                        .opacity(*alpha),
                    SelectionStyle::LastMove { color } => div()
                        .w(px(selection_size * 0.5))
                        .h(px(selection_size * 0.5))
                        .rounded_full()
                        .bg(*color),
                };

                selections = selections.child(
                    div()
                        .absolute()
                        .left(pixel_pos.x - px(selection_size / 2.0))
                        .top(pixel_pos.y - px(selection_size / 2.0))
                        .child(element),
                );
            }
        }

        selections
    }

    /// Render lines and arrows
    fn render_lines(&self, data: &BoardData) -> impl IntoElement {
        let mut lines = div().absolute().inset_0();
        let range = &data.range;

        for line in &data.lines {
            if range.contains(line.from) && range.contains(line.to) {
                let from_pixel = self.pos_to_pixel(line.from, range, point(px(0.0), px(0.0)));
                let to_pixel = self.pos_to_pixel(line.to, range, point(px(0.0), px(0.0)));

                // Calculate line length and angle
                let dx = to_pixel.x.0 - from_pixel.x.0;
                let dy = to_pixel.y.0 - from_pixel.y.0;
                let length = (dx * dx + dy * dy).sqrt();
                let _angle = dy.atan2(dx); // TODO: implement rotation

                let (color, width) = match &line.style {
                    LineStyle::Line { color, width } => (*color, *width),
                    LineStyle::Arrow { color, width } => (*color, *width),
                };

                // Simple line rendering (arrows would need more complex SVG or canvas)
                lines = lines.child(
                    div()
                        .absolute()
                        .left(from_pixel.x)
                        .top(from_pixel.y - px(width / 2.0))
                        .w(px(length))
                        .h(px(width))
                        .bg(color), // Note: Transform rotation would need proper implementation
                );
            }
        }

        lines
    }

    /// Render coordinate labels
    fn render_coordinates(
        &self,
        data: &BoardData,
        grid_width: f32,
        grid_height: f32,
        margin: f32,
    ) -> impl IntoElement {
        let mut coords = div().absolute().inset_0();
        let range = &data.range;

        // Top and bottom coordinates (letters)
        for x in range.x.0..=range.x.1 {
            let relative_x = (x - range.x.0) as f32;
            let pixel_x = margin + relative_x * self.vertex_size + self.vertex_size / 2.0;
            let label = self.x_coordinate_label(x);

            // Top
            coords = coords.child(
                div()
                    .absolute()
                    .left(px(pixel_x - self.theme.coord_size / 2.0))
                    .top(px(0.0))
                    .w(px(self.theme.coord_size))
                    .h(px(self.theme.coord_size))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(self.theme.coord_size))
                    .text_color(self.theme.coordinates)
                    .child(label.clone()),
            );

            // Bottom
            coords = coords.child(
                div()
                    .absolute()
                    .left(px(pixel_x - self.theme.coord_size / 2.0))
                    .top(px(margin + grid_height))
                    .w(px(self.theme.coord_size))
                    .h(px(self.theme.coord_size))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(self.theme.coord_size))
                    .text_color(self.theme.coordinates)
                    .child(label),
            );
        }

        // Left and right coordinates (numbers)
        for y in range.y.0..=range.y.1 {
            let relative_y = (y - range.y.0) as f32;
            let pixel_y = margin + relative_y * self.vertex_size + self.vertex_size / 2.0;
            let label = self.y_coordinate_label(y, data.size.1);

            // Left
            coords = coords.child(
                div()
                    .absolute()
                    .left(px(0.0))
                    .top(px(pixel_y - self.theme.coord_size / 2.0))
                    .w(px(self.theme.coord_size))
                    .h(px(self.theme.coord_size))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(self.theme.coord_size))
                    .text_color(self.theme.coordinates)
                    .child(label.clone()),
            );

            // Right
            coords = coords.child(
                div()
                    .absolute()
                    .left(px(margin + grid_width))
                    .top(px(pixel_y - self.theme.coord_size / 2.0))
                    .w(px(self.theme.coord_size))
                    .h(px(self.theme.coord_size))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(self.theme.coord_size))
                    .text_color(self.theme.coordinates)
                    .child(label),
            );
        }

        coords
    }

    // =============================================================================
    // UTILITY METHODS
    // =============================================================================

    /// Convert position to pixel coordinates
    fn pos_to_pixel(&self, pos: Pos, range: &Range, offset: Point<Pixels>) -> Point<Pixels> {
        let relative_x = (pos.x - range.x.0) as f32;
        let relative_y = (pos.y - range.y.0) as f32;

        Point::new(
            offset.x + px(relative_x * self.vertex_size + self.vertex_size / 2.0),
            offset.y + px(relative_y * self.vertex_size + self.vertex_size / 2.0),
        )
    }

    /// Calculate star point positions for standard board sizes
    fn calculate_star_points(&self, data: &BoardData) -> Vec<Pos> {
        let (width, height) = data.size;
        let mut points = Vec::new();

        match (width, height) {
            (19, 19) => {
                points.extend([
                    Pos::new(3, 3),
                    Pos::new(9, 3),
                    Pos::new(15, 3),
                    Pos::new(3, 9),
                    Pos::new(9, 9),
                    Pos::new(15, 9),
                    Pos::new(3, 15),
                    Pos::new(9, 15),
                    Pos::new(15, 15),
                ]);
            }
            (13, 13) => {
                points.extend([
                    Pos::new(3, 3),
                    Pos::new(9, 3),
                    Pos::new(6, 6),
                    Pos::new(3, 9),
                    Pos::new(9, 9),
                ]);
            }
            (9, 9) => {
                points.extend([
                    Pos::new(2, 2),
                    Pos::new(6, 2),
                    Pos::new(4, 4),
                    Pos::new(2, 6),
                    Pos::new(6, 6),
                ]);
            }
            _ => {
                // Custom size - add corner and center points
                if width >= 7 && height >= 7 {
                    let offset = if width <= 11 { 2 } else { 3 };
                    points.extend([
                        Pos::new(offset, offset),
                        Pos::new(width - 1 - offset, offset),
                        Pos::new(offset, height - 1 - offset),
                        Pos::new(width - 1 - offset, height - 1 - offset),
                    ]);

                    if width % 2 == 1 && height % 2 == 1 {
                        points.push(Pos::new(width / 2, height / 2));
                    }
                }
            }
        }

        points
    }

    /// Convert heat strength to color
    fn strength_to_color(&self, strength: u8) -> Rgba {
        let intensity = (strength as f32 / 9.0).min(1.0);
        let alpha = 0.3 + intensity * 0.5;

        if intensity <= 0.33 {
            // Blue to cyan
            let hue = 240.0 - (intensity / 0.33) * 60.0;
            hsla(hue / 360.0, 0.8, 0.6, alpha).into()
        } else if intensity <= 0.66 {
            // Cyan to yellow
            let hue = 180.0 - ((intensity - 0.33) / 0.33) * 120.0;
            hsla(hue / 360.0, 0.8, 0.6, alpha).into()
        } else {
            // Yellow to red
            let hue = 60.0 - ((intensity - 0.66) / 0.34) * 60.0;
            hsla(hue / 360.0, 0.9, 0.5, alpha).into()
        }
    }

    /// Convert x coordinate to letter label
    fn x_coordinate_label(&self, x: usize) -> String {
        let letter = if x < 8 {
            (b'A' + x as u8) as char
        } else {
            (b'A' + x as u8 + 1) as char // Skip 'I'
        };
        letter.to_string()
    }

    /// Convert y coordinate to number label (inverted for Go)
    fn y_coordinate_label(&self, y: usize, board_height: usize) -> String {
        (board_height - y).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_renderer_creation() {
        let renderer = Renderer::new(24.0, Theme::default());
        assert_eq!(renderer.vertex_size, 24.0);
    }

    #[test]
    fn test_star_point_calculation() {
        let renderer = Renderer::new(20.0, Theme::default());
        let data = BoardData::new(19, 19);
        let stars = renderer.calculate_star_points(&data);
        assert_eq!(stars.len(), 9); // 19x19 has 9 star points
        assert!(stars.contains(&Pos::new(9, 9))); // Center point
    }

    #[test]
    fn test_pos_to_pixel() {
        let renderer = Renderer::new(20.0, Theme::default());
        let range = Range::new((0, 18), (0, 18));
        let offset = point(px(10.0), px(10.0));

        let pixel = renderer.pos_to_pixel(Pos::new(0, 0), &range, offset);
        assert_eq!(pixel.x, px(20.0)); // 10 + 0*20 + 10
        assert_eq!(pixel.y, px(20.0));

        let pixel = renderer.pos_to_pixel(Pos::new(9, 9), &range, offset);
        assert_eq!(pixel.x, px(200.0)); // 10 + 9*20 + 10
        assert_eq!(pixel.y, px(200.0));
    }

    #[test]
    fn test_coordinate_labels() {
        let renderer = Renderer::new(20.0, Theme::default());

        assert_eq!(renderer.x_coordinate_label(0), "A");
        assert_eq!(renderer.x_coordinate_label(7), "H");
        assert_eq!(renderer.x_coordinate_label(8), "J"); // Skip I
        assert_eq!(renderer.x_coordinate_label(17), "S");

        assert_eq!(renderer.y_coordinate_label(0, 19), "19");
        assert_eq!(renderer.y_coordinate_label(18, 19), "1");
    }

    #[test]
    fn test_strength_to_color() {
        let renderer = Renderer::new(20.0, Theme::default());

        let color_0 = renderer.strength_to_color(0);
        let color_5 = renderer.strength_to_color(5);
        let color_9 = renderer.strength_to_color(9);

        // Colors should be different
        assert_ne!(color_0, color_5);
        assert_ne!(color_5, color_9);
    }
}
