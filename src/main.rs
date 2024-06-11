#![allow(dead_code)]
#![allow(unused_variables)]

mod assets;
mod board;
mod game;
mod ui;

use assets::FontSpec;
///! Followed from the [2048 bevy course](https://www.rustadventure.dev/2048-with-bevy-ecs/bevy-0.10/updating-tile-display-when-point-values-change)
use bevy::prelude::*;
use board::{Board, BoardShiftDirection, NewTileEvent};
use game::{Game, GameState};

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
        .init_resource::<Game>()
        .add_state::<GameState>()
        .add_plugin(bevy_easings::EasingsPlugin)
        .add_plugin(ui::UIPlugin)
        .add_startup_systems(
            (
                setup_camera,
                Board::spawn,
                apply_system_buffers, // Forces the previously queued spawn commands to be ran
            )
                .chain(),
        )
        .add_event::<NewTileEvent>()
        .add_system((game::reset).in_schedule(OnEnter(GameState::Playing)))
        .add_systems(
            (
                Board::render_tiles,
                Board::render_tile_points,
                game::check_game_over,
                BoardShiftDirection::sys_handle_board_shift_on_keypress,
                Board::on_new_tile_handler,
            )
                .in_set(OnUpdate(GameState::Playing)),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// fn game_reset(mut commands: Commands, tiles: Query<Entity, With<Position>>, mut game ResMut<Game>)
