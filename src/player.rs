use bevy::{log, prelude::*};

use crate::{
    common::Direction,
    components::{Movable, Player, QueableDirection},
    map::MAP,
};

pub fn control_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Movable, &mut QueableDirection), With<Player>>,
) {
    for (movable, mut queue_dir) in &mut query {
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

        if new_dir == movable.direction {
            log::info!("New dir is same as old dir, ignoring");
            continue;
        }

        // Check if the new dir will be legal.
        if MAP.is_wall(&movable.target_tile.translate(&new_dir)) {
            // Not a legal move.
            continue;
        }

        queue_dir.next_direction = Some(new_dir);
    }
}
