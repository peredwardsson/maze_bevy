// File: plugin.rs

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use std::marker::PhantomData;

use crate::levels::{
    components::*,
    systems::*,
    ldtk_entities::*,
};

#[derive(Default)]
pub struct LevelPlugin<PlayerComponent, AnimationTimer> where
PlayerComponent: Component + Default,
AnimationTimer: Resource + Component + Default {
    player_bundle: PhantomData<PlayerComponent>,
    animation_timer: PhantomData<AnimationTimer>,
}

impl<PlayerComponent, AnimationTimer> Plugin for LevelPlugin<PlayerComponent, AnimationTimer>
where
PlayerComponent: Component + Default,
AnimationTimer: Resource + Component + Default,
{
    fn build(&self, app: &mut App) {
        app
            .add_plugins(LdtkPlugin)
            .insert_resource(LevelSelection::Index(1))
            .insert_resource(Colliders::default())
            .insert_resource(NextLevels::default())
            .insert_resource(Entrances::default())
            .insert_resource(CameFrom::default())
            .insert_resource(Keys::default())
            .insert_resource(Locks::default())
            .insert_resource(CarriedKeys::default())
            .register_ldtk_entity::<PlayerBundle::<PlayerComponent, AnimationTimer>>("PlayerSpawnPoint")
            .register_ldtk_entity::<NextLevelBundle>("SwitchLevel")
            .register_ldtk_entity::<EntranceBundle>("Entrance")
            .register_ldtk_entity::<KeyBundle>("Key")
            .register_ldtk_entity::<LockBundle>("Lock")
            .register_ldtk_int_cell_for_layer::<UnwalkablesBundle>("Unwalkables", 1)
            .add_systems(
                Update,
                (
                    cache_collider_location,
                    cache_goal_location,
                    cache_entrance_location,
                    cache_locks_and_keys,
                    check_for_goals::<PlayerComponent>,
                    move_player_to_entrance::<PlayerComponent>,
                    pickup_key::<PlayerComponent>,
                    is_near_lock::<PlayerComponent>,
                )
            )
            .add_systems(Startup, setup)
            ;
    }
}