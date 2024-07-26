use bevy::prelude::*;

use crate::game::{selectable::OnMouseClick, spawn::player::Player};

#[derive(Component)]
pub struct Kitchen;

pub(super) fn plugin(app: &mut App) {
    app.observe(on_selected);
}

fn on_selected(
    trigger: Trigger<OnMouseClick>,
    mut commands: Commands,
    q_players: Query<Entity, With<Player>>,
    mut q_kitchen: Query<&GlobalTransform, With<Kitchen>>,
) {
    let target = trigger.entity();

    if trigger.event().0 != MouseButton::Left {
        return;
    }

    if let Ok(kitchen) = q_kitchen.get_mut(target) {

        info!("Cooking Food!");
    }
}
