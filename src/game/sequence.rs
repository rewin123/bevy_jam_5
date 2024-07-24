#![allow(unused)]

use std::sync::Arc;

use bevy::prelude::*;
use rand::seq;

pub(crate) fn plugin(app: &mut App) {
    app.observe(on_next_action);
    app.observe(new_sequence);
}

/// Must do next action (and if we have current active action)
#[derive(Event)]
pub struct NextAction;

#[derive(Event)]
pub struct NewActionSequence {
    pub actions: ActionGroup,
    pub mode: NewMode,
}

pub enum NewMode {
    Replace,
    SoftReplace, //replace only if work group is not same
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
    pub actions: Vec<ActionGroup>,
    pub active: bool,
}

impl Sequence {
    pub fn push<T: IntoActionGroup + Send + Sync + 'static>(&mut self, action: T) {
        self.actions.push(action.into_action_group());
        if self.actions.len() == 1 {
            self.active = false;
        }
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
        if !sequence.actions.is_empty() && sequence.active {
            sequence.actions[0].terminate(&mut commands, target);
            sequence.active = false;
            sequence.actions[0].remove_first_action();
            if sequence.actions[0].is_empty() {
                sequence.actions.remove(0);
            }
        }

        if !sequence.actions.is_empty() {
            sequence.actions[0].start_action(&mut commands, target);
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
                if !sequence.actions.is_empty() && sequence.active {
                    sequence.actions[0].terminate(&mut commands, target);
                    sequence.active = false;
                }
                sequence.actions.clear();

                sequence.actions.push(trigger.event().actions.clone());
                commands.trigger_targets(NextAction, target);
            }
        }
        NewMode::Append => {
            if let Ok(mut sequence) = q_players.get_mut(target) {
                let need_new_action = !sequence.active;

                sequence.actions.push(trigger.event().actions.clone());
                if need_new_action {
                    commands.trigger_targets(NextAction, target);
                }
            }
        }
        NewMode::SoftReplace => {
            if let Ok(mut sequence) = q_players.get_mut(target) {
                if !sequence.actions.is_empty() {
                    if sequence.actions[0].name != trigger.event().actions.name {
                        sequence.actions[0].terminate(&mut commands, target);
                        sequence.actions.clear();
                        sequence.actions.push(trigger.event().actions.clone());
                        sequence.active = false;
                        commands.trigger_targets(NextAction, target);
                    }
                } else {
                    sequence.actions.push(trigger.event().actions.clone());
                    sequence.active = false;
                    commands.trigger_targets(NextAction, target);
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct ActionGroup {
    pub name: String,
    pub actions: Vec<Arc<dyn CharacterAction + Send + Sync>>,
}

impl ActionGroup {
    pub fn new(name: String) -> Self {
        Self {
            name,
            actions: vec![],
        }
    }

    pub fn add<T: CharacterAction + Send + Sync + 'static>(&mut self, action: T) {
        self.actions.push(Arc::new(action));
    }

    pub fn with_action<T: CharacterAction + Send + Sync + 'static>(mut self, action: T) -> Self {
        self.actions.push(Arc::new(action));
        self
    }

    pub fn start_action(&self, commands: &mut Commands, target: Entity) {
        if !self.actions.is_empty() {
            self.actions[0].trigger_start(commands, target);
        }
    }

    pub fn terminate(&self, commands: &mut Commands, target: Entity) {
        if !self.actions.is_empty() {
            self.actions[0].terminate(commands, target);
        }
    }

    pub fn remove_first_action(&mut self) {
        if !self.actions.is_empty() {
            self.actions.remove(0);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }
}

pub trait IntoActionGroup {
    fn into_action_group(self) -> ActionGroup;
}

impl<T: CharacterAction + Send + Sync + 'static> IntoActionGroup for T {
    fn into_action_group(self) -> ActionGroup {
        ActionGroup::new("".to_string()).with_action(self)
    }
}

impl IntoActionGroup for ActionGroup {
    fn into_action_group(self) -> ActionGroup {
        self
    }
}
