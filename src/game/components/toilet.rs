use bevy::prelude::*;
use bevy_mod_billboard::BillboardTextBundle;

use crate::game::{
    assets::{HandleMap, SfxKey},
    character::{CharState, CharacterStates, GoToAction},
    components::flowup_text::FlowUpText,
    daycycle::GameTime,
    difficult::RES_LIMIT,
    resources::{BadWater, GameResource, Generate, Pee},
    selectable::OnMouseClick,
    sequence::{ActionGroup, CharacterAction, NewActionSequence, NewMode, NextAction},
    spawn::{player::Player, spawn_commands::Toilet},
};

pub fn plugin(app: &mut App) {
    app.init_resource::<ToiletWorkConfig>();
    app.observe(on_selected);

    app.add_systems(Update, update_pee_work);
}

const TOILET_WORK_GROUP: &str = "toilet_work";

const TOILIET_TIME: f32 = 2.0;
const TOILET_RATE: f32 = RES_LIMIT / TOILIET_TIME;

fn on_selected(
    trigger: Trigger<OnMouseClick>,
    mut commands: Commands,
    q_players: Query<Entity, With<Player>>,
    mut q_pcs: Query<&GlobalTransform, With<Toilet>>,
) {
    let target = trigger.entity();

    if trigger.event().0 != MouseButton::Left {
        return;
    }

    if let Ok(pc_transform) = q_pcs.get_mut(target) {
        let mut actions = ActionGroup::new(TOILET_WORK_GROUP.to_string());

        actions.add(GoToAction {
            target,
            target_pos: pc_transform.translation(),
        });
        actions.add(ToiletWorkAction);

        commands.trigger_targets(
            NewActionSequence {
                actions,
                mode: NewMode::SoftReplace,
            },
            q_players.iter().collect::<Vec<_>>(),
        );

        info!("Pee working!");
    }
}

pub struct ToiletWorkAction;

#[derive(Component, Default)]
pub struct ToiletWork {
    pub work_time: f32,
}

#[derive(Resource)]
pub struct ToiletWorkConfig {
    /// Times it takes for a [`ToiletWorkAction`] takes
    pub work_time: f32,
    /**  
     * Amount a completed [`ToiletWorkAction`] decreases from [`Pee`]
     * and increases from [`BadWater`]
     */
    pub work_decrease: f32,
}

impl Default for ToiletWorkConfig {
    fn default() -> Self {
        Self {
            work_time: TOILIET_TIME,
            work_decrease: 10.0,
        }
    }
}

impl CharacterAction for ToiletWorkAction {
    fn trigger_start(&self, commands: &mut Commands, target: Entity) {
        commands.entity(target).insert(ToiletWork::default());
    }

    fn terminate(&self, commands: &mut Commands, target: Entity) {
        commands.entity(target).remove::<ToiletWork>();
    }
}

fn update_pee_work(
    mut commands: Commands,
    time: Res<GameTime>,
    mut q_toilet_work: Query<(Entity, &mut ToiletWork, &mut CharacterStates)>,
    toilet_work_config: Res<ToiletWorkConfig>,
    mut pee: EventWriter<Generate<Pee>>,
    real_pee: Res<Pee>,
    mut real_toilet: ResMut<crate::game::resources::Toilet>,
    mut toilet: EventWriter<Generate<crate::game::resources::Toilet>>,
    mut bad_water: EventWriter<Generate<BadWater>>,
    q_toilet: Query<&GlobalTransform, With<Toilet>>,
    sounds: Res<HandleMap<SfxKey>>,
) {
    for (entity, mut toilet_work, mut states) in q_toilet_work.iter_mut() {
        states.add(CharState::Peeing);

        toilet_work.work_time += time.delta_seconds();
        pee.send(Generate::new(-TOILET_RATE));
        toilet.send(Generate::new(-TOILET_RATE));
        bad_water.send(Generate::new(TOILET_RATE));

        if toilet_work.work_time > toilet_work_config.work_time || real_pee.amount() <= 0.0 {
            // pee.decrease(toilet_work_config.work_decrease);
            // bad_water.increase(toilet_work_config.work_decrease);
            // toilet.set_amount(0.0);
            // info!(
            //     "Peeing decreased : pee {}, bad water {}",
            //     pee.amount(),
            //     bad_water.amount()
            // );
            real_toilet.set_amount(0.0);
            commands.entity(entity).remove::<ToiletWork>();
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
                            format!("-{} PEE", toilet_work_config.work_decrease),
                            text_style,
                        ),
                        ..default()
                    })
                    .insert(FlowUpText { lifetime: 1.0 })
                    .insert(AudioBundle {
                        source: sounds[&SfxKey::ToiletFlush].clone_weak(),
                        ..default()
                    });
            }
        }
    }
}
