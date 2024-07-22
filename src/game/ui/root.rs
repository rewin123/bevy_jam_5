use bevy::prelude::*;
use bevy_mod_stylebuilder::{StyleBuilder, StyleBuilderBackground, StyleBuilderLayout};
use bevy_quill::*;
use bevy_quill_obsidian::controls::Slider;

use crate::game::resources::{Oxygen, Water};

use super::{context_menu, SelectedItem};

#[derive(Clone, PartialEq)]
pub(super) struct RootUi {
    pub camera: Entity,
}

fn root_style(sb: &mut StyleBuilder) {
    // Use the full screen
    sb.left(0)
        .right(0)
        .top(0)
        .bottom(0)
        .height(Val::Percent(100.0));
}

const RESOURCE_MENU_WIDTH: f32 = 200.0;
const RESOURCE_MENU_PADDING: f32 = 20.0;

fn o_slider_style(sb: &mut StyleBuilder) {
    sb.width(RESOURCE_MENU_PADDING.mul_add(-2.0, RESOURCE_MENU_WIDTH))
        .height(30);
}

impl ViewTemplate for RootUi {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let selected_item = cx.use_resource::<SelectedItem>();
        let oxygen = cx.use_resource::<Oxygen>();
        let water = cx.use_resource::<Water>();
        let position = selected_item.item;

        Element::<NodeBundle>::new().style(root_style).children((
            Element::<NodeBundle>::new()
                .style(|sb: &mut StyleBuilder| {
                    sb.display(Display::Flex)
                        .flex_direction(FlexDirection::Column)
                        .top(0)
                        .left(0)
                        .right(0)
                        .padding(RESOURCE_MENU_PADDING)
                        .row_gap(15)
                        .height(Val::Percent(100.0))
                        .width(RESOURCE_MENU_WIDTH)
                        .background_color(Srgba::new(1.0, 1.0, 1.0, 0.3));
                })
                .children((
                    Slider::new()
                        .range(0. ..=oxygen.limit)
                        .disabled(true)
                        .label("Oxygen")
                        .style(o_slider_style)
                        .value(oxygen.amount),
                    Slider::new()
                        .range(0. ..=water.limit)
                        .disabled(true)
                        .label("Water")
                        .style(o_slider_style)
                        .value(water.amount),
                )),
            // If the position of the menu is `Some` we show the Context Menu
            // Other wise we show nothing
            Cond::new(position.is_some(), context_menu::ContextMenu, ()),
        ))
    }
}
