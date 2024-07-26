use bevy::prelude::*;

use crate::game::{
    character::GoToAction, kitchen_work::CookFoodAction, selectable::OnMouseClick,
    sequence::ActionGroup, spawn::player::Player,
};

#[derive(Component)]
pub struct Kitchen;

pub(super) fn plugin(app: &mut App) {
    app.observe(on_selected);
}

const KITCHEN_WORK_GROUP: &str = "kitchen_work_group";

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

    if let Ok(kitchen_transform) = q_kitchen.get_mut(target) {
        let mut actions = ActionGroup::new(KITCHEN_WORK_GROUP.to_string());

        actions.add(GoToAction {
            target,
            target_pos: kitchen_transform.translation(),
        });

        actions.add(CookFoodAction);

        info!("Cooking Food!");
    }
}
