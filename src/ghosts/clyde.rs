use bevy::{prelude::*, sprite::Anchor};

use crate::{
    common::{Direction, PixelPos, TilePos},
    components::{AnimationIndices, AnimationTimer, Ghost, GhostTarget, Movable, Player, Position},
    ghosts::GhostName,
    map::TILE_SIZE,
};

#[derive(Component)]
pub struct Clyde;

pub fn clyde_bundle(
    texture: Handle<Image>,
    texture_atlas_layout: Handle<TextureAtlasLayout>,
) -> impl Bundle {
    let mut clyde_indices =
        AnimationIndices::new(vec![98, 99], vec![100, 101], vec![102, 103], vec![104, 105]);

    // let start_tile_pos = TilePos { x: 15, y: 14 };
    let start_tile_pos = TilePos { x: 13, y: 11 };
    let mut start_pos: PixelPos = (&start_tile_pos).into();
    start_pos.x += TILE_SIZE / 2;

    let visual_start_pos = start_pos.to_character_display_pos();

    let mut sprite = Sprite::from_atlas_image(
        texture,
        TextureAtlas {
            layout: texture_atlas_layout,
            index: clyde_indices.next(&Direction::Up),
        },
    );

    sprite.anchor = Anchor::TopLeft;

    (
        sprite,
        Clyde,
        GhostTarget { tile: None },
        Ghost {
            ghost: GhostName::Clyde,
        },
        Transform::from_translation(visual_start_pos),
        clyde_indices,
        AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
        Position(start_pos.clone()),
        Movable::new(start_tile_pos, Direction::Left, 0.75),
    )
}

pub fn clyde_update_target(
    clyde: Single<&mut GhostTarget, With<Clyde>>,
    pacman_pos: Single<&Position, With<Player>>,
) {
    let pacman_position: TilePos = (&pacman_pos.0).into();

    let mut ghost_target = clyde.into_inner();

    // TODO: Really implement clyde targetting.

    ghost_target.tile = Some(pacman_position);
}
