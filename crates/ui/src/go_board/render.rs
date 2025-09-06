use crate::go_board::core::*;
use gpui::{
    div, hsla, img, point, px, rgb, svg, AnyElement, FontWeight, Hsla, IntoElement, ParentElement,
    Pixels, Point, Rgba, Styled, StyledImage,
};

// =============================================================================
// SIMPLIFIED SPACING
// =============================================================================

const MARKER_SIZE_RATIO: f32 = 0.6;
const CROSS_LINE_RATIO: f32 = 0.9;
const DOT_SIZE_RATIO: f32 = 0.6;
const GHOST_SIZE_RATIO: f32 = 0.9;
const HEAT_SIZE_RATIO: f32 = 0.9;
const SELECTION_SIZE_RATIO: f32 = 0.9;
const GREEN_HUE: f32 = 120.0 / 360.0;
const RED_HUE: f32 = 0.0;
const GHOST_SATURATION: f32 = 0.6;

#[derive(Debug, Clone)]
pub struct ResponsiveSpacing {
    pub coord_margin_padding: f32,
    pub cross_line_width: f32,
    pub min_coord_size: f32,
    pub heat_text_size: f32,
}

impl ResponsiveSpacing {
    pub fn for_vertex_size(vertex_size: f32) -> Self {
        Self {
            coord_margin_padding: (vertex_size * 0.2).max(4.0),
            cross_line_width: (vertex_size * 0.08).clamp(1.0, 3.0),
            min_coord_size: (vertex_size * 0.6).clamp(12.0, 24.0),
            heat_text_size: (vertex_size * 0.3).clamp(8.0, 16.0),
        }
    }
}

// =============================================================================
// RENDERER - Simplified unified renderer
// =============================================================================

pub struct Renderer {
    vertex_size: f32,
    theme: Theme,
    coord_offset: Point<Pixels>,
    spacing: ResponsiveSpacing,
}

impl Renderer {
    pub fn new(vertex_size: f32, theme: Theme) -> Self {
        let spacing = ResponsiveSpacing::for_vertex_size(vertex_size);
        Self {
            vertex_size,
            theme,
            coord_offset: point(px(0.0), px(0.0)),
            spacing,
        }
    }

    pub fn vertex_size(&self) -> f32 {
        self.vertex_size
    }

    pub fn with_coordinates(mut self, show: bool) -> Self {
        if show {
            let margin = self.coord_margin();
            self.coord_offset = point(px(margin), px(margin));
        }
        self
    }

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

    fn render_board_only(&self, data: &BoardData, width: f32, height: f32) -> impl IntoElement {
        let mut board = div()
            .relative()
            .w(px(width))
            .h(px(height))
            .bg(self.theme.background)
            .border_1()
            .border_color(self.theme.border)
            .overflow_hidden();

        // Add board background asset if available
        if let Some(ref background_path) = self.theme.board_background_path {
            board = board.child(
                img(background_path.as_str())
                    .absolute()
                    .inset_0()
                    .w_full()
                    .h_full()
                    .object_fit(gpui::ObjectFit::Cover),
            );
        }

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

    fn render_with_coordinates(
        &self,
        data: &BoardData,
        grid_width: f32,
        grid_height: f32,
    ) -> AnyElement {
        let margin = self.coord_margin();
        let total_width = grid_width + 2.0 * margin;
        let total_height = grid_height + 2.0 * margin;

        let mut container = div().relative().w(px(total_width)).h(px(total_height));

        container = container.child(self.render_coordinates(data, grid_width, grid_height, margin));
        container = container.child(
            div()
                .absolute()
                .left(px(margin))
                .top(px(margin))
                .child(self.render_board_only(data, grid_width, grid_height)),
        );

        container.into_any_element()
    }

    // =============================================================================
    // LAYER RENDERING - Simplified
    // =============================================================================

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
                let pixel_pos = self.pos_to_pixel_grid(pos, range);
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

    fn render_stones(&self, data: &BoardData) -> impl IntoElement {
        let mut stones = div().absolute().inset_0();
        let range = &data.range;

        for (&pos, &stone) in &data.stones {
            if range.contains(pos) && stone != EMPTY {
                let pixel_pos = self.pos_to_pixel_grid(pos, range);
                let stone_size = self.vertex_size * self.theme.stone_size;

                let stone_element = match stone {
                    BLACK => {
                        if let Some(ref asset_path) = self.theme.black_stone_path {
                            // Use asset for black stone with img component
                            img(asset_path.as_str())
                                .w(px(stone_size))
                                .h(px(stone_size))
                                .flex_none()
                                .object_fit(gpui::ObjectFit::Cover)
                                .into_any_element()
                        } else {
                            // Fallback to colored div
                            div()
                                .w(px(stone_size))
                                .h(px(stone_size))
                                .rounded_full()
                                .bg(self.theme.black_stone)
                                .into_any_element()
                        }
                    }
                    WHITE => {
                        if let Some(ref asset_path) = self.theme.white_stone_path {
                            // Use asset for white stone with img component
                            img(asset_path.as_str())
                                .w(px(stone_size))
                                .h(px(stone_size))
                                .flex_none()
                                .object_fit(gpui::ObjectFit::Cover)
                                .into_any_element()
                        } else {
                            // Fallback to colored div
                            div()
                                .w(px(stone_size))
                                .h(px(stone_size))
                                .rounded_full()
                                .bg(self.theme.white_stone)
                                .border_1()
                                .border_color(self.theme.grid_lines)
                                .into_any_element()
                        }
                    }
                    _ => continue,
                };

                stones = stones.child(
                    div()
                        .absolute()
                        .left(pixel_pos.x - px(stone_size / 2.0))
                        .top(pixel_pos.y - px(stone_size / 2.0))
                        .child(stone_element),
                );
            }
        }

        stones
    }

    fn render_markers(&self, data: &BoardData) -> impl IntoElement {
        let mut markers = div().absolute().inset_0();
        let range = &data.range;

        for (&pos, marker) in &data.markers {
            if range.contains(pos) {
                let pixel_pos = self.pos_to_pixel_grid(pos, range);
                let marker_size = self.vertex_size * MARKER_SIZE_RATIO;
                markers = markers.child(self.render_marker(marker, pixel_pos, marker_size));
            }
        }

        markers
    }

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
                .child(
                    div()
                        .w(px(size * CROSS_LINE_RATIO))
                        .h(px(self.spacing.cross_line_width))
                        .bg(*color),
                )
                .child(
                    div()
                        .absolute()
                        .w(px(self.spacing.cross_line_width))
                        .h(px(size * CROSS_LINE_RATIO))
                        .bg(*color),
                ),
            Marker::Triangle { color } => div()
                .absolute()
                .left(pos.x - px(size / 2.0))
                .top(pos.y - px(size / 2.0))
                .w(px(size))
                .h(px(size))
                .bg(*color),
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
                .left(pos.x - px(size * DOT_SIZE_RATIO / 2.0))
                .top(pos.y - px(size * DOT_SIZE_RATIO / 2.0))
                .w(px(size * DOT_SIZE_RATIO))
                .h(px(size * DOT_SIZE_RATIO))
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

    fn render_ghosts(&self, data: &BoardData) -> impl IntoElement {
        let mut ghosts = div().absolute().inset_0();
        let range = &data.range;

        for (&pos, ghost) in &data.ghosts {
            if range.contains(pos) && ghost.stone != EMPTY {
                let pixel_pos = self.pos_to_pixel_grid(pos, range);
                let stone_size = self.vertex_size * self.theme.stone_size * GHOST_SIZE_RATIO;

                let ghost_element = match ghost.stone {
                    BLACK => {
                        if let Some(ref asset_path) = self.theme.black_stone_path {
                            // Use asset for black ghost stone with tinting
                            let mut svg_element = svg()
                                .path(asset_path.clone())
                                .w(px(stone_size))
                                .h(px(stone_size));

                            // Apply tinting based on ghost kind
                            svg_element = if ghost.is_good {
                                svg_element.text_color(hsla(GREEN_HUE, GHOST_SATURATION, 0.5, 1.0))
                            } else if ghost.is_bad {
                                svg_element.text_color(hsla(RED_HUE, GHOST_SATURATION, 0.5, 1.0))
                            } else {
                                svg_element
                            };

                            svg_element.into_any_element()
                        } else {
                            // Fallback to colored div with tinting
                            let mut hsla: Hsla = self.theme.black_stone;
                            if ghost.is_good {
                                hsla.h = GREEN_HUE;
                                hsla.s = GHOST_SATURATION;
                            } else if ghost.is_bad {
                                hsla.h = RED_HUE;
                                hsla.s = GHOST_SATURATION;
                            }

                            div()
                                .w(px(stone_size))
                                .h(px(stone_size))
                                .rounded_full()
                                .bg(hsla)
                                .opacity(ghost.alpha)
                                .into_any_element()
                        }
                    }
                    WHITE => {
                        if let Some(ref asset_path) = self.theme.white_stone_path {
                            // Use asset for white ghost stone with tinting
                            let mut svg_element = svg()
                                .path(asset_path.clone())
                                .w(px(stone_size))
                                .h(px(stone_size));

                            // Apply tinting based on ghost kind
                            svg_element = if ghost.is_good {
                                svg_element.text_color(hsla(GREEN_HUE, GHOST_SATURATION, 0.5, 1.0))
                            } else if ghost.is_bad {
                                svg_element.text_color(hsla(RED_HUE, GHOST_SATURATION, 0.5, 1.0))
                            } else {
                                svg_element
                            };

                            svg_element.into_any_element()
                        } else {
                            // Fallback to colored div with tinting
                            let mut hsla: Hsla = self.theme.white_stone;
                            if ghost.is_good {
                                hsla.h = GREEN_HUE;
                                hsla.s = GHOST_SATURATION;
                            } else if ghost.is_bad {
                                hsla.h = RED_HUE;
                                hsla.s = GHOST_SATURATION;
                            }

                            div()
                                .w(px(stone_size))
                                .h(px(stone_size))
                                .rounded_full()
                                .bg(hsla)
                                .border_1()
                                .border_color(self.theme.grid_lines)
                                .opacity(ghost.alpha)
                                .into_any_element()
                        }
                    }
                    _ => continue,
                };

                ghosts = ghosts.child(
                    div()
                        .absolute()
                        .left(pixel_pos.x - px(stone_size / 2.0))
                        .top(pixel_pos.y - px(stone_size / 2.0))
                        .opacity(ghost.alpha)
                        .child(ghost_element),
                );
            }
        }

        ghosts
    }

    fn render_heat(&self, data: &BoardData) -> impl IntoElement {
        let mut heat = div().absolute().inset_0();
        let range = &data.range;

        for (&pos, heat_data) in &data.heat {
            if range.contains(pos) && heat_data.strength > 0 {
                let pixel_pos = self.pos_to_pixel_grid(pos, range);
                let heat_size = self.vertex_size * HEAT_SIZE_RATIO;
                let color = self.strength_to_color(heat_data.strength);
                let text_color = self.get_heat_text_color(heat_data.strength);
                let text_size = self.spacing.heat_text_size;

                let mut heat_element = div()
                    .absolute()
                    .left(pixel_pos.x - px(heat_size / 2.0))
                    .top(pixel_pos.y - px(heat_size / 2.0))
                    .w(px(heat_size))
                    .h(px(heat_size))
                    .rounded(px(heat_size / 2.0))
                    .bg(color)
                    .flex()
                    .items_center()
                    .justify_center();

                if let Some(ref label) = heat_data.label {
                    heat_element = heat_element.child(
                        div()
                            .text_size(px(text_size))
                            .text_color(text_color)
                            .font_weight(FontWeight::BOLD)
                            .child(label.clone()),
                    );
                }

                heat = heat.child(heat_element);
            }
        }

        heat
    }

    fn render_territory(&self, data: &BoardData) -> impl IntoElement {
        let mut territory = div().absolute().inset_0();
        let range = &data.range;

        for (&pos, territory_data) in &data.territory {
            if range.contains(pos) {
                let pixel_pos = self.pos_to_pixel_grid(pos, range);
                let territory_size = self.vertex_size;

                let color = match territory_data.owner {
                    BLACK => self.theme.black_stone,
                    WHITE => self.theme.white_stone,
                    _ => rgb(0x808080).into(),
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

    fn render_selections(&self, data: &BoardData) -> impl IntoElement {
        let mut selections = div().absolute().inset_0();
        let range = &data.range;

        for (&pos, selection) in &data.selections {
            if range.contains(pos) {
                let pixel_pos = self.pos_to_pixel_grid(pos, range);
                let selection_size = self.vertex_size * SELECTION_SIZE_RATIO;

                let element = if selection.is_last_move {
                    // Use a full-size container and center a smaller dot inside it
                    // so the last-move indicator appears exactly at the vertex center.
                    let dot_size = selection_size * 0.5;
                    div()
                        .w(px(selection_size))
                        .h(px(selection_size))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .w(px(dot_size))
                                .h(px(dot_size))
                                .rounded_full()
                                .bg(selection.color),
                        )
                } else if selection.alpha < 1.0 {
                    div()
                        .w(px(selection_size))
                        .h(px(selection_size))
                        .bg(rgb(0x000000))
                        .opacity(selection.alpha)
                } else {
                    div()
                        .w(px(selection_size))
                        .h(px(selection_size))
                        .rounded(px(selection_size * 0.1))
                        .border_2()
                        .border_color(selection.color)
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

    fn render_lines(&self, data: &BoardData) -> impl IntoElement {
        let mut lines = div().absolute().inset_0();
        let range = &data.range;

        for line in &data.lines {
            if range.contains(line.from) && range.contains(line.to) {
                let from_pixel = self.pos_to_pixel_grid(line.from, range);
                let to_pixel = self.pos_to_pixel_grid(line.to, range);

                let dx = to_pixel.x.0 - from_pixel.x.0;
                let dy = to_pixel.y.0 - from_pixel.y.0;
                let length = (dx * dx + dy * dy).sqrt();
                let angle = dy.atan2(dx);

                let (color, width) = (line.color, line.width);

                // Create a smooth line using multiple overlapping segments
                let num_segments = ((length / width) * 4.0).max(8.0) as usize;
                let segment_length = length / num_segments as f32;

                for i in 0..num_segments {
                    let segment_start = i as f32 * segment_length;
                    let segment_end = (i + 1) as f32 * segment_length;

                    // Calculate segment start and end positions
                    let start_x = from_pixel.x.0 + segment_start * angle.cos();
                    let start_y = from_pixel.y.0 + segment_start * angle.sin();
                    let end_x = from_pixel.x.0 + segment_end * angle.cos();
                    let end_y = from_pixel.y.0 + segment_end * angle.sin();

                    // Calculate segment dimensions
                    let segment_width =
                        ((end_x - start_x).powi(2) + (end_y - start_y).powi(2)).sqrt();

                    let segment_element = div()
                        .absolute()
                        .left(px(start_x - width / 2.0))
                        .top(px(start_y - width / 2.0))
                        .w(px(segment_width + width))
                        .h(px(width))
                        .bg(color);

                    lines = lines.child(segment_element);
                }

                // For arrows, add a proper arrowhead
                if line.is_arrow {
                    let arrowhead_size = width * 3.0;
                    let arrowhead_distance = width * 2.0;

                    // Calculate arrowhead base position (where the arrowhead starts)
                    let base_x = from_pixel.x.0 + (length - arrowhead_distance) * angle.cos();
                    let base_y = from_pixel.y.0 + (length - arrowhead_distance) * angle.sin();

                    // Calculate arrowhead tip position
                    let tip_x = from_pixel.x.0 + length * angle.cos();
                    let tip_y = from_pixel.y.0 + length * angle.sin();

                    // Create the arrowhead triangle using multiple horizontal segments
                    let num_triangle_segments = 16;

                    for j in 0..num_triangle_segments {
                        let segment_progress = j as f32 / num_triangle_segments as f32;

                        // Calculate the width of this segment (narrower towards the tip)
                        let segment_width = arrowhead_size * (1.0 - segment_progress);

                        // Calculate the position of this segment along the arrowhead
                        let segment_x = base_x + (tip_x - base_x) * segment_progress;
                        let segment_y = base_y + (tip_y - base_y) * segment_progress;

                        // Create the segment rectangle
                        let triangle_segment = div()
                            .absolute()
                            .left(px(segment_x - segment_width / 2.0))
                            .top(px(segment_y - width / 2.0))
                            .w(px(segment_width))
                            .h(px(width))
                            .bg(color);

                        lines = lines.child(triangle_segment);
                    }
                }
            }
        }

        lines
    }

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

            coords = coords.child(
                div()
                    .absolute()
                    .left(px(pixel_x - self.effective_coord_size() / 2.0))
                    .top(px(margin - self.effective_coord_size()))
                    .w(px(self.effective_coord_size()))
                    .h(px(self.effective_coord_size()))
                    .flex()
                    .items_end()
                    .justify_center()
                    .text_size(px(self.effective_coord_size()))
                    .text_color(self.theme.coordinates)
                    .child(label.clone()),
            );

            coords = coords.child(
                div()
                    .absolute()
                    .left(px(pixel_x - self.effective_coord_size() / 2.0))
                    .top(px(margin + grid_height))
                    .w(px(self.effective_coord_size()))
                    .h(px(self.effective_coord_size()))
                    .flex()
                    .items_start()
                    .justify_center()
                    .text_size(px(self.effective_coord_size()))
                    .text_color(self.theme.coordinates)
                    .child(label),
            );
        }

        // Left and right coordinates (numbers)
        for y in range.y.0..=range.y.1 {
            let relative_y = (y - range.y.0) as f32;
            let pixel_y = margin + relative_y * self.vertex_size + self.vertex_size / 2.0;
            let label = self.y_coordinate_label(y, data.size.1);

            coords = coords.child(
                div()
                    .absolute()
                    .left(px(0.0))
                    .top(px(pixel_y - self.effective_coord_size() / 2.0))
                    .w(px(self.effective_coord_size()))
                    .h(px(self.effective_coord_size()))
                    .flex()
                    .items_center()
                    .justify_end()
                    .text_size(px(self.effective_coord_size()))
                    .text_color(self.theme.coordinates)
                    .child(label.clone()),
            );

            coords = coords.child(
                div()
                    .absolute()
                    .left(px(margin + grid_width))
                    .top(px(pixel_y - self.effective_coord_size() / 2.0))
                    .w(px(self.effective_coord_size()))
                    .h(px(self.effective_coord_size()))
                    .flex()
                    .items_center()
                    .justify_start()
                    .text_size(px(self.effective_coord_size()))
                    .text_color(self.theme.coordinates)
                    .child(label),
            );
        }

        coords
    }

    // =============================================================================
    // UTILITY METHODS - Simplified
    // =============================================================================

    fn coord_margin(&self) -> f32 {
        self.effective_coord_size() + self.spacing.coord_margin_padding
    }

    fn effective_coord_size(&self) -> f32 {
        self.theme.coord_size.max(self.spacing.min_coord_size)
    }

    fn get_heat_text_color(&self, strength: u8) -> Hsla {
        let intensity = (strength as f32 / 9.0).min(1.0);
        if intensity >= 0.5 {
            gpui::white()
        } else {
            gpui::black()
        }
    }

    fn pos_to_pixel_grid(&self, pos: Pos, range: &Range) -> Point<Pixels> {
        let relative_x = (pos.x - range.x.0) as f32;
        let relative_y = (pos.y - range.y.0) as f32;

        Point::new(
            px(relative_x * self.vertex_size + self.vertex_size / 2.0),
            px(relative_y * self.vertex_size + self.vertex_size / 2.0),
        )
    }

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

    fn strength_to_color(&self, strength: u8) -> Rgba {
        let intensity = (strength as f32 / 9.0).min(1.0);
        let alpha = (0.4 + intensity * 0.5).min(0.9);

        if intensity <= 0.2 {
            hsla(240.0 / 360.0, 0.8, 0.7, alpha).into()
        } else if intensity <= 0.4 {
            let t = (intensity - 0.2) / 0.2;
            let hue = 240.0 - t * 60.0;
            hsla(hue / 360.0, 0.8, 0.6, alpha).into()
        } else if intensity <= 0.6 {
            let t = (intensity - 0.4) / 0.2;
            let hue = 180.0 - t * 60.0;
            hsla(hue / 360.0, 0.8, 0.5, alpha).into()
        } else if intensity <= 0.8 {
            let t = (intensity - 0.6) / 0.2;
            let hue = 120.0 - t * 60.0;
            hsla(hue / 360.0, 0.9, 0.5, alpha).into()
        } else {
            let t = (intensity - 0.8) / 0.2;
            let hue = 60.0 - t * 60.0;
            hsla(hue / 360.0, 1.0, 0.5, alpha).into()
        }
    }

    fn x_coordinate_label(&self, x: usize) -> String {
        if x >= 25 {
            return "?".to_string();
        }
        let letter = if x < 8 {
            (b'A' + x as u8) as char
        } else {
            (b'A' + x as u8 + 1) as char
        };
        letter.to_string()
    }

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
        assert_eq!(stars.len(), 9);
        assert!(stars.contains(&Pos::new(9, 9)));
    }

    #[test]
    fn test_pos_to_pixel() {
        let renderer = Renderer::new(20.0, Theme::default());
        let range = Range::new((0, 18), (0, 18));

        let pixel = renderer.pos_to_pixel_grid(Pos::new(0, 0), &range);
        // For pos (0,0): (0 - 0) * 20 + 20/2 = 0 + 10 = 10
        assert_eq!(pixel.x, px(10.0));
        assert_eq!(pixel.y, px(10.0));

        let pixel = renderer.pos_to_pixel_grid(Pos::new(9, 9), &range);
        // For pos (9,9): (9 - 0) * 20 + 20/2 = 180 + 10 = 190
        assert_eq!(pixel.x, px(190.0));
        assert_eq!(pixel.y, px(190.0));
    }

    #[test]
    fn test_coordinate_labels() {
        let renderer = Renderer::new(20.0, Theme::default());

        assert_eq!(renderer.x_coordinate_label(0), "A");
        assert_eq!(renderer.x_coordinate_label(7), "H");
        assert_eq!(renderer.x_coordinate_label(8), "J");
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

        assert_ne!(color_0, color_5);
        assert_ne!(color_5, color_9);

        let low_color: Hsla = renderer.strength_to_color(1).into();
        let high_color: Hsla = renderer.strength_to_color(9).into();

        assert!(low_color.h > high_color.h);
    }

    #[test]
    fn test_heat_text_color() {
        let renderer = Renderer::new(20.0, Theme::default());

        let low_text = renderer.get_heat_text_color(2);
        assert_eq!(low_text, gpui::black());

        let high_text = renderer.get_heat_text_color(8);
        assert_eq!(high_text, gpui::white());
    }

    // =============================================================================
    // RENDERING EDGE CASES TESTS - TDD Approach
    // =============================================================================

    #[test]
    fn test_empty_board_rendering() {
        let renderer = Renderer::new(24.0, Theme::default());
        let empty_board = BoardData::new(19, 19);

        // Empty board should render without errors
        let rendered = renderer.render(&empty_board, false);
        let _element = rendered.into_any_element(); // Just verify it compiles

        // Should have no stones, markers, etc.
        assert_eq!(empty_board.stones.len(), 0);
        assert_eq!(empty_board.markers.len(), 0);
        assert_eq!(empty_board.ghosts.len(), 0);
        assert_eq!(empty_board.selections.len(), 0);
        assert_eq!(empty_board.lines.len(), 0);
    }

    #[test]
    fn test_large_board_rendering() {
        let renderer = Renderer::new(12.0, Theme::default());
        let large_board = BoardData::new(25, 25);

        // Large board should render without errors
        let rendered = renderer.render(&large_board, false);
        let _element = rendered.into_any_element(); // Just verify it compiles

        // Should have correct dimensions
        assert_eq!(large_board.size, (25, 25));
        assert_eq!(large_board.range.width(), 25);
        assert_eq!(large_board.range.height(), 25);
    }

    #[test]
    fn test_coordinate_overflow_handling() {
        let renderer = Renderer::new(20.0, Theme::default());

        // Test coordinate labels for very large boards
        assert_eq!(renderer.x_coordinate_label(25), "?");
        assert_eq!(renderer.x_coordinate_label(26), "?");

        // Test coordinate labels for standard positions
        assert_eq!(renderer.x_coordinate_label(0), "A");
        assert_eq!(renderer.x_coordinate_label(7), "H");
        assert_eq!(renderer.x_coordinate_label(8), "J");
        assert_eq!(renderer.x_coordinate_label(17), "S");
        assert_eq!(renderer.x_coordinate_label(24), "Z");
    }

    #[test]
    fn test_star_point_calculation_edge_cases() {
        let renderer = Renderer::new(20.0, Theme::default());

        // Test very small boards
        let tiny_board = BoardData::new(3, 3);
        let tiny_stars = renderer.calculate_star_points(&tiny_board);
        assert_eq!(tiny_stars.len(), 0); // Too small for star points

        // Test rectangular boards
        let rect_board = BoardData::new(19, 13);
        let rect_stars = renderer.calculate_star_points(&rect_board);
        assert!(rect_stars.len() > 0);

        // Test even-sized boards (20x20)
        let even_board = BoardData::new(20, 20);
        let even_stars = renderer.calculate_star_points(&even_board);
        assert!(even_stars.len() > 0);
        // For 20x20 board, offset is 3, so star points should be at (3,3) and (16,16)
        assert!(even_stars.contains(&Pos::new(3, 3)));
        assert!(even_stars.contains(&Pos::new(16, 16)));
    }

    #[test]
    fn test_responsive_spacing_calculation() {
        let small_spacing = ResponsiveSpacing::for_vertex_size(15.0);
        let medium_spacing = ResponsiveSpacing::for_vertex_size(30.0);
        let large_spacing = ResponsiveSpacing::for_vertex_size(60.0);

        // Small vertex size should have reduced spacing
        assert!(small_spacing.coord_margin_padding < medium_spacing.coord_margin_padding);
        assert!(small_spacing.min_coord_size < medium_spacing.min_coord_size);

        // Large vertex size should have increased spacing
        assert!(large_spacing.coord_margin_padding > medium_spacing.coord_margin_padding);
        assert!(large_spacing.min_coord_size > medium_spacing.min_coord_size);

        // All spacing should have minimum values
        assert!(small_spacing.coord_margin_padding >= 4.0);
        assert!(small_spacing.min_coord_size >= 12.0);
        assert!(small_spacing.heat_text_size >= 8.0);
    }

    #[test]
    fn test_theme_integration() {
        let dark_theme = Theme::dark();
        let renderer = Renderer::new(24.0, dark_theme.clone());
        let board = BoardData::new(19, 19);

        // Dark theme should render without errors
        let rendered = renderer.render(&board, false);
        let _element = rendered.into_any_element(); // Just verify it compiles

        // Theme colors should be applied
        assert_eq!(renderer.theme.background, dark_theme.background);
        assert_eq!(renderer.theme.border, dark_theme.border);
        assert_eq!(renderer.theme.grid_lines, dark_theme.grid_lines);
    }

    #[test]
    fn test_coordinate_rendering_edge_cases() {
        let _renderer = Renderer::new(20.0, Theme::default());
        let board = BoardData::new(19, 19);

        // Test coordinate rendering with very small vertex size
        let small_renderer = Renderer::new(8.0, Theme::default());
        let rendered_with_coords = small_renderer.render(&board, true);
        let _element_with_coords = rendered_with_coords.into_any_element(); // Just verify it compiles

        // Test coordinate rendering with very large vertex size
        let large_renderer = Renderer::new(50.0, Theme::default());
        let rendered_large = large_renderer.render(&board, true);
        let _element_large = rendered_large.into_any_element(); // Just verify it compiles
    }

    #[test]
    fn test_marker_rendering_edge_cases() {
        let renderer = Renderer::new(20.0, Theme::default());
        let mut board = BoardData::new(9, 9);

        // Test all marker types render without errors
        board.set_marker(Pos::new(4, 4), Some(Marker::circle()));
        board.set_marker(Pos::new(3, 3), Some(Marker::cross()));
        board.set_marker(Pos::new(5, 5), Some(Marker::triangle()));
        board.set_marker(Pos::new(2, 2), Some(Marker::square()));
        board.set_marker(Pos::new(6, 6), Some(Marker::dot()));
        board.set_marker(Pos::new(1, 1), Some(Marker::label("Test")));

        let rendered = renderer.render(&board, false);
        let _element = rendered.into_any_element(); // Just verify it compiles

        // All markers should be present
        assert_eq!(board.markers.len(), 6);
    }
}
