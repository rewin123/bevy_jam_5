use bevy::{
    audio::{AddAudioSource, PlaybackMode, Volume},
    prelude::*,
};
use bevy_mod_billboard::BillboardTextBundle;

use crate::game::{components::flowup_text::FlowUpText, sequence::NextAction};

use super::{
    assets::{HandleMap, SfxKey},
    character::{CharState, CharacterStates},
    components::kitchen::Kitchen,
    daycycle::GameTime,
    difficult::RACION_SIZE,
    resources::{Food, GameResource, Hungry, Pee},
    sequence::CharacterAction,
};

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(KitchenWorkConfig {
        work_time: 0.25,
        amount_after_work: 10.0,
        multiplier: 1,
        last_updated: 0.0,
    });
    app.add_systems(Update, update_work_in_kitchen);
}

#[derive(Resource)]
pub struct KitchenWorkConfig {
    pub work_time: f32,
    pub amount_after_work: f32,
    pub multiplier: i32,
    pub last_updated: f32,
}

#[derive(Component, Default)]
pub struct KitchenWork {
    pub work_time: f32,
}

pub struct KitchenWorkAction(pub Handle<AudioSource>);

impl CharacterAction for KitchenWorkAction {
    fn trigger_start(&self, commands: &mut Commands, target: Entity) {
        commands
            .entity(target)
            .insert(KitchenWork::default())
            .insert(AudioBundle {
                source: self.0.clone_weak(),
                settings: PlaybackSettings {
                    mode: PlaybackMode::Remove,
                    volume: Volume::new(1.0),
                    ..Default::default()
                },
                ..default()
            });
    }

    fn terminate(&self, commands: &mut Commands, target: Entity) {
        commands.entity(target).remove::<KitchenWork>();
    }
}

pub fn update_work_in_kitchen(
    mut commands: Commands,
    time: Res<GameTime>,
    mut kitchen_work_config: ResMut<KitchenWorkConfig>,
    mut q_kitchen_work: Query<(Entity, &mut KitchenWork, &mut CharacterStates)>,
    q_kitchen: Query<&GlobalTransform, With<Kitchen>>,
    mut hungry: ResMut<Hungry>,
    mut food: ResMut<Food>,
    mut pee: ResMut<Pee>,
    sounds: Res<HandleMap<SfxKey>>,
) {
    for (entity, mut kitchen_work, mut states) in q_kitchen_work.iter_mut() {
        states.add(CharState::Working);

        kitchen_work.work_time += time.delta_seconds();
        if kitchen_work.work_time > kitchen_work_config.work_time {
            let current_time = time.elapsed_seconds();

            kitchen_work_config.last_updated = current_time;
            // todo : Add poop so that you need to go to the toilet again. Recycle so that you can produce food
            // todo: complete so that you can change ressource

            commands.entity(entity).remove::<KitchenWork>();
            commands.trigger_targets(NextAction, entity);

            if food.amount() > RACION_SIZE {
                food.decrease(RACION_SIZE);
                hungry.set_amount(0.0);
                pee.increase(RACION_SIZE / 2.0);

                if let Ok(pc_transform) = q_kitchen.get_single() {
                    let text_style = TextStyle {
                        color: Color::linear_rgb(0.0, 1.0, 0.0),
                        font_size: 94.0,
                        ..default()
                    };
                    commands
                        .spawn(BillboardTextBundle {
                            transform: Transform::from_translation(pc_transform.translation())
                                .with_scale(Vec3::splat(0.01)),
                            text: Text::from_section("Eating", text_style),
                            ..default()
                        })
                        .insert(FlowUpText { lifetime: 1.0 })
                        .insert(AudioBundle {
                            source: sounds[&SfxKey::Eating].clone_weak(),
                            ..default()
                        });
                }
            } else {
                if let Ok(pc_transform) = q_kitchen.get_single() {
                    let text_style = TextStyle {
                        color: Color::linear_rgb(0.0, 1.0, 0.0),
                        font_size: 94.0,
                        ..default()
                    };
                    commands
                        .spawn(BillboardTextBundle {
                            transform: Transform::from_translation(pc_transform.translation())
                                .with_scale(Vec3::splat(0.01)),
                            text: Text::from_section("Not Enough Food", text_style),
                            ..default()
                        })
                        .insert(FlowUpText { lifetime: 1.0 })
                        .insert(AudioBundle {
                            source: sounds[&SfxKey::NotEnoughResource].clone_weak(),
                            ..default()
                        });
                }
            }
        }
    }
}
