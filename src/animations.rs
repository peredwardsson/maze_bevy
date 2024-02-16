use bevy::prelude::*;

use crate::{Player, PlayerFacing, PlayerWantsToMove, Direction};

pub struct Animator;

impl Plugin for Animator {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(AnimationIndex::default())
        .add_systems(
            Update,
            (
                update_player_idx,
                update_player_sprite
            )
            .run_if(resource_exists::<PlayerFacing>())
            .run_if(resource_exists::<PlayerWantsToMove>())
        )
        ;
    }
}

#[derive(Resource, DerefMut, Deref, Clone, Copy, Default)]
pub struct AnimationIndex {
    first: usize,
    last: usize,
    #[deref]
    current: usize,
}

#[derive(Resource, Deref, Component, DerefMut)]
pub struct AnimationTimer(pub Timer);

impl Default for AnimationTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}

impl From<&Animations> for AnimationIndex {
    fn from(value: &Animations) -> Self {
        match value {
            Animations::IdleSouth => Self::default(),
            Animations::IdleNorth => Self::new(16, 16, None),
            Animations::IdleEast => Self::new(32, 32, None),
            Animations::WalkSouth => Self::new(48, 50, None),
            Animations::WalkNorth => Self::new(52, 54, None),
            Animations::WalkEast => Self::new(64, 66, None),
        }
    }
}

impl AnimationIndex {
    fn new(first: usize, last: usize, current: Option<usize>) -> Self {
        if let Some(current) = current {
            Self {
                first,
                last,
                current
            }
        } else {
            Self {
                first,
                last,
                current: first
            }
        }
    }

    fn from_facing(facing: PlayerFacing, moving: PlayerWantsToMove) -> Self {
        if moving.0 {
            match facing.facing {
                Direction::North => Self::from(&Animations::WalkNorth),
                Direction::East => Self::from(&Animations::WalkEast),
                Direction::South => Self::from(&Animations::WalkSouth),
                Direction::West => Self::from(&Animations::WalkEast),
            }
        } else {
            match facing.facing {
                Direction::North => Self::from(&Animations::IdleNorth),
                Direction::East => Self::from(&Animations::IdleEast),
                Direction::South => Self::from(&Animations::IdleSouth),
                Direction::West => Self::from(&Animations::IdleEast),
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Animations {
    IdleSouth,
    IdleNorth,
    IdleEast,
    WalkSouth,
    WalkNorth,
    WalkEast,
}

fn update_player_idx(
    mut idx: ResMut<AnimationIndex>,
    facing: Res<PlayerFacing>,
    should_move: Res<PlayerWantsToMove>,
    time: Res<Time>,
    mut player_timer: Query<&mut AnimationTimer, With<Player>>,
) {
    let idx_old = idx.current;
    let mut new_idx = AnimationIndex::from_facing(*facing, *should_move);
    new_idx.current = idx_old;
    *idx = new_idx;
    if should_move.0 {
        // walking indices
        let Ok(mut timer) = player_timer.get_single_mut() else {return;};
        if timer.tick(time.delta()).just_finished() {
            let offset = idx.last - idx.first + 1;
            idx.current = ((idx.current + 1) % offset) + idx.first;
        }
    } else {
        // TODO(PE): idle indices
    }
}

fn update_player_sprite(
    mut player: Query<&mut TextureAtlasSprite, With<Player>>,
    idx: Res<AnimationIndex>,
) {
    let Ok(mut player) = player.get_single_mut() else {
        error!("No player found!");
        return;
    };

    player.index = idx.current;
}