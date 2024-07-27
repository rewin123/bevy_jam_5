use crate::game::assets::SfxKey;
use crate::game::components::flowup_text::FlowUpText;

use super::assets::HandleMap;
use super::character::{CharState, CharacterStates};
use super::components::pc::Pc;
use super::daycycle::GameTime;
use super::debt::Debt;
use super::sequence::{CharacterAction, NextAction};
use bevy::prelude::*;
use bevy_mod_billboard::BillboardTextBundle;

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(PcWorkConfig {
        work_time: 0.25,
        amount_after_work: 10.0,
        multiplier: 1,
        last_updated: 0.0,
    });

    app.add_systems(Update, update_pc_work);
}

#[derive(Resource)]
pub struct PcWorkConfig {
    pub work_time: f32,
    pub amount_after_work: f32,
    pub multiplier: i32,
    pub last_updated: f32,
}

#[derive(Component, Default)]
pub struct PcWork {
    pub work_time: f32,
}

pub struct PcWorkAction;

impl CharacterAction for PcWorkAction {
    fn trigger_start(&self, commands: &mut Commands, target: Entity) {
        commands.entity(target).insert(PcWork::default());
    }

    fn terminate(&self, commands: &mut Commands, target: Entity) {
        commands.entity(target).remove::<PcWork>();
    }
}

fn update_pc_work(
    mut commands: Commands,
    time: Res<GameTime>,
    mut q_pc_work: Query<(Entity, &mut PcWork, &mut CharacterStates)>,
    mut work_config: ResMut<PcWorkConfig>,
    mut debt: ResMut<Debt>,
    q_pcs: Query<&GlobalTransform, With<Pc>>,
    sounds: Res<HandleMap<SfxKey>>,
) {
    for (entity, mut pc_work, mut states) in q_pc_work.iter_mut() {
        states.add(CharState::Working);

        pc_work.work_time += time.delta_seconds();
        if pc_work.work_time >= work_config.work_time {
            let current_time = time.elapsed_seconds();
            if current_time - work_config.last_updated < 1.0 {
                work_config.multiplier += 1;
            } else {
                work_config.multiplier = 1;
            }
            work_config.last_updated = current_time;

            info!("Debt decreased by {}", work_config.amount_after_work);
            let dept_decrease = work_config.amount_after_work * work_config.multiplier as f32;
            debt.amount -= dept_decrease;
            commands.entity(entity).remove::<PcWork>();
            commands.trigger_targets(NextAction, entity);

            if let Ok(pc_transform) = q_pcs.get_single() {
                let text_style = TextStyle {
                    color: Color::linear_rgb(0.0, 1.0, 0.0),
                    font_size: 94.0,
                    ..default()
                };
                commands
                    .spawn(BillboardTextBundle {
                        transform: Transform::from_translation(pc_transform.translation())
                            .with_scale(Vec3::splat(0.01)),
                        text: Text::from_section(format!("+{}$", dept_decrease), text_style),
                        ..default()
                    })
                    .insert(FlowUpText { lifetime: 1.0 })
                    .insert(AudioBundle {
                        source: sounds[&SfxKey::Coin].clone_weak(),
                        ..default()
                    });
            }
        }
    }
}
