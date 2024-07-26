use bevy::{ecs::world::unsafe_world_cell::UnsafeWorldCell, prelude::*};
use node_tree::{
    tree::{IntoNodeTree, NodeTree},
    InsertNodumEntity,
};

use crate::game::resources::*;
use super::*;

const OXYGEN_COLOR: &str = "#4a4a8c";
const WATER_COLOR: &str = "#4a8c4a";
const PEE_COLOR: &str = "#8c8c4a";
const BAD_WATER_COLOR: &str = "#8c4a4a";
const METAL_COLOR: &str = "#8c4a8c";
const METAL_WASTE_COLOR: &str = "#4a8c4a";
const CO2_COLOR: &str = "#8c4a4a";
const THIRST_COLOR: &str = "#4a8ccc";

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Startup, |mut cmds: Commands| {
        cmds.spawn(ResourcePanel);
    });

    app.add_systems(Update, draw_resource_panel);
}

#[derive(Component)]
pub struct ResourcePanel;

struct ResourcePanelStyle {
    text: TextStyle,
}

fn draw_resource_panel(world: &mut World) {
    // info!("draw resource panel");
    unsafe {
        let mut cell = world.as_unsafe_world_cell();

        let asset_server = cell.world().resource::<AssetServer>();

        let style = ResourcePanelStyle {
            text: TextStyle {
                font: asset_server.load(FONT_PATH),
                font_size: 20.0,
                ..Default::default()
            },
        };

        let tree = NodeTree::default()
            .with_bundle(NodeBundle {
                style: Style {
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    position_type: PositionType::Absolute,
                    width: Val::Percent(20.0),
                    height: Val::Percent(100.0),
                    border: UiRect::all(Val::Px(1.0)),
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: BackgroundColor(hex2color(BACKGROUND_COLOR)),
                border_color: BorderColor(hex2color(BORDER_COLOR)),
                border_radius: BorderRadius::all(Val::Px(5.0)),
                ..default()
            })
            .with_child(oxygen_cycle(&mut cell, &style))
            .with_child(water_cycle(&mut cell, &style))
            .with_child(other_resources(&mut cell, &style));

        let root = cell
            .world_mut()
            .query_filtered::<Entity, With<ResourcePanel>>()
            .iter(cell.world())
            .next()
            .unwrap();

        cell.world_mut().commands().add(InsertNodumEntity {
            entity: root,
            nodum: tree,
        });
    }
}



unsafe fn oxygen_cycle(cell: &mut UnsafeWorldCell, style: &ResourcePanelStyle) -> NodeTree {
    NodeBundle {
        style: Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            height: Val::Percent(30.0),
            width: Val::Percent(100.0),
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        ..default()
    }
    .into_node_tree()
    .with_child(
        // Top text
        NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,

                ..default()
            },
            ..default()
        }
        .into_node_tree()
        .with_child(TextBundle::from_section("Oxygen Cycle", style.text.clone()))
        .with_child(TextBundle::from_section("--Breath-->", style.text.clone())),
    )
    .with_child(
        //bars
        NodeBundle {
            style: Style {
                display: Display::Flex,
                justify_content: JustifyContent::SpaceAround,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        }
        .into_node_tree()
        .with_child(bar::<Oxygen>(
            cell,
            ResourceBar {
                name: "Oxygen",
                color: hex2color(OXYGEN_COLOR),
                text_style: style.text.clone(),
            },
        ))
        .with_child(bar::<CarbonDioxide>(
            cell,
            ResourceBar {
                name: "CO2",
                color: hex2color(CO2_COLOR),
                text_style: style.text.clone(),
            },
        )),
    )
    .with_child(
        //bottom text
        NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }
        .into_node_tree()
        .with_child(TextBundle::from_section("<--Recycle--", style.text.clone())),
    )
}

unsafe fn water_cycle(cell: &mut UnsafeWorldCell, style: &ResourcePanelStyle) -> NodeTree {
    NodeBundle {
        style: Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            height: Val::Percent(35.0),
            width: Val::Percent(100.0),
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        ..default()
    }
    .into_node_tree()
    .with_child(
        TextBundle::from_section("Water Cycle", style.text.clone()).with_style(Style {
            align_self: AlignSelf::Center,
            ..default()
        }),
    )
    .with_child(
        TextBundle::from_section("--Hydroponic-->", style.text.clone()).with_style(Style {
            align_self: AlignSelf::Center,
            ..default()
        }),
    )
    .with_child(
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(20.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceAround,
                ..default()
            },
            ..default()
        }
        .into_node_tree()
        .with_child(TextBundle::from_section("-Drink->", style.text.clone()))
        .with_child(TextBundle::from_section("-Toilet->", style.text.clone())),
    )
    .with_child(
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        }
        .into_node_tree()
        .with_child(bar::<Water>(
            cell,
            ResourceBar {
                name: "Water",
                color: hex2color(WATER_COLOR),
                text_style: style.text.clone(),
            },
        ))
        .with_child(bar::<Pee>(
            cell,
            ResourceBar {
                name: "Pee",
                color: hex2color(PEE_COLOR),
                text_style: style.text.clone(),
            },
        ))
        .with_child(bar::<BadWater>(
            cell,
            ResourceBar {
                name: "Bad Water",
                color: hex2color(BAD_WATER_COLOR),
                text_style: style.text.clone(),
            },
        )),
    )
    .with_child(
        TextBundle::from_section("<-- Recycle --", style.text.clone()).with_style(Style {
            align_self: AlignSelf::Center,
            ..default()
        }),
    )
}

unsafe fn other_resources(cell: &mut UnsafeWorldCell, style: &ResourcePanelStyle) -> NodeTree {
    NodeBundle {
        style: Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            height: Val::Percent(30.0),
            width: Val::Percent(100.0),
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        ..default()
    }
    .into_node_tree()
    .with_child(
        TextBundle::from_section("Other Resources", style.text.clone()).with_style(Style {
            align_self: AlignSelf::Center,
            margin: UiRect::bottom(Val::Px(10.0)),
            ..default()
        }),
    )
    .with_child(
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        }
        .into_node_tree()
        .with_child(bar::<Metal>(
            cell,
            ResourceBar {
                name: "Metal",
                color: hex2color(METAL_COLOR),
                text_style: style.text.clone(),
            },
        ))
        .with_child(bar::<MetalTrash>(
            cell,
            ResourceBar {
                name: "Metal Trash",
                color: hex2color(METAL_WASTE_COLOR),
                text_style: style.text.clone(),
            },
        ))
        .with_child(bar::<Thirst>(
            cell,
            ResourceBar {
                name: "Thirst",
                color: hex2color(THIRST_COLOR),
                text_style: style.text.clone(),
            },
        )),
    )
}

/// Resource bar <-----------------------------------------------------------------------
struct ResourceBar {
    name: &'static str,
    color: Color,
    text_style: TextStyle,
}

unsafe fn bar<T: GameResource>(cell: &UnsafeWorldCell, bar: ResourceBar) -> NodeTree {
    let val = cell.world().resource::<T>();
    let info = cell.world().resource::<GameResInfo<T>>();

    let rate = info.generation_rate;

    let limit = val.limit().unwrap_or(100.0);
    let lvl = val.amount() / limit;
    let lvl = lvl.max(0.0).min(1.0);

    let mut bar_tree = NodeTree::default()
        .with_bundle(NodeBundle {
            style: Style {
                width: Val::Px(30.0),
                height: Val::Percent(100.0),
                border: UiRect::all(Val::Px(1.0)),
                position_type: PositionType::Relative,
                align_self: AlignSelf::Center,
                ..default()
            },
            border_color: BorderColor(hex2color(BORDER_COLOR)),
            border_radius: BorderRadius::all(Val::Px(5.0)),
            ..default()
        })
        .with_child(NodeBundle {
            style: Style {
                width: Val::Px(30.0),
                height: Val::Percent(lvl * 100.0),
                top: Val::Percent(lvl.mul_add(-100.0, 100.0)),
                ..default()
            },
            background_color: BackgroundColor(bar.color),
            ..default()
        });

    //show rate
    let rate_color = if rate > 0.0 {
        bar.color.lighter(0.1)
    } else {
        bar.color.darker(0.1)
    };

    let dp = if rate > 0.0 {
        -rate / limit * 100.0
    } else {
        rate / limit * 100.0
    };

    bar_tree = bar_tree.with_child(NodeBundle {
        style: Style {
            width: Val::Px(27.0),
            height: Val::Percent((100.0 * rate / limit).abs()),
            top: Val::Percent(lvl.mul_add(-100.0, 100.0) + dp),
            left: Val::Px(1.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        background_color: BackgroundColor(rate_color),
        ..default()
    });

    // show limits
    let (min_warn, max_warn) = val.warning_thresholds();

    if let Some(min_warn) = min_warn {
        bar_tree = bar_tree.with_child(NodeBundle {
            style: Style {
                width: Val::Px(28.0),
                height: Val::Px(2.0),
                top: Val::Percent((min_warn / limit).mul_add(-100.0, 100.0)),
                left: Val::Px(1.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: BackgroundColor(Color::srgb(0.5, 0.5, 0.0)),
            ..default()
        });
    }

    if let Some(max_warn) = max_warn {
        bar_tree = bar_tree.with_child(NodeBundle {
            style: Style {
                width: Val::Px(28.0),
                height: Val::Px(2.0),
                top: Val::Percent((max_warn / limit).mul_add(-100.0, 100.0)),
                left: Val::Px(1.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: BackgroundColor(Color::srgb(0.5, 0.5, 0.0)),
            ..default()
        });
    }

    NodeBundle {
        style: Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            height: Val::Percent(100.0),
            ..default()
        },
        ..default()
    }
    .into_node_tree()
    .with_child(
        //bar
        bar_tree,
    )
    .with_child(
        // name
        TextBundle::from_section(bar.name, bar.text_style).with_style(Style {
            height: Val::Px(10.0),
            align_self: AlignSelf::Center,
            margin: UiRect::all(Val::Px(2.0)),
            ..default()
        }),
    )
}
