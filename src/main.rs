use bevy::{prelude::*, window::PresentMode};
use common::{PixelPos, TilePos};
use components::{AnimationIndices, AnimationTimer, Movable};
use ghosts::blinky::blinky_update_target;
use map::spawn_map;
use player::{control_player, eat, pacman_bundle, player_take_move_decision};
use score::Score;
use ui::{setup_ui, update_score_text};

use crate::{
    debug::{DebugRes, run_if_debug, toggle_debug_mode},
    events::CharacterReachedTargetEvent,
    ghosts::{
        GhostName,
        clyde::{clyde_bundle, clyde_debug, clyde_update_target},
        ghost_debug::{
            debug_plot_ghost_path, ghost_debug_bundle, ghost_mode_debug_update, update_ghost_debug,
        },
        ghost_mode::GhostModeRes,
        ghost_movement::{ghost_handle_scatter, ghost_movement},
        inky::{inky_debug, inky_update_target},
        pinky::pinky_update_target,
    },
    movement::{move_character, visually_move_character},
    ui::update_debug_text,
};

pub mod common;
pub mod components;
pub mod debug;
pub mod events;
pub mod ghosts;
pub mod map;
pub mod movement;
pub mod player;
pub mod score;
pub mod ui;

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
        .insert_resource(Score::new())
        .insert_resource(GhostModeRes::default())
        .insert_resource(DebugRes::default())
        .add_event::<CharacterReachedTargetEvent>()
        .add_systems(Startup, (setup_world, setup_ui))
        .add_systems(
            FixedUpdate,
            (debug_plot_ghost_path, inky_debug, clyde_debug).run_if(run_if_debug),
        )
        .add_systems(
            Update,
            (
                update_debug_text,
                update_ghost_debug,
                ghost_mode_debug_update,
            )
                .run_if(run_if_debug),
        )
        .add_systems(
            Update,
            (
                toggle_debug_mode,
                animate_sprite,
                control_player,
                move_character,
                visually_move_character,
                player_take_move_decision,
                ghost_movement,
                ghost_handle_scatter,
                eat,
                update_score_text,
                blinky_update_target,
                pinky_update_target,
                inky_update_target,
                clyde_update_target,
            )
                .chain(),
        )
        .run();
}

fn setup_world(
    mut commands: Commands,
    mut config_store: ResMut<GizmoConfigStore>,
    assert_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    config_store
        .config_mut::<DefaultGizmoConfigGroup>()
        .0
        .line
        .width = 3.0;

    let camera_pos: PixelPos = TilePos { x: 14, y: 15 }.into();

    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 0.4,
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

    commands.spawn(pacman_bundle(texture.clone(), texture_atlas_layout.clone()));

    // commands.spawn(blinky_bundle(texture.clone(), texture_atlas_layout.clone()));
    // commands.spawn(ghost_debug_bundle(GhostName::Blinky));

    // commands.spawn(pinky_bundle(texture.clone(), texture_atlas_layout.clone()));
    // commands.spawn(ghost_debug_bundle(GhostName::Pinky));

    // commands.spawn(inky_bundle(texture.clone(), texture_atlas_layout.clone()));
    // commands.spawn(ghost_debug_bundle(GhostName::Inky));

    commands.spawn(clyde_bundle(texture.clone(), texture_atlas_layout.clone()));
    commands.spawn(ghost_debug_bundle(GhostName::Clyde));
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
