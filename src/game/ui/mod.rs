use bevy::prelude::*;
use bevy::ui;
use bevy_mod_stylebuilder::*;
use bevy_quill::prelude::*;
use bevy_quill::{View, ViewTemplate};
use bevy_quill_obsidian::colors;
use bevy_quill_obsidian::controls::{Button, ButtonVariant, IconButton};
use bevy_quill_obsidian::size::Size;
use bevy_quill_obsidian::RoundedCorners;

use super::selectable::Computer;
use super::selectable::OnDeselect;
use super::selectable::OnSelect;
use super::spawn::level::SpawnLevel;

mod computer_menu;
mod context_menu;

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
    mut commands: Commands,
) {
    let Ok(entity) = camera_q.get_single() else {
        return ();
    };

    commands.spawn((ConfigMenu { camera: entity }.to_root(), ConfigUiComponent));
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

#[derive(Component, Clone)]
pub struct ConfigUiComponent;

#[derive(Clone, PartialEq)]
struct ConfigMenu {
    camera: Entity,
}

fn style_row(ss: &mut StyleBuilder) {
    ss.display(Display::Flex)
        .flex_direction(FlexDirection::Row)
        .align_items(ui::AlignItems::Center)
        .column_gap(4);
}

impl ViewTemplate for ConfigMenu {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let context = cx.use_resource::<SelectedItem>();
        let position = context.item;

        Element::<NodeBundle>::new().children(
            // If the position of the menu is `Some` we show the Context Menu
            // Other wise we show nothing
            Cond::new(position.is_some(), context_menu::ContextMenu, ()),
        )
    }
}
