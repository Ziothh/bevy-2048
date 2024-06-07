#![allow(dead_code)]
#![allow(unused_variables)]

///! Followed from the [2048 bevy course](https://www.rustadventure.dev/2048-with-bevy-ecs/bevy-0.10/updating-tile-display-when-point-values-change)
use std::{cmp::Ordering, ops};

use bevy::prelude::*;
use assets::FontSpec;
use game::Game;
use itertools::Itertools;
use rand::{self, seq::IteratorRandom};

mod assets;
mod game;
mod ui;

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
        .add_plugin(ui::UIPlugin)
        .add_startup_systems(
            (
                setup,
                Board::spawn,
                apply_system_buffers, // Forces the previously queued spawn commands to be ran
            )
                .chain(),
        )
        .add_event::<NewTileEvent>()
        .add_systems((
            Board::render_tiles,
            Board::render_tile_points,
            BoardShiftDirection::sys_handle_board_shift_on_keypress,
            Board::on_new_tile_handler,
        ))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// fn game_reset(mut commands: Commands, tiles: Query<Entity, With<Position>>, mut game ResMut<Game>)

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

    fn spawn(mut commands: Commands, font_spec: Res<FontSpec>, tiles: Query<&Position>) {
        let board = Board { size: 4 };

        let offset = -board.physical_size() / 2. + Board::TILE_SIZE / 2.;

        board.spawn_tiles(&mut commands, &font_spec, &tiles, 2);

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
                    println!("({x}, {y})");
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

    fn spawn_tiles(
        &self,
        commands: &mut Commands,
        font_spec: &Res<FontSpec>,
        tiles: &Query<&Position>,
        amount: usize,
    ) {
        let mut rng = rand::thread_rng();
        let new_tiles: Vec<Position> = self
            .iter_dimensions()
            .map(|(x, y)| Position { x, y })
            .filter(|&pos| {
                tiles
                    .iter()
                    .find(|&&occupied_pos| occupied_pos == pos)
                    .is_none()
            })
            .choose_multiple(&mut rng, amount);

        for pos in new_tiles.iter() {
            let render_pos = self.cell_position_to_physical(pos.x, pos.y);

            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Board::TILE_COLOR,
                        custom_size: Some(Vec2::new(Board::TILE_SIZE, Board::TILE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(render_pos.x, render_pos.y, 2.),
                    ..default()
                })
                .with_children(|child_builder| {
                    child_builder
                        .spawn(Text2dBundle {
                            text: Text::from_section(
                                "",
                                TextStyle {
                                    font_size: Board::TILE_SIZE,
                                    color: Color::BLACK,
                                    font: font_spec.family.clone(),
                                    ..default()
                                },
                            )
                            .with_alignment(TextAlignment::Center),
                            transform: Transform::from_xyz(0., 0., 1.),
                            ..default()
                        })
                        .insert(TileText);
                })
                .insert(Points { value: 2 })
                .insert(pos.clone());
        }
    }

    fn render_tile_points(
        mut texts: Query<&mut Text, With<TileText>>,
        tiles: Query<(&Points, &Children)>,
    ) {
        for (points, children) in tiles.iter() {
            // We expect that our tiles will have the entity with the Text component as the first child
            // (because that's how we built them) so we access that with children.first().
            if let Some(entity) = children.first() {
                let mut text = texts.get_mut(*entity).expect("expect Text to exist");

                let text_section = text
                    .sections
                    .first_mut()
                    .expect("expect first section to be accessible as mutable");
                text_section.value = points.value.to_string();
            }
        }
    }

    fn render_tiles(
        mut tiles: Query<(&mut Transform, &Position, Changed<Position>)>,
        query_board: Query<&Board>,
    ) {
        let board = query_board.single();

        for (mut transform, position, postion_changed) in tiles.iter_mut() {
            if !postion_changed {
                continue;
            }

            let physical_position = board.cell_position_to_physical(position.x, position.y);

            transform.translation.x = physical_position.x;
            transform.translation.y = physical_position.y;
        }
    }

    fn on_new_tile_handler(
        mut event_reader: EventReader<NewTileEvent>,
        mut commands: Commands,
        query_board: Query<&Board>,
        tiles: Query<&Position>,
        font_spec: Res<FontSpec>,
    ) {
        let board = query_board.single();

        for _event in event_reader.iter() {
            board.spawn_tiles(&mut commands, &font_spec, &tiles, 1);
        }
    }
}

enum BoardShiftDirection {
    Left,
    Right,
    Up,
    Down,
}
impl BoardShiftDirection {
    /// TODO: make it sort direction dependent
    fn sort_tiles(&self, a: &Position, b: &Position) -> Ordering {
        match self {
            BoardShiftDirection::Left => match Ord::cmp(&a.y, &b.y) {
                Ordering::Equal => Ord::cmp(&a.x, &b.x),
                ordering => ordering,
            },
            BoardShiftDirection::Right => match Ord::cmp(&b.y, &a.y) {
                Ordering::Equal => Ord::cmp(&b.x, &a.x),
                ordering => ordering,
            },
            BoardShiftDirection::Up => match Ord::cmp(&b.x, &a.x) {
                Ordering::Equal => Ord::cmp(&b.y, &a.y),
                ordering => ordering,
            },
            BoardShiftDirection::Down => match Ord::cmp(&a.x, &b.x) {
                Ordering::Equal => Ord::cmp(&a.y, &b.y),
                ordering => ordering,
            },
        }
    }

    fn set_position_column(&self, board_size: u8, position: &mut Mut<Position>, new_column: u8) {
        match self {
            BoardShiftDirection::Left => position.x = new_column,
            BoardShiftDirection::Right => position.x = board_size - 1 - new_column,
            BoardShiftDirection::Up => position.y = board_size - 1 - new_column,
            BoardShiftDirection::Down => position.y = new_column,
        }
    }
    fn get_position_row(&self, position: &Position) -> u8 {
        match self {
            BoardShiftDirection::Left | BoardShiftDirection::Right => position.y,
            BoardShiftDirection::Up | BoardShiftDirection::Down => position.x,
        }
    }

    fn sys_handle_board_shift_on_keypress(
        mut commands: Commands,
        input: Res<Input<KeyCode>>,
        mut tiles: Query<(Entity, &mut Position, &mut Points)>,
        query_board: Query<&Board>,
        mut game: ResMut<Game>,
        mut event_writer: EventWriter<NewTileEvent>,
    ) {
        let board = query_board.single();

        let Some(direction) = input
            .get_just_pressed()
            .find_map(|key_code| BoardShiftDirection::try_from(key_code).ok())
        else {
            return;
        };

        let original_tile_info = tiles
            .iter()
            .map(|(_, &postion, &points)| (postion.clone(), points.clone()))
            .collect::<Vec<_>>();

        let mut ordered_tiles = tiles
            .iter_mut()
            .sorted_by(|a, b| direction.sort_tiles(&a.1, &b.1))
            .peekable();

        // Column is shift direction dependent
        let mut column: u8 = 0;
        while let Some(mut tile) = ordered_tiles.next() {
            direction.set_position_column(board.size, &mut tile.1, column);

            let Some(next_tile) = ordered_tiles.peek() else {
                continue;
            };

            if direction.get_position_row(&tile.1) != direction.get_position_row(&next_tile.1) {
                // Different rows, don't merge
                column = 0;
            } else if tile.2.value != next_tile.2.value {
                // Different values, don't merge
                column += 1;
            } else {
                // Merge
                let real_next_tile = ordered_tiles
                    .next()
                    .expect("A peekable tile should always exist when calling .next()");

                // Update the values
                let merged_value = tile.2.value + real_next_tile.2.value;
                tile.2.value = merged_value;
                game.score += merged_value;

                commands.entity(real_next_tile.0).despawn_recursive();

                if let Some(future) = ordered_tiles.peek() {
                    if direction.get_position_row(&tile.1) != direction.get_position_row(&future.1)
                    {
                        // Reset if it's on another row
                        column = 0;
                    } else {
                        column = column + 1;
                    }
                }
            }
        }

        // No point in checking for different length because the tile gets despawned later on.
        if original_tile_info
            .iter()
            .zip(tiles.iter())
            .any(|(original, updated)| original.0 != *updated.1 || original.1 != *updated.2)
        {
            // If a tile has moved / merged create a new tile
            event_writer.send(NewTileEvent);
        }
    }
}

impl TryFrom<&KeyCode> for BoardShiftDirection {
    type Error = ();

    fn try_from(value: &KeyCode) -> Result<Self, Self::Error> {
        use BoardShiftDirection::*;

        return match value {
            KeyCode::Left | KeyCode::H | KeyCode::A => Ok(Left),
            KeyCode::Right | KeyCode::L | KeyCode::D => Ok(Right),
            KeyCode::Down | KeyCode::J | KeyCode::S => Ok(Down),
            KeyCode::Up | KeyCode::K | KeyCode::W => Ok(Up),
            _ => Err(()),
        };
    }
}

#[derive(Component, Debug, PartialEq, Clone, Copy)]
struct Points {
    value: u32,
}

#[derive(Component, Debug, PartialEq, Clone, Copy)]
struct Position {
    x: u8,
    y: u8,
}

#[derive(Component)]
struct TileText;

struct NewTileEvent;
