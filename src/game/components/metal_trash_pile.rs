use bevy::prelude::*;

use crate::game::{
    character::GoToAction,
    metal_trash::GatherMetalWorkAction,
    selectable::OnMouseClick,
    sequence::{NewActionSequence, NewMode, Sequence},
    spawn::{player::Player, spawn_commands::MetalTrashPile},
};

pub(crate) fn plugin(app: &mut App) {
    app.observe(on_selected);
}

const METAL_TRASH_WORK_GROUP: &str = "gathering_metal_trash_work";

fn on_selected(
    trigger: Trigger<OnMouseClick>,
    mut commands: Commands,
    q_players: Query<(Entity, &Sequence), With<Player>>,
    mut q_metal_trash_piles: Query<&GlobalTransform, With<MetalTrashPile>>,
) {
    let target = trigger.entity();

    if trigger.event().0 != MouseButton::Left {
        return;
    }

    if let Ok(metal_trash_pile_transform) = q_metal_trash_piles.get_mut(target) {
        let mut sequence = Sequence::default();

        sequence.push_with_group(
            GoToAction {
                target,
                target_pos: metal_trash_pile_transform.translation(),
            },
            METAL_TRASH_WORK_GROUP.to_string(),
        );
        sequence.push_with_group(GatherMetalWorkAction, METAL_TRASH_WORK_GROUP.to_string());

        let targets = q_players
            .iter()
            .filter_map(|(entity, seq)| {
                if seq.actions.is_empty() || seq.actions[0].group_name != METAL_TRASH_WORK_GROUP {
                    Some(entity)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if !targets.is_empty() {
            commands.trigger_targets(
                NewActionSequence {
                    actions: sequence,
                    mode: NewMode::Append,
                },
                targets,
            );
        }

        info!("Gathering Metal Trash!");
    }
}
