use std::{any::Any, sync::Arc};
use bevy::prelude::*;

pub struct TypedComponentHolder {
    pub val : Box<dyn Any + Send + Sync>,
    pub insert_fn: Box<dyn Fn(&mut EntityWorldMut, Box<dyn Any + Send + Sync>) + Send + Sync>,
    pub write_fn: Box<dyn Fn(&mut EntityWorldMut, Box<dyn Any + Send + Sync>) + Send + Sync>,
    pub remove_fn: Arc<dyn Fn(&mut EntityWorldMut) + Send + Sync>,
}

pub struct StupidBox<T> {
    pub val : T
}

impl <T> StupidBox<T> {
    fn new(val : T) -> Self {
        Self {
            val
        }
    }

    fn into_inner(self) -> T {
        self.val
    }
}


impl TypedComponentHolder {
    pub fn new<T : Component>(val : T) -> Self {
        Self {
            val : Box::new(StupidBox::new(val)),
            insert_fn : Box::new(|e, val| {
                let val = val.downcast::<StupidBox<T>>().unwrap();
                e.insert(val.into_inner());
            }),
            write_fn : Box::new(|e, val| {
                let val = val.downcast::<StupidBox<T>>().unwrap();
                *e.get_mut::<T>().unwrap() = val.into_inner();
            }),
            remove_fn : Arc::new(|e| {
                e.remove::<T>();
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Component, PartialEq, Debug, Clone)]
    struct TestComponent(u32);

    #[test]
    fn test_new() {
        let component = TestComponent(42);
        let holder = TypedComponentHolder::new(component.clone());

        let boxed_component = holder.val.downcast_ref::<StupidBox<TestComponent>>().unwrap();
        assert_eq!(boxed_component.val, component);
    }

    #[test]
    fn test_insert() {
        let mut world = World::new();
        world.init_component::<TestComponent>();
        let mut entity = world.spawn_empty();

        let component = TestComponent(42);
        let holder = TypedComponentHolder::new(component.clone());

        (holder.insert_fn)(&mut entity, holder.val);

        assert!(entity.contains::<TestComponent>());
        assert_eq!(*entity.get::<TestComponent>().unwrap(), component);
    }

    #[test]
    fn test_write() {
        let mut world = World::new();
        world.init_component::<TestComponent>();
        let mut entity = world.spawn_empty();

        let component = TestComponent(42);
        entity.insert(component);

        let new_component = TestComponent(24);
        let holder = TypedComponentHolder::new(new_component.clone());

        (holder.write_fn)(&mut entity, holder.val);

        assert!(entity.contains::<TestComponent>());
        assert_eq!(*entity.get::<TestComponent>().unwrap(), new_component);
    }

    #[test]
    fn test_remove() {
        let mut world = World::new();
        world.init_component::<TestComponent>();
        let mut entity = world.spawn_empty();

        let component = TestComponent(42);
        entity.insert(component.clone());

        let holder = TypedComponentHolder::new(component);

        assert!(entity.contains::<TestComponent>());

        (holder.remove_fn)(&mut entity);

        assert!(!entity.contains::<TestComponent>());
    }
}