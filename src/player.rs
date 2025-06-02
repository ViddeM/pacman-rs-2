use bevy::{log, prelude::*, sprite::Anchor};

use crate::{
    common::{Direction, PixelPos, TilePos},
    components::{AnimationIndices, AnimationTimer, Movable, Player, Position},
    map::MAP,
    score::{Scorable, Score},
};

pub fn pacman_bundle(
    texture: Handle<Image>,
    texture_atlas_layout: Handle<TextureAtlasLayout>,
) -> impl Bundle {
    let mut pacman_indices = AnimationIndices::new(
        vec![0, 1, 2, 1],
        vec![14, 15, 2, 15],
        vec![28, 29, 2, 29],
        vec![42, 43, 2, 43],
    );

    let start_pos: PixelPos = TilePos { x: 14, y: 17 }.into();
    let first_target = TilePos { x: 15, y: 17 };

    let visual_start_pos = start_pos.to_character_display_pos();

    let mut sprite = Sprite::from_atlas_image(
        texture,
        TextureAtlas {
            layout: texture_atlas_layout,
            index: pacman_indices.next(&Direction::Right),
        },
    );
    sprite.anchor = Anchor::TopLeft;

    (
        sprite,
        Player,
        Transform::from_translation(visual_start_pos),
        pacman_indices,
        AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
        Position(start_pos.clone()),
        Movable::new(first_target, Direction::Right, 0.8),
    )
}

pub fn control_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut pacman_movable: Single<&mut Movable, With<Player>>,
) {
    let new_dir = if keyboard_input.just_pressed(KeyCode::KeyW)
        || keyboard_input.just_pressed(KeyCode::ArrowUp)
    {
        Direction::Up
    } else if keyboard_input.just_pressed(KeyCode::KeyA)
        || keyboard_input.just_pressed(KeyCode::ArrowLeft)
    {
        Direction::Left
    } else if keyboard_input.just_pressed(KeyCode::KeyS)
        || keyboard_input.just_pressed(KeyCode::ArrowDown)
    {
        Direction::Down
    } else if keyboard_input.just_pressed(KeyCode::KeyD)
        || keyboard_input.just_pressed(KeyCode::ArrowRight)
    {
        Direction::Right
    } else {
        return;
    };

    if new_dir == pacman_movable.direction {
        log::info!("New dir is same as old dir, ignoring");
        return;
    }

    let new_target = pacman_movable.target_tile.translate(&new_dir);

    // Check if the new dir will be legal.
    if MAP.is_wall(&new_target) {
        // Not a legal move.
        return;
    }

    pacman_movable.target_tile = new_target;
    pacman_movable.direction = new_dir;
}

pub fn player_take_move_decision(player: Single<(&Position, &mut Movable), With<Player>>) {
    let (position, mut movable) = player.into_inner();

    let tile_pos: TilePos = position.0.clone().into();
    let has_reached_destination = tile_pos == movable.target_tile && position.in_middle_of_tile();

    if !has_reached_destination {
        return;
    }

    let new_target = tile_pos.translate(&movable.direction);

    if MAP.is_wall(&new_target) {
        return;
    }

    movable.target_tile = new_target;
}

pub fn eat(
    mut commands: Commands,
    mut score: ResMut<Score>,
    pacman: Single<(&Position, &mut Movable), With<Player>>,
    food_query: Query<(&Position, &Scorable, Entity)>,
) {
    let (pacman_position, mut movable) = pacman.into_inner();

    if !pacman_position.in_middle_of_tile() {
        return;
    }

    for (position, scorable, entity) in food_query {
        if position == pacman_position {
            score.gain_score(scorable);
            movable.pause(scorable);
            commands.entity(entity).despawn();
        }
    }
}
