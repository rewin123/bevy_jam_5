use bevy::prelude::*;
use bevy_quill::View;

use super::resources::Water;
use super::selectable::Computer;
use super::selectable::OnDeselect;
use super::selectable::OnSelect;
use super::spawn::level::SpawnLevel;

mod computer_menu;
mod context_menu;
mod root;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<SelectedItem>();
    app.observe(spawn_root_ui);
    app.observe(open_context);
    app.observe(set_context_menu_position);
    app.observe(clear_context_menu_position);
    // app.observe(spawn_ui);
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ResourceType {
    Computer,
    // Placeholder
    Unknown,
}
#[derive(Resource)]
struct SelectedItem {
    // Wrap everything in an Option because on Quill we can't query for `Option<Res<...>>`
    item: Option<(Entity, Vec2, ResourceType)>,
}

impl Default for SelectedItem {
    fn default() -> Self {
        Self { item: None }
    }
}

fn spawn_root_ui(
    _trigger: Trigger<SpawnLevel>,
    camera_q: Query<Entity, With<IsDefaultUiCamera>>,
    mut water: ResMut<Water>,
    mut commands: Commands,
) {
    let Ok(entity) = camera_q.get_single() else {
        return ();
    };
    water.amount = water.amount + 1.0;

    commands.spawn(root::RootUi { camera: entity }.to_root());
}

fn clear_context_menu_position(
    _trigger: Trigger<OnDeselect>,
    mut context_menu: ResMut<SelectedItem>,
) {
    context_menu.item = None;
}

#[derive(Event)]
pub struct OpenContext(ResourceType);

fn open_context(
    _trigger: Trigger<OnSelect>,
    computers_q: Query<&Computer>,
    mut commands: Commands,
) {
    let entity = _trigger.entity();
    if computers_q.contains(entity) {
        commands.trigger_targets(OpenContext(ResourceType::Computer), entity);
    } else {
        commands.trigger_targets(OpenContext(ResourceType::Unknown), entity);
    }
}

// Set the position for the context Menu
fn set_context_menu_position(
    _trigger: Trigger<OpenContext>,
    global_transform_q: Query<&GlobalTransform, Without<IsDefaultUiCamera>>,
    camera_q: Query<(Entity, &Camera, &GlobalTransform), With<IsDefaultUiCamera>>,
    mut context_menu: ResMut<SelectedItem>,
) {
    let entity = _trigger.entity();
    let Ok((_, camera, camera_transform)) = camera_q.get_single() else {
        return ();
    };
    let Ok(transform) = global_transform_q.get(entity) else {
        return ();
    };

    let Some(position) = camera.world_to_viewport(camera_transform, transform.translation()) else {
        return ();
    };
    context_menu.item = Some((entity, position, _trigger.event().0));
}
