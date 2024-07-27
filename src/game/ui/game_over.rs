use bevy::prelude::*;

use crate::game::daycycle::PlayerDied;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PostUpdate, spawn_game_over_screen);
}

#[derive(Component)]
pub(crate) struct GameOverScreen;

fn spawn_game_over_screen(mut commands: Commands, mut death_event: EventReader<PlayerDied>) {
    let Some(event) = death_event.read().next() else {
        return;
    };
    commands
        .spawn((
            Name::new("Game Over Screen"),
            GameOverScreen,
            NodeBundle {
                z_index: ZIndex::Global(10),
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    position_type: PositionType::Absolute,
                    padding: UiRect::all(Val::Px(50.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
                ..default()
            },
        ))
        .with_children(|builder| {
            builder.spawn(
                TextBundle::from_sections([
                    TextSection::new(
                        "You Died\n",
                        TextStyle {
                            font_size: 60.0,
                            ..default()
                        },
                    ),
                    TextSection::new(
                        event.0.clone(),
                        TextStyle {
                            font_size: 40.0,
                            ..default()
                        },
                    ),
                ])
                .with_text_justify(JustifyText::Center),
            );
        });
}
