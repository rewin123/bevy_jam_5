use bevy::prelude::*;

use crate::game::{
    bilboard_state::BillboardContent,
    character::{CharState, CharacterStates, GoToAction},
    daycycle::GameTime,
    device_state::{DevceStatePlugin, DeviceState},
    resources::OxygenRecycling,
    selectable::OnMouseClick,
    sequence::{CharacterAction, NewActionSequence, NewMode, NextAction, Sequence},
    spawn::{player::Player, spawn_commands::OxygenRecyler},
};

pub(super) fn plugin(app: &mut App) {
    app.observe(on_selected);
    app.add_systems(Update, update_oxygen_recycler_work);
    app.add_systems(Update, update_oxigen_recycler_state);

    app.add_plugins(DevceStatePlugin::<OxygenRegenratorState>::default());
}

#[derive(Component, PartialEq, Clone)]
pub enum OxygenRegenratorState {
    Idle,
    Work,
}

impl DeviceState for OxygenRegenratorState {
    fn content(&self) -> BillboardContent {
        match self {
            OxygenRegenratorState::Idle => BillboardContent::None,
            OxygenRegenratorState::Work => BillboardContent::Text(Text::from_section(
                "Oxigen++",
                TextStyle {
                    color: Color::linear_rgb(0.1, 0.1, 1.0),
                    ..default()
                },
            )),
        }
    }
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
    mut q_oxygen_recycler_work: Query<(Entity, &mut OxygenRecyclerWork, &mut CharacterStates)>,
    // work_config: Res<PcWorkConfig>,
    mut oxygen_recycling: ResMut<OxygenRecycling>,
) {
    for (entity, mut or_work, mut states) in q_oxygen_recycler_work.iter_mut() {
        or_work.work_time += time.delta_seconds();
        states.add(CharState::Working);
        if or_work.work_time >= 0.25 {
            oxygen_recycling.working = !oxygen_recycling.working;
            commands.entity(entity).remove::<OxygenRecyclerWork>();
            commands.trigger_targets(NextAction, entity);
        }
    }
}

fn update_oxigen_recycler_state(
    mut commands: Commands,
    q_oxygen_recyclers: Query<Entity, With<OxygenRecyler>>,
    recycling: Res<OxygenRecycling>,
) {
    for entity in q_oxygen_recyclers.iter() {
        if recycling.working {
            commands.entity(entity).insert(OxygenRegenratorState::Work);
        } else {
            commands.entity(entity).insert(OxygenRegenratorState::Idle);
        }
    }
}
