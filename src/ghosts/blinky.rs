use bevy::{prelude::*, sprite::Anchor};

use crate::{
    common::{Direction, PixelPos, TilePos},
    components::{AnimationIndices, AnimationTimer, Ghost, GhostTarget, Movable, Player, Position},
    ghosts::{GhostName, ghost_mode::GhostMode},
    map::TILE_SIZE,
};

#[derive(Component)]
pub struct Blinky;

pub fn blinky_bundle(
    texture: Handle<Image>,
    texture_atlas_layout: Handle<TextureAtlasLayout>,
) -> impl Bundle {
    let mut blinky_indices =
        AnimationIndices::new(vec![56, 57], vec![58, 59], vec![60, 61], vec![62, 63]);

    let mut start_pos: PixelPos = TilePos { x: 13, y: 11 }.into();
    start_pos.x += TILE_SIZE / 2;

    let visual_start_pos = start_pos.to_character_display_pos();

    let first_target = TilePos { x: 13, y: 11 };

    let mut sprite = Sprite::from_atlas_image(
        texture,
        TextureAtlas {
            layout: texture_atlas_layout,
            index: blinky_indices.next(&Direction::Right),
        },
    );
    sprite.anchor = Anchor::TopLeft;

    (
        sprite,
        Blinky,
        GhostTarget::default(),
        Ghost::new(GhostName::Blinky, TilePos { x: 25, y: -4 }),
        Transform::from_translation(visual_start_pos),
        blinky_indices,
        AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
        Position(start_pos.clone()),
        Movable::new(first_target, Direction::Left, 0.75),
    )
}

pub fn blinky_update_target(
    blinky: Single<(&mut GhostTarget, &Ghost), With<Blinky>>,
    pacman_pos: Single<&Position, With<Player>>,
) {
    let pacman_position: TilePos = (&pacman_pos.0).into();
    let (mut ghost_target, ghost) = blinky.into_inner();

    if ghost.current_mode != GhostMode::Chase {
        return;
    }

    ghost_target.tile = Some(pacman_position);
}
