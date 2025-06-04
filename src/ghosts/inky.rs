use bevy::{prelude::*, sprite::Anchor};

use crate::{
    common::{Direction, PixelPos, TilePos},
    components::{AnimationIndices, AnimationTimer, Ghost, GhostTarget, Movable, Player, Position},
    ghosts::{GhostName, blinky::Blinky, ghost_mode::GhostMode},
    map::TILE_SIZE,
};

#[derive(Component)]
pub struct Inky {
    intermediate_tile: Option<TilePos>,
}

pub fn inky_bundle(
    texture: Handle<Image>,
    texture_atlas_layout: Handle<TextureAtlasLayout>,
) -> impl Bundle {
    let mut inky_indices =
        AnimationIndices::new(vec![84, 85], vec![86, 87], vec![88, 89], vec![90, 91]);

    // let start_tile_pos = TilePos { x: 11, y: 14 };
    let start_tile_pos = TilePos { x: 13, y: 11 };
    let mut start_pos: PixelPos = (&start_tile_pos).into();
    start_pos.x += TILE_SIZE / 2;

    let visual_start_pos = start_pos.to_character_display_pos();

    let mut sprite = Sprite::from_atlas_image(
        texture,
        TextureAtlas {
            layout: texture_atlas_layout,
            index: inky_indices.next(&Direction::Up),
        },
    );

    sprite.anchor = Anchor::TopLeft;

    (
        sprite,
        Inky {
            intermediate_tile: None,
        },
        GhostTarget::default(),
        Ghost::new(GhostName::Inky, TilePos { x: 27, y: 31 }),
        Transform::from_translation(visual_start_pos),
        inky_indices,
        AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
        Position(start_pos.clone()),
        Movable::new(start_tile_pos, Direction::Left, 0.75),
    )
}

pub fn inky_update_target(
    inky: Single<(&mut GhostTarget, &mut Inky, &Ghost)>,
    blinky: Single<&Position, With<Blinky>>,
    pacman_pos: Single<(&Position, &Movable), With<Player>>,
) {
    let (pos, movable) = pacman_pos.into_inner();
    let pacman_pos: TilePos = (&pos.0).into();

    let (mut ghost_target, mut inky, ghost) = inky.into_inner();

    if ghost.current_mode != GhostMode::Chase {
        return;
    }

    let intermediate_pos: TilePos = match movable.direction {
        Direction::Up => TilePos {
            x: pacman_pos.x - 2,
            y: pacman_pos.y - 2,
        },
        Direction::Right => TilePos {
            x: pacman_pos.x + 2,
            y: pacman_pos.y,
        },
        Direction::Down => TilePos {
            x: pacman_pos.x,
            y: pacman_pos.y + 2,
        },
        Direction::Left => TilePos {
            x: pacman_pos.x - 2,
            y: pacman_pos.y,
        },
    };
    let blinky_pos: TilePos = (&blinky.0).into();

    let delta = intermediate_pos.clone() - blinky_pos;

    let inky_target = intermediate_pos.clone() + delta;

    inky.intermediate_tile = Some(intermediate_pos);
    ghost_target.tile = Some(inky_target);
}

pub fn inky_debug(
    inky: Single<(&Inky, &GhostTarget, &Ghost)>,
    blinky: Single<&Position, With<Blinky>>,
    mut gizmos: Gizmos,
) {
    let (inky, target, ghost) = inky.into_inner();

    if ghost.current_mode != GhostMode::Chase {
        return;
    }

    let Some(target) = target.tile.as_ref() else {
        return;
    };

    let color = GhostName::Inky.get_color();

    let Some(intermediate_tile) = inky.intermediate_tile.as_ref() else {
        return;
    };

    let intermediate_tile_pos = intermediate_tile.to_center_display_pos();

    gizmos.rect_2d(
        Isometry2d::from_translation(Vec2::new(
            intermediate_tile_pos.x as f32,
            intermediate_tile_pos.y as f32,
        )),
        Vec2::splat(6.),
        color,
    );

    let blinky_pos: TilePos = (&blinky.0).into();

    gizmos.line_2d(
        blinky_pos.to_center_display_pos(),
        target.to_center_display_pos(),
        color,
    );
}
