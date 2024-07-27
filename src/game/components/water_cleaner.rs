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
    difficult::RES_LIMIT,
    resources::{BadWater, GameResource, Generate, Water},
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

const WATER_CLEARING_TIME: f32 = 1.5;
const WATER_CLEARING_RATE: f32 = RES_LIMIT / WATER_CLEARING_TIME;

/// Separated values for increase/decrease to allow for difficulty changes
#[derive(Resource)]
pub struct WaterCleanerConfig {
    /// Times it takes for a [`WaterDispenserWorkAction`] takes
    pub work_time: f32,
}

impl Default for WaterCleanerConfig {
    fn default() -> Self {
        Self {
            work_time: WATER_CLEARING_TIME,
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

    mut water_events: EventWriter<Generate<Water>>,
    mut bad_water_events: EventWriter<Generate<BadWater>>,

    q_toilet: Query<&GlobalTransform, With<Toilet>>,
    sounds: Res<HandleMap<SfxKey>>,
) {
    for (entity, mut toilet_work, mut states) in q_toilet_work.iter_mut() {
        states.add(CharState::Working);

        toilet_work.work_time += time.delta_seconds();

        water_events.send(Generate::new(WATER_CLEARING_RATE));
        bad_water_events.send(Generate::new(-WATER_CLEARING_RATE));

        if toilet_work.work_time > water_cleaner_config.work_time || bad_water.amount() <= 0.0 {
            info!(
                "Clean Water: water {}, bad water {}",
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
                            format!(
                                "Clean Water: water {}, bad water {}",
                                water.amount(),
                                bad_water.amount(),
                            ),
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
