use bevy::prelude::*;
use bevy::utils::{HashSet, HashMap};
use bevy_ecs_ldtk::prelude::*;

use crate::levels::{
    components::*,
    ldtk_entities::*,
    utils::*,
};

use crate::constants::*;


pub(crate) fn cache_locks_and_keys(
    mut keys: ResMut<Keys>,
    mut locks: ResMut<Locks>,
    mut level_events: EventReader<LevelEvent>,
    key_query: Query<(&GridCoords, &LockKeyColor), With<Key>>,
    lock_query: Query<(&GridCoords, &LockKeyColor), With<Lock>>,
) {
    for level_event in level_events.iter() {
        if let LevelEvent::Spawned(_) = level_event {
            let mut color_location_pairing = HashMap::new();
            for (gc, color) in key_query.iter() {
                color_location_pairing.insert(*color, *gc);
            }
            *keys = Keys {
                color_location_pairing,
            };

            color_location_pairing = HashMap::new();
            for (gc, color) in lock_query.iter() {
                color_location_pairing.insert(*color, *gc);
            }
            *locks = Locks {
                color_location_pairing,
            };
            info!("Keys: {:?}", keys);
            info!("Locks: {:?}", locks);
        }
    }
}

pub(crate) fn cache_collider_location(
    mut level_colliders: ResMut<Colliders>,
    mut level_events: EventReader<LevelEvent>,
    unwalkables: Query<&GridCoords, With<Unwalkable>>,
    locks: Query<&GridCoords, With<Lock>>,
    ldtk_project_entities: Query<&Handle<LdtkAsset>>,
    ldtk_project_assets: Res<Assets<LdtkAsset>>,
) {
    for level_event in level_events.iter() {
        if let LevelEvent::Spawned(level_iid) = level_event {
            debug!("Spawned level {}", level_iid);
            let ldtk_project = ldtk_project_assets
                .get(ldtk_project_entities.single())
                .expect("LdtkProject should be loaded when level is spawned");
            let level = ldtk_project
                .get_level(&LevelSelection::Iid(level_iid.to_string()))
                .expect("spawned level should exist in project");
            let collider_locations = unwalkables.iter().chain(locks.iter()).copied().collect();
            let new_collider_locations = Colliders {
                collider_locations,
                level_width: level.px_wid / GRID_SIZE as i32,
                level_height: level.px_hei / GRID_SIZE as i32,
            };
            *level_colliders = new_collider_locations;
        }
    }
}

pub(crate) fn cache_goal_location(
    mut level_goals: ResMut<NextLevels>,
    mut level_events: EventReader<LevelEvent>,
    next_level_query: Query<&GridCoords, With<NextLevel>>
) {
    for level_event in level_events.iter() {
        if let LevelEvent::Spawned(_) = level_event {
            // print all the colors in the enum of the level
            let locations: HashSet<GridCoords> = next_level_query.iter().copied().collect();
            *level_goals = NextLevels {
                locations
            };
        }
    }
}

pub(crate) fn cache_entrance_location(
    mut entrances: ResMut<Entrances>,
    mut level_events: EventReader<LevelEvent>,
    entrance_query: Query<(&GridCoords, &Entrance)>
) {
    for level_event in level_events.iter() {
        if let LevelEvent::Spawned(_) = level_event {
            let mut locations = HashMap::new();
            for (coord, entr) in entrance_query.iter() {
                locations.insert(entr.name.clone(), *coord);
            }
            debug!("Entrances: {:?}", locations);
            *entrances = Entrances {
                locations
            }
        }
    }
}

pub(crate) fn check_for_goals<Player: Component>(
    player_position: Query<&Transform, With<Player>>,
    goals: Query<(&Transform, &NextLevel), Without<Player>>,
    mut level: ResMut<LevelSelection>,
    mut came_from: ResMut<CameFrom>,
) {
    if let Ok(player_tf) = player_position.get_single() {
        let grid_tf = to_grid_coords(*player_tf);
        for (tf, nl) in goals.iter() {
            let goal_tf = to_grid_coords(*tf);
            let diff = grid_tf - goal_tf;
            if diff.x == 0 && diff.y == 0 {
                info!("Player stepped on goal! {:?}", nl);
                *level = LevelSelection::Identifier(nl.next_level.clone());
                came_from.from = nl.entrance.clone();
            }
        }
    }
}

pub(crate) fn move_player_to_entrance<Player: Component>(
    mut level_events: EventReader<LevelEvent>,
    mut player_position: Query<&mut Transform, With<Player>>,
    entrance_query: Query<(&GridCoords, &Entrance)>,
    came_from: Res<CameFrom>,
) {
    for level_event in level_events.iter() {
        if let Ok(mut tf) = player_position.get_single_mut() {
            let entrances: Vec<(&GridCoords, &Entrance)> = entrance_query.iter().collect();
            if let LevelEvent::Spawned(_) = level_event {
                if let Some((gc, _)) = entrances.iter().find(|(_, ent)| ent.name == came_from.from) {
                    tf.translation = to_translation(**gc, tf.translation.z);
                } else {
                    error!("Wanted to find {:?}, did not find entrance!?", came_from.from);
                }
            }
        }
    }
}

pub(crate) fn pickup_key<Player: Component>(
    mut carried_keys: ResMut<CarriedKeys>,
    player_position: Query<&Transform, With<Player>>,
    keys: Query<(Entity, &GridCoords, &LockKeyColor), With<Key>>,
    mut commands: Commands,
) {
    if let Ok(tf) = player_position.get_single() {
        let grid_tf = to_grid_coords(*tf);
        for (entity, gc, color) in keys.iter() {
            let diff = grid_tf - *gc;
            if diff.x.abs() <= 1 && diff.y.abs() <= 1 {
                carried_keys.keys.insert(*color);
                commands.entity(entity).despawn_recursive();
                info!("Picked up key {:?}", color);
            }
        }
    }
}

pub(crate) fn is_near_lock<Player: Component>(
    mut carried_keys: ResMut<CarriedKeys>,
    player_position: Query<&Transform, With<Player>>,
    locks: Query<(Entity, &GridCoords, &LockKeyColor), With<Lock>>,
    mut level_colliders: ResMut<Colliders>,
    mut commands: Commands,
) {
    if let Ok(tf) = player_position.get_single() {
        let grid_tf = to_grid_coords(*tf);
        for (entity, gc, color) in locks.iter() {
            let diff = grid_tf - *gc;
            if diff.x.abs() <= 3 && diff.y.abs() <= 3 {
                if carried_keys.keys.contains(color) {
                    commands.entity(entity).despawn_recursive();
                    level_colliders.collider_locations.remove(gc);
                    info!("Unlocked lock {:?}", color);
                }
            }
        }
    }
}


pub(crate) fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(
        LdtkWorldBundle {
            ldtk_handle: asset_server.load("farms.ldtk"),
            ..default()
        }
    );
}