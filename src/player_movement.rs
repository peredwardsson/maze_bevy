use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;
use std::marker::PhantomData;
use crate::{Direction, GRID_SIZE, Player};

#[derive(Resource, Default, PartialEq, Clone, Copy)]
pub struct PlayerWantsToMove(pub bool);


#[derive(Resource, Default, Clone, Copy)]
pub struct PlayerFacing {
    pub facing: Direction,
}

pub trait Collider {
    fn on_collider(&self, other: &GridCoords) -> bool;
}

#[derive(Default)]
pub struct PlayerMover<U: Collider + Resource> {
    // main_state: T,
    collider: PhantomData<U>,
}

impl<U> PlayerMover<U> where
    U: Collider + Resource {
    pub fn move_player(
        mut player: Query<&mut Transform, With<Player>>,
        facing: Res<PlayerFacing>,
        time: Res<Time>,
        colliders: Res<U>,
    ) {
        let Ok(mut player) = player.get_single_mut() else {
            error!("No player found!!");
            return;
        };
        let dt = time.delta_seconds();
        let speed = 100. * dt;
        let mut next_position = Vec3::from(player.translation);
        match facing.facing {
            Direction::East => next_position.x += speed,
            Direction::West => next_position.x -= speed,
            Direction::North => next_position.y += speed,
            Direction::South => next_position.y -= speed,
        }
        let player_grid = bevy_ecs_ldtk::utils::translation_to_grid_coords(
            next_position.truncate(),
            IVec2::splat(GRID_SIZE)
        );
        if !colliders.on_collider(&player_grid) {
            player.translation = next_position;
        }
    }
}

impl<U> Plugin for PlayerMover<U>
    where U: Collider + Resource {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(PlayerFacing::default())
        .insert_resource(PlayerWantsToMove::default())
            .add_systems(
                Update,
                (PlayerMover::<U>::move_player)
                    .run_if(resource_exists_and_equals(PlayerWantsToMove(true))
                )
            )
            ;
    }
}