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
#[derive(Clone)]
pub struct ActionHolder {
    pub action: Arc<dyn CharacterAction + Send + Sync>,
    pub group_name: String,
}

impl ActionHolder {
    pub fn trigger_start(&self, commands: &mut Commands, target: Entity) {
        self.action.trigger_start(commands, target);
    }

    pub fn terminate(&self, commands: &mut Commands, target: Entity) {
        self.action.terminate(commands, target);
    }
}


// Action sequence for character
#[derive(Component, Default, Clone)]
pub struct Sequence {
    pub actions: Vec<ActionHolder>,
    pub active: bool,
}

impl Sequence {
    pub fn push<T: CharacterAction + Send + Sync + 'static>(&mut self, action: T) {
        self.actions.push(ActionHolder {
            action: Arc::new(action),
            group_name: "".to_string(),
        });
        if self.actions.len() == 1 {
            self.active = false;
        }
    }

    pub fn push_with_group<T: CharacterAction + Send + Sync + 'static>(
        &mut self,
        action: T,
        group_name: String,
    ) {
        self.actions.push(ActionHolder {
            action: Arc::new(action),
            group_name,
        });
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
        if !sequence.actions.is_empty() {
            if sequence.active {
                sequence.actions[0].action.terminate(&mut commands, target);
                sequence.active = false;
                sequence.actions.remove(0);
            } else {
               
            }
        }

        if !sequence.actions.is_empty() {
            sequence.actions[0].trigger_start(&mut commands, target);
            sequence.active = true;
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
                if !sequence.actions.is_empty() {
                    if sequence.active {
                        sequence.actions[0].terminate(&mut commands, target);
                        sequence.active = false;
                    }
                }
                sequence.actions.clear();
            }

            commands
                .entity(target)
                .insert(trigger.event().actions.clone());
            commands.trigger_targets(NextAction, target);
        }
        NewMode::Append => {
            if let Ok(mut sequence) = q_players.get_mut(target) {
                if !sequence.actions.is_empty() {
                    if sequence.active {
                        sequence.actions[0].terminate(&mut commands, target);
                        sequence.active = false;
                    }
                }
                sequence.actions.extend(trigger.event().actions.actions.iter().cloned());
                commands.trigger_targets(NextAction, target);
            } else {
                commands
                    .entity(target)
                    .insert(trigger.event().actions.clone());
                commands.trigger_targets(NextAction, target);
            }
        }
    }
}
