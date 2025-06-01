use bevy::prelude::*;

use crate::{
    common::{Direction, TilePos},
    components::{Movable, Player, Position},
};

pub fn control_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Movable, &Position), With<Player>>,
) {
    for (mut movable, position) in &mut query {
        let new_dir = if keyboard_input.just_pressed(KeyCode::KeyW) {
            Direction::Up
        } else if keyboard_input.just_pressed(KeyCode::KeyA) {
            Direction::Left
        } else if keyboard_input.just_pressed(KeyCode::KeyS) {
            Direction::Down
        } else if keyboard_input.just_pressed(KeyCode::KeyD) {
            Direction::Right
        } else {
            continue;
        };

        let tile_pos: TilePos = position.0.clone().into();
        movable.target_tile = tile_pos.translate(&new_dir);
        movable.direction = new_dir;
    }
}
