use crate::game::daycycle::GameTime;
use bevy::prelude::*;
use rand::prelude::*;
use rand_distr::{Distribution, Poisson};

use super::{components::fire::InFire, selectable::Selectable};

#[derive(Resource, Debug)]
pub struct TroublePlanner {
    pub peace_time: f32,
    pub distribution: f32,
}
pub const MIN_DISTRIBUTION: f32 = 5.0;
pub const DEFAULT_PEACE_TIME: f32 = 10.0;
pub const DEFAULT_DISTRIBUTION: f32 = 10.0;
pub const DIFFICULTY_PROGRESSION: f32 = 0.5;

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(TroublePlanner {
        peace_time: DEFAULT_PEACE_TIME,
        distribution: DEFAULT_DISTRIBUTION,
    });

    app.add_systems(Update, plan_trouble);
}

fn plan_trouble(
    mut commands: Commands,
    mut trouble_planner: ResMut<TroublePlanner>,
    time: Res<GameTime>,
    q_selectable: Query<Entity, (With<Selectable>, Without<InFire>)>,
) {
    trouble_planner.peace_time -= time.delta_seconds();

    if trouble_planner.peace_time <= 0.0 {
        let items = q_selectable.iter().collect::<Vec<_>>();
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..items.len());

        commands.entity(items[index]).insert(InFire::default());

        let poi = Poisson::new(trouble_planner.distribution).unwrap();
        let v = poi.sample(&mut rand::thread_rng());
        trouble_planner.peace_time = v;
        if trouble_planner.distribution < MIN_DISTRIBUTION {
            trouble_planner.distribution -= DIFFICULTY_PROGRESSION;
        }
    }
}