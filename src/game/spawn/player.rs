//! Spawn the player.

use bevy::prelude::*;

use crate::game::{
    assets::{HandleMap, SceneKey}, character::Sequence, movement::{Movement, MovementController}
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
    app.register_type::<Player>();
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    scene_handler: Res<HandleMap<SceneKey>>,
) {
    // let capsule = asset_server.load("models/guy.glb#Scene0");
    // let material = StandardMaterial {
    //     base_color: Color::linear_rgb(1.0, 0.0, 0.0),
    //     ..default()
    // };

    commands.spawn((
        Name::new("Player"),
        Player,
        SceneBundle {
            scene: scene_handler[&SceneKey::Player].clone_weak(),
            transform: Transform::from_translation(Vec3::new(5.0, 0.7, 5.0))
                .with_scale(Vec3::splat(0.25)),
            ..default()
        },
        MovementController::default(),
        Movement { speed: 5.0 },
        Sequence::default()
    ));
}
