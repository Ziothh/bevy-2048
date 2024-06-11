use bevy::prelude::*;

use crate::{assets::FontSpec, game::Game};

pub struct UIPlugin;

impl UIPlugin {
    fn on_startup(mut commands: Commands, font_spec: Res<FontSpec>) {
        commands
            .spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::FlexStart,
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect::all(Val::Px(50.0)),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "2048",
                    TextStyle {
                        font: font_spec.family.clone(),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                ));

                parent
                    .spawn(NodeBundle {
                        style: Style {
                            justify_content: JustifyContent::Center,
                            size: Size::AUTO,
                            gap: Size::all(Val::Px(20.0)),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        // scorebox
                        parent
                            .spawn(NodeBundle {
                                style: score_box::CONTAINER_STYLE,
                                background_color: BackgroundColor(score_box::BG_COLOR),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn(
                                    TextBundle::from_section(
                                        "Score",
                                        TextStyle {
                                            font: font_spec.family.clone(),
                                            font_size: 15.0,
                                            color: Color::WHITE,
                                        },
                                    )
                                    .with_text_alignment(TextAlignment::Center),
                                );
                                parent.spawn((
                                    TextBundle::from_section(
                                        "<score>",
                                        TextStyle {
                                            font: font_spec.family.clone(),
                                            font_size: 20.0,
                                            color: Color::WHITE,
                                        },
                                    )
                                    .with_text_alignment(TextAlignment::Center),
                                    ScoreDisplay,
                                ));
                            });
                        // end scorebox
                        // best scorebox
                        parent
                            .spawn(NodeBundle {
                                style: score_box::CONTAINER_STYLE,
                                background_color: BackgroundColor(score_box::BG_COLOR),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn(
                                    TextBundle::from_section(
                                        "Best",
                                        TextStyle {
                                            font: font_spec.family.clone(),
                                            font_size: 15.0,
                                            color: Color::WHITE,
                                        },
                                    )
                                    .with_text_alignment(TextAlignment::Center),
                                );
                                parent.spawn((
                                    TextBundle::from_section(
                                        "<score>",
                                        TextStyle {
                                            font: font_spec.family.clone(),
                                            font_size: 20.0,
                                            color: Color::WHITE,
                                        },
                                    )
                                    .with_text_alignment(TextAlignment::Center),
                                    BestScoreDisplay,
                                ));
                            });
                        // end best scorebox
                    });

                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(130.0), Val::Px(50.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: button::colors::NORMAL.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text::from_section(
                                "Button",
                                TextStyle {
                                    font: font_spec.family.clone(),
                                    font_size: 20.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ),
                            ..default()
                        });
                    });
            });
    }

    fn sys_score_board(
        game: Res<Game>,
        mut query_score: Query<&mut Text, (With<ScoreDisplay>, Without<BestScoreDisplay>)>,
        mut query_best_score: Query<&mut Text, (With<BestScoreDisplay>, Without<ScoreDisplay>)>,
    ) {
        let mut text = query_score.single_mut();
        text.sections[0].value = game.score.to_string();

        let mut text = query_best_score.single_mut();
        text.sections[0].value = game.best_score.to_string();
    }
}

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(UIPlugin::on_startup).add_systems((
            UIPlugin::sys_score_board,
            button::interaction_system,
            button::text_system,
        ));
    }
}

#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct BestScoreDisplay;

mod score_box {
    use bevy::prelude::*;

    pub const CONTAINER_STYLE: Style = Style {
        flex_direction: FlexDirection::ColumnReverse,
        align_items: AlignItems::Center,
        padding: UiRect {
            left: Val::Px(20.0),
            right: Val::Px(20.0),
            top: Val::Px(10.0),
            bottom: Val::Px(10.0),
        },
        ..Style::DEFAULT
    };

    pub const BG_COLOR: Color = Color::Lcha {
        lightness: 0.55,
        chroma: 0.5,
        hue: 315.0,
        alpha: 1.0,
    };
}

mod button {
    use bevy::prelude::*;

    use crate::game::GameState;

    pub(super) mod colors {
        use bevy::prelude::*;

        pub const NORMAL: Color = Color::Lcha {
            lightness: 0.15,
            chroma: 0.5,
            hue: 281.0,
            alpha: 1.0,
        };
        pub const HOVERED: Color = Color::Lcha {
            lightness: 0.55,
            chroma: 0.5,
            hue: 281.0,
            alpha: 1.0,
        };
        pub const PRESSED: Color = Color::Lcha {
            lightness: 0.75,
            chroma: 0.5,
            hue: 281.0,
            alpha: 1.0,
        };
    }

    pub fn interaction_system(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<Button>),
        >,
        run_state: Res<State<GameState>>,
        mut next_state: ResMut<NextState<GameState>>,
    ) {
        for (&interaction, mut bg_color) in interaction_query.iter_mut() {
            match interaction {
                Interaction::Clicked => {
                    *bg_color = colors::PRESSED.into();
                    match run_state.0 {
                        GameState::Playing => {
                            next_state.set(GameState::GameOver);
                        }
                        GameState::GameOver => {
                            next_state.set(GameState::Playing);
                        }
                    };
                }
                Interaction::Hovered => {
                    *bg_color = colors::HOVERED.into();
                    // text.sections[0].value = "Restart".into();
                }
                Interaction::None => *bg_color = colors::NORMAL.into(),
            };
        }
    }

    pub fn text_system(
        button_query: Query<&Children, With<Button>>,
        mut text_query: Query<&mut Text>,
        run_state: Res<State<GameState>>,
    ) {
        let first_child_entity = button_query
            .single()
            .first()
            .expect("Expect button to have one child");
        let mut text = text_query.get_mut(*first_child_entity).unwrap();

        match run_state.0 {
            GameState::Playing => text.sections[0].value = "End Game".to_string(),
            GameState::GameOver => text.sections[0].value = "New Game".to_string(),
        }
    }
}
