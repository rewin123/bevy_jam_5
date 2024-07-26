use bevy::prelude::*;

use super::{
    character::CharacterStates, components::kitchen::Kitchen, daycycle::GameTime, resources::Food,
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
    // todo : implement the update on the work done in the kitchen
}