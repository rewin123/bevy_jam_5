#![allow(unused)]

use bevy::{prelude::*, transform::commands};
use bevy_mod_billboard::{
    Billboard, BillboardTextBundle, BillboardTextureBundle, BillboardTextureHandle,
};

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(PostUpdate, sync_inner);
    app.add_systems(Update, (manage_billboard, sync_billboard_position));
}

#[derive(Component)]
pub struct BillboardSpawner {
    pub content: BillboardContent,
    pub size: Vec2,
}

#[derive(Component)]
struct BillboardInner {
    pub billboard: Option<Entity>,
}

pub enum BillboardContent {
    Text(Text),
    Image(Handle<Image>),
    None,
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
) {
    for (billboard, mut inner) in q_billboards.iter_mut() {
        let target = inner.billboard;
        if let Some(target) = target {
            commands.entity(target).despawn_recursive();
        }
        inner.billboard = None;

        match &billboard.content {
            BillboardContent::Text(data) => {
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
    mut q_billboards: Query<&mut Transform, With<Billboard>>,
    mut q_src: Query<(&GlobalTransform, &BillboardInner), Without<Billboard>>,
) {
    for (transform, billboard) in q_src.iter_mut() {
        let Some(bill) = billboard.billboard else {
            continue;
        };
        let Ok(mut bill_transform) = q_billboards.get_mut(bill) else {
            continue;
        };
        bill_transform.translation = transform.translation() + Vec3::new(0.0, 2.0, 0.0);
    }
}
