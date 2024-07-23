use bevy::prelude::*;
use bevy_mod_billboard::BillboardTextBundle;

use crate::game::{components::flowup_text::FlowUpText, sequence::NextAction};

use super::{
    character::{CharState, CharacterStates},
    daycycle::GameTime,
    resources::MetalTrash,
    sequence::CharacterAction,
    spawn::spawn_commands::MetalTrashPile,
};

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(GatherMetalTrashWorkConfig {
        work_time: 0.25,
        amount_after_work: 10.0,
        multiplier: 1,
        last_updated: 0.0,
    });
    app.add_systems(Update, update_gather_metal_work);
}

#[derive(Resource)]
pub struct GatherMetalTrashWorkConfig {
    pub work_time: f32,
    pub amount_after_work: f32,
    pub multiplier: i32,
    pub last_updated: f32,
}

#[derive(Component, Default)]
pub struct GatherMetalTrashWork {
    pub work_time: f32,
}

pub struct GatherMetalWorkAction;

impl CharacterAction for GatherMetalWorkAction {
    fn trigger_start(&self, commands: &mut Commands, target: Entity) {
        commands
            .entity(target)
            .insert(GatherMetalTrashWork::default());
    }

    fn terminate(&self, commands: &mut Commands, target: Entity) {
        commands.entity(target).remove::<GatherMetalTrashWork>();
    }
}

fn update_gather_metal_work(
    mut commands: Commands,
    time: Res<GameTime>,
    mut metal_config: ResMut<GatherMetalTrashWorkConfig>,
    mut q_gather_metal_work: Query<(Entity, &mut GatherMetalTrashWork, &mut CharacterStates)>,
    mut metal_trash: ResMut<MetalTrash>,
    q_metal_trash: Query<&GlobalTransform, With<MetalTrashPile>>,
) {
    for (entity, mut gather_metal_work, mut states) in q_gather_metal_work.iter_mut() {
        states.add(CharState::Working);

        gather_metal_work.work_time += time.delta_seconds();
        if gather_metal_work.work_time >= metal_config.work_time {
            let current_time = time.elapsed_seconds();
            if current_time - metal_config.last_updated < 1.0 {
                metal_config.multiplier += 1;
            } else {
                metal_config.multiplier = 1;
            }
            metal_config.last_updated = current_time;

            info!(
                "Metal trash increased by {}",
                metal_config.amount_after_work
            );
            let metal_trash_collected =
                metal_config.amount_after_work * metal_config.multiplier as f32;
            metal_trash.amount -= metal_trash_collected;

            commands.entity(entity).remove::<GatherMetalTrashWork>();
            commands.trigger_targets(NextAction, entity);

            if let Ok(pc_transform) = q_metal_trash.get_single() {
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
                            format!("+{} Metal Trash", metal_trash_collected),
                            text_style,
                        ),
                        ..default()
                    })
                    .insert(FlowUpText { lifetime: 1.0 });
            }
        }
    }
}
