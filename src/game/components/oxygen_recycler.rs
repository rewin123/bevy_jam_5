use bevy::prelude::*;

use crate::game::{
    bilboard_state::BillboardContent,
    character::{CharState, CharacterStates, GoToAction},
    daycycle::GameTime,
    device_state::{DeviceState, DeviceStatePlugin},
    resources::OxygenRecycling,
    selectable::OnMouseClick,
    sequence::{ActionGroup, CharacterAction, NewActionSequence, NewMode, NextAction, Sequence},
    spawn::{player::Player, spawn_commands::OxygenRecyler},
};

use super::fire::InFire;

pub(super) fn plugin(app: &mut App) {
    app.observe(on_selected);
    app.add_systems(Update, update_oxygen_recycler_work);
    app.add_systems(Update, update_oxygen_recycler_state);
    app.add_systems(PostUpdate, disable_oxygen_if_no_recycler);

    app.add_plugins(DeviceStatePlugin::<OxygenRegenratorState>::default());
}

#[derive(Component, PartialEq, Clone)]
pub enum OxygenRegenratorState {
    Idle,
    Work,
    InFire(f32),
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
            OxygenRegenratorState::InFire(time) => BillboardContent::time_remaining(*time),
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
        let mut actions = ActionGroup::new(OXYGEN_RECYCLER_WORK_GROUP.to_string());
        actions.add(
            GoToAction {
                target,
                target_pos: or_transform.translation(),
            }
        );
        actions.add(OxygenRecyclerAction);
        
        commands
            .trigger_targets(NewActionSequence {
                actions: actions,
                mode: NewMode::Replace,
            }, 
            q_players
                .iter()
                .map(|(entity, sequence)| entity)
                .collect::<Vec<_>>()
        );

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

fn update_oxygen_recycler_state(
    mut commands: Commands,
    q_oxygen_recyclers: Query<Entity, With<OxygenRecyler>>,
    on_fire: Query<(Entity, &InFire)>,
    recycling: Res<OxygenRecycling>,
    time: Res<GameTime>,
) {
    for entity in q_oxygen_recyclers.iter() {
        if let Ok((_, fire)) = on_fire.get(entity) {
            // The entity may be despawned
            let Some(mut entity_cms) = commands.get_entity(entity) else {
                continue;
            };
            entity_cms.insert(OxygenRegenratorState::InFire(
                fire.time_remaining(time.elapsed_seconds()),
            ));
        } else if recycling.working {
            commands.entity(entity).insert(OxygenRegenratorState::Work);
        } else {
            commands.entity(entity).insert(OxygenRegenratorState::Idle);
        }
    }
}

fn disable_oxygen_if_no_recycler(
    q_oxygen_recyclers: Query<Entity, With<OxygenRecyler>>,
    mut recycling: ResMut<OxygenRecycling>,
) {
    if q_oxygen_recyclers.is_empty() {
        recycling.working = false;
    }
}
