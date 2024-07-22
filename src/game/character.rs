use bevy::prelude::*;
use std::sync::Arc;

use super::{
    daycycle::GameTime,
    selectable::{OnSelect, Selected},
    spawn::player::Player,
};


pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, move_player_to_target);
    app.observe(add_target);
    app.observe(on_next_action);
}


/// Must do next action (and if we have current active action)
#[derive(Event)]
pub struct NextAction;

/// Must do next action (and if we haven't current active action)
#[derive(Event)]
pub struct NextActionIfNone;

#[derive(Event)]
pub struct NewActionSequence {
    pub actions: Sequence,
    pub mode: NewMode
}

pub enum NewMode {
    Replace,
    Append
}

pub trait CharacterAction {
    fn trigger_start(&self, commands: &mut Commands, target: Entity); // Start the action with trigger
    fn terminate(&self, commands: &mut Commands, target: Entity);
}


// Action sequence for character
#[derive(Component, Default)]
pub struct Sequence(Vec<Arc<dyn CharacterAction + Send + Sync>>);

impl Sequence {
    pub fn push<T: CharacterAction + Send + Sync + 'static>(&mut self, action: T) {
        self.0.push(Arc::new(action));
    }
}


fn on_next_action(
    trigger: Trigger<NextAction>,
    mut commands: Commands,
    mut q_players: Query<&mut Sequence>) 
{
    let target = trigger.entity();
    if let Ok(mut sequence) = q_players.get_mut(target) {
        if !sequence.0.is_empty() {
            sequence.0.remove(0);
        }

        if !sequence.0.is_empty() {
            let action = &mut sequence.0[0];
            action.trigger_start(&mut commands, target);
        }
    }
}

fn new_sequence(
    trigger: Trigger<NewActionSequence>,
    mut commands: Commands,
    mut q_players: Query<&mut Sequence>) {

    let target = trigger.entity();

    match trigger.event().mode {
        NewMode::Replace => {
            if let Ok(mut sequence) = q_players.get_mut(target) {
                if !sequence.0.is_empty() {
                    sequence.0[0].terminate(&mut commands, target);
                }
                sequence.0.clear();
            }

            commands.entity(target).insert(Sequence(trigger.event().actions.0.clone()));
            commands.trigger_targets(NextAction, target);
        },
        NewMode::Append => {
            if let Ok(mut sequence) = q_players.get_mut(target) {
                if !sequence.0.is_empty() {
                   sequence.0.append(&mut trigger.event().actions.0.clone());
                } else {
                    commands.entity(target).insert(Sequence(trigger.event().actions.0.clone()));
                    commands.trigger_targets(NextAction, target);
                }
            } else {
                commands.entity(target).insert(Sequence(trigger.event().actions.0.clone()));
                commands.trigger_targets(NextAction, target);
            }
        },
    }
    
}


pub const PLAYER_SPEED: f32 = 5.0;

#[derive(Component)]
struct DestinationTarget {
    #[allow(dead_code)]
    pub target: Entity,
    pub target_pos: Vec3,
    pub accept_radius: f32,
}

pub struct GoToAction {
    pub target: Entity,
    pub target_pos: Vec3,
}

impl CharacterAction for GoToAction {
    fn trigger_start(&self, commands: &mut Commands, target: Entity) {
        commands.entity(target).insert(DestinationTarget {
            target: self.target,
            target_pos: self.target_pos,
            accept_radius: 0.5,
        });
    }

    fn terminate(&self, commands: &mut Commands, target: Entity) {
        commands.entity(target).remove::<DestinationTarget>();
    }
}

fn add_target(
    trigger: Trigger<OnSelect>,
    mut commands: Commands,
    q_player: Query<Entity, With<Player>>,
    q_selected: Query<&GlobalTransform, With<Selected>>,
) {
    let clicked_entity = trigger.entity();

    let Ok(target_component) = q_selected.get_single() else {
        return;
    };

    for player in q_player.iter() {
        commands.entity(player).insert(DestinationTarget {
            target: clicked_entity,
            target_pos: target_component.translation(),
            accept_radius: 0.5,
        });
    }
}

fn move_player_to_target(
    mut commands: Commands,
    time: Res<GameTime>,
    mut query: Query<(Entity, &mut Transform, &DestinationTarget), With<Player>>,
) {
    for (player_entity, mut transform, target) in query.iter_mut() {
        let player_position = transform.translation;
        let target_position = target.target_pos;

        let direction = (target_position - player_position).normalize();
        let distance = player_position.distance(target_position);

        if distance > target.accept_radius {
            transform.translation += direction * time.delta_seconds() * PLAYER_SPEED;
        } else {
            commands.entity(target.target).remove::<DestinationTarget>();
            commands.trigger_targets(NextAction, player_entity);
        }
    }
}
