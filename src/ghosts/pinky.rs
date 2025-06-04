use bevy::{prelude::*, sprite::Anchor};

use crate::{
    common::{Direction, PixelPos, TilePos},
    components::{AnimationIndices, AnimationTimer, Ghost, GhostTarget, Movable, Player, Position},
    ghosts::{GhostName, ghost_mode::GhostMode},
    map::TILE_SIZE,
};

#[derive(Component)]
pub struct Pinky;

pub fn pinky_bundle(
    texture: Handle<Image>,
    texture_atlas_layout: Handle<TextureAtlasLayout>,
) -> impl Bundle {
    let mut pinky_indices =
        AnimationIndices::new(vec![70, 71], vec![72, 73], vec![74, 75], vec![76, 77]);

    // let start_tile_pos = TilePos { x: 13, y: 14 };
    let start_tile_pos = TilePos { x: 13, y: 11 };
    let mut start_pos: PixelPos = (&start_tile_pos).into();
    start_pos.x += TILE_SIZE / 2;

    let visual_start_pos = start_pos.to_character_display_pos();

    let mut sprite = Sprite::from_atlas_image(
        texture,
        TextureAtlas {
            layout: texture_atlas_layout,
            index: pinky_indices.next(&Direction::Up),
        },
    );

    sprite.anchor = Anchor::TopLeft;

    (
        sprite,
        Pinky,
        GhostTarget::default(),
        Ghost::new(GhostName::Pinky, TilePos { x: 2, y: -3 }),
        Transform::from_translation(visual_start_pos),
        pinky_indices,
        AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
        Position(start_pos.clone()),
        Movable::new(start_tile_pos, Direction::Left, 0.75),
    )
}

pub fn pinky_update_target(
    pinky: Single<(&mut GhostTarget, &Ghost), With<Pinky>>,
    pacman_pos: Single<(&Position, &Movable), With<Player>>,
) {
    let (pos, movable) = pacman_pos.into_inner();

    let (mut ghost_target, ghost) = pinky.into_inner();

    if ghost.current_mode != GhostMode::Chase {
        return;
    }

    let pacman_pos: TilePos = (&pos.0).into();

    let pinky_target = match movable.direction {
        Direction::Up => TilePos {
            x: pacman_pos.x - 4,
            y: pacman_pos.y - 4,
        },
        Direction::Right => TilePos {
            x: pacman_pos.x + 4,
            y: pacman_pos.y,
        },
        Direction::Down => TilePos {
            x: pacman_pos.x,
            y: pacman_pos.y + 4,
        },
        Direction::Left => TilePos {
            x: pacman_pos.x - 4,
            y: pacman_pos.y,
        },
    };

    ghost_target.tile = Some(pinky_target);
}
