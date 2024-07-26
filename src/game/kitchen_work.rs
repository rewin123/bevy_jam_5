use bevy::prelude::*;
use bevy_mod_billboard::BillboardTextBundle;

use crate::game::{
    components::flowup_text::FlowUpText, sequence::NextAction, spawn::spawn_commands::SpawnKitchen,
};

use super::{
    character::{CharState, CharacterStates},
    components::kitchen::Kitchen,
    daycycle::GameTime,
    resources::Food,
    sequence::CharacterAction,
};

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(KitchenWorkConfig {
        work_time: 0.25,
        amount_after_work: 5.0,
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

pub struct KitchenWorkAction;

impl CharacterAction for KitchenWorkAction {
    fn trigger_start(&self, commands: &mut Commands, target: Entity) {
        commands.entity(target).insert(KitchenWork::default());
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
    mut food: ResMut<Food>,
    q_kitchen: Query<&GlobalTransform, With<Kitchen>>,
) {
    for (entity, mut kitchen_work, mut states) in q_kitchen_work.iter_mut() {
        states.add(CharState::Working);

        kitchen_work.work_time += time.delta_seconds();
        if kitchen_work.work_time > kitchen_work_config.work_time {
            // todo : Add poop so that you need to go to the toilet again. Recycle so that you can produce food
            // todo: complete so that you can change ressource

            info!("Doing Kitchen Work");
            commands.entity(entity).remove::<KitchenWork>();
            commands.trigger_targets(NextAction, entity);

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
                        text: Text::from_section(format!("Making Food"), text_style),
                        ..default()
                    })
                    .insert(FlowUpText { lifetime: 1.0 });
            }
        }
    }
}
