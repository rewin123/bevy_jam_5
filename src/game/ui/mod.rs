use bevy::prelude::*;
use bevy::ui;
use bevy_mod_stylebuilder::*;
use bevy_quill::prelude::*;
use bevy_quill::{View, ViewTemplate};
use bevy_quill_obsidian::colors;
use bevy_quill_obsidian::controls::{Button, ButtonVariant, IconButton};
use bevy_quill_obsidian::size::Size;
use bevy_quill_obsidian::RoundedCorners;

use super::selectable::OnDeselect;
use super::selectable::OnSelect;
use super::spawn::level::SpawnLevel;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<ContextMenu>();
    app.observe(spawn_root_ui);
    app.observe(set_context_menu_position);
    app.observe(clear_context_menu_position);
    // app.observe(spawn_ui);
}

#[derive(Resource)]
struct ContextMenu {
    position: Option<Vec2>,
    // TODO Add menu type, computer, metal, etc.
}

impl Default for ContextMenu {
    fn default() -> Self {
        Self { position: None }
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
    mut context_menu: ResMut<ContextMenu>,
) {
    context_menu.position = None;
}

// Set the position for the context Menu
fn set_context_menu_position(
    _trigger: Trigger<OnSelect>,
    global_transform_q: Query<&GlobalTransform, Without<IsDefaultUiCamera>>,
    camera_q: Query<(Entity, &Camera, &GlobalTransform), With<IsDefaultUiCamera>>,
    mut context_menu: ResMut<ContextMenu>,
) {
    let Ok((_, camera, camera_transform)) = camera_q.get_single() else {
        return ();
    };
    let Ok(transform) = global_transform_q.get(_trigger.entity()) else {
        return ();
    };

    let Some(position) = camera.world_to_viewport(camera_transform, transform.translation()) else {
        return ();
    };
    context_menu.position = Some(position);
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
        let context = cx.use_resource::<ContextMenu>();
        let position = context.position;

        Element::<NodeBundle>::new().children(
            // If the position of the menu is `Some` we show the Context Menu
            // Other wise we show nothing
            Cond::new(
                position.is_some(),
                Element::<NodeBundle>::new()
                    .insert_dyn(TargetCamera, self.camera)
                    .style_dyn(
                        |position, style_builder| {
                            style_builder
                                .flex_direction(FlexDirection::Column)
                                .position(ui::PositionType::Absolute)
                                .padding(3)
                                // Use the position of the context menu to position the menu
                                .top(position.unwrap().y)
                                .left(position.unwrap().x)
                                .width(100)
                                .height(100)
                                .row_gap(4)
                                .background_color(colors::U2);
                        },
                        context.position,
                    )
                    .children((
                        "Actions",
                        Element::<NodeBundle>::new()
                            .style(style_row)
                            .children((Button::new()
                                .on_click(cx.create_callback(|| info!("clicked Recycle")))
                                .children("Recycle"),)),
                    )),
                (),
            ),
        )
    }
}
