use std::cmp::Ordering;

use bevy::prelude::*;

use crate::{
    common::{Direction, TilePos},
    components::{Ghost, GhostTarget, Movable, Position},
    ghosts::ghost_mode::{GhostMode, GhostModeRes},
    map::MAP,
};

pub fn ghost_movement(ghosts: Query<(&Position, &mut Movable, &mut GhostTarget)>) {
    for (position, mut movable, mut target) in ghosts {
        let tile_pos: TilePos = (&position.0).into();

        let has_reached_destination =
            tile_pos == movable.target_tile && position.in_middle_of_tile();

        if !has_reached_destination {
            return;
        }

        // Check if we should reverse.
        if target.should_reverse {
            target.should_reverse = false;
            movable.direction = movable.direction.opposite();
        }

        let Some(target_tile) = target.tile.as_ref() else {
            return;
        };

        let Some((new_dest, new_dir)) = next_tile(&tile_pos, &movable.direction, target_tile)
        else {
            return;
        };

        movable.target_tile = new_dest;
        movable.direction = new_dir;
    }
}

pub fn next_tile(
    current_pos: &TilePos,
    current_dir: &Direction,
    target_pos: &TilePos,
) -> Option<(TilePos, Direction)> {
    let mut neighbours = MAP
        .get_empty_neighbours(&current_pos)
        .into_iter()
        .filter(|(_, dir)| &dir.opposite() != current_dir)
        .filter(|(tile, _)| !MAP.is_wall(tile))
        .filter(|(_, dir)| !(dir == &Direction::Up && MAP.is_in_ghost_up_block_area(&current_pos)))
        .map(|(pos, dir)| (pos.dist_to(target_pos), pos, dir))
        .collect::<Vec<_>>();

    neighbours.sort_by(|(dist_a, _, dir_a), (dist_b, _, dir_b)| {
        let dist_cmp = dist_a.total_cmp(dist_b);
        if dist_cmp != Ordering::Equal {
            return dist_cmp;
        }

        dir_a.cmp(dir_b)
    });

    neighbours.reverse();

    if neighbours.is_empty() && MAP.get_tp_positions().contains(current_pos) {
        return None;
    }

    let (_, new_dest, new_dir) = neighbours.pop().expect("No reasonable target tiles");

    Some((new_dest, new_dir))
}

pub fn ghost_handle_scatter(
    ghosts: Query<(&mut Ghost, &mut GhostTarget)>,
    mode_res: Res<GhostModeRes>,
) {
    for (mut ghost, mut target) in ghosts {
        if mode_res.global_mode != ghost.current_mode {
            target.should_reverse = true;
            ghost.current_mode = mode_res.global_mode.clone();
        }

        if ghost.current_mode == GhostMode::Scatter {
            target.tile = Some(ghost.corner_tile.clone());
        }
    }
}
