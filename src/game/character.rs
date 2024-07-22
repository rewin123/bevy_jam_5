use bevy::prelude::*;

use super::{
    daycycle::GameTime,
    movement::MovementController,
    selectable::{OnSelect, Selected},
    spawn::player::{self, Player},
};

pub const PLAYER_SPEED: f32 = 5.0;

#[derive(Component)]
struct DestinationTarget {
    pub target: Entity,
    pub target_pos: Vec3,
    pub accept_radius: f32,
}

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, move_player_to_target);
    app.observe(add_target);
}

fn add_target(
    trigger: Trigger<OnSelect>,
    mut commands: Commands,
    q_player: Query<Entity, With<Player>>,
    q_selected: Query<&GlobalTransform, With<Selected>>,
) {
    let clicked_entity = trigger.entity();

    let Ok(target_component) = q_selected.get_single() else {
        return;
    };

    for player in q_player.iter() {
        commands.entity(player).insert(DestinationTarget {
            target: clicked_entity,
            target_pos: target_component.translation(),
            accept_radius: 0.5,
        });
    }
}

fn move_player_to_target(
    time: Res<GameTime>,
    mut query: Query<(&mut Transform, &DestinationTarget), With<Player>>,
) {
    for (mut transform, target) in query.iter_mut() {
        let player_position = transform.translation;
        let target_position = target.target_pos;

        let cos_result = Quat::from_scaled_axis(player_position - target_position);
        let direction = (target_position - player_position).normalize();
        let distance = player_position.distance(target_position);

        if distance > target.accept_radius {
            transform.translation += direction * time.delta_seconds() * PLAYER_SPEED;
            // the character should just spin on its y axis at the beginning
            transform.rotate_local_y(cos_result.y);
        } else {
        }
    }
}
