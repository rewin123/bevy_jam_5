/*
Nodum is an entity tree that minimizes entity reallocation and automatically applies changes to entity components relative to the previous Nodum state. In essence, Nodum functions as an auto-diff tree. This example demonstrates Nodum's capabilities for automatic entity creation and deletion.

The program displays the elapsed time since the game's start in the top-left corner of the screen. The number of text fields shown cycles between 1 and 5, based on the remainder of the elapsed seconds divided by 5. Thanks to Nodum's features, manual entity deletion or modification is not required. Nodum handles all of this automatically by comparing the new tree structure with the previous one.

Key features demonstrated:
1. Automatic entity management: Nodum creates and removes entities as needed.
2. Time-based UI updates: The display changes every second, showing the elapsed time.
3. Dynamic child count: The number of text fields varies based on the elapsed time.
4. Efficient updates: Nodum only applies necessary changes, minimizing performance overhead.
5. Declarative UI structure: The UI is defined as a tree structure, which Nodum efficiently manages.

This example uses Bevy game engine with the Nodum plugin to create a simple, dynamic UI that updates in real-time, showcasing the power and simplicity of using Nodum for entity management in game development.
*/


use bevy::prelude::*;
use node_tree::{tree::NodeTree, *};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(NodumTreePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, time_text)
        .run();    
}

// Component to mark the entity that will display time
#[derive(Component)]
struct TimeMarker;

// Setup function to initialize the camera and time marker
fn setup(mut commands : Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(TimeMarker);
}

// System to update and display the elapsed time
fn time_text(
    mut commands : Commands,
    time: Res<Time>,
    q_times: Query<Entity, With<TimeMarker>>,
) {
    // Calculate the number of seconds elapsed
    let secs = time.elapsed_seconds() as usize;
    // Determine the number of child elements to display (cycles between 1 and 5)
    let child_count = secs % 5 + 1;

    // Iterate over entities with TimeMarker component
    for e in q_times.iter() {
        let mut children = vec![];
        // Create child Nodum entities with text displaying elapsed time
        for i in 0..child_count {
            children.push(
                NodeTree::default()
                    .with_bundle(TextBundle::from_section(format!("{}", time.elapsed_seconds()), TextStyle::default())),
            );
        }

        // Insert a new Nodum entity as a child of the TimeMarker entity
        commands.add(
            InsertNodumEntity {
                entity: e,
                nodum: NodeTree::default()
                    .with_bundle(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(children),
            }
        );
        println!("spawned");
    }
}