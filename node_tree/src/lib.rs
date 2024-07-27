pub mod byte_holder;
pub mod component_holder;
pub mod raw_component_holder;
pub mod styling;
pub mod tree;
pub mod typed_component_holder;

use byte_holder::*;
use component_holder::*;
use raw_component_holder::*;
use styling::Styling;
use tree::*;
use typed_component_holder::*;

use std::{any::TypeId, sync::Arc};

use bevy::{
    app::MainScheduleOrder,
    ecs::{schedule::ScheduleLabel, world::Command},
    prelude::*,
};

pub mod prelude {
    pub use super::{
        byte_holder::*, component_holder::*, raw_component_holder::*, tree::*,
        typed_component_holder::*, InsertNodumEntity, NodumTreePlugin, SpawnSchedule,
    };
}

pub struct NodumTreePlugin;

impl Plugin for NodumTreePlugin {
    fn build(&self, app: &mut App) {
        app.init_schedule(MasterSchedule);
        app.init_schedule(SpawnSchedule);

        app.init_resource::<SpawnedCount>();

        app.world_mut()
            .resource_mut::<MainScheduleOrder>()
            .insert_after(Update, MasterSchedule);

        app.add_systems(MasterSchedule, master_schedule_system);

        app.add_systems(SpawnSchedule, children_cache_system);
    }
}

#[derive(Resource, Default)]
struct SpawnedCount(usize);

fn master_schedule_system(world: &mut World) {
    // info!("Start recursive spawn");
    world.resource_mut::<SpawnedCount>().0 = 0;
    world.run_schedule(SpawnSchedule);

    while world.resource::<SpawnedCount>().0 != 0 {
        // println!("{} entities spawned", world.resource::<SpawnedCount>().0);
        world.resource_mut::<SpawnedCount>().0 = 0;
        world.run_schedule(SpawnSchedule);
    }
}

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct MasterSchedule;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpawnSchedule;

#[derive(Component, Clone)]
struct OldNodumTemplate {
    pub component_types: Vec<TypeId>,
    pub remove_fns: Vec<Arc<dyn Fn(&mut EntityWorldMut) + Send + Sync>>,
}

pub struct InsertNodumEntity {
    pub entity: Entity,
    pub nodum: NodeTree,
}

#[derive(Component)]
struct NodumChildrenCache(Option<Vec<NodeTree>>);

impl Command for InsertNodumEntity {
    fn apply(self, world: &mut World) {
        let NodeTree {
            components,
            children,
            register_queue,
        } = self.nodum;

        for f in register_queue {
            f(world);
        }

        world.resource_mut::<SpawnedCount>().0 += 1;

        let new_component_types = components.iter().map(|(k, _)| *k).collect::<Vec<_>>();

        let components_ids = new_component_types
            .iter()
            .map(|k| world.components().get_id(*k).unwrap())
            .collect::<Vec<_>>();

        let existed = world.entity(self.entity).get::<OldNodumTemplate>().cloned();

        let existed_ids = existed
            .as_ref()
            .map(|existed| {
                existed
                    .component_types
                    .iter()
                    .map(|k| world.components().get_id(*k).unwrap())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let mut e = world.entity_mut(self.entity);
        let mut new_template = OldNodumTemplate {
            component_types: new_component_types.clone(),
            remove_fns: Vec::new(),
        };
        components.into_iter().for_each(|(k, v)| match v {
            ComponentHolder::Raw(v) => {
                if e.contains_type_id(k) {
                    let fun = v.write_fn;
                    fun(&mut e, v.val);
                } else {
                    let fun = v.insert_fn;
                    fun(&mut e, v.val);
                }
                new_template.remove_fns.push(v.remove_fn);
            }
            ComponentHolder::Typed(v) => {
                if e.contains_type_id(k) {
                    let fun = v.write_fn;
                    fun(&mut e, v.val);
                } else {
                    let fun = v.insert_fn;
                    fun(&mut e, v.val);
                }
                new_template.remove_fns.push(v.remove_fn);
            }
        });

        if let Some(existed) = e.get::<OldNodumTemplate>().cloned() {
            for (idx, id) in existed_ids.iter().enumerate() {
                if !components_ids.contains(&id) {
                    let fun = existed.remove_fns[idx].clone();
                    fun(&mut e);
                }
            }
        }

        e.insert(new_template);
        e.insert(NodumChildrenCache(Some(children)));
    }
}

fn children_cache_system(
    mut commands: Commands,
    mut q_cache: Query<(Entity, &mut NodumChildrenCache, Option<&Children>)>,
    mut spawned_count: ResMut<SpawnedCount>,
) {
    // info!("Start child cache unwrap");
    for (entity, mut cache, children) in q_cache.iter_mut() {
        // info!("Unpacking {}", entity);
        let children_nodums = cache.0.take();
        if let Some(children_nodums) = children_nodums {
            let children_count = children_nodums.len();
            for (idx, nodum) in children_nodums.into_iter().enumerate() {
                let child;
                if let Some(children) = children {
                    child = match children.get(idx) {
                        Some(child) => *child,
                        None => {
                            let child = commands.spawn_empty().id();
                            commands.entity(entity).add_child(child);
                            child
                        }
                    };
                } else {
                    child = commands.spawn_empty().id();
                    commands.entity(entity).add_child(child);
                }
                // info!("{} {}", entity, child);
                commands.add(InsertNodumEntity {
                    entity: child,
                    nodum: nodum,
                });
            }

            if let Some(children) = children {
                if children.len() > children_count {
                    for child in children.iter().skip(children_count) {
                        commands.entity(*child).despawn_recursive();
                    }
                }
            }
        }
        commands.entity(entity).remove::<NodumChildrenCache>();
        spawned_count.0 += 1;
    }
}


pub fn div() -> NodeTree {
    NodeBundle::default()
        .into_node_tree()
        .with_width(Val::Percent(100.0))
        .with_height(Val::Percent(100.0))
}
