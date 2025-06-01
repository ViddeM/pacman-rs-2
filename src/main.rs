use bevy::{prelude::*, window::PresentMode};
use common::{Direction, PixelPos, TilePos};
use components::{
    AnimationIndices, AnimationTimer, FULL_SPEED_PIXELS_PER_SECOND, Movable, Player, Position,
};
use map::{Corner, MAP, MapType, WallType};
use player::control_player;

pub mod common;
pub mod components;
pub mod map;
pub mod player;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pacman by Vidde".into(),
                        resolution: (3000., 1600.).into(),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                }),
        ) // prevents blurry sprites
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup_world)
        .add_systems(
            Update,
            (
                animate_sprite,
                control_player,
                move_character,
                visually_move_character,
            )
                .chain(),
        )
        .run();
}

fn setup_world(
    mut commands: Commands,
    assert_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let camera_pos: PixelPos = TilePos { x: 14, y: 15 }.into();

    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 0.25,
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_translation(Vec3::new(camera_pos.x as f32, -camera_pos.y as f32, 0.)),
    ));

    spawn_characters(&mut commands, &assert_server, &mut texture_atlas_layouts);

    spawn_map(&mut commands, &assert_server, &mut texture_atlas_layouts);
}

fn spawn_characters(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("sprites/pacman_spritesheet_2.png");
    let layout =
        TextureAtlasLayout::from_grid(UVec2::splat(16), 14, 13, None, Some(UVec2 { x: 456, y: 0 }));
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let mut pacman_indices = AnimationIndices::new(
        vec![0, 1, 2, 1],
        vec![14, 15, 2, 15],
        vec![28, 29, 2, 29],
        vec![42, 43, 2, 43],
    );

    let start_pos: PixelPos = TilePos { x: 14, y: 17 }.into();
    let first_target = TilePos { x: 15, y: 17 };

    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: pacman_indices.next(&Direction::Right),
            },
        ),
        Player,
        Transform::from_translation(Vec3::new(start_pos.x as f32, -start_pos.y as f32, 0.)),
        pacman_indices,
        AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
        Position(start_pos.clone()),
        Movable {
            direction: Direction::Right,
            progress: 1.,
            speed: 0.8,
            target_tile: first_target,
        },
    ));
}

fn spawn_map(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("sprites/pacman_spritesheet_2.png");
    let layout =
        TextureAtlasLayout::from_grid(UVec2::splat(8), 28, 31, None, Some(UVec2 { x: 228, y: 0 }));
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // Spawn maze
    MAP.iter().enumerate().for_each(|(row_num, row)| {
        row.iter().enumerate().for_each(|(col_num, tile)| {
            let sprite_index = match tile {
                MapType::Wall(wall_type) => sprite_index_for_wall_type(wall_type),
                MapType::GhostOnlyBarrier => 350,
                MapType::Open(_open_content) => 44,
            };

            let tile_pos = TilePos {
                x: col_num as i32,
                y: row_num as i32,
            };
            let pos: PixelPos = tile_pos.into();
            commands.spawn((
                Position(pos.clone()),
                Sprite::from_atlas_image(
                    texture.clone(),
                    TextureAtlas {
                        layout: texture_atlas_layout.clone(),
                        index: sprite_index,
                    },
                ),
                Transform::from_translation(Vec3::new(pos.x as f32, -pos.y as f32, -1.0)),
            ));
        })
    })
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationIndices,
        &mut AnimationTimer,
        &mut Sprite,
        &Movable,
    )>,
) {
    for (mut indices, mut timer, mut sprite, movable) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = indices.next(&movable.direction)
            }
        }
    }
}

pub fn sprite_index_for_wall_type(wall_type: &WallType) -> usize {
    match wall_type {
        WallType::Straight(Direction::Up) => 115,
        WallType::Straight(Direction::Right) => 41,
        WallType::Straight(Direction::Down) => 59,
        WallType::Straight(Direction::Left) => 42,
        WallType::DoubleStraight(Direction::Up) => 1,
        WallType::DoubleStraight(Direction::Right) => 55,
        WallType::DoubleStraight(Direction::Down) => 253,
        WallType::DoubleStraight(Direction::Left) => 28,
        WallType::DoubleCorner(Corner::TopRight) => 27,
        WallType::DoubleCorner(Corner::BottomRight) => 279,
        WallType::DoubleCorner(Corner::BottomLeft) => 252,
        WallType::DoubleCorner(Corner::TopLeft) => 0,
        WallType::VerticalLineInnerCorner(Corner::TopRight) => 727,
        WallType::VerticalLineInnerCorner(Corner::BottomRight) => 699,
        WallType::VerticalLineInnerCorner(Corner::BottomLeft) => 672,
        WallType::VerticalLineInnerCorner(Corner::TopLeft) => 700,
        WallType::HorizontalLineInnerCornerTopRight => 13,
        WallType::HorizontalLineInnerCornerTopLeft => 14,
        WallType::OuterCorner(Corner::TopRight) => 125,
        WallType::OuterCorner(Corner::BottomRight) => 58,
        WallType::OuterCorner(Corner::BottomLeft) => 61,
        WallType::OuterCorner(Corner::TopLeft) => 126,
        WallType::InnerCorner(Corner::TopRight) => 209,
        WallType::InnerCorner(Corner::BottomRight) => 271,
        WallType::InnerCorner(Corner::BottomLeft) => 260,
        WallType::InnerCorner(Corner::TopLeft) => 210,
        WallType::NestCorner(Corner::TopRight) => 458,
        WallType::NestCorner(Corner::BottomRight) => 346,
        WallType::NestCorner(Corner::BottomLeft) => 353,
        WallType::NestCorner(Corner::TopLeft) => 465,
        WallType::NestEntranceLeftEdge => 348,
        WallType::NestEntranceRightEdge => 351,
        WallType::Inner => 44,
    }
}

fn move_character(time: Res<Time>, mut query: Query<(&mut Movable, &mut Position)>) {
    for (mut movable, mut position) in query.iter_mut() {
        let tile_pos: TilePos = position.0.clone().into();
        let has_reached_destination =
            tile_pos == movable.target_tile && position.in_middle_of_tile();

        if !has_reached_destination {
            movable.progress += time.delta_secs();

            let time_per_pixel = (1. / FULL_SPEED_PIXELS_PER_SECOND) * movable.speed;

            if movable.progress >= time_per_pixel {
                movable.progress -= time_per_pixel;
                match movable.direction {
                    Direction::Up => position.y -= 1,
                    Direction::Right => position.x += 1,
                    Direction::Down => position.y += 1,
                    Direction::Left => position.x -= 1,
                }
            }
        }
    }
}

fn visually_move_character(query: Query<(&Position, &mut Transform), With<Movable>>) {
    for (position, mut transform) in query {
        transform.translation.x = position.x as f32;
        transform.translation.y = -position.y as f32;
    }
}
