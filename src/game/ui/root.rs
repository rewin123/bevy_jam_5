use bevy::prelude::*;
use bevy_mod_stylebuilder::{StyleBuilder, StyleBuilderBackground, StyleBuilderLayout};
use bevy_quill::*;

use crate::game::{
    daycycle::PlayerState,
    resources::{CarbonDioxide, Food, Oxygen, OxygenRecycling, Water},
};

use super::{
    components::{
        end_screen::{EndScreen, EndType},
        resource_slider::ResourceSlider,
    },
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
        let oxygen = cx.use_resource::<Oxygen>();
        let oxygen_recycling = cx.use_resource::<OxygenRecycling>();
        let co2 = cx.use_resource::<CarbonDioxide>();
        let food = cx.use_resource::<Food>();
        let water = cx.use_resource::<Water>();
        let position = selected_item.item;

        let oxygen_status = if oxygen_recycling.working {
            oxygen_recycling.oxygen_generation_rate - oxygen.consumption_rate
        } else {
            -oxygen.consumption_rate
        };
        let co2_status = if oxygen_recycling.working {
            -(oxygen_recycling.co2_consumption_rate - co2.generation_rate)
        } else {
            co2.generation_rate
        };

        let game_ended = *player_state != PlayerState::Alive;

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
                        .children((
                            ResourceSlider::new()
                                .limit(oxygen.limit)
                                .amount(oxygen.amount)
                                .label(format!("Oxygen ({:+})", oxygen_status)),
                            ResourceSlider::new()
                                .limit(co2.limit)
                                .amount(co2.amount)
                                .label(format!("CO2 ({:+})", co2_status)),
                            ResourceSlider::new()
                                .limit(water.limit)
                                .amount(water.amount)
                                .label("Water"),
                            ResourceSlider::new()
                                .limit(food.limit)
                                .amount(food.amount)
                                .label("Food"),
                        )),
                    // If the position of the menu is `Some` we show the Context Menu
                    // Other wise we show nothing
                    Cond::new(position.is_some(), context_menu::ContextMenu, ()),
                ),
            ))
    }
}
