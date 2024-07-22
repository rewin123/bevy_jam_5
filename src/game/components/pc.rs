use bevy::prelude::*;

use crate::game::{
    character::GoToAction,
    pc_work::{PcWork, PcWorkAction},
    selectable::OnMouseClick,
    sequence::{NewActionSequence, NewMode, Sequence},
    spawn::player::Player,
};

#[derive(Component)]
pub struct Pc;

pub(crate) fn plugin(app: &mut App) {
    app.observe(on_selected);
}

fn on_selected(
    trigger: Trigger<OnMouseClick>,
    mut commands: Commands,
    q_players: Query<(Entity, Option<&PcWork>), With<Player>>,
    mut q_pcs: Query<&GlobalTransform, With<Pc>>,
) {
    let target = trigger.entity();

    if trigger.event().0 != MouseButton::Left {
        return;
    }

    if let Ok(pc_transform) = q_pcs.get_mut(target) {
        let mut sequence = Sequence::default();
        sequence.push(GoToAction {
            target,
            target_pos: pc_transform.translation(),
        });
        sequence.push(PcWorkAction);
        let targets = q_players
            .iter()
            .filter_map(
                |(entity, pc)| {
                    if pc.is_some() {
                        None
                    } else {
                        Some(entity)
                    }
                },
            )
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
