//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{dev_tools::states::log_transitions, prelude::*};
use node_tree::{
    tree::{IntoNodeTree, NodeTree},
    InsertNodumEntity,
};

use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    // Print state transitions in dev builds
    app.add_systems(Update, log_transitions::<Screen>);

    // Add debug panel
    app.init_resource::<DebugPanel>();
    app.add_systems(Startup, setup_debug_panel);
    app.add_systems(PreUpdate, clear_debug_panel);
    app.add_systems(PostUpdate, apply_debug_panel);

    app.add_systems(Update, add_fps);
}

#[derive(Resource, Default)]
pub struct DebugPanel {
    pub data: Vec<(String, NodeTree)>,
}

fn setup_debug_panel(mut commands: Commands) {
    commands.spawn(DebugPanelMarker);
}

#[derive(Component)]
struct DebugPanelMarker;

impl DebugPanel {
    pub fn add(&mut self, name: impl ToString, tree: impl IntoNodeTree) {
        self.data.push((name.to_string(), tree.into_node_tree()));
    }
}

fn clear_debug_panel(mut debug_panel: ResMut<DebugPanel>) {
    debug_panel.data.clear();
}

fn add_fps(mut debug_panel: ResMut<DebugPanel>, time: Res<Time>) {
    debug_panel.add("FPS", format!("FPS: {:.2}", 1.0 / time.delta_seconds()));
}

fn apply_debug_panel(
    mut commands: Commands,
    mut debug_panel: ResMut<DebugPanel>,
    mut debug_panel_query: Query<Entity, With<DebugPanelMarker>>,
) {
    let mut top_node = NodeTree::default()
        .with_bundle(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                border: UiRect::all(Val::Px(1.0)),
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::linear_rgba(0.01, 0.01, 0.01, 0.8)),
            border_color: BorderColor(Color::WHITE),
            border_radius: BorderRadius::all(Val::Px(5.0)),
            ..default()
        })
        .with_child(NodeTree::default().with_bundle(TextBundle::from_section(
            "Debug Panel",
            TextStyle::default(),
        )));

    debug_panel.data.sort_by_key(|(name, _)| name.clone());

    for (_name, tree) in debug_panel.data.drain(..) {
        top_node = top_node.with_child(tree);
    }

    if let Some(entity) = debug_panel_query.iter_mut().next() {
        commands.add(InsertNodumEntity {
            entity,
            nodum: top_node,
        })
    }
}
