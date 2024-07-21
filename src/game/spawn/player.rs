//! Spawn the player.

use bevy::prelude::*;

use crate::{
    game::{
        animation::PlayerAnimation,
        assets::{HandleMap, ImageKey},
        movement::{Movement, MovementController, WrapWithinWindow},
    },
    screen::Screen,
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let capsule = meshes.add(Capsule3d::new(0.3, 1.0));
    let material = StandardMaterial {
        base_color: Color::linear_rgb(1.0, 0.0, 0.0),
        ..default()
    };

    commands.spawn((
        Name::new("Player"),
        Player,
        PbrBundle {
            mesh: capsule.clone(),
            material: materials.add(material),
            transform: Transform::from_translation(Vec3::new(5.0, 1.0, 5.0)),
            ..default()
        },
        MovementController::default(),
        Movement { speed: 5.0 },
    ));
}
