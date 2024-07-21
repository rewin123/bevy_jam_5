use bevy::{ecs::{system::RunSystemOnce, world::Command}, prelude::*};

use crate::game::selectable::Selectable;


pub struct SpawnOxygenGenerator {
    pub pos: Vec3
}

impl Command for SpawnOxygenGenerator {
    fn apply(self, world: &mut World) {
        let scene = world.resource::<AssetServer>().load("models/oxygen_generator.glb#Scene0");

        let bundle = SceneBundle {
            scene,
            transform: Transform::from_translation(self.pos).with_scale(Vec3::splat(0.5)),
            ..default()
        };

        world.spawn(bundle)
            .insert(Selectable);
    }
}



pub struct SpawnHydroponic {
    pub pos: Vec3
}


impl Command for SpawnHydroponic {
    fn apply(self, world: &mut World) {
        let scene = world.resource::<AssetServer>().load("models/hydroponic.glb#Scene0");

        let bundle = SceneBundle {
            scene,
            transform: Transform::from_translation(self.pos).with_scale(Vec3::splat(0.5)),
            ..default()
        };

        world.spawn(bundle)
            .insert(Selectable);
    }
}