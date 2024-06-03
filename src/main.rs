#![allow(dead_code)]
#![allow(unused_variables)]

use bevy::prelude::*;

fn main() {
    App::new()
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

    fn physical_size(&self) -> f32 {
        return f32::from(self.size) * Board::TILE_SIZE;
    }

    fn spawn(mut commands: Commands) {
        let board = Board { size: 4 };

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(board.physical_size(), board.physical_size())),
                    ..default()
                },
                ..default()
            })
            .insert(board);
    }
}
