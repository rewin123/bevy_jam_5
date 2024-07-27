//! Spawn the main level by triggering other observers.

use std::f32::consts::PI;

use bevy::prelude::*;

use crate::game::{
    assets::{HandleMap, SceneKey},
    components::pc::Pc,
    daycycle::{NightLight, TimeSpeed},
    selectable::Selectable,
};

use super::{
    player::{Player, SpawnPlayer},
    spawn_commands::{
        SpawnEarth, SpawnHydroponic, SpawnKitchen, SpawnMetalTrashPile, SpawnOxygenGenerator,
        SpawnToilet, SpawnWaterCleaner, SpawnWaterDispenser,
    },
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
    app.observe(setup_camera);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(
    _trigger: Trigger<SpawnLevel>,
    mut commands: Commands,
    scene_handler: Res<HandleMap<SceneKey>>,
    player: Query<Entity, With<Player>>,
) {
    if let Ok(_player) = player.get_single() {
        return;
    }
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    commands.trigger(SpawnPlayer);

    commands.insert_resource(AmbientLight {
        brightness: 80.0,
        color: Color::srgb(0.9, 0.9, 1.0)
    });


    let light_grid_size = 3;
    let map_size = 9.0;
    let light_dist = map_size / (light_grid_size as f32 + 1.0);
    let h = 15.0;
    let outer_angle = (light_dist * 2.0_f32.sqrt() / h / 2.0).atan();
    let inner_angle = (light_dist / h / 2.0).atan();

    for x in 0..light_grid_size {
        for y in 0..light_grid_size {
            let x_pos = (x as f32 + 1.0) * light_dist;
            let y_pos = (y as f32 + 1.0) * light_dist;

            commands
                .spawn(NightLight)
                .insert(SpotLightBundle {
                    transform: Transform::from_translation(Vec3::new(x_pos, h, y_pos)),
                    spot_light: SpotLight {
                        inner_angle: inner_angle,
                        outer_angle: outer_angle,
                        ..default()
                    },
                    ..default()
                });
        }
    }

    // let sun_id = commands.spawn(DirectionalLightBundle {
    //     transform: Transform::from_translation(Vec3::new(2.0, 5.0, 0.0))
    //         .looking_at(Vec3::ZERO, Vec3::Y),
    //     directional_light: DirectionalLight {
    //         shadows_enabled: true,
    //         illuminance: 2000.0,
    //         ..default()
    //     },
    //     ..default()
    // }).id();

    commands.insert_resource(TimeSpeed::Normal);

    commands
        .spawn(SceneBundle {
            scene: scene_handler[&SceneKey::Pc].clone_weak(),
            transform: Transform::from_translation(Vec3::new(4.0, 0.9, 5.0))
                .with_scale(Vec3::splat(0.5)),
            ..default()
        })
        .insert(Selectable)
        .insert(Pc);

    // Light Blue Tank
    commands
        .spawn(SceneBundle {
            scene: scene_handler[&SceneKey::OxygenTank].clone_weak(),
            transform: Transform::from_translation(Vec3::new(1.0, 0.1, 1.0))
                .with_scale(Vec3::splat(1.0)),
            ..default()
        })
        .insert(Selectable);

    // Dark Blue Tank
    commands
        .spawn(SceneBundle {
            scene: scene_handler[&SceneKey::WaterTank].clone_weak(),
            transform: Transform::from_translation(Vec3::new(1.0, 0.1, 2.0))
                .with_scale(Vec3::splat(1.0)),
            ..default()
        })
        .insert(Selectable);

    // Yellow Tank
    commands
        .spawn(SceneBundle {
            scene: scene_handler[&SceneKey::PeeWaterTank].clone_weak(),
            transform: Transform::from_translation(Vec3::new(1.0, 0.1, 3.0))
                .with_scale(Vec3::splat(1.0)),
            ..default()
        })
        .insert(Selectable);

    // Brown Tank
    commands
        .spawn(SceneBundle {
            scene: scene_handler[&SceneKey::BadWaterTank].clone_weak(),
            transform: Transform::from_translation(Vec3::new(1.0, 0.1, 4.0))
                .with_scale(Vec3::splat(1.0)),
            ..default()
        })
        .insert(Selectable);

    // Red Tank
    commands
        .spawn(SceneBundle {
            scene: scene_handler[&SceneKey::HydrogenTank].clone_weak(),
            transform: Transform::from_translation(Vec3::new(1.0, 0.1, 5.0))
                .with_scale(Vec3::splat(1.0)),
            ..default()
        })
        .insert(Selectable);

    commands.add(SpawnOxygenGenerator {
        pos: Vec3::new(3.0, 0.1, 7.0),
    });

    commands.add(SpawnKitchen {
        pos: Vec3::new(4.5, 0.5, 1.1),
    });

    commands.add(SpawnHydroponic {
        pos: Vec3::new(1.0, 0.1, 7.0),
    });

    commands.add(SpawnMetalTrashPile {
        pos: Vec3::new(6.0, 0.1, 6.0),
    });

    commands.add(SpawnToilet {
        pos: Vec3::new(8.0, 0.1, 7.5),
        rot: Some(Quat::from_rotation_y((90.0_f32).to_radians())),
    });

    commands.add(SpawnWaterDispenser {
        pos: Vec3::new(8.0, 0.1, 6.0),
        rot: Some(Quat::from_rotation_y((-90.0_f32).to_radians())),
    });

    commands.add(SpawnWaterCleaner {
        pos: Vec3::new(8.0, 0.1, 3.0),
        rot: Some(Quat::from_rotation_y((-90.0_f32).to_radians())),
    });

    commands.add(SpawnEarth);
}

fn setup_camera(_: Trigger<SpawnLevel>, mut q_cameras: Query<&mut Transform, With<Camera>>) {
    for mut transform in &mut q_cameras {
        transform.translation = Vec3::new(10.0, 10.0, 10.0);
        transform.look_at(Vec3::new(5.0, 0.0, 5.0), Vec3::Y);
    }
}
