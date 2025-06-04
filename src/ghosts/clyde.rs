use bevy::{prelude::*, sprite::Anchor};

use crate::{
    common::{Direction, PixelPos, TilePos},
    components::{AnimationIndices, AnimationTimer, Ghost, GhostTarget, Movable, Player, Position},
    ghosts::GhostName,
    map::TILE_SIZE,
};

#[derive(Component)]
pub struct Clyde {
    pacman_chase_radius: f32,
}

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
        Clyde {
            pacman_chase_radius: 8.,
        },
        GhostTarget { tile: None },
        Ghost {
            ghost: GhostName::Clyde,
            corner_tile: TilePos { x: 0, y: 31 },
        },
        Transform::from_translation(visual_start_pos),
        clyde_indices,
        AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
        Position(start_pos.clone()),
        Movable::new(start_tile_pos, Direction::Left, 0.75),
    )
}

pub fn clyde_update_target(
    clyde: Single<(&Position, &mut GhostTarget, &Clyde, &Ghost)>,
    pacman_pos: Single<&Position, With<Player>>,
) {
    let pacman_position: TilePos = (&pacman_pos.0).into();

    let (clyde_position, mut target, clyde, ghost) = clyde.into_inner();
    let clyde_tile: TilePos = (&clyde_position.0).into();

    let dist = clyde_tile.dist_to(&pacman_position);

    if dist < clyde.pacman_chase_radius {
        target.tile = Some(ghost.corner_tile.clone())
    } else {
        target.tile = Some(pacman_position);
    }
}
