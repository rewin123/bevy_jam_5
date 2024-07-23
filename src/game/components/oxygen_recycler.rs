use bevy::prelude::*;

use crate::game::{
    character::GoToAction,
    daycycle::GameTime,
    resources::OxygenRecycling,
    selectable::OnMouseClick,
    sequence::{CharacterAction, NewActionSequence, NewMode, NextAction, Sequence},
    spawn::{player::Player, spawn_commands::OxygenRecyler},
};

pub(super) fn plugin(app: &mut App) {
    app.observe(on_selected);
    app.add_systems(Update, update_oxygen_recycler_work);
}

const OXYGEN_RECYCLER_WORK_GROUP: &str = "oxygen_recycler";

fn on_selected(
    trigger: Trigger<OnMouseClick>,
    mut commands: Commands,
    q_players: Query<(Entity, &Sequence), With<Player>>,
    mut q_oxygen_recyclers: Query<&GlobalTransform, With<OxygenRecyler>>,
) {
    let target = trigger.entity();

    if trigger.event().0 != MouseButton::Left {
        return;
    }

    if let Ok(or_transform) = q_oxygen_recyclers.get_mut(target) {
        let mut sequence = Sequence::default();
        sequence.push_with_group(
            GoToAction {
                target,
                target_pos: or_transform.translation(),
            },
            OXYGEN_RECYCLER_WORK_GROUP.to_string(),
        );
        sequence.push_with_group(OxygenRecyclerAction, OXYGEN_RECYCLER_WORK_GROUP.to_string());
        let targets = q_players
            .iter()
            .filter_map(|(entity, seq)| {
                if seq.actions.is_empty() || seq.actions[0].group_name != OXYGEN_RECYCLER_WORK_GROUP
                {
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

        info!("Oxygen Recycling!");
    }
}

pub struct OxygenRecyclerAction;

#[derive(Component, Default)]
pub struct OxygenRecyclerWork {
    pub work_time: f32,
}

impl CharacterAction for OxygenRecyclerAction {
    fn trigger_start(&self, commands: &mut Commands, target: Entity) {
        info!("trigger start or");
        commands
            .entity(target)
            .insert(OxygenRecyclerWork::default());
    }

    fn terminate(&self, commands: &mut Commands, target: Entity) {
        commands.entity(target).remove::<OxygenRecyclerWork>();
    }
}

fn update_oxygen_recycler_work(
    time: Res<GameTime>,
    mut commands: Commands,
    mut q_oxygen_recycler_work: Query<(Entity, &mut OxygenRecyclerWork)>,
    // work_config: Res<PcWorkConfig>,
    mut oxygen_recycling: ResMut<OxygenRecycling>,
) {
    for (entity, mut or_work) in q_oxygen_recycler_work.iter_mut() {
        or_work.work_time += time.delta_seconds();
        if or_work.work_time >= 0.25 {
            oxygen_recycling.working = !oxygen_recycling.working;
            commands.entity(entity).remove::<OxygenRecyclerWork>();
            commands.trigger_targets(NextAction, entity);
        }
    }
}
