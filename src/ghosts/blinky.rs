use bevy::{log, prelude::*};

use crate::{
    common::{Direction, PixelPos, TilePos},
    components::{AnimationIndices, AnimationTimer, Movable, Player, Position},
    map::{MAP, TILE_SIZE},
};

#[derive(Component)]
pub struct Blinky;

pub fn blinky_bundle(
    texture: Handle<Image>,
    texture_atlas_layout: Handle<TextureAtlasLayout>,
) -> impl Bundle {
    let mut pacman_indices =
        AnimationIndices::new(vec![56, 57], vec![58, 59], vec![60, 61], vec![62, 63]);

    let mut start_pos: PixelPos = TilePos { x: 13, y: 11 }.into();
    start_pos.x += TILE_SIZE / 2;

    let first_target = TilePos { x: 13, y: 11 };

    (
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: pacman_indices.next(&Direction::Right),
            },
        ),
        Blinky,
        Transform::from_translation(Vec3::new(start_pos.x as f32, -start_pos.y as f32, 0.)),
        pacman_indices,
        AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
        Position(start_pos.clone()),
        Movable::new(first_target, Direction::Left, 0.75),
    )
}

pub fn blinky_take_move_decision(
    blinky: Single<(&Position, &mut Movable), With<Blinky>>,
    pacman_pos: Single<&Position, With<Player>>,
) {
    let pacman_position: TilePos = (&pacman_pos.0).into();

    let (position, mut movable) = blinky.into_inner();

    let tile_pos: TilePos = position.0.clone().into();
    let has_reached_destination = tile_pos == movable.target_tile && position.in_middle_of_tile();

    if !has_reached_destination {
        return;
    }

    log::info!(
        "Blinky is taking a new decision, current pos: {tile_pos:?}, dir: {:?}, pacman current pos: {pacman_position:?}",
        movable.direction
    );

    let mut neighbours = MAP
        .get_empty_neighbours(&tile_pos)
        .into_iter()
        .filter(|(_, dir)| dir.opposite() != movable.direction)
        .filter(|(tile, _)| !MAP.is_wall(tile))
        .filter(|(_, dir)| !(dir == &Direction::Up && MAP.is_in_ghost_up_block_area(&tile_pos)))
        .map(|(pos, dir)| (pos.dist_to(&pacman_position), pos, dir))
        .collect::<Vec<_>>();

    neighbours.sort_by(|(dist_a, _, _), (dist_b, _, _)| dist_a.total_cmp(dist_b));

    neighbours.reverse();

    log::info!("Blinky sorted options: {neighbours:?}");

    if neighbours.is_empty() && MAP.get_tp_position(&tile_pos).is_some() {
        // We do nothing and let the other system handle teleportation.
        return;
    }

    let (_, new_dest, new_dir) = neighbours.pop().expect("No reasonable target tiles");

    log::info!("Blinky new destination: {new_dest:?} {new_dir:?}");

    movable.target_tile = new_dest;
    movable.direction = new_dir;
}
