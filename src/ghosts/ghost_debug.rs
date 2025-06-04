use bevy::{prelude::*, sprite::Anchor};

use crate::{
    common::{Direction, TilePos},
    components::{Ghost, GhostDebug, GhostTarget, Movable, Position},
    ghosts::{GhostName, ghost_movement::next_tile},
    map::TILE_SIZE,
};

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
