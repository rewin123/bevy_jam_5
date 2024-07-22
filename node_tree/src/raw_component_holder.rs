use crate::ByteHolder;
use bevy::{prelude::*, ptr::OwningPtr};
use std::{
    any::TypeId,
    ptr::NonNull,
    sync::{atomic::Ordering, Arc},
};

pub struct RawComponentHolder {
    pub val: ByteHolder,
    #[allow(dead_code)]
    type_id: TypeId,
    pub insert_fn: Box<dyn Fn(&mut EntityWorldMut, ByteHolder) + Send + Sync>,
    pub write_fn: Box<dyn Fn(&mut EntityWorldMut, ByteHolder) + Send + Sync>,
    pub remove_fn: Arc<dyn Fn(&mut EntityWorldMut) + Send + Sync>,
}

impl RawComponentHolder {
    pub fn new(val: ByteHolder, type_id: TypeId) -> Self {
        Self {
            val,
            type_id,
            insert_fn: Box::new(move |e: &mut EntityWorldMut, data| {
                let c_id = e.world().components().get_id(type_id).unwrap();
                unsafe {
                    e.insert_by_id(
                        c_id,
                        OwningPtr::new(NonNull::new(data.bytes.load(Ordering::SeqCst)).unwrap()),
                    );
                }
            }),
            write_fn: Box::new(move |e, data| {
                let c_id = e.world().components().get_id(type_id).unwrap();
                unsafe {
                    e.insert_by_id(
                        c_id,
                        OwningPtr::new(NonNull::new(data.bytes.load(Ordering::SeqCst)).unwrap()),
                    );
                }
            }),
            remove_fn: Arc::new(move |e| {
                let c_id = e.world().components().get_id(type_id).unwrap();
                e.remove_by_id(c_id);
            }),
        }
    }

    pub fn get<'a, T>(&'a self) -> Option<&T> {
        Some(unsafe { self.val.downcast_ref() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;

    #[derive(Component, Clone, PartialEq, Debug)]
    pub struct Test(u32);

    #[test]
    fn test_new() {
        let value = Test(42);
        let byte_holder = ByteHolder::from_ref(&value);

        let raw_component_holder = RawComponentHolder::new(byte_holder, TypeId::of::<u32>());

        assert_eq!(raw_component_holder.type_id, TypeId::of::<u32>());
        assert!(raw_component_holder.get::<Test>().is_some());
        assert_eq!(*raw_component_holder.get::<Test>().unwrap(), value);
    }

    #[test]
    fn test_insert_and_write() {
        let mut world = World::new();
        world.init_component::<Test>();
        let mut entity = world.spawn_empty();

        let value = Test(32);
        let byte_holder = ByteHolder::from_ref(&value);

        let raw_component_holder = RawComponentHolder::new(byte_holder, TypeId::of::<Test>());

        (raw_component_holder.insert_fn)(&mut entity, raw_component_holder.val.clone());
        assert!(entity.contains::<Test>());
        assert_eq!(*entity.get::<Test>().unwrap(), value);

        let new_value = Test(42);
        let new_byte_holder = ByteHolder::from_ref(&new_value);

        (raw_component_holder.write_fn)(&mut entity, new_byte_holder);
        assert!(entity.contains::<Test>());
        assert_eq!(*entity.get::<Test>().unwrap(), new_value);
    }

    #[test]
    fn test_remove() {
        let mut world = World::new();
        world.init_component::<Test>();
        let mut entity = world.spawn_empty();

        let value = Test(32);
        let byte_holder = ByteHolder::from_ref(&value);

        let raw_component_holder = RawComponentHolder::new(byte_holder, TypeId::of::<Test>());

        (raw_component_holder.insert_fn)(&mut entity, raw_component_holder.val.clone());
        assert!(entity.contains::<Test>());

        (raw_component_holder.remove_fn)(&mut entity);
        assert!(!entity.contains::<Test>());
    }
}
