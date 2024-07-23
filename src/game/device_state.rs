use bevy::prelude::*;

use super::bilboard_state::{BillboardContent, BillboardSinPos, BillboardSpawner};

pub struct DevceStatePlugin<T: DeviceState> {
    _phantom: std::marker::PhantomData<T>
}

impl<T: DeviceState> Default for DevceStatePlugin<T> {
    fn default() -> Self {
        Self { _phantom: std::marker::PhantomData }
    }
}

impl<T: DeviceState> Plugin for DevceStatePlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, render_state::<T>);
    }
}


pub trait DeviceState : PartialEq + Component + Clone {
    fn content(&self) -> BillboardContent;
}

#[derive(Component)]
struct Old<T: DeviceState>(T);



fn render_state<T: DeviceState>(
    mut commands: Commands,
    q_devices: Query<(Entity, &T, Option<&Old<T>>), Changed<T>>
) {
    for (e, new, old) in q_devices.iter() {
        let mut need_rerender = false;

        if let Some(old) = old {
            if old.0 != *new {
                need_rerender = true;
            }
        } else {
            need_rerender = true;
        }

        if need_rerender {
            commands.entity(e).insert(Old(new.clone()));

            let spawner = BillboardSpawner {
                content: new.content(),
                size: Vec2::new(1.0, 1.0),
            };

            commands
                .entity(e)
                .insert(spawner)
                .insert(BillboardSinPos);
        }
    }
}