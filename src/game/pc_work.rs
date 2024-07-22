use bevy::prelude::*;
use crate::game::character::CharacterAction;

use super::character::NextAction;
use super::daycycle::GameTime;
use super::debt::Debt;

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(PcWorkConfig {
        work_time: 0.5,
        amount_after_work: 10.0,
    });

    app.add_systems(Update, update_pc_work);
}

#[derive(Resource)]
pub struct PcWorkConfig {
    pub work_time: f32,
    pub amount_after_work: f32,
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
    mut q_pc_work: Query<(Entity, &mut PcWork)>,
    work_config: Res<PcWorkConfig>,
    mut debt : ResMut<Debt>,
) {
    for (entity, mut pc_work) in q_pc_work.iter_mut() {
        pc_work.work_time += time.delta_seconds();
        if pc_work.work_time >= work_config.work_time {
            debt.amount -= work_config.amount_after_work;
            commands.entity(entity).remove::<PcWork>();
            commands.trigger_targets(NextAction, entity);
        }
    }
}