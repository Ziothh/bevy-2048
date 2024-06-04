#![allow(dead_code)]
#![allow(unused_variables)]

use std::ops;

use bevy::prelude::*;
use itertools::Itertools;
use rand::{self, seq::IteratorRandom};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("#1f2638").unwrap()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "2048".into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<FontSpec>()
        .add_startup_systems(
            (
                setup,
                Board::spawn,
                apply_system_buffers, // Forces the previously queued spawn commands to be ran
                Board::spawn_tiles,
            )
                .chain(),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct Board {
    size: u8,
}

impl Board {
    pub const COLOR: Color = Color::Lcha {
        lightness: 0.06,
        chroma: 0.088,
        hue: 281.0,
        alpha: 1.,
    };

    pub const TILE_SIZE: f32 = 40.;
    pub const TILE_SPACING: f32 = 10.;
    pub const TILE_COLOR: Color = Color::Lcha {
        lightness: 0.85,
        chroma: 0.5,
        hue: 315.0,
        alpha: 1.0,
    };
    pub const TILE_PLACEHOLDER_COLOR: Color = Color::Lcha {
        lightness: 0.55,
        chroma: 0.5,
        hue: 315.0,
        alpha: 1.0,
    };

    fn new(size: u8) -> Self {
        return Self { size };
    }

    fn dimensions(&self) -> (u8, u8) {
        return (self.size, self.size);
    }

    fn physical_size(&self) -> Vec2 {
        let (width, height) = self.dimensions();
        return Vec2::new(
            width as f32 * Board::TILE_SIZE + (width + 1) as f32 * Board::TILE_SPACING,
            height as f32 * Board::TILE_SIZE + (height + 1) as f32 * Board::TILE_SPACING,
        );
    }

    fn origin(&self) -> Vec2 {
        let size = self.physical_size();

        return Vec2::new(
            -size.x / 2. + Board::TILE_SIZE / 2.,
            -size.y / 2. + Board::TILE_SIZE / 2.,
        );
    }

    fn cell_position_to_physical(&self, x: u8, y: u8) -> Vec2 {
        // Offset to the bottom left corner of the board
        let offset = -self.physical_size() / 2. + Board::TILE_SIZE / 2.;

        return Vec2::new(
            offset.x + x as f32 * Board::TILE_SIZE + (x + 1) as f32 * Board::TILE_SPACING,
            offset.y + y as f32 * Board::TILE_SIZE + (y + 1) as f32 * Board::TILE_SPACING,
        );
    }

    fn iter_dimensions(&self) -> itertools::Product<ops::Range<u8>, ops::Range<u8>> {
        let (width, height) = self.dimensions();
        return (0..width).cartesian_product(0..height);
    }

    fn spawn(mut commands: Commands) {
        let board = Board { size: 4 };

        let offset = -board.physical_size() / 2. + Board::TILE_SIZE / 2.;

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(board.physical_size()),
                    color: Board::COLOR,
                    ..default()
                },
                ..default()
            })
            .with_children(|builder| {
                for (x, y) in board.iter_dimensions() {
                    let pos = board.cell_position_to_physical(x, y);

                    builder.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Board::TILE_PLACEHOLDER_COLOR,
                            custom_size: Some(Vec2::new(Board::TILE_SIZE, Board::TILE_SIZE)),
                            ..default()
                        },
                        transform: Transform::from_xyz(pos.x, pos.y, 1.),
                        ..default()
                    });
                }
            })
            .insert(board);
    }

    fn spawn_tiles(mut commands: Commands, query_board: Query<&Board>, font_spec: Res<FontSpec>) {
        let board = query_board.single();

        let mut rng = rand::thread_rng();
        let starting_tiles: Vec<(u8, u8)> = board.iter_dimensions().choose_multiple(&mut rng, 2);

        for pos in starting_tiles.iter().map(|&(x, y)| Position { x, y }) {
            let render_pos = board.cell_position_to_physical(pos.x, pos.y);

            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Board::TILE_COLOR,
                        custom_size: Some(Vec2::new(Board::TILE_SIZE, Board::TILE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(render_pos.x, render_pos.y, 1.),
                    ..default()
                })
                .with_children(|child_builder| {
                    child_builder.spawn(Text2dBundle {
                        text: Text::from_section(
                            "2",
                            TextStyle {
                                font_size: 40.,
                                color: Color::BLACK,
                                font: font_spec.family.clone(),
                                ..default()
                            },
                        )
                        .with_alignment(TextAlignment::Center),
                        transform: Transform::from_xyz(0., 0., 1.),
                        ..default()
                    });
                })
                .insert(Points { value: 2 })
                .insert(pos);
        }
    }
}

#[derive(Resource)]
struct FontSpec {
    family: Handle<Font>,
}
impl FromWorld for FontSpec {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world
            .get_resource_mut::<AssetServer>()
            .expect("AssetServer to be initialised with the DefaultPlugins");

        return FontSpec {
            family: asset_server.load("fonts/FiraSans-Bold.ttf"),
        };
    }
}

#[derive(Component)]
struct Points {
    value: u32,
}

#[derive(Component)]
struct Position {
    x: u8,
    y: u8,
}

#[derive(Component)]
struct TileText;
