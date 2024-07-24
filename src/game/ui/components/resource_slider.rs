use bevy::{
    color::palettes::{self, css::GREY},
    prelude::{Children, NodeBundle},
    ui::{Display, FlexDirection},
};
use bevy_mod_stylebuilder::{
    StyleBuilder, StyleBuilderBackground, StyleBuilderBorderColor, StyleBuilderBorderRadius,
    StyleBuilderLayout, StyleHandle,
};
use bevy_quill::{Cx, Element, View, ViewTemplate};
use bevy_quill_obsidian::{
    colors::{self, X_RED, Y_GREEN},
    controls::Slider,
};

use crate::game::ui::{
    self,
    constants::{RESOURCE_MENU_PADDING, RESOURCE_MENU_WIDTH},
};

#[derive(Clone, PartialEq)]
pub(crate) struct ResourceSlider {
    pub limit: f32,
    pub amount: f32,
    pub label: String,
    pub style: StyleHandle,
}

impl Default for ResourceSlider {
    fn default() -> Self {
        Self {
            // Copied from Obsidian slider
            limit: 1.0,
            amount: 0.0,
            label: "Resource".to_string(),
            style: StyleHandle::default(),
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

    fn create(&self, cx: &mut Cx) -> Self::View {
        Element::<NodeBundle>::new()
            .style_dyn(
                |ct, ss| {
                    if ct < 20.0 {
                        ss.border(3).border_color(Y_GREEN).border_radius(8.0);
                    } else if ct > 80.0 {
                        ss.border(3).border_color(X_RED).border_radius(8.0);
                    } else {
                        ss.border(3).border_color(GREY).border_radius(8.0);
                    }
                },
                self.amount,
            )
            .children((Slider::new()
                .range(0. ..=self.limit)
                .disabled(true)
                .label(self.label.clone())
                .style((o_slider_style, self.style.clone()))
                .value(self.amount),))
    }
}
