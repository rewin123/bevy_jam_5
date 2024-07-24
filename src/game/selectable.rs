use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, selectable_add);
}

#[derive(Component)]
pub struct Selectable;

#[derive(Component)]
pub struct Selected;

#[derive(Event)]
pub struct OnSelect;

#[derive(Event)]
pub struct OnDeselect;

#[derive(Event)]
pub struct OnMouseOver;

#[derive(Event)]
pub struct OnMouseOut;

fn selectable_add(mut commands: Commands, q_selectable: Query<Entity, Added<Selectable>>) {
    for entity in q_selectable.iter() {
        commands
            .entity(entity)
            .insert(PickableBundle {
                pickable: Pickable {
                    should_block_lower: true,
                    is_hoverable: true,
                },
                ..default()
            })
            .insert(On::<Pointer<Click>>::run(
                |mut event: ListenerMut<Pointer<Click>>,
                 mut commands: Commands,
                 q_selected: Query<Entity, With<Selected>>,
                 q_selectable: Query<Entity, With<Selectable>>| {
                    // Only trigger handle on selecteable entities
                    if !q_selectable.contains(event.listener()) {
                        return;
                    }

                    let is_selected = q_selected.contains(event.listener());

                    // Clear old selections
                    for entity in q_selected.iter() {
                        commands.entity(entity).remove::<Selected>();
                        commands.trigger_targets(OnDeselect, entity);
                        println!("OnDeselect {}", entity);
                    }
                    // If the element wasn't previously selected, marked it as selected
                    if !is_selected {
                        commands.entity(event.listener()).insert(Selected);
                        commands.trigger_targets(OnSelect, event.listener());
                        println!("OnSelect {}", event.listener());
                    }

                    event.stop_propagation();
                },
            ))
            .insert(On::<Pointer<Out>>::run(
                |mut event: ListenerMut<Pointer<Out>>,
                 mut commands: Commands,
                 q_selectable: Query<Entity, With<Selectable>>| {
                    if !q_selectable.contains(event.listener()) {
                        return;
                    }

                    commands.trigger_targets(OnMouseOut, event.listener());
                    info!("OnMouseOut {}", event.listener());
                    event.stop_propagation();
                },
            ))
            .insert(On::<Pointer<Over>>::run(
                |mut event: ListenerMut<Pointer<Over>>,
                 mut commands: Commands,
                 q_selectable: Query<Entity, With<Selectable>>| {
                    if !q_selectable.contains(event.listener()) {
                        return;
                    }

                    commands.trigger_targets(OnMouseOver, event.listener());
                    info!("OnMouseOver {}", event.listener());
                    event.stop_propagation();
                },
            ));
    }
}
