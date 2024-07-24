#![allow(clippy::type_complexity)]

use bevy::prelude::*;

use crate::game::{
    character::{GoToAction, IgnoreJustMoving},
    pc_work::PcWorkAction,
    selectable::OnMouseClick,
    sequence::{ActionGroup, NewActionSequence, NewMode},
    spawn::{
        player::Player,
        spawn_commands::{MetalTrashPile, OxygenRecyler},
    },
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
    q_new: Query<Entity, Or<(Added<Pc>, Added<OxygenRecyler>, Added<MetalTrashPile>)>>,
) {
    for entity in q_new.iter() {
        commands.entity(entity).insert(IgnoreJustMoving);
    }
}

fn on_selected(
    trigger: Trigger<OnMouseClick>,
    mut commands: Commands,
    q_players: Query<Entity, With<Player>>,
    mut q_pcs: Query<&GlobalTransform, With<Pc>>,
) {
    let target = trigger.entity();

    if trigger.event().0 != MouseButton::Left {
        return;
    }

    if let Ok(pc_transform) = q_pcs.get_mut(target) {
        let mut actions = ActionGroup::new(PC_WORK_GROUP.to_string());

        actions.add(
            GoToAction {
                target,
                target_pos: pc_transform.translation(),
            }
        );
        actions.add(PcWorkAction);


        commands.trigger_targets(
            NewActionSequence {
                actions: actions,
                mode: NewMode::SoftReplace
            },
            q_players.iter().collect::<Vec<_>>()
        );

        info!("PC working!");
    }
}
