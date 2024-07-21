use bevy::prelude::*;
use bevy_mod_stylebuilder::{StyleBuilder, StyleBuilderLayout};
use bevy_quill::*;
use bevy_quill_obsidian::controls::Slider;

use super::{context_menu, SelectedItem};

#[derive(Clone, PartialEq)]
pub(super) struct RootUi {
    pub camera: Entity,
}

fn root_style(sb: &mut StyleBuilder) {
    // Use the full screen
    sb.left(0).right(0).top(0).bottom(0);
}

fn slider_style(sb: &mut StyleBuilder) {
    sb.width(100);
}

impl ViewTemplate for RootUi {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let context = cx.use_resource::<SelectedItem>();
        let position = context.item;

        let slider_value = cx.create_mutable::<f32>(50.);

        Element::<NodeBundle>::new().style(root_style).children((
            Element::<NodeBundle>::new()
                .style(|sb: &mut StyleBuilder| {
                    sb.display(Display::Flex)
                        .top(0)
                        .left(0)
                        .right(0)
                        .width(100)
                        .padding_top(20)
                        .padding_left(20)
                        .gap(10);
                })
                .children((
                    Slider::new()
                        .range(0. ..=100.)
                        .disabled(true)
                        .label("Oxygen")
                        .style(slider_style)
                        .value(slider_value.get(cx)),
                    Slider::new()
                        .range(0. ..=100.)
                        .disabled(true)
                        .label("Food")
                        .style(slider_style)
                        .value(slider_value.get(cx)),
                )),
            // If the position of the menu is `Some` we show the Context Menu
            // Other wise we show nothing
            Cond::new(position.is_some(), context_menu::ContextMenu, ()),
        ))
    }
}
