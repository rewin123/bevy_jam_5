use bevy::prelude::*;
use bevy_mod_billboard::BillboardTextBundle;

use crate::game::{
    character::{CharState, CharacterStates, GoToAction}, components::flowup_text::FlowUpText, daycycle::GameTime, difficult::RES_LIMIT, resources::{GameResource, Generate, Pee, Thirst, Water}, selectable::OnMouseClick, sequence::{ActionGroup, CharacterAction, NewActionSequence, NewMode, NextAction}, spawn::{
        player::Player,
        spawn_commands::{Toilet, WaterDispenser},
    }
};

pub fn plugin(app: &mut App) {
    app.init_resource::<WaterDispenserConfig>();
    app.observe(on_selected);

    app.add_systems(Update, updated_water_drinking);
}

const WATER_DISPENSER_GROUP: &str = "water_dispenser_work";

fn on_selected(
    trigger: Trigger<OnMouseClick>,
    mut commands: Commands,
    q_players: Query<Entity, With<Player>>,
    mut q_pcs: Query<&GlobalTransform, With<WaterDispenser>>,
) {
    let target = trigger.entity();

    if trigger.event().0 != MouseButton::Left {
        return;
    }

    if let Ok(pc_transform) = q_pcs.get_mut(target) {
        let mut actions = ActionGroup::new(WATER_DISPENSER_GROUP.to_string());

        actions.add(GoToAction {
            target,
            target_pos: pc_transform.translation(),
        });
        actions.add(WaterDispenserWorkAction);

        commands.trigger_targets(
            NewActionSequence {
                actions,
                mode: NewMode::SoftReplace,
            },
            q_players.iter().collect::<Vec<_>>(),
        );

        info!("water dispenser working!");
    }
}

pub struct WaterDispenserWorkAction;

#[derive(Component, Default)]
pub struct WaterDispenserWork {
    pub work_time: f32,
}

/// Separated values for increase/decrease to allow for difficulty changes
#[derive(Resource)]
pub struct WaterDispenserConfig {
    /// Times it takes for a [`WaterDispenserWorkAction`] takes
    pub work_time: f32,
    // Amount the [`Thirst`] goes decreases
    pub thirst_down: f32,
    // Amount the [`Water`] goes decreases
    pub water_down: f32,
    // the [`Pee`] goes increases
    pub pee_up: f32,
}

const DRINK_TIME: f32 = 0.5;
const DRINK_RATE: f32 = RES_LIMIT / DRINK_TIME;
const WATER_SPENT_RATE: f32 = 10.0 / DRINK_TIME;

impl Default for WaterDispenserConfig {
    fn default() -> Self {
        Self {
            work_time: DRINK_TIME,
            thirst_down: 10.0,
            water_down: 10.0,
            pee_up: 10.0,
        }
    }
}

impl CharacterAction for WaterDispenserWorkAction {
    fn trigger_start(&self, commands: &mut Commands, target: Entity) {
        commands
            .entity(target)
            .insert(WaterDispenserWork::default());
    }

    fn terminate(&self, commands: &mut Commands, target: Entity) {
        commands.entity(target).remove::<WaterDispenserWork>();
    }
}

fn updated_water_drinking(
    mut commands: Commands,
    time: Res<GameTime>,
    mut q_toilet_work: Query<(Entity, &mut WaterDispenserWork, &mut CharacterStates)>,
    water_dispenser_config: Res<WaterDispenserConfig>,
    mut water: ResMut<Water>,
    mut pee: ResMut<Pee>,
    mut thirst: ResMut<Thirst>,

    mut pee_events: EventWriter<Generate<Pee>>,
    mut water_events: EventWriter<Generate<Water>>,
    mut thrist_events: EventWriter<Generate<Thirst>>,

    q_toilet: Query<&GlobalTransform, With<Toilet>>,
) {
    for (entity, mut toilet_work, mut states) in q_toilet_work.iter_mut() {
        states.add(CharState::Driking);

        toilet_work.work_time += time.delta_seconds();

        pee_events.send(Generate::new(WATER_SPENT_RATE));
        water_events.send(Generate::new(-WATER_SPENT_RATE));
        thrist_events.send(Generate::new(-DRINK_RATE));


        if toilet_work.work_time > water_dispenser_config.work_time || water.amount() <= 0.0 || thirst.amount() <= 0.0 {
           
            thirst.set_amount(0.0);
            info!(
                "Drinking decreased : thirst {}, water {}, pee {}, limit {:#?}",
                thirst.amount(),
                water.amount(),
                pee.amount(),
                pee.limit(),
            );
            commands.entity(entity).remove::<WaterDispenserWork>();
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
                            format!("- THIRST"),
                            text_style,
                        ),
                        ..default()
                    })
                    .insert(FlowUpText { lifetime: 1.0 });
            }
        }
    }
}
