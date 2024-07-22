use bevy::prelude::*;
use bevy_quill::View;

use super::components::pc::Pc;
use super::selectable::CloseContextMenu;
use super::selectable::OpenContextMenu;
use super::spawn::level::SpawnLevel;

pub mod components;
mod computer_menu;
pub mod constants;
mod context_menu;
mod root;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<SelectedItem>();
    app.add_event::<StartWorking>();
    app.observe(spawn_root_ui);
    app.observe(open_context);
    app.observe(set_context_menu_position);
    app.observe(clear_context_menu_position);
}

#[derive(Event, Debug)]
pub(super) struct StartWorking;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ResourceType {
    Computer,
    // Placeholder
    Unknown,
}
#[derive(Resource, Default)]
struct SelectedItem {
    // Wrap everything in an Option because on Quill we can't query for `Option<Res<...>>`
    item: Option<(Entity, Vec2, ResourceType)>,
}

fn spawn_root_ui(
    _trigger: Trigger<SpawnLevel>,
    camera_q: Query<Entity, With<IsDefaultUiCamera>>,
    mut commands: Commands,
) {
    let Ok(entity) = camera_q.get_single() else {
        return;
    };

    commands.spawn(root::RootUi { camera: entity }.to_root());
}

fn clear_context_menu_position(
    _trigger: Trigger<CloseContextMenu>,
    mut context_menu: ResMut<SelectedItem>,
) {
    context_menu.item = None;
}

#[derive(Event)]
pub struct OpenContext(ResourceType);

fn open_context(
    trigger: Trigger<OpenContextMenu>,
    computers_q: Query<&Pc>,
    mut commands: Commands,
) {
    let entity = trigger.entity();
    if computers_q.contains(entity) {
        commands.trigger_targets(OpenContext(ResourceType::Computer), entity);
    } else {
        commands.trigger_targets(OpenContext(ResourceType::Unknown), entity);
    }
}

// Set the position for the context Menu
fn set_context_menu_position(
    trigger: Trigger<OpenContext>,
    global_transform_q: Query<&GlobalTransform, Without<IsDefaultUiCamera>>,
    camera_q: Query<(Entity, &Camera, &GlobalTransform), With<IsDefaultUiCamera>>,
    mut context_menu: ResMut<SelectedItem>,
) {
    let entity = trigger.entity();
    let Ok((_, camera, camera_transform)) = camera_q.get_single() else {
        return;
    };
    let Ok(transform) = global_transform_q.get(entity) else {
        return;
    };

    let Some(position) = camera.world_to_viewport(camera_transform, transform.translation()) else {
        return;
    };
    context_menu.item = Some((entity, position, trigger.event().0));
}
