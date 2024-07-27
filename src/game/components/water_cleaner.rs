use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};
use bevy_mod_billboard::BillboardTextBundle;

use crate::game::{
    assets::{HandleMap, SfxKey},
    character::{CharState, CharacterStates, GoToAction},
    components::flowup_text::FlowUpText,
    daycycle::GameTime,
    resources::{BadWater, GameResource, Water},
    selectable::OnMouseClick,
    sequence::{ActionGroup, CharacterAction, NewActionSequence, NewMode, NextAction},
    spawn::{
        player::Player,
        spawn_commands::{Toilet, WaterCleaner},
    },
};

pub fn plugin(app: &mut App) {
    app.init_resource::<WaterCleanerConfig>();
    app.observe(on_selected);

    app.add_systems(Update, updated_water_cleaner);
}

const WATER_CLEANER_GROUP: &str = "water_cleaner_work";

fn on_selected(
    trigger: Trigger<OnMouseClick>,
    mut commands: Commands,
    q_players: Query<Entity, With<Player>>,
    mut q_pcs: Query<&GlobalTransform, With<WaterCleaner>>,
    sounds: Res<HandleMap<SfxKey>>,
) {
    let target = trigger.entity();

    if trigger.event().0 != MouseButton::Left {
        return;
    }

    if let Ok(pc_transform) = q_pcs.get_mut(target) {
        let mut actions = ActionGroup::new(WATER_CLEANER_GROUP.to_string());

        actions.add(GoToAction {
            target,
            target_pos: pc_transform.translation(),
        });
        actions.add(WaterCleanerWorkAction(sounds[&SfxKey::Valve].clone_weak()));

        commands.trigger_targets(
            NewActionSequence {
                actions,
                mode: NewMode::SoftReplace,
            },
            q_players.iter().collect::<Vec<_>>(),
        );

        info!("water cleaner working!");
    }
}

pub struct WaterCleanerWorkAction(Handle<AudioSource>);

#[derive(Component, Default)]
pub struct WaterCleanerWork {
    pub work_time: f32,
}

/// Separated values for increase/decrease to allow for difficulty changes
#[derive(Resource)]
pub struct WaterCleanerConfig {
    /// Times it takes for a [`WaterDispenserWorkAction`] takes
    pub work_time: f32,
    // Amount the [`BadWater`] goes decreases
    pub bad_water_down: f32,
    // Amount the [`Water`] goes increases
    pub water_up: f32,
}

impl Default for WaterCleanerConfig {
    fn default() -> Self {
        Self {
            work_time: 0.25,
            bad_water_down: 10.0,
            water_up: 10.0,
        }
    }
}

impl CharacterAction for WaterCleanerWorkAction {
    fn trigger_start(&self, commands: &mut Commands, target: Entity) {
        commands
            .entity(target)
            .insert(WaterCleanerWork::default())
            .insert(AudioBundle {
                source: self.0.clone_weak(),
                settings: PlaybackSettings {
                    mode: PlaybackMode::Remove,
                    volume: Volume::new(3.0),
                    ..Default::default()
                },
                ..default()
            });
    }

    fn terminate(&self, commands: &mut Commands, target: Entity) {
        commands.entity(target).remove::<WaterCleanerWork>();
    }
}

fn updated_water_cleaner(
    mut commands: Commands,
    time: Res<GameTime>,
    mut q_toilet_work: Query<(Entity, &mut WaterCleanerWork, &mut CharacterStates)>,
    water_cleaner_config: Res<WaterCleanerConfig>,
    mut water: ResMut<Water>,
    mut bad_water: ResMut<BadWater>,
    q_toilet: Query<&GlobalTransform, With<Toilet>>,
    sounds: Res<HandleMap<SfxKey>>,
) {
    for (entity, mut toilet_work, mut states) in q_toilet_work.iter_mut() {
        states.add(CharState::Working);

        toilet_work.work_time += time.delta_seconds();
        if toilet_work.work_time > water_cleaner_config.work_time {
            if bad_water.amount() < water_cleaner_config.bad_water_down {
                commands.entity(entity).remove::<WaterCleanerWork>();
                commands.trigger_targets(NextAction, entity);

                if let Ok(pc_transform) = q_toilet.get_single() {
                    let text_style = TextStyle {
                        color: Color::linear_rgb(0.0, 1.0, 1.0),
                        font_size: 94.0,
                        ..default()
                    };
                    commands
                        .spawn(BillboardTextBundle {
                            transform: Transform::from_translation(pc_transform.translation())
                                .with_scale(Vec3::splat(0.01)),
                            text: Text::from_section("Not Enough Bad Water", text_style),
                            ..default()
                        })
                        .insert(FlowUpText { lifetime: 1.0 });
                }
            } else {
                water.increase(water_cleaner_config.water_up);
                bad_water.decrease(water_cleaner_config.bad_water_down);
                info!(
                    "Clean Water: , water {}, bad_water {}",
                    water.amount(),
                    bad_water.amount(),
                );
                commands.entity(entity).remove::<WaterCleanerWork>();
                commands.trigger_targets(NextAction, entity);

                if let Ok(pc_transform) = q_toilet.get_single() {
                    let text_style = TextStyle {
                        color: Color::linear_rgb(0.0, 1.0, 0.0),
                        font_size: 94.0,
                        ..default()
                    };
                    commands
                        .spawn(BillboardTextBundle {
                            transform: Transform::from_translation(pc_transform.translation())
                                .with_scale(Vec3::splat(0.01)),
                            text: Text::from_section(
                                format!("+{} Water", water_cleaner_config.bad_water_down),
                                text_style,
                            ),
                            ..default()
                        })
                        .insert(FlowUpText { lifetime: 1.0 })
                        .insert(AudioBundle {
                            source: sounds[&SfxKey::Water].clone_weak(),
                            ..default()
                        });
                }
            }
        }
    }
}
