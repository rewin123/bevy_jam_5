use bevy::{ecs::world::Command, prelude::*};

use crate::game::{
    assets::{HandleMap, SceneKey},
    components::earth::Earth,
    selectable::Selectable,
};

pub struct SpawnOxygenGenerator {
    pub pos: Vec3,
}

impl Command for SpawnOxygenGenerator {
    fn apply(self, world: &mut World) {
        let scene =
            world.resource::<HandleMap<SceneKey>>()[&SceneKey::OxygenGenerator].clone_weak();

        let bundle = SceneBundle {
            scene,
            transform: Transform::from_translation(self.pos).with_scale(Vec3::splat(0.5)),
            ..default()
        };

        world.spawn(bundle).insert(Selectable);
    }
}

pub struct SpawnHydroponic {
    pub pos: Vec3,
}

impl Command for SpawnHydroponic {
    fn apply(self, world: &mut World) {
        let scene = world.resource::<HandleMap<SceneKey>>()[&SceneKey::Hydroponic].clone_weak();

        let bundle = SceneBundle {
            scene,
            transform: Transform::from_translation(self.pos).with_scale(Vec3::splat(0.5)),
            ..default()
        };

        world.spawn(bundle).insert(Selectable);
    }
}

pub struct SpawnEarth;

impl Command for SpawnEarth {
    fn apply(self, world: &mut World) {
        let scene = world.resource::<HandleMap<SceneKey>>()[&SceneKey::Earth].clone_weak();
        let dir = Vec3::new(-0.0, -1.0, -0.0).normalize();

        let earth_r = 6371000.0; // 6 371 km
        let space_height = 2000000.0; // 1000 km

        let distance = earth_r + space_height;
        let pos = dir * distance;

        let bundle = SceneBundle {
            scene,
            transform: Transform::from_translation(pos)
                .with_scale(Vec3::splat(earth_r))
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            ..default()
        };

        world.spawn(bundle).insert(Earth);
    }
}
