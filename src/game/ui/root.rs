use bevy::prelude::*;
use bevy_mod_stylebuilder::{StyleBuilder, StyleBuilderBackground, StyleBuilderLayout};
use bevy_quill::*;

use crate::game::{daycycle::PlayerState, resources::AllResourcesGetter};

use super::{
    components::end_screen::{EndScreen, EndType},
    constants::{RESOURCE_MENU_PADDING, RESOURCE_MENU_WIDTH},
    context_menu, SelectedItem,
};

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
        let player_state = cx.use_resource::<State<PlayerState>>();
        let selected_item = cx.use_resource::<SelectedItem>();

        let position = selected_item.item;

        let game_ended = *player_state != PlayerState::Alive;

        let getters = cx.use_resource::<AllResourcesGetter>();
        let mut sliders = vec![];
        for i in 0..getters.res_plugin.len() {
            sliders.push(getters.res_plugin[i](cx));
        }

        Element::<NodeBundle>::new()
            .style(root_style)
            .style_dyn(
                |ended, sb| {
                    if ended {
                        sb.width(Val::Percent(100.0));
                    } else {
                        sb.width(Val::Auto);
                    }
                },
                game_ended,
            )
            .children(Cond::new(
                game_ended,
                EndScreen::new()
                    .end_type(if *player_state == PlayerState::Dead {
                        EndType::Lose
                    } else {
                        EndType::Win
                    })
                    .text(if *player_state == PlayerState::Dead {
                        "You died. Capitalism Won"
                    } else {
                        "You won. Now you live in space debt free. Alone. Cold. But with Money"
                    }),
                (
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
                        .children(
                            sliders
                                .into_iter()
                                .map(|slider| slider.into_view_child())
                                .collect::<Vec<_>>(),
                        ),
                    // If the position of the menu is `Some` we show the Context Menu
                    // Other wise we show nothing
                    Cond::new(position.is_some(), context_menu::ContextMenu, ()),
                ),
            ))
    }
}
