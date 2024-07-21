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
    app.observe(spawn_ui);
}

fn spawn_ui(
    _trigger: Trigger<OnSelect>,
    mut commands: Commands,
    global_transform_q: Query<&GlobalTransform, Without<IsDefaultUiCamera>>,
    camera_q: Query<(Entity, &Camera, &GlobalTransform), With<IsDefaultUiCamera>>,
) {
    let Ok((entity, camera, camera_transform)) = camera_q.get_single() else {
        return ();
    };
    let Ok(transform) = global_transform_q.get(_trigger.entity()) else {
        return ();
    };

    let Some(position) = camera.world_to_viewport(camera_transform, transform.translation()) else {
        return ();
    };

    commands.spawn((
        ConfigMenu {
            camera: entity,
            position,
        }
        .to_root(),
        ConfigUiComponent,
    ));
}

#[derive(Component, Clone)]
pub struct ConfigUiComponent;

#[derive(Clone, PartialEq)]
struct ConfigMenu {
    camera: Entity,
    position: Vec2,
}

fn container_style(ss: &mut StyleBuilder) {
    ss.display(Display::Flex)
        .flex_direction(FlexDirection::Column)
        .position(ui::PositionType::Absolute)
        .padding(3)
        .top(0)
        .left(0)
        .width(20)
        .height(20)
        .row_gap(4)
        .background_color(colors::U2);
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
        let click = cx.create_callback(|| {
            info!("Clicked!");
        });
        Element::<NodeBundle>::new()
            .insert_dyn(TargetCamera, self.camera)
            .style_dyn(
                |position, style_builder| {
                    style_builder
                        .flex_direction(FlexDirection::Column)
                        .position(ui::PositionType::Absolute)
                        .padding(3)
                        .top(position.y)
                        .left(position.x)
                        .width(100)
                        .height(100)
                        .row_gap(4)
                        .background_color(colors::U2);
                },
                self.position,
            )
            .children((
                "Variants",
                Element::<NodeBundle>::new()
                    .style(style_row)
                    .children((Button::new().on_click(click).children("Default"),)),
            ))
    }
}
