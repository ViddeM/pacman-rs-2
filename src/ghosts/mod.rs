use std::cmp::Ordering;

use bevy::{prelude::*, sprite::Anchor};

use crate::{
    common::{Direction, TilePos},
    components::{Ghost, GhostDebug, GhostTarget, Movable, Position},
    map::{MAP, TILE_SIZE},
};

pub mod blinky;
pub mod pinky;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GhostName {
    Blinky,
    Inky,
    Pinky,
    Clyde,
}

impl GhostName {
    fn get_color(&self) -> Color {
        match self {
            GhostName::Blinky => Color::linear_rgb(1.0, 0., 0.),
            GhostName::Inky => Color::linear_rgb(0., 0.8, 0.8),
            GhostName::Pinky => Color::linear_rgb(0.8, 0., 0.8),
            GhostName::Clyde => Color::linear_rgb(0.8, 0.8, 0.),
        }
    }
}

pub fn ghost_movement(ghosts: Query<(&Position, &mut Movable, &GhostTarget)>) {
    for (position, mut movable, target) in ghosts {
        let Some(target) = target.tile.as_ref() else {
            return;
        };

        let tile_pos: TilePos = (&position.0).into();

        let has_reached_destination =
            tile_pos == movable.target_tile && position.in_middle_of_tile();

        if !has_reached_destination {
            return;
        }

        let Some((new_dest, new_dir)) = next_tile(&tile_pos, &movable.direction, target) else {
            return;
        };

        movable.target_tile = new_dest;
        movable.direction = new_dir;
    }
}

fn next_tile(
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

pub fn ghost_debug_bundle(ghost_name: GhostName) -> impl Bundle {
    let color = ghost_name.get_color();

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

pub fn plot_ghost_path(
    ghosts: Query<(&GhostTarget, &Position, &Movable, &Ghost)>,
    mut gizmos: Gizmos,
) {
    for (ghost_target, position, movable, ghost) in ghosts {
        let Some(target) = ghost_target.tile.as_ref() else {
            continue;
        };

        let path = estimate_ghost_path(&(&position.0).into(), &movable.direction, target);

        for (i, tile_a) in path.iter().enumerate() {
            let Some(tile_b) = path.get(i + 1) else {
                break;
            };

            let start = tile_a.to_center_display_pos();
            let end = tile_b.to_center_display_pos();

            let start = Vec2::new(start.x, start.y);
            let end = Vec2::new(end.x, end.y);

            gizmos.line_2d(start, end, ghost.ghost.get_color());
        }
    }
}

fn estimate_ghost_path(
    current_pos: &TilePos,
    current_dir: &Direction,
    target_pos: &TilePos,
) -> Vec<TilePos> {
    let mut dir = current_dir.clone();
    let mut curr = current_pos.clone();

    let mut path = vec![];

    while &curr != target_pos {
        let Some((new_tile, new_dir)) = next_tile(&curr, &dir, target_pos) else {
            return vec![];
        };

        if path.contains(&new_tile) {
            // Stuck in a loop.
            return path;
        }

        dir = new_dir;
        curr = new_tile.clone();
        path.push(new_tile);
    }

    path
}
