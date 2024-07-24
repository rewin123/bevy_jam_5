use bevy::{
    color::palettes,
    prelude::{Children, NodeBundle},
};
use bevy_mod_stylebuilder::{
    StyleBuilder, StyleBuilderBorderColor, StyleBuilderLayout, StyleHandle,
};
use bevy_quill::{Element, View, ViewTemplate};
use bevy_quill_obsidian::controls::Slider;

use crate::game::ui::constants::{RESOURCE_MENU_PADDING, RESOURCE_MENU_WIDTH};

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

    fn create(&self, _: &mut bevy_quill::Cx) -> Self::View {
        let a = false;
        Element::<NodeBundle>::new()
            .style_dyn(
                |ct, ss| {
                    if ct {
                        ss.border_color(palettes::css::RED).border(3);
                    } else {
                        ss.border_color(palettes::css::LIME).border(3);
                    }
                },
                a,
            )
            .children("Style")

        // Slider::new()
        //     .range(0. ..=self.limit)
        //     .disabled(true)
        //     .label(self.label.clone())
        //     .style((o_slider_style, self.style.clone()))
        //     .value(self.amount)
    }
}
