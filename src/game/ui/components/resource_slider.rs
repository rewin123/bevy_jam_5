use bevy::{color::palettes::css::GREY, prelude::NodeBundle};
use bevy_mod_stylebuilder::{
    StyleBuilder, StyleBuilderBorderColor, StyleBuilderBorderRadius, StyleBuilderLayout,
    StyleHandle,
};
use bevy_quill::{Cx, Element, View, ViewTemplate};
use bevy_quill_obsidian::{
    colors::{X_RED, Y_GREEN},
    controls::Slider,
};

use crate::game::ui::constants::{RESOURCE_MENU_PADDING, RESOURCE_MENU_WIDTH};

#[derive(Clone, PartialEq)]
pub(crate) struct ResourceSlider {
    pub limit: f32,
    pub amount: f32,
    pub label: String,
    pub style: StyleHandle,
    pub upper_threshold_warning: f32,
    pub lower_threshold_warning: f32,
    pub inverse_warning: bool,
}

impl Default for ResourceSlider {
    fn default() -> Self {
        Self {
            limit: 1.0,
            amount: 0.0,
            label: "Resource".to_string(),
            style: StyleHandle::default(),
            upper_threshold_warning: 80.0,
            lower_threshold_warning: 20.0,
            inverse_warning: false,
        }
    }
}

fn o_slider_style(sb: &mut StyleBuilder) {
    sb.width(RESOURCE_MENU_PADDING.mul_add(-2.0, RESOURCE_MENU_WIDTH))
        .height(30);
}

impl ResourceSlider {
    // pub fn new() -> Self {
    //     Self::default()
    // }
    // pub const fn limit(mut self, l: f32) -> Self {
    //     self.limit = l;
    //     self
    // }
    // pub const fn amount(mut self, a: f32) -> Self {
    //     self.amount = a;
    //     self
    // }
    // pub fn label(mut self, l: impl Into<String>) -> Self {
    //     self.label = l.into();
    //     self
    // }

    // Left for future use
    #[allow(dead_code)]
    pub fn style(mut self, s: StyleHandle) -> Self {
        self.style = s;
        self
    }
}

impl ViewTemplate for ResourceSlider {
    type View = impl View;

    fn create(&self, _cx: &mut Cx) -> Self::View {
        Element::<NodeBundle>::new()
            .style_dyn(
                |(amount, lower_threshold, upper_threshold, inverse_warning), ss| {
                    let (upper_limit, lower_limit) = if inverse_warning {
                        (lower_threshold, upper_threshold)
                    } else {
                        (upper_threshold, lower_threshold)
                    };

                    if amount > upper_limit {
                        ss.border(3).border_color(Y_GREEN).border_radius(8.0);
                    } else if amount < lower_limit {
                        ss.border(3).border_color(X_RED).border_radius(8.0);
                    } else {
                        ss.border(3).border_color(GREY).border_radius(8.0);
                    }
                },
                (
                    self.amount,
                    self.lower_threshold_warning,
                    self.upper_threshold_warning,
                    self.inverse_warning,
                ),
            )
            .children((Slider::new()
                .range(0. ..=self.limit)
                .disabled(true)
                .label(self.label.clone())
                .style((o_slider_style, self.style.clone()))
                .value(self.amount),))
    }
}
