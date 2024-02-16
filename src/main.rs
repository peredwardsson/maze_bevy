

use bevy::{
    core_pipeline::{
        bloom::BloomSettings,
        tonemapping::Tonemapping,
    },
    prelude::*
};
use bevy_asset_loader::prelude::*;
// use bevy_ecs_ldtk::prelude::*;

mod animations;
mod player_movement;
mod utils;
mod levels;
mod constants;

// use crate::levelss::{LevelPlugin, Colliders};
use crate::animations::{AnimationTimer, Animator};
use crate::player_movement::{PlayerMover, PlayerFacing, PlayerWantsToMove};
use crate::levels::prelude::LevelPlugin;
use crate::constants::GRID_SIZE;

const SCREEN_WIDTH: f32 = 640.;
const SCREEN_HEIGHT: f32 = 480.;

// const GRID_SIZE: i32 = 32;

#[derive(Default, Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    #[default]
    South,
    West
}


#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameStates {
    #[default]
    AssetLoading,
    Main,
}

impl GameStates {
    pub fn main() -> GameStates {
        Self::Main
    }
}

#[derive(Component, Default)]
pub struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(
                WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                        ..default()
                    }),
                    ..default()
                }
            )
            .set(ImagePlugin::default_nearest())
        )
        // .add_plugins(LdtkPlugin)
        .add_plugins(Animator)
        .add_plugins(PlayerMover::<Colliders>::default())
        .add_plugins(LevelPlugin::<Player, AnimationTimer>::default())
        .add_state::<GameStates>()
        .add_loading_state(
            LoadingState::new(GameStates::AssetLoading).continue_to_state(GameStates::Main)
        )
        .add_systems(OnEnter(GameStates::main()), setup)
        .add_systems(Update,
            (
                take_input,
            ).run_if(in_state(GameStates::Main)))
        .run();
}

fn take_input(
    input: Res<Input<KeyCode>>,
    mut player: Query<&mut TextureAtlasSprite, With<Player>>,
    mut facing: ResMut<PlayerFacing>,
    mut player_wants_to_move: ResMut<PlayerWantsToMove>
) {
    let Ok(mut player) = player.get_single_mut() else {
        error!("Unable to find player!");
        return;
    };

    let movement_keys = vec![KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D];

    if input.just_pressed(KeyCode::D) {
        facing.facing = Direction::East;
        player.flip_x = false;
    }

    if input.just_pressed(KeyCode::A) {
        facing.facing = Direction::West;
        player.flip_x = true;
    }

    if input.just_pressed(KeyCode::W) {
        facing.facing = Direction::North;
        player.flip_x = false;
    }

    if input.just_pressed(KeyCode::S) {
        facing.facing = Direction::South;
        player.flip_x = false;
    }

    *player_wants_to_move = PlayerWantsToMove(input.any_pressed(movement_keys));
}


fn setup(
    mut commands: Commands,
) {

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            transform: Transform::from_xyz(SCREEN_WIDTH/2., SCREEN_HEIGHT/2., -100.),
            // projection: OrthographicProjection::default(),
            //     scale: 1.0,
            ..default()
        },
        BloomSettings::default(), // 3. Enable bloom for the camera
    ));
}

