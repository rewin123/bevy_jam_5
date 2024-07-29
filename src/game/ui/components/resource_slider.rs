// use bevy::{
//     color::{palettes::css::GREY, Srgba},
//     prelude::NodeBundle,
// };
// use bevy_mod_stylebuilder::{
//     StyleBuilder, StyleBuilderBorderColor, StyleBuilderBorderRadius, StyleBuilderLayout,
//     StyleHandle,
// };
// use bevy_quill::{Cx, Element, View, ViewTemplate};
// use bevy_quill_obsidian::{
//     colors::{X_RED, Y_GREEN},
//     controls::Slider,
// };

// use crate::game::{
//     resources::ResourceThreshold,
//     ui::constants::{RESOURCE_MENU_PADDING, RESOURCE_MENU_WIDTH},
// };

// #[derive(Clone, PartialEq)]
// pub(crate) struct ResourceSlider {
//     pub limit: f32,
//     pub amount: f32,
//     pub label: String,
//     pub style: StyleHandle,
//     pub min_threshold: Option<f32>,
//     pub max_threshold: Option<f32>,
//     pub resource_threshold: ResourceThreshold,
// }

// impl Default for ResourceSlider {
//     fn default() -> Self {
//         Self {
//             limit: 1.0,
//             amount: 0.0,
//             label: "Resource".to_string(),
//             style: StyleHandle::default(),
//             min_threshold: None,
//             max_threshold: None,
//             resource_threshold: ResourceThreshold::Limitless,
//         }
//     }
// }

// fn o_slider_style(sb: &mut StyleBuilder) {
//     sb.width(RESOURCE_MENU_PADDING.mul_add(-2.0, RESOURCE_MENU_WIDTH))
//         .height(30);
// }

// impl ResourceSlider {
//     // pub fn new() -> Self {
//     //     Self::default()
//     // }
//     // pub const fn limit(mut self, l: f32) -> Self {
//     //     self.limit = l;
//     //     self
//     // }
//     // pub const fn amount(mut self, a: f32) -> Self {
//     //     self.amount = a;
//     //     self
//     // }
//     // pub fn label(mut self, l: impl Into<String>) -> Self {
//     //     self.label = l.into();
//     //     self
//     // }

//     // Left for future use
//     #[allow(dead_code)]
//     pub fn style(mut self, s: StyleHandle) -> Self {
//         self.style = s;
//         self
//     }
// }

// impl ViewTemplate for ResourceSlider {
//     type View = impl View;

//     fn create(&self, _cx: &mut Cx) -> Self::View {
//         Element::<NodeBundle>::new()
//             .style_dyn(
//                 |(amount, lower_threshold, upper_threshold, threshold_type), ss| {
//                     let border_color: Srgba =
//                         match (lower_threshold, upper_threshold, threshold_type) {
//                             (_, _, ResourceThreshold::Limitless) => GREY,
//                             (_, Some(max), ResourceThreshold::HealthyRange) if max > amount => {
//                                 X_RED
//                             }
//                             (Some(min), _, ResourceThreshold::HealthyRange) if min < amount => {
//                                 X_RED
//                             }
//                             (_, Some(max), ResourceThreshold::HealthyRange) if max < amount => {
//                                 Y_GREEN
//                             }
//                             (Some(min), _, ResourceThreshold::HealthyRange) if min > amount => {
//                                 Y_GREEN
//                             }
//                             (Some(min), _, ResourceThreshold::Necessity) if min > amount => Y_GREEN,
//                             (Some(min), _, ResourceThreshold::Necessity) if min < amount => X_RED,
//                             (_, Some(max), ResourceThreshold::Waste) if max > amount => Y_GREEN,
//                             (_, Some(max), ResourceThreshold::Waste) if max < amount => X_RED,
//                             _ => GREY,
//                         };

//                     ss.border(3).border_color(border_color).border_radius(8.0);
//                 },
//                 (
//                     self.amount,
//                     self.max_threshold,
//                     self.min_threshold,
//                     self.resource_threshold,
//                 ),
//             )
//             .children((Slider::new()
//                 .range(0. ..=self.limit)
//                 .disabled(true)
//                 .label(self.label.clone())
//                 .style((o_slider_style, self.style.clone()))
//                 .value(self.amount),))
//     }
// }
