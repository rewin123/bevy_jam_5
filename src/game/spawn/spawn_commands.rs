use std::f32::consts::PI;

use bevy::{ecs::world::Command, prelude::*};

use crate::game::{
    assets::{HandleMap, SceneKey},
    character::IgnoreJustMoving,
    components::{
        earth::Earth,
        hydroponic::{Hydroponic, HydroponicState},
        kitchen::Kitchen,
    },
    selectable::Selectable,
};

pub struct SpawnOxygenGenerator {
    pub pos: Vec3,
}

#[derive(Component)]
pub struct OxygenRecyler;

impl Command for SpawnOxygenGenerator {
    fn apply(self, world: &mut World) {
        let scene =
            world.resource::<HandleMap<SceneKey>>()[&SceneKey::OxygenGenerator].clone_weak();

        let bundle = SceneBundle {
            scene,
            transform: Transform::from_translation(self.pos).with_scale(Vec3::splat(0.5)),
            ..default()
        };

        world.spawn(bundle).insert(Selectable).insert(OxygenRecyler);
    }
}

pub struct SpawnKitchen {
    pub pos: Vec3,
}

impl Command for SpawnKitchen {
    fn apply(self, world: &mut World) {
        let scene = world.resource::<HandleMap<SceneKey>>()[&SceneKey::Kitchen].clone_weak();

        let bundle = SceneBundle {
            scene,
            transform: Transform::from_translation(self.pos)
                .with_scale(Vec3::splat(0.2))
                .with_rotation(Quat::from_rotation_y(-PI / 2.0)),
            ..default()
        };

        world.spawn(bundle).insert(Selectable).insert(Kitchen);
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

        world
            .spawn(bundle)
            .insert(Selectable)
            .insert(IgnoreJustMoving)
            .insert(Hydroponic::default())
            .insert(HydroponicState::Growing);
    }
}

#[derive(Component)]
pub struct MetalTrashPile;

pub struct SpawnMetalTrashPile {
    pub pos: Vec3,
}

impl Command for SpawnMetalTrashPile {
    fn apply(self, world: &mut World) {
        let scene = world.resource::<HandleMap<SceneKey>>()[&SceneKey::MetalTrash].clone_weak();

        let bundle = SceneBundle {
            scene,
            transform: Transform::from_translation(self.pos).with_scale(Vec3::splat(0.5)),
            ..default()
        };

        world
            .spawn(bundle)
            .insert(Selectable)
            .insert(MetalTrashPile);
    }
}

pub struct SpawnEarth;

impl Command for SpawnEarth {
    fn apply(self, world: &mut World) {
        let dir = Vec3::new(-0.0, -1.0, -0.0).normalize();
        let earth_r = 6371000.0; // 6 371 km
        let space_height = 2000000.0; // 1000 km
        let distance = earth_r + space_height;
        let pos = dir * distance;

        let sun_id = world
            .spawn(DirectionalLightBundle {
                transform: Transform::from_translation(-Vec3::Z).looking_at(Vec3::ZERO, Vec3::X),
                directional_light: DirectionalLight {
                    shadows_enabled: true,
                    illuminance: 2000.0,
                    ..default()
                },
                ..default()
            })
            .id();

        let scene = world.resource::<HandleMap<SceneKey>>()[&SceneKey::Earth].clone_weak();

        let bundle = SceneBundle {
            scene,
            transform: Transform::from_translation(pos)
                .with_scale(Vec3::splat(earth_r))
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            ..default()
        };

        world.spawn(bundle).insert(Earth).add_child(sun_id);
    }
}

#[derive(Component)]
pub struct Toilet;

pub struct SpawnToilet {
    pub pos: Vec3,
    pub rot: Option<Quat>,
}

impl Command for SpawnToilet {
    fn apply(self, world: &mut World) {
        let scene = world.resource::<HandleMap<SceneKey>>()[&SceneKey::Toilet].clone_weak();
        let bundle = SceneBundle {
            scene,
            transform: Transform::from_translation(self.pos)
                .with_rotation(self.rot.unwrap_or_default())
                .with_scale(Vec3::splat(0.35)),
            ..default()
        };

        world
            .spawn(bundle)
            .insert((Selectable, Toilet, IgnoreJustMoving));
    }
}
#[derive(Component)]
pub struct WaterDispenser;

pub struct SpawnWaterDispenser {
    pub pos: Vec3,
    pub rot: Option<Quat>,
}

impl Command for SpawnWaterDispenser {
    fn apply(self, world: &mut World) {
        let scene = world.resource::<HandleMap<SceneKey>>()[&SceneKey::WaterDispenser].clone_weak();
        let bundle = SceneBundle {
            scene,
            transform: Transform::from_translation(self.pos)
                .with_rotation(self.rot.unwrap_or_default())
                .with_scale(Vec3::splat(0.35)),
            ..default()
        };

        world
            .spawn(bundle)
            .insert((Selectable, WaterDispenser, IgnoreJustMoving));
    }
}

#[derive(Component)]
pub struct WaterCleaner;

pub struct SpawnWaterCleaner {
    pub pos: Vec3,
    pub rot: Option<Quat>,
}

impl Command for SpawnWaterCleaner {
    fn apply(self, world: &mut World) {
        let scene = world.resource::<HandleMap<SceneKey>>()[&SceneKey::WaterCleaner].clone_weak();
        let bundle = SceneBundle {
            scene,
            transform: Transform::from_translation(self.pos)
                .with_rotation(self.rot.unwrap_or_default())
                .with_scale(Vec3::splat(0.30)),
            ..default()
        };

        world
            .spawn(bundle)
            .insert((Selectable, WaterCleaner, IgnoreJustMoving));
    }
}
