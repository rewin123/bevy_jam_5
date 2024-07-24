use crate::game::daycycle::GameTime;
use bevy::prelude::*;
use rand::prelude::*;
use rand_distr::{Distribution, Poisson};

use super::{
    assets::{HandleMap, SceneKey},
    character::DestinationTarget,
    components::fire::InFire,
    selectable::Selectable,
    spawn::{player::Player, spawn_commands::MetalTrashPile},
};

#[derive(Resource, Debug)]
pub struct TroublePlanner {
    pub peace_time: f32,
    pub distribution: f32,
}
pub const DEFAULT_PEACE_TIME: f32 = 5.0;
pub const DEFAULT_DISTRIBUTION: f32 = 10.0;

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(TroublePlanner {
        peace_time: DEFAULT_PEACE_TIME,
        distribution: DEFAULT_DISTRIBUTION,
    });

    app.add_systems(Update, plan_trouble);
    app.add_systems(Update, fix_trouble);
    app.add_systems(PostUpdate, tick_fire);
}

fn plan_trouble(
    mut commands: Commands,
    mut trouble_planner: ResMut<TroublePlanner>,
    time: Res<GameTime>,
    q_selectable: Query<Entity, (With<Selectable>, Without<InFire>, Without<MetalTrashPile>)>,
) {
    trouble_planner.peace_time -= time.delta_seconds();

    if trouble_planner.peace_time <= 0.0 {
        let items = q_selectable.iter().collect::<Vec<_>>();
        if !items.is_empty() {
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..items.len());

            commands.entity(items[index]).insert(InFire {
                fire_created: false,
                started_at: time.elapsed_seconds(),
            });
        }

        let poi = Poisson::new(trouble_planner.distribution).unwrap();
        let v = poi.sample(&mut rand::thread_rng());
        trouble_planner.peace_time = v;
        trouble_planner.distribution *= 0.99;
    }
}

fn fix_trouble(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &DestinationTarget), With<Player>>,
    q_items_in_fire: Query<Entity, With<InFire>>,
) {
    for (transform, target) in query.iter_mut() {
        let player_position = transform.translation;
        let target_position = target.target_pos;

        let distance = player_position.distance(target_position);

        if distance <= target.accept_radius {
            for items_in_fire in q_items_in_fire.iter() {
                if target.target == items_in_fire {
                    commands.entity(target.target).remove::<InFire>();
                }
            }
        }
    }
}

// Fire will destroy things if they are burning for X amount of time
fn tick_fire(
    mut commands: Commands,
    scene_handler: Res<HandleMap<SceneKey>>,
    q_items_in_fire: Query<(Entity, &InFire, &Transform)>,
    gametime: Res<GameTime>,
) {
    for (entity, fire, transform) in q_items_in_fire.iter() {
        if fire.time_ended(gametime.elapsed_seconds()) {
            commands
                .spawn(SceneBundle {
                    scene: scene_handler[&SceneKey::MetalTrash].clone_weak(),
                    transform: *transform,
                    ..default()
                })
                .insert(Selectable)
                .insert(MetalTrashPile);
            commands.entity(entity).despawn_recursive();
        }
    }
}
