use std::cmp::Ordering;

use bevy::{prelude::*, sprite::Anchor};

use crate::{
    common::{Direction, TilePos},
    components::{Ghost, GhostDebug, GhostTarget, Movable, Position},
    map::{MAP, TILE_SIZE},
};

pub mod blinky;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GhostName {
    Blinky,
    Inky,
    Pinky,
    Clyde,
}

pub fn ghost_movement(ghost: Single<(&Position, &mut Movable, &GhostTarget)>) {
    let (position, mut movable, target) = ghost.into_inner();

    let Some(target) = target.tile.as_ref() else {
        return;
    };

    let tile_pos: TilePos = (&position.0).into();

    let has_reached_destination = tile_pos == movable.target_tile && position.in_middle_of_tile();

    if !has_reached_destination {
        return;
    }

    let mut neighbours = MAP
        .get_empty_neighbours(&tile_pos)
        .into_iter()
        .filter(|(_, dir)| dir.opposite() != movable.direction)
        .filter(|(tile, _)| !MAP.is_wall(tile))
        .filter(|(_, dir)| !(dir == &Direction::Up && MAP.is_in_ghost_up_block_area(&tile_pos)))
        .map(|(pos, dir)| (pos.dist_to(&target), pos, dir))
        .collect::<Vec<_>>();

    neighbours.sort_by(|(dist_a, _, dir_a), (dist_b, _, dir_b)| {
        let dist_cmp = dist_a.total_cmp(dist_b);
        if dist_cmp != Ordering::Equal {
            return dist_cmp;
        }

        dir_a.cmp(dir_b)
    });

    neighbours.reverse();

    if neighbours.is_empty() && MAP.get_tp_positions().contains(&tile_pos) {
        return;
    }

    let (_, new_dest, new_dir) = neighbours.pop().expect("No reasonable target tiles");

    movable.target_tile = new_dest;
    movable.direction = new_dir;
}

pub fn ghost_debug_bundle(ghost_name: GhostName, color: Color) -> impl Bundle {
    let mut sprite = Sprite::from_color(
        color.with_alpha(0.5),
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
    );

    sprite.anchor = Anchor::TopLeft;

    (
        GhostDebug { ghost: ghost_name },
        sprite,
        Transform::from_translation(Vec3::new(0., 0., -10.)),
    )
}

pub fn update_ghost_debug(
    ghosts: Query<(&GhostTarget, &Ghost)>,
    ghost_debugs: Query<(&mut Transform, &GhostDebug)>,
) {
    for (mut transform, debug) in ghost_debugs {
        for (target, ghost) in ghosts.iter() {
            if ghost.ghost != debug.ghost {
                continue;
            }

            let Some(target_tile) = target.tile.as_ref() else {
                continue;
            };

            transform.translation = target_tile.to_maze_display_pos().with_z(1.0);
        }
    }
}
