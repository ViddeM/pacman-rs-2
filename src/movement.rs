use bevy::{log, prelude::*};

use crate::{
    common::{Character, Direction, TilePos},
    components::{Movable, Position},
    events::CharacterReachedTargetEvent,
    map::MAP,
};

const FULL_SPEED_PIXELS_PER_SECOND: f32 = 75.75757625;
const PAUSE_FRAME_TIME: f32 = 1. / 60.;

pub fn move_character(
    time: Res<Time>,
    mut query: Query<(&mut Movable, &mut Position, &Character)>,
    mut reach_target_event_writer: EventWriter<CharacterReachedTargetEvent>,
) {
    for (mut movable, mut position, character) in query.iter_mut() {
        if movable.pause_frames.is_some() {
            log::info!(
                "{character} is pausing this frame ({} frames remaining) ",
                movable.pause_frames.unwrap_or_default()
            );
            movable.pause_time += time.delta_secs();
            if movable.pause_time >= PAUSE_FRAME_TIME {
                movable.pause_time -= PAUSE_FRAME_TIME;
                movable.reduce_pause_time();
            }
            continue;
        }

        let tile_pos: TilePos = position.0.clone().into();
        let mut has_reached_destination =
            tile_pos == movable.target_tile && position.in_middle_of_tile();

        if has_reached_destination {
            // We're standing still
            continue;
        }

        movable.progress += time.delta_secs();

        let time_per_pixel = 1. / (FULL_SPEED_PIXELS_PER_SECOND * movable.speed);

        if movable.progress >= time_per_pixel {
            movable.progress -= time_per_pixel;
            match movable.direction {
                Direction::Up => position.y -= 1,
                Direction::Right => position.x += 1,
                Direction::Down => position.y += 1,
                Direction::Left => position.x -= 1,
            }
        }

        // Check if we have reached our destination now!
        has_reached_destination = tile_pos == movable.target_tile && position.in_middle_of_tile();

        if has_reached_destination {
            log::info!(
                "{character} reached destination {:?} with direction {:?}",
                movable.target_tile,
                movable.direction
            );
            reach_target_event_writer.write(CharacterReachedTargetEvent::new(
                character.clone(),
                tile_pos.clone(),
            ));
            match movable.direction {
                Direction::Right => {
                    if tile_pos == MAP.right_tp_position() {
                        let left_tp_pos = MAP.left_tp_position();
                        position.0 = (&left_tp_pos).into();
                        movable.target_tile = left_tp_pos.translate(&movable.direction);

                        log::info!(
                            "{character} teleporting from {tile_pos:?} {:?} to {left_tp_pos:?}",
                            movable.direction
                        );
                    }
                }
                Direction::Left => {
                    if tile_pos == MAP.left_tp_position() {
                        let right_tp_pos = MAP.right_tp_position();
                        position.0 = (&right_tp_pos).into();
                        movable.target_tile = right_tp_pos.translate(&movable.direction);

                        log::info!(
                            "{character} teleporting from {tile_pos:?} {:?} to {right_tp_pos:?}",
                            movable.direction
                        );
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn visually_move_character(query: Query<(&Position, &mut Transform), With<Movable>>) {
    for (position, mut transform) in query {
        let visual_pos = position.to_character_display_pos();
        transform.translation.x = visual_pos.x as f32;
        transform.translation.y = visual_pos.y as f32;
    }
}
