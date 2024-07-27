use bevy::prelude::*;
use bevy_mod_stylebuilder::{StyleBuilder, StyleBuilderLayout};
use bevy_quill::*;

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

impl ViewTemplate for RootUi {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let selected_item = cx.use_resource::<SelectedItem>();

        let position = selected_item.item;

        Element::<NodeBundle>::new()
            .style(root_style)
            .children((Cond::new(position.is_some(), context_menu::ContextMenu, ()),))
    }
}
