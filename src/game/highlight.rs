use bevy::prelude::*;
use bevy_mod_outline::*;

use super::selectable::{OnDeselect, OnMouseOut, OnMouseOver, OnSelect};

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins((
        OutlinePlugin,
        AsyncSceneInheritOutlinePlugin,
        AutoGenerateOutlineNormalsPlugin,
    ));

    app.observe(add_highlight);
    app.observe(remove_highlight);
}

#[derive(Component)]
pub struct HighlightCounter(i32);

fn add_highlight(
    trigger: Trigger<OnMouseOver>,
    mut commands: Commands,
    q_children: Query<&Children>,
    mut q_counter: Query<&mut HighlightCounter>,
) {
    let entity = trigger.entity();

    let counter_val;
    if let Ok(mut counter) = q_counter.get_mut(entity) {
        counter.0 += 1;
        counter_val = counter.0;
    } else {
        commands.entity(entity).insert(HighlightCounter(1));
        counter_val = 1;
    }

    if counter_val > 0 {
        recursive_add_highlight(entity, &mut commands, &q_children);
    }
}

fn recursive_add_highlight(entity: Entity, commands: &mut Commands, q_children: &Query<&Children>) {
    commands.entity(entity).insert(OutlineBundle {
        outline: OutlineVolume {
            visible: true,
            width: 3.0,
            colour: Color::linear_rgba(1.0, 1.0, 0.0, 0.7),
        },
        mode: OutlineMode::RealVertex,
        ..default()
    });

    if let Ok(children) = q_children.get(entity) {
        for child in children.iter() {
            recursive_add_highlight(*child, commands, q_children);
        }
    }
}

fn remove_highlight(
    trigger: Trigger<OnMouseOut>,
    mut commands: Commands,
    q_children: Query<&Children>,
    mut q_counter: Query<&mut HighlightCounter>,
) {
    let entity = trigger.entity();

    let counter_val;
    if let Ok(mut counter) = q_counter.get_mut(entity) {
        counter.0 -= 1;
        counter_val = counter.0;
    } else {
        counter_val = 0;
    }

    if counter_val <= 0 {
        recursive_remove_highlight(entity, &mut commands, &q_children);
    }
}

fn recursive_remove_highlight(
    entity: Entity,
    commands: &mut Commands,
    q_children: &Query<&Children>,
) {
    commands.entity(entity).remove::<OutlineBundle>();

    if let Ok(children) = q_children.get(entity) {
        for child in children.iter() {
            recursive_remove_highlight(*child, commands, q_children);
        }
    }
}
