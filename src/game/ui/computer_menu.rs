use bevy::{prelude::*, ui};
use bevy_mod_stylebuilder::*;
use bevy_quill::prelude::*;

use super::{components::action_button::ActionButton, StartWorking};

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
                .children((ActionButton::new()
                    .on_click(cx.create_callback(|mut events: EventWriter<StartWorking>| {
                        events.send(StartWorking);
                        info!("Clicked Work");
                    }))
                    .label("Work"),)),
        )
    }
}
