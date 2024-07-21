use bevy::prelude::*;
use bevy_mod_outline::*;

use super::selectable::{OnDeselect, OnSelect};


pub(crate) fn plugin(app: &mut App) {
    app.add_plugins((OutlinePlugin, AsyncSceneInheritOutlinePlugin, AutoGenerateOutlineNormalsPlugin));

    app.observe(add_highlight);
    app.observe(remove_highlight);
}


fn add_highlight(trigger: Trigger<OnSelect>, mut commands: Commands, q_children: Query<(&Children)>) {
    let entity = trigger.entity();
    recursive_add_highlight(entity, &mut commands, &q_children);
}

fn recursive_add_highlight(entity : Entity, commands: &mut Commands, q_children: &Query<(&Children)>) {
    commands.entity(entity)
        .insert(
            OutlineBundle {
                outline: OutlineVolume { visible: true, width: 3.0, colour: Color::linear_rgba(1.0, 1.0, 0.0, 0.7) },
                mode: OutlineMode::RealVertex,
                ..default()
            }
        );

    if let Ok(children) = q_children.get(entity) {
        for child in children.iter() {
            recursive_add_highlight(*child, commands, q_children);
        }
    }    
}

fn remove_highlight(trigger: Trigger<OnDeselect>, mut commands: Commands, q_children: Query<(&Children)>) {
    let entity = trigger.entity();
    recursive_remove_highlight(entity, &mut commands, &q_children);
}

fn recursive_remove_highlight(entity : Entity, commands: &mut Commands, q_children: &Query<(&Children)>) {
    commands.entity(entity)
        .remove::<OutlineBundle>();
    
    if let Ok(children) = q_children.get(entity) {
        for child in children.iter() {
            recursive_remove_highlight(*child, commands, q_children);
        }
    }    
}