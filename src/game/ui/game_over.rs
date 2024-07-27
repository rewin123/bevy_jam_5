use bevy::prelude::*;
use bevy_mod_picking::{
    events::{Click, Pointer},
    prelude::{EntityEvent, ListenerInput, On},
};

use crate::{
    game::daycycle::PlayerDied,
    ui::{
        palette::{
            BUTTON_HOVERED_BACKGROUND, BUTTON_PRESSED_BACKGROUND, BUTTON_TEXT, NODE_BACKGROUND,
        },
        prelude::InteractionPalette,
    },
};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<ResetGame>();
    app.add_systems(PostUpdate, spawn_game_over_screen);
}

#[derive(Event, Clone, EntityEvent)]
pub(crate) struct ResetGame {
    #[target]
    pub target: Entity,
}

impl From<ListenerInput<Pointer<Click>>> for ResetGame {
    fn from(value: ListenerInput<Pointer<Click>>) -> Self {
        Self {
            target: value.target(),
        }
    }
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

            builder
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(65.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(NODE_BACKGROUND),
                        ..default()
                    },
                    InteractionPalette {
                        none: NODE_BACKGROUND,
                        hovered: BUTTON_HOVERED_BACKGROUND,
                        pressed: BUTTON_PRESSED_BACKGROUND,
                    },
                    On::<Pointer<Click>>::send_event::<ResetGame>(),
                ))
                .with_children(|builder| {
                    builder.spawn((
                        Name::new("reset button"),
                        TextBundle::from_section(
                            "Reset",
                            TextStyle {
                                font_size: 40.0,
                                color: BUTTON_TEXT,
                                ..default()
                            },
                        ),
                    ));
                });
        });
}
