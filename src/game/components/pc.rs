#![allow(clippy::type_complexity)]

use bevy::prelude::*;

use crate::game::{
    character::{GoToAction, IgnoreJustMoving},
    pc_work::PcWorkAction,
    selectable::OnMouseClick,
    sequence::{NewActionSequence, NewMode, Sequence},
    spawn::{player::Player, spawn_commands::OxygenRecyler},
};

#[derive(Component)]
pub struct Pc;

pub(crate) fn plugin(app: &mut App) {
    app.observe(on_selected);

    app.add_systems(Update, auto_add_complex_moving);
}

const PC_WORK_GROUP: &str = "pc_work";

fn auto_add_complex_moving(
    mut commands: Commands,
    q_new: Query<Entity, Or<(Added<Pc>, Added<OxygenRecyler>)>>,
) {
    for entity in q_new.iter() {
        commands.entity(entity).insert(IgnoreJustMoving);
    }
}

fn on_selected(
    trigger: Trigger<OnMouseClick>,
    mut commands: Commands,
    q_players: Query<(Entity, &Sequence), With<Player>>,
    mut q_pcs: Query<&GlobalTransform, With<Pc>>,
) {
    let target = trigger.entity();

    if trigger.event().0 != MouseButton::Left {
        return;
    }

    if let Ok(pc_transform) = q_pcs.get_mut(target) {
        let mut sequence = Sequence::default();

        sequence.push_with_group(
            GoToAction {
                target,
                target_pos: pc_transform.translation(),
            },
            PC_WORK_GROUP.to_string(),
        );
        sequence.push_with_group(PcWorkAction, PC_WORK_GROUP.to_string());

        let targets = q_players
            .iter()
            .filter_map(|(entity, seq)| {
                if seq.actions.is_empty() || seq.actions[0].group_name != PC_WORK_GROUP {
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

        info!("PC working!");
    }
}
