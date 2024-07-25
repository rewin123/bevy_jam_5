use bevy::{ecs::world::unsafe_world_cell::UnsafeWorldCell, prelude::*};
use bevy_egui::*;
use node_tree::{tree::{IntoNodeTree, NodeTree}, InsertNodumEntity};

const BORDER_COLOR: &str = "#8080b3";
const OXYGEN_COLOR: &str = "#4a4a8c";
const WATER_COLOR: &str = "#4a8c4a";
const PEE_COLOR: &str = "#8c8c4a";
const BAD_WATER_COLOR: &str = "#8c4a4a";
const METAL_COLOR: &str = "#8c4a8c";
const METAL_WASTE_COLOR: &str = "#4a8c4a";
const CO2_COLOR: &str = "#8c4a4a";

pub(crate) fn plugin(app: &mut App) {

    app.add_systems(Startup, |mut cmds: Commands| {
        cmds.spawn(ResourcePanel);
    });

    app.add_systems(Update, draw_resource_panel);
}

#[derive(Component)]
pub struct ResourcePanel;

struct ResourcePanelStyle {
    text: TextStyle
}


fn draw_resource_panel(
    world: &mut World 
) {
    info!("draw resource panel");
    unsafe {
        let mut cell = world.as_unsafe_world_cell();
        
        let asset_server = cell
            .world()
            .resource::<AssetServer>();

        let style = ResourcePanelStyle {
            text: TextStyle { 
                font: asset_server.load("fonts/karma/Karma Suture.otf"),
                font_size: 20.0,
                ..Default::default()
            }
        };
        
        let mut tree = NodeTree::default()
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
                background_color: BackgroundColor(hex2color("#2a2a3a")),
                border_color: BorderColor(hex2color(BORDER_COLOR)),
                border_radius: BorderRadius::all(Val::Px(5.0)),
                ..default()
            })
            .with_child(oxygen_cycle(&mut cell, &style));


        let root = cell
            .world_mut()
            .query_filtered::<Entity, With<ResourcePanel>>()
            .iter(cell.world())
            .next()
            .unwrap();


        cell.world_mut().commands().add(InsertNodumEntity {
            entity: root,
            nodum: tree
        });
    }
}

fn hex2color(hex: &str) -> Color {
    let hex = hex.strip_prefix('#').unwrap();
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    Color::srgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
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
    }.into_node_tree()
    .with_child( // Top text
        NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }.into_node_tree()
        .with_child(TextBundle::from_section("-- Breath -->", style.text.clone()))
    )
    .with_child( //bars
        NodeBundle {
            style: Style {
                display: Display::Flex,
                justify_content: JustifyContent::SpaceBetween,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        }.into_node_tree()
        .with_child(bar(Bar {
            val: 0.5,
            name: "Oxygen",
            color: hex2color(OXYGEN_COLOR),
            text_style: style.text.clone()
        }))
        .with_child(bar(Bar {
            val: 0.5,
            name: "CO2",
            color: hex2color(CO2_COLOR),
            text_style: style.text.clone()
        }))
    )
    .with_child( //bottom text
        NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }.into_node_tree()
        .with_child(TextBundle::from_section("<-- Recycle --", style.text.clone()))
    )
}

struct Bar {
    val: f32,
    name: &'static str,
    color: Color,
    text_style: TextStyle
}

fn bar(val : Bar) -> NodeTree {
    NodeBundle {
        style: Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            height: Val::Percent(100.0),
            ..default()
        },
        ..default()
    }.into_node_tree() 
    .with_child( //bar
        NodeTree::default()
            .with_bundle(NodeBundle {
                style: Style {
                    width: Val::Px(30.0),
                    height: Val::Percent(100.0),
                    border: UiRect::all(Val::Px(1.0)),
                    position_type: PositionType::Relative,
                    ..default()
                },
                border_color: BorderColor(hex2color(BORDER_COLOR)),
                border_radius: BorderRadius::all(Val::Px(5.0)),
                ..default()
            })
            .with_child(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(val.val * 100.0),
                    top: Val::Percent(100.0 - val.val * 100.0),
                    ..default()
                },
                background_color: BackgroundColor(val.color),
                ..default()
            })
    ).with_child( // name
        TextBundle::from_section(val.name, val.text_style)
            .with_style(Style {
                height: Val::Px(10.0),
                align_self: AlignSelf::Center,
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            })
    )
}