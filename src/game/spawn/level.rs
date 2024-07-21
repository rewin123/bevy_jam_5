//! Spawn the main level by triggering other observers.

use bevy::prelude::*;
use rand::Rng;

use crate::game::{
    daycycle::TimeSpeed,
    selectable::{Computer, Selectable},
};

use super::{
    player::SpawnPlayer,
    spawn_commands::{SpawnHydroponic, SpawnOxygenGenerator},
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
    asset_server: Res<AssetServer>,
) {
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    commands.trigger(SpawnPlayer);

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::new(2.0, 5.0, 2.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    commands.insert_resource(TimeSpeed::Normal);

    commands
        .spawn(SceneBundle {
            scene: asset_server.load("models/pc.glb#Scene0"),
            transform: Transform::from_translation(Vec3::new(4.0, 0.9, 5.0))
                .with_scale(Vec3::splat(0.5)),
            ..default()
        })
        .insert(Selectable)
        .insert(Computer);

    commands
        .spawn(SceneBundle {
            scene: asset_server.load("models/water_tank.glb#Scene0"),
            transform: Transform::from_translation(Vec3::new(5.0, 0.1, 2.0))
                .with_scale(Vec3::splat(1.0)),
            ..default()
        })
        .insert(Selectable);

    commands
        .spawn(SceneBundle {
            scene: asset_server.load("models/oxygen_tank.glb#Scene0"),
            transform: Transform::from_translation(Vec3::new(5.0, 0.1, 1.0))
                .with_scale(Vec3::splat(1.0)),
            ..default()
        })
        .insert(Selectable);

    commands
        .spawn(SceneBundle {
            scene: asset_server.load("models/bad_water_tank.glb#Scene0"),
            transform: Transform::from_translation(Vec3::new(3.0, 0.1, 1.0))
                .with_scale(Vec3::splat(1.0)),
            ..default()
        })
        .insert(Selectable);

    commands
        .spawn(SceneBundle {
            scene: asset_server.load("models/hydrogen_tank.glb#Scene0"),
            transform: Transform::from_translation(Vec3::new(3.0, 0.1, 2.0))
                .with_scale(Vec3::splat(1.0)),
            ..default()
        })
        .insert(Selectable);

    commands
        .spawn(SceneBundle {
            scene: asset_server.load("models/pee_tank.glb#Scene0"),
            transform: Transform::from_translation(Vec3::new(3.0, 0.1, 3.0))
                .with_scale(Vec3::splat(1.0)),
            ..default()
        })
        .insert(Selectable);

    commands
        .spawn(SceneBundle {
            scene: asset_server.load("models/metal_trash.glb#Scene0"),
            transform: Transform::from_translation(Vec3::new(6.0, 0.1, 6.0))
                .with_scale(Vec3::splat(0.5)),
            ..default()
        })
        .insert(Selectable);

    commands.add(SpawnOxygenGenerator {
        pos: Vec3::new(3.0, 0.1, 7.0),
    });

    commands.add(SpawnHydroponic {
        pos: Vec3::new(1.0, 0.1, 7.0),
    });
}

fn setup_camera(_: Trigger<SpawnLevel>, mut q_cameras: Query<&mut Transform, With<Camera>>) {
    for mut transform in &mut q_cameras {
        transform.translation = Vec3::new(10.0, 10.0, 10.0);
        transform.look_at(Vec3::new(5.0, 0.0, 5.0), Vec3::Y);
    }
}
