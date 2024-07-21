//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{dev_tools::states::log_transitions, prelude::*};
use node_tree::tree::NodeTree;

use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    // Print state transitions in dev builds

    app.add_systems(Update, log_transitions::<Screen>);
}


#[derive(Resource)]
pub struct DebugPanel {
    pub data : Vec<(String, Box<dyn Fn() -> NodeTree + Send + Sync>)>
}

#[derive(Component)]
struct DebugPanelMarker;

impl DebugPanel {
    fn add(&mut self, name : String, tree : NodeTree) {
        self.data.push((name, Box::new(move || tree)));
    }
}

fn clear_debug_panel(mut debug_panel : ResMut<DebugPanel>) {
    debug_panel.data.clear();
}


fn apply_debug_panel(
    mut commands : Commands,
    mut debug_panel : ResMut<DebugPanel>,
    mut debug_panel_query : Query<Entity, With<DebugPanelMarker>>,
) {
    let mut top_node = NodeTree::default()
        .with_bundle(NodeBundle {
            style: Style { 
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                right: Val::Px(100.0),
                top: Val::Px(0.0),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::linear_rgba(0.1, 0.1, 0.1, 0.5)),
            border_color: BorderColor(Color::WHITE),
            border_radius: BorderRadius::all(Val::Px(5.0)),
            ..default()
        });

    debug_panel.data.sort_by_key(|(name, _)| name.clone());

    for (name, tree) in debug_panel.data {
        top_node = top_node.with_child(tree);
    }


}

