#![allow(dead_code)]
#![allow(unused_variables)]

use bevy::prelude::*;
use itertools::Itertools;

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
        .add_startup_systems((setup, Board::spawn))
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
    pub const TILE_SIZE: f32 = 40.;
    pub const TILE_SPACING: f32 = 10.;

    pub const COLOR: Color = Color::Lcha {
        lightness: 0.06,
        chroma: 0.088,
        hue: 281.0,
        alpha: 1.,
    };

    fn new(size: u8) -> Self {
        return Self { size };
    }

    fn physical_size(&self) -> Vec2 {
        let size =
            self.size as f32 * Board::TILE_SIZE + (self.size + 1) as f32 * Board::TILE_SPACING;
        return Vec2::new(size, size);
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
                for (x, y) in (0..board.size).cartesian_product(0..board.size) {
                    let pos = board.cell_position_to_physical(x, y);

                    builder.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Tile::PLACEHOLDER_COLOR,
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
}

struct Tile {}

impl Tile {
    pub const PLACEHOLDER_COLOR: Color = Color::Lcha {
        lightness: 0.55,
        chroma: 0.5,
        hue: 315.0,
        alpha: 1.0,
    };
}
