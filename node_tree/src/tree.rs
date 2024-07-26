use std::any::{Any, TypeId};

use bevy::{
    ecs::{bundle::DynamicBundle, component::Components, storage::Storages},
    prelude::*,
    utils::HashMap,
};

use crate::{ByteHolder, ComponentHolder, RawComponentHolder, TypedComponentHolder};

#[derive(Default)]
pub struct NodeTree {
    pub components: HashMap<TypeId, ComponentHolder>,
    pub children: Vec<NodeTree>,
    pub register_queue: Vec<Box<dyn Fn(&mut World) + Send + Sync>>,
}

impl NodeTree {
    pub fn with<T: Component + Any>(mut self, component: T) -> Self {
        self.components.insert(
            TypeId::of::<T>(),
            ComponentHolder::Typed(TypedComponentHolder::new(component)),
        );
        self.register_queue.push(Box::new(move |world| {
            world.init_component::<T>();
        }));
        self
    }

    pub fn with_bundle<T: DynamicBundle + Bundle>(mut self, bundle: T) -> Self {
        self.insert_bundle(bundle);
        self
    }

    pub fn insert_bundle<T: DynamicBundle + Bundle>(&mut self, bundle: T) {
        let mut components = Components::default();
        let mut storages = Storages::default();
        let mut local_ids = vec![];
        T::component_ids(&mut components, &mut storages, &mut |c_id| {
            local_ids.push(c_id);
        });

        self.register_queue.push(Box::new(move |world| {
            world.init_bundle::<T>();
        }));

        let mut owning_ptrs = vec![];
        bundle.get_components(&mut |_storage, ptr| {
            let idx = owning_ptrs.len();
            let info = components.get_info(local_ids[idx]).unwrap();
            let layout = info.layout();
            unsafe {
                //create flat array
                let slice = std::slice::from_raw_parts_mut(ptr.as_ptr() as *mut u8, layout.size());
                //move owning to own bytes
                let byte_holder = ByteHolder::from_slice(slice, layout);

                owning_ptrs.push(byte_holder);
            }
        });

        for (idx, ptr) in owning_ptrs.into_iter().enumerate() {
            let id = local_ids[idx];
            let info = components.get_info(id).unwrap();
            let type_id = info.type_id().unwrap();
            let holder = RawComponentHolder::new(ptr, type_id);
            self.components
                .insert(type_id, ComponentHolder::Raw(holder));
        }
    }

    pub fn with_child(mut self, child: impl IntoNodeTree) -> Self {
        self.children.push(child.into_node_tree());
        self
    }

    pub fn with_children(mut self, children: impl IntoIterator<Item = NodeTree>) -> Self {
        self.children.extend(children);
        self
    }

    pub fn contains<T: Component>(&self) -> bool {
        self.components.contains_key(&TypeId::of::<T>())
    }

    pub fn get<T: Component>(&self) -> Option<&T> {
        if let Some(component) = self.components.get(&TypeId::of::<T>()) {
            return component.downcast_ref();
        }
        None
    }
}

pub trait IntoNodeTree {
    fn into_node_tree(self) -> NodeTree;
}

impl IntoNodeTree for NodeTree {
    fn into_node_tree(self) -> NodeTree {
        self
    }
}

impl IntoNodeTree for &str {
    fn into_node_tree(self) -> NodeTree {
        NodeTree::default().with_bundle(TextBundle::from_section(
            self.to_string(),
            TextStyle::default(),
        ))
    }
}

impl IntoNodeTree for String {
    fn into_node_tree(self) -> NodeTree {
        NodeTree::default().with_bundle(TextBundle::from_section(
            self,
            TextStyle::default(),
        ))
    }
}

impl IntoNodeTree for NodeBundle {
    fn into_node_tree(self) -> NodeTree {
        NodeTree::default().with_bundle(self)
    }
}

impl IntoNodeTree for TextBundle {
    fn into_node_tree(self) -> NodeTree {
        NodeTree::default().with_bundle(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Component, PartialEq, Eq, Clone, Debug)]
    struct TestComponent(u32);

    #[derive(Component, PartialEq, Eq, Clone, Debug)]
    struct AnotherComponent(String);

    #[derive(Bundle, PartialEq, Eq, Clone, Debug)]
    struct TestBundle {
        a: TestComponent,
        b: AnotherComponent,
    }

    #[test]
    fn test_with_component() {
        let entity = NodeTree::default().with(TestComponent(42));

        assert!(entity.contains::<TestComponent>());
        assert_eq!(
            entity.get::<TestComponent>().cloned(),
            Some(TestComponent(42))
        );
    }

    #[test]
    fn test_with_bundle() {
        let entity = NodeTree::default().with_bundle(TestBundle {
            a: TestComponent(42),
            b: AnotherComponent("Hello".to_string()),
        });

        assert!(entity.contains::<TestComponent>());
        assert!(entity.contains::<AnotherComponent>());
        assert_eq!(
            entity.get::<TestComponent>().cloned(),
            Some(TestComponent(42))
        );
        assert_eq!(
            entity.get::<AnotherComponent>().cloned(),
            Some(AnotherComponent("Hello".to_string()))
        );
    }

    #[test]
    fn test_with_child() {
        let child = NodeTree::default().with(TestComponent(24));

        let entity = NodeTree::default()
            .with(TestComponent(42))
            .with_child(child);

        assert_eq!(entity.children.len(), 1);
        assert!(entity.children[0].contains::<TestComponent>());
        assert_eq!(
            entity.children[0].get::<TestComponent>().cloned(),
            Some(TestComponent(24))
        );
    }

    #[test]
    fn test_with_children() {
        let child1 = NodeTree::default().with(TestComponent(24));

        let child2 = NodeTree::default().with(AnotherComponent("World".to_string()));

        let entity = NodeTree::default()
            .with(TestComponent(42))
            .with_children(vec![child1, child2]);

        assert_eq!(entity.children.len(), 2);
        assert!(entity.children[0].contains::<TestComponent>());
        assert_eq!(
            entity.children[0].get::<TestComponent>().cloned(),
            Some(TestComponent(24))
        );
        assert!(entity.children[1].contains::<AnotherComponent>());
        assert_eq!(
            entity.children[1].get::<AnotherComponent>().cloned(),
            Some(AnotherComponent("World".to_string()))
        );
    }
}
