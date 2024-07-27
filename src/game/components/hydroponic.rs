use bevy::prelude::*;
use bevy_mod_billboard::BillboardTextBundle;

use crate::game::{
    assets::{HandleMap, SfxKey},
    billboard_state::BillboardContent,
    character::{CharState, CharacterStates, GoToAction},
    daycycle::GameTime,
    device_state::{DeviceState, DeviceStatePlugin},
    difficult::{
        HYDROPONIC_FOOD_PER_HARVEST, HYDROPONIC_OXYGEN_RATE, HYDROPONIC_TIME_TO_FOOD,
        HYDROPONIC_WATER_MAX, HYDROPONIC_WATER_RATE,
    },
    resources::*,
    selectable::OnMouseClick,
    sequence::{ActionGroup, CharacterAction, NewActionSequence, NewMode, NextAction},
    spawn::player::Player,
};

use super::flowup_text::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, update_hydroponic);
    app.add_systems(Update, hydroponic_work);
    app.observe(on_clicked);

    app.add_plugins(DeviceStatePlugin::<HydroponicState>::default());
}

#[derive(Component, PartialEq, Clone)]
pub enum HydroponicState {
    Growing,
    Growed,
    NeedWater(f32),
    Dead,
}

impl DeviceState for HydroponicState {
    fn content(&self) -> BillboardContent {
        match *self {
            HydroponicState::Growing => BillboardContent::None,
            HydroponicState::Growed => BillboardContent::Text(Text::from_section(
                "Grown",
                TextStyle {
                    color: Color::srgb(0.0, 1.0, 0.0),
                    ..default()
                },
            )),
            HydroponicState::NeedWater(time) => BillboardContent::Text(Text::from_section(
                format!("Need water!! {:.0}", time),
                TextStyle {
                    color: Color::srgb(1.0, 0.0, 0.0),
                    ..default()
                },
            )),
            HydroponicState::Dead => BillboardContent::Text(Text::from_section(
                "Dead",
                TextStyle {
                    color: Color::srgb(1.0, 0.0, 0.0),
                    ..default()
                },
            )),
        }
    }
}

#[derive(Component)]
pub struct Hydroponic {
    pub water: f32,
    pub max_water: f32,
    pub time_to_food: f32,
    pub food_per_cycle: f32,
    pub water_consumption_rate: f32,
    pub dead: bool,
}

impl Default for Hydroponic {
    fn default() -> Self {
        Self {
            water: 10.0,
            max_water: HYDROPONIC_WATER_MAX,
            time_to_food: HYDROPONIC_TIME_TO_FOOD,
            food_per_cycle: HYDROPONIC_FOOD_PER_HARVEST * 1.5,
            water_consumption_rate: HYDROPONIC_WATER_RATE,
            dead: false,
        }
    }
}

fn update_hydroponic(
    time: Res<Time>,
    mut query: Query<(&mut HydroponicState, &mut Hydroponic)>,
    mut bad_water: EventWriter<Generate<BadWater>>,
    mut oxygen: EventWriter<Generate<Oxygen>>,
    mut co2: EventWriter<Generate<CarbonDioxide>>,
) {
    for (mut state, mut hydroponic) in query.iter_mut() {
        let dt = time.delta_seconds();

        if hydroponic.dead {
            *state = HydroponicState::Dead;
            continue;
        }

        if hydroponic.water < 0.0 {
            *state = HydroponicState::Dead;
            hydroponic.dead = true;
            hydroponic.time_to_food = HYDROPONIC_TIME_TO_FOOD;
            continue;
        }

        if hydroponic.time_to_food > 0.0 {
            hydroponic.time_to_food -= dt;
            hydroponic.water -= hydroponic.water_consumption_rate * dt;
            bad_water.send(Generate::new(hydroponic.water_consumption_rate * 0.5));
            oxygen.send(Generate::new(HYDROPONIC_OXYGEN_RATE));
            co2.send(Generate::new(-HYDROPONIC_OXYGEN_RATE));
        }

        if hydroponic.water < 3.0 {
            *state =
                HydroponicState::NeedWater(hydroponic.water / hydroponic.water_consumption_rate);
        } else if hydroponic.time_to_food > 0.0 {
            *state = HydroponicState::Growing;
        } else {
            *state = HydroponicState::Growed;
        }
    }
}

const HYDROPONIC_GROUP: &str = "hydroponic";

fn on_clicked(
    trigger: Trigger<OnMouseClick>,
    mut commands: Commands,
    q_players: Query<Entity, With<Player>>,
    mut q_hydroponics: Query<(Entity, &GlobalTransform), With<Hydroponic>>,
) {
    let target = trigger.entity();

    if trigger.event().0 != MouseButton::Left {
        return;
    }

    if let Ok((hydro, hydro_transform)) = q_hydroponics.get_mut(target) {
        let mut actions = ActionGroup::new(HYDROPONIC_GROUP.to_string());

        actions.add(GoToAction {
            target,
            target_pos: hydro_transform.translation(),
        });
        actions.add(HydroponicAction { target });

        commands.trigger_targets(
            NewActionSequence {
                actions,
                mode: NewMode::SoftReplace,
            },
            q_players.iter().collect::<Vec<_>>(),
        );

        info!("Hydroponic working!");
    }
}

pub struct HydroponicAction {
    pub target: Entity,
}

impl CharacterAction for HydroponicAction {
    fn trigger_start(&self, commands: &mut Commands, target: Entity) {
        commands
            .entity(target)
            .insert(HydroponicWork::new(self.target));
    }

    fn terminate(&self, commands: &mut Commands, target: Entity) {
        commands.entity(target).remove::<HydroponicWork>();
    }
}

#[derive(Component)]
pub struct HydroponicWork {
    pub target: Entity,
    pub work_time: f32,
}

impl HydroponicWork {
    pub const fn new(target: Entity) -> Self {
        Self {
            target,
            work_time: 0.5,
        }
    }
}

fn hydroponic_work(
    mut commands: Commands,
    mut q_hydroponics: Query<(&mut HydroponicState, &mut Hydroponic, &GlobalTransform)>,
    mut q_players: Query<(Entity, &mut HydroponicWork, &mut CharacterStates)>,
    time: Res<GameTime>,
    mut water: ResMut<Water>,
    mut food: ResMut<Food>,
    sounds: Res<HandleMap<SfxKey>>,
) {
    for (player_entity, mut work, mut states) in q_players.iter_mut() {
        let Ok((state, mut hydrponic, hydroponic_transform)) = q_hydroponics.get_mut(work.target)
        else {
            commands.entity(player_entity).remove::<HydroponicWork>();
            commands.trigger_targets(NextAction, player_entity);
            continue;
        };

        if work.work_time > 0.0 {
            work.work_time -= time.delta_seconds();
            states.add(CharState::Working);
        } else {
            //refill water
            let dw = hydrponic.max_water - hydrponic.water;
            let free_water = water.amount().min(dw);
            hydrponic.water += free_water;
            water.decrease(free_water);

            let size = 0.01;

            let text_style = TextStyle {
                color: Color::linear_rgb(0.0, 1.0, 0.0),
                font_size: 94.0,
                ..default()
            };

            match *state {
                HydroponicState::Growing => {
                    commands
                        .spawn(BillboardTextBundle {
                            transform: Transform::from_translation(
                                hydroponic_transform.translation() + Vec3::Y,
                            )
                            .with_scale(Vec3::splat(size)),
                            text: Text::from_section("Water refilled".to_string(), text_style),
                            ..default()
                        })
                        .insert(FlowUpText { lifetime: 1.0 })
                        .insert(AudioBundle {
                            source: sounds[&SfxKey::SprayPlant].clone_weak(),
                            ..default()
                        });
                }
                HydroponicState::Growed => {
                    food.increase(hydrponic.food_per_cycle);
                    hydrponic.time_to_food = HYDROPONIC_TIME_TO_FOOD;

                    commands
                        .spawn(BillboardTextBundle {
                            transform: Transform::from_translation(
                                hydroponic_transform.translation() + Vec3::Y,
                            )
                            .with_scale(Vec3::splat(size)),
                            text: Text::from_section(
                                format!("    +{} Food", hydrponic.food_per_cycle),
                                text_style,
                            ),
                            ..default()
                        })
                        .insert(FlowUpText { lifetime: 1.0 })
                        .insert(AudioBundle {
                            source: sounds[&SfxKey::Eating].clone_weak(),
                            ..default()
                        });
                }
                HydroponicState::NeedWater(_) => {
                    commands
                        .spawn(BillboardTextBundle {
                            transform: Transform::from_translation(
                                hydroponic_transform.translation() + Vec3::Y,
                            )
                            .with_scale(Vec3::splat(size)),
                            text: Text::from_section("Water refilled".to_string(), text_style),
                            ..default()
                        })
                        .insert(FlowUpText { lifetime: 1.0 })
                        .insert(AudioBundle {
                            source: sounds[&SfxKey::SprayPlant].clone_weak(),
                            ..default()
                        });
                }
                HydroponicState::Dead => {
                    hydrponic.dead = false;

                    commands
                        .spawn(BillboardTextBundle {
                            transform: Transform::from_translation(
                                hydroponic_transform.translation() + Vec3::Y,
                            )
                            .with_scale(Vec3::splat(size)),
                            text: Text::from_section("Growing start".to_string(), text_style),
                            ..default()
                        })
                        .insert(FlowUpText { lifetime: 1.0 });
                }
            }

            commands.entity(player_entity).remove::<HydroponicWork>();
            commands.trigger_targets(NextAction, player_entity);
        }
    }
}
