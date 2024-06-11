use bevy::{prelude::*, utils::HashMap};

use crate::{assets, board};

#[derive(Default, Resource)]
pub struct Game {
    pub score: u32,
    pub best_score: u32,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash, bevy::prelude::States)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

pub fn check_game_over(
    tiles: Query<(&board::tile::Position, &board::tile::Points)>,
    query_board: Query<&board::Board>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let board = query_board.single();

    if tiles.iter().len() != board.total_tiles() as usize {
        return;
    }

    let map: HashMap<&board::tile::Position, &board::tile::Points> = tiles.iter().collect();

    const NEIGHBOR_POINTS: [(i8, i8); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let board_axis_range = 0..(board.size as i8);

    let has_move = tiles.iter().any(|(pos, value)| {
        NEIGHBOR_POINTS
            .iter()
            .filter_map(|(x2, y2)| {
                let new_x = pos.x as i8 - x2;
                let new_y = pos.y as i8 - y2;

                if !board_axis_range.contains(&new_x) || !board_axis_range.contains(&new_y) {
                    return None;
                };

                map.get(&board::tile::Position {
                    x: new_x.try_into().unwrap(),
                    y: new_y.try_into().unwrap(),
                })
            })
            .any(|&v| v == value)
    });

    if has_move == false {
        game_state.set(GameState::GameOver);
    }
}

pub fn reset(
    mut commands: Commands,
    tile_entities: Query<Entity, With<board::tile::Position>>,
    mut game: ResMut<Game>,
    query_board: Query<&board::Board>,
    font_spec: Res<assets::FontSpec>,
) {
    for entity in tile_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let board = query_board.single();
    board.spawn_tiles(&mut commands, &font_spec, None, 2);

    game.score = 0;
}
