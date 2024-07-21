use bevy::prelude::*;
use bevy::ui;
use bevy_mod_stylebuilder::*;
use bevy_quill::prelude::*;
use bevy_quill::{View, ViewTemplate};
use bevy_quill_obsidian::colors;
use bevy_quill_obsidian::controls::{Button, ButtonVariant, IconButton};
use bevy_quill_obsidian::size::Size;
use bevy_quill_obsidian::RoundedCorners;

use super::spawn::level::SpawnLevel;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_ui);
}

fn spawn_ui(
    _trigger: Trigger<SpawnLevel>,
    mut commands: Commands,
    camera_q: Query<Entity, With<IsDefaultUiCamera>>,
) {
    let Ok(camera) = camera_q.get_single() else {
        return ();
    };

    commands.spawn(ConfigMenu { camera }.to_root());
}

#[derive(Clone, PartialEq)]
struct ConfigMenu {
    camera: Entity,
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
            .style(container_style)
            .children((
                "Variants",
                Element::<NodeBundle>::new()
                    .style(style_row)
                    .children((Button::new().on_click(click).children("Default"),)),
            ))
    }
}
