#![allow(unused)]

use bevy::{prelude::*, utils::HashSet};

use super::{
    bilboard_state::{BillboardContent, BillboardSpawner},
    daycycle::GameTime,
    resources::{GameResource, Oxygen, OxygenRecycling},
    selectable::OnMouseClick,
    sequence::{CharacterAction, NewActionSequence, NewMode, NextAction, Sequence},
    spawn::player::Player,
};

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, move_player_to_target);
    app.observe(add_target);

    app.add_systems(PreUpdate, clear_states);
    app.add_systems(Update, check_oxigen);
    app.add_systems(PostUpdate, (print_state,));
}

pub const PLAYER_SPEED: f32 = 5.0;

#[derive(Component)]
pub struct IgnoreJustMoving;

#[derive(Component)]
pub struct DestinationTarget {
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
    trigger: Trigger<OnMouseClick>,
    mut commands: Commands,
    q_player: Query<Entity, With<Player>>,
    q_selected: Query<&GlobalTransform, Without<IgnoreJustMoving>>,
) {
    let clicked_entity = trigger.entity();

    if trigger.event().0 != MouseButton::Left {
        return;
    }

    let Ok(target_component) = q_selected.get(clicked_entity) else {
        return;
    };

    let mut sequence = Sequence::default();
    sequence.push(GoToAction {
        target: clicked_entity,
        target_pos: target_component.translation(),
    });

    if !q_player.is_empty() {
        commands.trigger_targets(
            NewActionSequence {
                actions: sequence,
                mode: NewMode::Replace,
            },
            q_player.iter().next().unwrap(),
        );
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

        let cos_result = Quat::from_scaled_axis(player_position - target_position);
        let direction = (target_position - player_position).normalize();
        let distance = player_position.distance(target_position);

        if distance > target.accept_radius {
            transform.translation += direction * time.delta_seconds() * PLAYER_SPEED;
            // the character should just spin on its y axis at the beginning
            transform.rotate_local_y(cos_result.y);
        } else {
            commands.entity(player_entity).remove::<DestinationTarget>();
            commands.trigger_targets(NextAction, player_entity);
        }
    }
}

/// State logic

#[derive(Default, PartialEq, Clone, Copy, Hash, Eq, Debug)]
pub enum CharState {
    #[default]
    Idle,
    Working,
    Peeing,
    WantEat,
    WantSleep,
    WantDrink,
    WantOxigen,
    WantPee,

    TooManyOxigen,

    Dead,
}

impl CharState {
    const fn weight(&self) -> i32 {
        match self {
            CharState::Idle => 0,
            CharState::Working => 1,
            CharState::Peeing => 2,
            CharState::WantEat => 3,
            CharState::WantSleep => 4,
            CharState::WantDrink => 5,
            CharState::WantOxigen => 6,
            CharState::TooManyOxigen => 6,
            CharState::WantPee => 7,
            CharState::Dead => 8,
        }
    }
}

#[derive(Component, Default)]
pub struct CharacterStates {
    pub states: HashSet<CharState>,
}

impl CharacterStates {
    pub fn get_importantest_state(&self) -> CharState {
        let mut state = CharState::Idle;
        for s in self.states.iter() {
            if s.weight() > state.weight() {
                state = *s;
            }
        }
        state
    }

    pub fn add(&mut self, state: CharState) {
        self.states.insert(state);
    }
}

fn clear_states(mut q: Query<&mut CharacterStates>) {
    for mut state in q.iter_mut() {
        state.states.clear();
    }
}

fn print_state(mut q_char: Query<(&mut CharacterStates, &mut BillboardSpawner)>) {
    for (mut state, mut spawner) in q_char.iter_mut() {
        let state = state.get_importantest_state();

        let usual_text = TextStyle::default();
        let warning_text = TextStyle {
            color: Color::linear_rgb(1.0, 0.0, 0.0),
            ..default()
        };

        let content = match state {
            CharState::Idle => BillboardContent::None,
            CharState::Working => BillboardContent::Text(Text::from_section("Working", usual_text)),
            CharState::Peeing => BillboardContent::Text(Text::from_section("Peeing", usual_text)),
            CharState::WantEat => {
                BillboardContent::Text(Text::from_section("Want eat", warning_text))
            }
            CharState::WantSleep => {
                BillboardContent::Text(Text::from_section("Want sleep", warning_text))
            }
            CharState::WantDrink => {
                BillboardContent::Text(Text::from_section("Want drink", warning_text))
            }
            CharState::WantOxigen => {
                BillboardContent::Text(Text::from_section("Want oxigen", warning_text))
            }
            CharState::TooManyOxigen => {
                BillboardContent::Text(Text::from_section("Too many oxigen", warning_text))
            }
            CharState::WantPee => {
                BillboardContent::Text(Text::from_section("Want pee", warning_text))
            }
            CharState::Dead => BillboardContent::Text(Text::from_section("Dead", warning_text)),
        };

        spawner.content = content;
        spawner.set_changed();
    }
}

fn check_oxigen(
    mut q_char: Query<&mut CharacterStates>,
    oxigen: Res<Oxygen>,
    oxigen_regeneration: Res<OxygenRecycling>,
) {
    for mut states in q_char.iter_mut() {
        if oxigen.amount() <= 10.0 {
            states.add(CharState::WantOxigen);
        } else if oxigen.amount() > oxigen.limit().unwrap() * 0.9 && oxigen_regeneration.working {
            states.add(CharState::TooManyOxigen);
        }
    }
}
