use bevy::{log, prelude::*};

use crate::{
    common::Direction,
    components::{Movable, Player, Position, QueableDirection},
    map::MAP,
};

const MIN_DIST_FOR_QUEUEING: f32 = 0.45;

pub fn control_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Movable, &mut QueableDirection, &Position), With<Player>>,
) {
    for (movable, mut queue_dir, position) in &mut query {
        let new_dir = if keyboard_input.just_pressed(KeyCode::KeyW)
            || keyboard_input.just_pressed(KeyCode::ArrowUp)
        {
            Direction::Up
        } else if keyboard_input.just_pressed(KeyCode::KeyA)
            || keyboard_input.just_pressed(KeyCode::ArrowLeft)
        {
            Direction::Left
        } else if keyboard_input.just_pressed(KeyCode::KeyS)
            || keyboard_input.just_pressed(KeyCode::ArrowDown)
        {
            Direction::Down
        } else if keyboard_input.just_pressed(KeyCode::KeyD)
            || keyboard_input.just_pressed(KeyCode::ArrowRight)
        {
            Direction::Right
        } else {
            continue;
        };

        log::info!("Got new dir {new_dir:?}");

        if new_dir == movable.direction {
            log::info!("New dir is same as old dir, ignoring");
            continue;
        }

        let remaining_dist = position.dist_to(&(&movable.target_tile).into());
        if remaining_dist > MIN_DIST_FOR_QUEUEING && movable.direction.opposite() != new_dir {
            log::info!(
                "Not close enough to target to switch dirs yet {remaining_dist} > {MIN_DIST_FOR_QUEUEING}"
            );
        }

        // Check if the new dir will be legal.
        if MAP.is_wall(&movable.target_tile.translate(&new_dir)) {
            // Not a legal move.
            continue;
        }

        queue_dir.next_direction = Some(new_dir);
    }
}
