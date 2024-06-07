use bevy::prelude::*;

use crate::common::FontSpec;

pub struct UIPlugin;

impl UIPlugin {
    fn init(mut commands: Commands, font_spec: Res<FontSpec>) {
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
}

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(UIPlugin::init);
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
