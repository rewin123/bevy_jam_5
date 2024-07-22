use bevy::prelude::*;

use crate::game::{
    character::GoToAction,
    pc_work::PcWorkAction,
    selectable::OnSelect,
    sequence::{NewActionSequence, NewMode, Sequence},
    spawn::player::Player,
};

#[derive(Component)]
pub struct Pc;

pub(crate) fn plugin(app: &mut App) {
    app.observe(on_selected);
}

fn on_selected(
    trigger: Trigger<OnSelect>,
    mut commands: Commands,
    q_players: Query<Entity, With<Player>>,
    mut q_pcs: Query<&GlobalTransform, With<Pc>>,
) {
    let target = trigger.entity();

    if let Ok(pc_transform) = q_pcs.get_mut(target) {
        let mut sequence = Sequence::default();
        sequence.push(GoToAction {
            target,
            target_pos: pc_transform.translation(),
        });
        sequence.push(PcWorkAction);
        commands.trigger_targets(
            NewActionSequence {
                actions: sequence,
                mode: NewMode::Replace,
            },
            q_players.iter().collect::<Vec<_>>(),
        );

        info!("PC working!");
    }
}
