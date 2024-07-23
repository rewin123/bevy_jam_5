use bevy::prelude::*;

use crate::game::auto_anim::{AnimRange, AnimSet, AutoAnim, AutoAnimPlugin};

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(AutoAnimPlugin::<FireSet>::default());

    app.add_systems(PreUpdate, in_fire);
}

#[derive(Default, Component)]
pub struct InFire {
    pub fire_created: bool,
}

#[derive(Component)]
pub struct FireFor(Entity);

fn in_fire(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut q_in_fire: Query<(Entity, &mut InFire, &Transform)>,
    mut fire: Query<(Entity, &mut FireFor)>,
    q_cameras: Query<&Transform, With<Camera>>,
) {
    let Ok(cam_transform) = q_cameras.get_single() else {
        return;
    };

    for (entity, mut in_fire, transform) in q_in_fire.iter_mut() {
        if !in_fire.fire_created {
            in_fire.fire_created = true;

            info!("Spawn fire {}", entity);

            let mesh = meshes.add(Plane3d::new(
                Vec3::new(0.0, 0.0, 1.0).normalize(),
                Vec2::splat(1.5),
            ));
            let material = materials.add(StandardMaterial {
                alpha_mode: AlphaMode::Blend,
                cull_mode: None,
                ..default()
            });

            let fire_transform = Transform::from_translation(transform.translation)
                .looking_at(cam_transform.translation, Vec3::Y);
            // fire_transform.rotate_z(std::f32::consts::FRAC_PI_2);

            commands
                .spawn(SpatialBundle::from_transform(fire_transform))
                .insert(FireFor(entity))
                .with_children(|children| {
                    children
                        .spawn(PbrBundle {
                            mesh: mesh.clone(),
                            material: material.clone(),
                            transform: Transform::from_translation(Vec3::new(0.0, 1.5, -0.5)),
                            ..default()
                        })
                        .insert(AutoAnim::new(FireSet, 0.01));

                    children.spawn(PointLightBundle {
                        transform: Transform::from_translation(Vec3::new(0.0, 1.0, -1.0).normalize()),
                        point_light: PointLight {
                            color: Color::linear_rgb(1.0, 123.0 / 255.0, 0.0),
                            intensity: 1000000.0,
                            range: 2.0,
                            radius: 0.5,
                            ..default()
                        },
                        ..default()
                    });
                });
        }
    }

    for (entity, fire_for) in fire.iter_mut() {
        if !q_in_fire.contains(fire_for.0) {
            commands.entity(entity).despawn_recursive();
        }
    }
}




#[derive(Default)]
pub struct FireSet;

impl AnimSet for FireSet {
    fn get_folder_path() -> String {
        "animations/fire".to_string()
    }

    fn get_index_range(&self) -> crate::game::auto_anim::AnimRange {
        AnimRange::new(0, 18)
    }

    fn get_tile_count() -> usize {
        19
    }
}
