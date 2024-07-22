#![allow(unused)]

use std::sync::Arc;

use bevy::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.observe(on_next_action);
    app.observe(new_sequence);
}

/// Must do next action (and if we have current active action)
#[derive(Event)]
pub struct NextAction;

#[derive(Event)]
pub struct NewActionSequence {
    pub actions: Sequence,
    pub mode: NewMode,
}

pub enum NewMode {
    Replace,
    Append,
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
    mut q_players: Query<&mut Sequence>,
) {
    let target = trigger.entity();
    info!("OnNextAction {}", target);
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
    mut q_players: Query<&mut Sequence>,
) {
    let target = trigger.entity();

    info!("NewSequence {}", target);

    match trigger.event().mode {
        NewMode::Replace => {
            if let Ok(mut sequence) = q_players.get_mut(target) {
                if !sequence.0.is_empty() {
                    sequence.0[0].terminate(&mut commands, target);
                }
                sequence.0.clear();
            }

            commands
                .entity(target)
                .insert(Sequence(trigger.event().actions.0.clone()));
            commands.trigger_targets(NextAction, target);
        }
        NewMode::Append => {
            if let Ok(mut sequence) = q_players.get_mut(target) {
                if !sequence.0.is_empty() {
                    sequence.0.append(&mut trigger.event().actions.0.clone());
                } else {
                    commands
                        .entity(target)
                        .insert(Sequence(trigger.event().actions.0.clone()));
                    commands.trigger_targets(NextAction, target);
                }
            } else {
                commands
                    .entity(target)
                    .insert(Sequence(trigger.event().actions.0.clone()));
                commands.trigger_targets(NextAction, target);
            }
        }
    }
}
