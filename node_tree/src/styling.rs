use bevy::prelude::*;

use crate::NodeTree;

impl Styling for NodeTree {
    fn get_style(&self) -> Style {
        if let Some(style) = self.get::<Style>() {
            return style.clone();
        } else {
            return Style::default();
        }
    }

    fn set_style(&mut self, s: Style) {
        self.insert_bundle(s);
    }
}

pub trait Styling: Sized {
    fn get_style(&self) -> Style;
    fn set_style(&mut self, s: Style);

    fn with_display(mut self, display: Display) -> Self {
        let mut style = self.get_style();
        style.display = display;
        self.set_style(style);
        self
    }

    fn with_position_type(mut self, position_type: PositionType) -> Self {
        let mut style = self.get_style();
        style.position_type = position_type;
        self.set_style(style);
        self
    }

    fn with_overflow(mut self, overflow: Overflow) -> Self {
        let mut style = self.get_style();
        style.overflow = overflow;
        self.set_style(style);
        self
    }

    fn with_direction(mut self, direction: Direction) -> Self {
        let mut style = self.get_style();
        style.direction = direction;
        self.set_style(style);
        self
    }

    fn with_left(mut self, left: Val) -> Self {
        let mut style = self.get_style();
        style.left = left;
        self.set_style(style);
        self
    }

    fn with_right(mut self, right: Val) -> Self {
        let mut style = self.get_style();
        style.right = right;
        self.set_style(style);
        self
    }

    fn with_top(mut self, top: Val) -> Self {
        let mut style = self.get_style();
        style.top = top;
        self.set_style(style);
        self
    }

    fn with_bottom(mut self, bottom: Val) -> Self {
        let mut style = self.get_style();
        style.bottom = bottom;
        self.set_style(style);
        self
    }

    fn with_width(mut self, width: Val) -> Self {
        let mut style = self.get_style();
        style.width = width;
        self.set_style(style);
        self
    }

    fn with_height(mut self, height: Val) -> Self {
        let mut style = self.get_style();
        style.height = height;
        self.set_style(style);
        self
    }

    fn with_min_width(mut self, min_width: Val) -> Self {
        let mut style = self.get_style();
        style.min_width = min_width;
        self.set_style(style);
        self
    }

    fn with_min_height(mut self, min_height: Val) -> Self {
        let mut style = self.get_style();
        style.min_height = min_height;
        self.set_style(style);
        self
    }

    fn with_max_width(mut self, max_width: Val) -> Self {
        let mut style = self.get_style();
        style.max_width = max_width;
        self.set_style(style);
        self
    }

    fn with_max_height(mut self, max_height: Val) -> Self {
        let mut style = self.get_style();
        style.max_height = max_height;
        self.set_style(style);
        self
    }

    fn with_aspect_ratio(mut self, aspect_ratio: Option<f32>) -> Self {
        let mut style = self.get_style();
        style.aspect_ratio = aspect_ratio;
        self.set_style(style);
        self
    }

    fn with_align_items(mut self, align_items: AlignItems) -> Self {
        let mut style = self.get_style();
        style.align_items = align_items;
        self.set_style(style);
        self
    }

    fn with_justify_items(mut self, justify_items: JustifyItems) -> Self {
        let mut style = self.get_style();
        style.justify_items = justify_items;
        self.set_style(style);
        self
    }

    fn with_align_self(mut self, align_self: AlignSelf) -> Self {
        let mut style = self.get_style();
        style.align_self = align_self;
        self.set_style(style);
        self
    }

    fn with_justify_self(mut self, justify_self: JustifySelf) -> Self {
        let mut style = self.get_style();
        style.justify_self = justify_self;
        self.set_style(style);
        self
    }

    fn with_align_content(mut self, align_content: AlignContent) -> Self {
        let mut style = self.get_style();
        style.align_content = align_content;
        self.set_style(style);
        self
    }

    fn with_justify_content(mut self, justify_content: JustifyContent) -> Self {
        let mut style = self.get_style();
        style.justify_content = justify_content;
        self.set_style(style);
        self
    }

    fn with_margin(mut self, margin: UiRect) -> Self {
        let mut style = self.get_style();
        style.margin = margin;
        self.set_style(style);
        self
    }

    fn with_padding(mut self, padding: UiRect) -> Self {
        let mut style = self.get_style();
        style.padding = padding;
        self.set_style(style);
        self
    }

    fn with_border(mut self, border: UiRect) -> Self {
        let mut style = self.get_style();
        style.border = border;
        self.set_style(style);
        self
    }

    fn with_flex_direction(mut self, flex_direction: FlexDirection) -> Self {
        let mut style = self.get_style();
        style.flex_direction = flex_direction;
        self.set_style(style);
        self
    }

    fn with_flex_wrap(mut self, flex_wrap: FlexWrap) -> Self {
        let mut style = self.get_style();
        style.flex_wrap = flex_wrap;
        self.set_style(style);
        self
    }

    fn with_flex_grow(mut self, flex_grow: f32) -> Self {
        let mut style = self.get_style();
        style.flex_grow = flex_grow;
        self.set_style(style);
        self
    }

    fn with_flex_shrink(mut self, flex_shrink: f32) -> Self {
        let mut style = self.get_style();
        style.flex_shrink = flex_shrink;
        self.set_style(style);
        self
    }

    fn with_flex_basis(mut self, flex_basis: Val) -> Self {
        let mut style = self.get_style();
        style.flex_basis = flex_basis;
        self.set_style(style);
        self
    }

    fn with_row_gap(mut self, row_gap: Val) -> Self {
        let mut style = self.get_style();
        style.row_gap = row_gap;
        self.set_style(style);
        self
    }

    fn with_column_gap(mut self, column_gap: Val) -> Self {
        let mut style = self.get_style();
        style.column_gap = column_gap;
        self.set_style(style);
        self
    }

    fn with_grid_auto_flow(mut self, grid_auto_flow: GridAutoFlow) -> Self {
        let mut style = self.get_style();
        style.grid_auto_flow = grid_auto_flow;
        self.set_style(style);
        self
    }

    fn with_grid_template_rows(mut self, grid_template_rows: Vec<RepeatedGridTrack>) -> Self {
        let mut style = self.get_style();
        style.grid_template_rows = grid_template_rows;
        self.set_style(style);
        self
    }

    fn with_grid_template_columns(mut self, grid_template_columns: Vec<RepeatedGridTrack>) -> Self {
        let mut style = self.get_style();
        style.grid_template_columns = grid_template_columns;
        self.set_style(style);
        self
    }

    fn with_grid_auto_rows(mut self, grid_auto_rows: Vec<GridTrack>) -> Self {
        let mut style = self.get_style();
        style.grid_auto_rows = grid_auto_rows;
        self.set_style(style);
        self
    }

    fn with_grid_auto_columns(mut self, grid_auto_columns: Vec<GridTrack>) -> Self {
        let mut style = self.get_style();
        style.grid_auto_columns = grid_auto_columns;
        self.set_style(style);
        self
    }

    fn with_grid_row(mut self, grid_row: GridPlacement) -> Self {
        let mut style = self.get_style();
        style.grid_row = grid_row;
        self.set_style(style);
        self
    }

    fn with_grid_column(mut self, grid_column: GridPlacement) -> Self {
        let mut style = self.get_style();
        style.grid_column = grid_column;
        self.set_style(style);
        self
    }
}

impl NodeTree {
    pub fn with_background_color(mut self, color: Color) -> Self {
        self.insert_bundle(BackgroundColor(color));
        self
    }

    pub fn with_border_color(mut self, color: Color) -> Self {
        self.insert_bundle(BorderColor(color));
        self
    }
}
