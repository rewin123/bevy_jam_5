use bevy::prelude::*;

pub mod map_generator;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, update_map);
    app.add_plugins(map_generator::plugin);
}

#[derive(Clone)]
pub enum Tile {
    Wall,
    Floor,
    Nothing,
}

#[derive(Component)]
pub struct ShipMap {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl ShipMap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            tiles: vec![Tile::Nothing; width * height],
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn tile(&self, x: usize, y: usize) -> &Tile {
        &self.tiles[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, tile: Tile) {
        self.tiles[y * self.width + x] = tile
    }
}

fn update_map(
    mut commands: Commands,
    mut q_maps: Query<(Entity, &ShipMap, Option<&Children>), Changed<ShipMap>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, map, children) in &mut q_maps {
        info!("Updating map {}", entity);

        let cube = meshes.add(Cuboid::new(1.0, 2.0, 1.0));
        let material = materials.add(StandardMaterial {
            base_color: Color::linear_rgb(0.8, 0.7, 0.6),
            ..default()
        });

        let floor = meshes.add(Cuboid::new(1.0, 0.1, 1.0));

        let wall_pbr = PbrBundle {
            mesh: cube.clone(),
            material: material.clone(),
            ..default()
        };

        let floor_pbr = PbrBundle {
            mesh: floor.clone(),
            material: material.clone(),
            ..default()
        };

        // Despawn old children
        if let Some(children) = children {
            for child in children.iter() {
                commands.entity(*child).despawn_recursive();
            }
        }

        // Spawn map
        for y in 0..map.height {
            for x in 0..map.width {
                let tile = &map.tiles[y * map.width + x];
                match tile {
                    Tile::Wall => {
                        let id = commands
                            .spawn(wall_pbr.clone())
                            .insert(Transform::from_translation(Vec3::new(
                                x as f32, 1.0, y as f32,
                            )))
                            .id();
                        commands.entity(entity).add_child(id);
                    }
                    Tile::Floor => {
                        let id = commands
                            .spawn(floor_pbr.clone())
                            .insert(Transform::from_translation(Vec3::new(
                                x as f32, 0.0, y as f32,
                            )))
                            .id();

                        commands.entity(entity).add_child(id);
                    }
                    Tile::Nothing => {}
                }
            }
        }
    }
}
