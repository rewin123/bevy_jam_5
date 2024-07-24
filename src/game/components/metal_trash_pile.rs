use bevy::prelude::*;

use crate::game::{
    character::GoToAction,
    metal_trash::GatherMetalWorkAction,
    selectable::OnMouseClick,
    sequence::{ActionGroup, NewActionSequence, NewMode},
    spawn::{player::Player, spawn_commands::MetalTrashPile},
};

pub(crate) fn plugin(app: &mut App) {
    app.observe(on_selected);
}

const METAL_TRASH_WORK_GROUP: &str = "gathering_metal_trash_work";

fn on_selected(
    trigger: Trigger<OnMouseClick>,
    mut commands: Commands,
    q_players: Query<Entity, With<Player>>,
    mut q_metal_trash_piles: Query<&GlobalTransform, With<MetalTrashPile>>,
) {
    let target = trigger.entity();

    if trigger.event().0 != MouseButton::Left {
        return;
    }

    if let Ok(metal_trash_pile_transform) = q_metal_trash_piles.get_mut(target) {
        let mut actions = ActionGroup::new(METAL_TRASH_WORK_GROUP.to_string());

        actions.add(
            GoToAction {
                target,
                target_pos: metal_trash_pile_transform.translation(),
            }
        );
        actions.add(GatherMetalWorkAction);

        commands.trigger_targets(
            NewActionSequence { actions, mode: NewMode::SoftReplace}, 
            q_players.iter().collect::<Vec<_>>()
        );

        info!("Gathering Metal Trash!");
    }
}
