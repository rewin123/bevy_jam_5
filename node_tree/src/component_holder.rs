use crate::{RawComponentHolder, StupidBox, TypedComponentHolder};
use bevy::prelude::*;

pub enum ComponentHolder {
    Raw(RawComponentHolder),
    Typed(TypedComponentHolder),
}

impl ComponentHolder {
    pub fn downcast_ref<T : Component>(&self) -> Option<&T> {
        match self {
            ComponentHolder::Raw(holder) => holder.get::<T>(),
            ComponentHolder::Typed(holder) => Some(&holder.val.downcast_ref::<StupidBox<T>>()?.val)
        }
    }
}