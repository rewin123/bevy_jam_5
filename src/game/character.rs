#![allow(unused)]

use std::any::{Any, TypeId};

use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
    utils::HashSet,
};

use crate::screen::Screen;

use super::{
    assets::{HandleMap, SfxKey},
    billboard_state::{BillboardContent, BillboardSpawner},
    daycycle::{GameTime, TimeSpeed},
    resources::{
        CarbonDioxide, GameResource, Oxygen, OxygenRecycling, Pee, ResourceThreshold, Thirst,
    },
    selectable::OnMouseClick,
    sequence::{ActionGroup, CharacterAction, NewActionSequence, NewMode, NextAction, Sequence},
    spawn::player::Player,
};

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, move_player_to_target);
    app.observe(add_target);
    app.init_state::<HouseState>();
    app.add_systems(PreUpdate, clear_states);
    app.add_systems(Update, set_resource_warnings::<Oxygen>);
    app.add_systems(Update, set_resource_warnings::<Pee>);
    app.add_systems(Update, set_resource_warnings::<Thirst>);
    app.add_systems(PostUpdate, (print_state, set_house_state).chain());
    app.enable_state_scoped_entities::<HouseState>();
    app.add_systems(OnEnter(HouseState::Alarm), play_alarm);
}

pub const PLAYER_SPEED: f32 = 5.0;

#[derive(States, Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HouseState {
    #[default]
    Normal,
    Alarm,
}

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

    let mut actions = ActionGroup::new("GoTo".to_string());

    actions.add(GoToAction {
        target: clicked_entity,
        target_pos: target_component.translation(),
    });

    if !q_player.is_empty() {
        commands.trigger_targets(
            NewActionSequence {
                actions,
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
    Drinking,
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
            CharState::Drinking => 3,
            CharState::WantEat => 4,
            CharState::WantSleep => 5,
            CharState::WantDrink => 6,
            CharState::WantOxigen => 7,
            CharState::TooManyOxigen => 8,
            CharState::WantPee => 9,
            CharState::Dead => 10,
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

/// States that should trigger the alarm
const ALARM_CHAR_STATES: [CharState; 6] = [
    CharState::WantOxigen,
    CharState::TooManyOxigen,
    CharState::WantDrink,
    CharState::WantPee,
    CharState::WantEat,
    CharState::WantEat,
];

fn set_house_state(
    mut q_char: Query<(&CharacterStates)>,
    mut next_state: ResMut<NextState<HouseState>>,
) {
    let has_warning = q_char
        .iter()
        .map(|char| char.get_importantest_state())
        .any(|state| ALARM_CHAR_STATES.contains(&state));

    if (has_warning) {
        next_state.set(HouseState::Alarm);
    } else {
        next_state.set(HouseState::Normal);
    }
}

fn play_alarm(mut commands: Commands, sounds: Res<HandleMap<SfxKey>>) {
    commands.spawn((
        AudioBundle {
            source: sounds[&SfxKey::Alarm].clone_weak(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::new(3.0),
                ..Default::default()
            },
        },
        StateScoped(HouseState::Alarm),
    ));
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
            CharState::Drinking => {
                BillboardContent::Text(Text::from_section("Drinking", usual_text))
            }
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
                BillboardContent::Text(Text::from_section("Too much oxigen", warning_text))
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

fn set_resource_warnings<T: GameResource + Clone>(
    mut q_char: Query<&mut CharacterStates>,
    resource: ResMut<T>,
    mut time_speed: ResMut<TimeSpeed>,
    screen: Res<State<Screen>>,
) {
    if *screen != Screen::Playing {
        return;
    }

    let amount = resource.amount();
    let (min_o, max_o) = resource.warning_thresholds();
    let state: CharState = match (resource.resource_threshold(), min_o, max_o) {
        (ResourceThreshold::HealthyRange, Some(min), _) if min >= amount => {
            resource_to_state(resource.clone(), true)
        }
        (ResourceThreshold::HealthyRange, _, Some(max)) if max <= amount => {
            resource_to_state(resource.clone(), false)
        }
        (ResourceThreshold::Necessity, Some(min), _) if min >= amount => {
            resource_to_state(resource.clone(), true)
        }
        (ResourceThreshold::Waste, _, Some(max)) if max <= amount => {
            resource_to_state(resource.clone(), false)
        }
        _ => CharState::Idle,
    };

    for mut states in q_char.iter_mut() {
        states.add(state);
    }
}

/**
 * Map from a [`GameResource`] to it's [`CharState`]
 * Used in [`set_resources_warnings`] to automatically set billboards
 */
fn resource_to_state<T: GameResource + Any>(res: T, is_deficiency: bool) -> CharState {
    let oxygen_id = TypeId::of::<Oxygen>();
    let pee_id = TypeId::of::<Pee>();
    let thirst_id = TypeId::of::<Thirst>();

    match (res.type_id(), is_deficiency) {
        (oxygen_id, false) => CharState::TooManyOxigen,
        (oxygen_id, true) => CharState::WantOxigen,
        (pee_id, false) => CharState::WantPee,
        (thirst_id, false) => CharState::WantDrink,
        _ => CharState::Idle,
    }
}
