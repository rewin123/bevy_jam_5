#![allow(unused)]

use bevy::{prelude::*, transform::commands};
use bevy_mod_billboard::{
    Billboard, BillboardTextBundle, BillboardTextureBundle, BillboardTextureHandle,
};

use super::assets::{FontKey, HandleMap};

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(PostUpdate, sync_inner);
    app.add_systems(
        PreUpdate,
        (manage_billboard, apply_deferred, sync_billboard_position).chain(),
    );
}

#[derive(Component)]
pub struct BillboardSpawner {
    pub content: BillboardContent,
    pub size: Vec2,
}

// Floating billboards by sinus equation
#[derive(Component)]
pub struct BillboardSinPos;

#[derive(Component)]
struct BillboardInner {
    pub billboard: Option<Entity>,
}

pub enum BillboardContent {
    Text(Text),
    Image(Handle<Image>),
    None,
}

impl BillboardContent {
    pub fn time_remaining(seconds_remaining: f32) -> Self {
        BillboardContent::Text(Text::from_section(
            format!("Destroyed in {}", seconds_remaining as i32),
            TextStyle {
                color: Color::linear_rgb(1.0, 0.1, 0.1),
                ..default()
            },
        ))
    }
}

fn sync_inner(
    mut commands: Commands,
    mut q_without_inner: Query<(Entity, &mut BillboardSpawner), Without<BillboardInner>>,
    q_without_spawner: Query<(Entity, &BillboardInner), (Without<BillboardSpawner>)>,
) {
    for (e, mut spawner) in q_without_inner.iter_mut() {
        spawner.set_changed();

        commands
            .entity(e)
            .insert(BillboardInner { billboard: None });
    }

    for (e, inner) in q_without_spawner.iter() {
        if let Some(billboard) = inner.billboard {
            commands.entity(billboard).despawn_recursive();
        }
        commands.entity(e).remove::<BillboardInner>();
    }
}

fn manage_billboard(
    mut commands: Commands,
    mut q_billboards: Query<(&BillboardSpawner, &mut BillboardInner), Changed<BillboardSpawner>>,
    mut meshes: ResMut<Assets<Mesh>>,
    fonts: Res<HandleMap<FontKey>>,
) {
    for (billboard, mut inner) in q_billboards.iter_mut() {
        let target = inner.billboard;
        if let Some(target) = target {
            commands.entity(target).despawn_recursive();
        }
        inner.billboard = None;

        match &billboard.content {
            BillboardContent::Text(data) => {
                let mut data = data.clone();
                for s in data.sections.iter_mut() {
                    s.style.font = fonts.get(&FontKey::Pixel).unwrap().clone_weak();
                }
                let target = commands.spawn(BillboardTextBundle {
                    text: data.clone(),
                    transform: Transform::from_scale(
                        Vec3::new(billboard.size.x, billboard.size.y, 1.0) / 48.0,
                    ),

                    ..default()
                });

                inner.billboard = Some(target.id());
            }
            BillboardContent::Image(data) => {
                let target = commands.spawn(BillboardTextureBundle {
                    texture: BillboardTextureHandle(data.clone()),
                    transform: Transform::from_scale(
                        Vec3::new(billboard.size.x, billboard.size.y, 1.0) / 48.0,
                    ),
                    ..default()
                });

                inner.billboard = Some(target.id());
            }
            BillboardContent::None => {}
        }
    }
}

fn sync_billboard_position(
    mut q_billboards: Query<(&mut Transform), With<Billboard>>,
    mut q_src: Query<
        (&GlobalTransform, &BillboardInner, Option<&BillboardSinPos>),
        Without<Billboard>,
    >,
    time: Res<Time>,
) {
    for (transform, billboard, sin_pos) in q_src.iter_mut() {
        let Some(bill) = billboard.billboard else {
            continue;
        };
        let Ok((mut bill_transform)) = q_billboards.get_mut(bill) else {
            continue;
        };

        if sin_pos.is_some() {
            bill_transform.translation = transform.translation()
                + Vec3::new(0.0, 2.0 + 0.2 * (time.elapsed_seconds() * 2.0).sin(), 0.0);
        } else {
            bill_transform.translation = transform.translation() + Vec3::new(0.0, 2.0, 0.0);
        }
    }
}
