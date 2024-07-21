use bevy::{prelude::*, ui};
use bevy_mod_stylebuilder::*;
use bevy_quill::prelude::*;
use bevy_quill_obsidian::controls::Button;

#[derive(Clone, PartialEq)]
pub(super) struct ComputerMenu;

fn style_row(ss: &mut StyleBuilder) {
    ss.display(Display::Flex)
        .flex_direction(FlexDirection::Row)
        .align_items(ui::AlignItems::Center)
        .column_gap(4);
}
impl ViewTemplate for ComputerMenu {
    type View = impl View;

    fn create(&self, cx: &mut bevy_quill::Cx) -> Self::View {
        (
            "Actions",
            Element::<NodeBundle>::new()
                .style(style_row)
                .children((Button::new()
                    .on_click(cx.create_callback(|| info!("clicked Recycle")))
                    .children("Recycle"),)),
        )
    }
}
