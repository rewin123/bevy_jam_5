use bevy::{prelude::*, ui};
use bevy_mod_stylebuilder::*;
use bevy_quill::prelude::*;
use bevy_quill_obsidian::{colors, controls::Button};

use super::{computer_menu, SelectedItem};

#[derive(Clone, PartialEq)]
pub(super) struct ContextMenu;

fn style_row(ss: &mut StyleBuilder) {
    ss.display(Display::Flex)
        .flex_direction(FlexDirection::Row)
        .align_items(ui::AlignItems::Center)
        .column_gap(4);
}
impl ViewTemplate for ContextMenu {
    type View = impl View;

    fn create(&self, cx: &mut bevy_quill::Cx) -> Self::View {
        let context = cx.use_resource::<SelectedItem>();
        let position = context.position;

        Element::<NodeBundle>::new()
            .style_dyn(
                |position, style_builder| {
                    style_builder
                        .flex_direction(FlexDirection::Column)
                        .position(ui::PositionType::Absolute)
                        .padding(3)
                        // Use the position of the context menu to position the menu
                        .top(position.1.y)
                        .left(position.1.x)
                        .width(100)
                        .height(100)
                        .row_gap(4)
                        .background_color(colors::U2);
                },
                position.unwrap(),
            )
            .children((computer_menu::ComputerMenu))
    }
}
