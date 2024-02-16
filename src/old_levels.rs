use std::marker::PhantomData;

use bevy::utils::HashMap;
use bevy::{prelude::*, utils::HashSet};
use bevy_ecs_ldtk::prelude::*;

use crate::player_movement::Collider;
use crate::GRID_SIZE;

const GRID_SIZE_IVEC: IVec2 = IVec2::splat(GRID_SIZE);

fn to_grid_coords(
    tf: Transform
) -> GridCoords {
    bevy_ecs_ldtk::utils::translation_to_grid_coords(tf.translation.truncate(), GRID_SIZE_IVEC)
}

fn to_translation(
    gc: GridCoords,
    z: f32,
) -> Vec3 {
    bevy_ecs_ldtk::utils::grid_coords_to_translation(gc, GRID_SIZE_IVEC).extend(z)
}

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
            ;
    }
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle<PlayerComponent, AnimationTimer> where
PlayerComponent: Component + Default,
AnimationTimer: Resource + Component + Default, {
    player: PlayerComponent,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    timer: AnimationTimer,
}

#[derive(Default, Resource)]
struct NextLevels {
    locations: HashSet<GridCoords>,
}

#[derive(Default, Component, Debug)]
struct NextLevel {
    next_level: String,
    entrance: String,
}

impl LdtkEntity for NextLevel {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _tileset: Option<&Handle<Image>>,
        _tileset_definition: Option<&TilesetDefinition>,
        _asset_server: &AssetServer,
        _texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        let next_level = entity_instance
            .get_enum_field("to_level")
            .expect("to_level should exist");
        let entrance = entity_instance
            .get_enum_field("entrance")
            .expect("entrance should exist");
        let s = Self {
            next_level: (*next_level).clone(),
            entrance: (*entrance).clone(),
        };
        debug!("Making NextLevel with next_level = {:?}", s);
        s
    }
}


#[derive(Default, Bundle, LdtkEntity)]
struct NextLevelBundle {
    #[grid_coords]
    grid_coords: GridCoords,
    #[ldtk_entity]
    next_level: NextLevel,
}

#[derive(Default, Resource)]
pub struct Colliders {
    collider_locations: HashSet<GridCoords>,
    level_width: i32,
    level_height: i32,
}


impl Collider for Colliders {
    fn on_collider(&self, grid_coords: &GridCoords) -> bool {
        grid_coords.x < 0
        || grid_coords.y < 0
        || grid_coords.x >= self.level_width
        || grid_coords.y >= self.level_height
        || self.collider_locations.contains(grid_coords)
    }
}

#[derive(Default, Component, Debug)]
struct Entrance {
    name: String,
}

impl LdtkEntity for Entrance {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _tileset: Option<&Handle<Image>>,
        _tileset_definition: Option<&TilesetDefinition>,
        _asset_server: &AssetServer,
        _texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        let name = entity_instance
            .get_enum_field("name")
            .expect("Should have a name")
            .clone();
        Self {
            name
        }
    }
}

#[derive(Default, Bundle, LdtkEntity)]
struct EntranceBundle {
    #[grid_coords]
    grid_coords: GridCoords,
    #[ldtk_entity]
    entrance: Entrance,
}

#[derive(Default, Resource)]
struct Entrances {
    locations: HashMap<String, GridCoords>,
}

#[derive(Default, Component)]
struct Unwalkable;

#[derive(Default, Bundle, LdtkIntCell)]
struct UnwalkablesBundle {
    unwalkable: Unwalkable,
}
#[derive(Resource, Default, Debug)]
struct CameFrom {
    from: String,
}

#[derive(Default, Component, PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum LockKeyColor {
    #[default]
    Red,
    Blue,
    Green,
    Yellow,
    Brown
}

impl ToString for LockKeyColor {
    fn to_string(&self) -> String {
        match self {
            Self::Red => "red".to_string(),
            Self::Blue => "blue".to_string(),
            Self::Brown => "brown".to_string(),
            Self::Green => "green".to_string(),
            Self::Yellow => "yellow".to_string()
        }
    }
}

impl From<&str> for LockKeyColor {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "red" => Self::Red,
            "blue" => Self::Blue,
            "brown" => Self::Brown,
            "green" => Self::Green,
            "yellow" => Self::Yellow,
            _ => panic!("Unknown color {}", s)
        }
    }
}

impl From<String> for LockKeyColor {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl From<&String> for LockKeyColor {
    fn from(s: &String) -> Self {
        Self::from(s.as_str())
    }
}

impl LdtkEntity for LockKeyColor {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _tileset: Option<&Handle<Image>>,
        _tileset_definition: Option<&TilesetDefinition>,
        _asset_server: &AssetServer,
        _texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        entity_instance
            .get_enum_field("LockColor")
            .expect("color should exist")
            .into()
    }
}

#[derive(Default, Component)]
struct Key;

#[derive(Default, Component)]
struct Lock {
    is_locked: bool,
}

#[derive(Default, Bundle, LdtkEntity)]
struct KeyBundle {
    #[sprite_sheet_bundle(no_grid)]
    sprite_sheet: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
    #[ldtk_entity]
    color: LockKeyColor,
    key: Key,
}

#[derive(Default, Bundle, LdtkEntity)]
struct LockBundle {
    #[sprite_sheet_bundle(no_grid)]
    sprite_sheet: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
    #[ldtk_entity]
    color: LockKeyColor,
    lock: Lock,
}

#[derive(Default, Resource, Debug)]
struct Keys {
    color_location_pairing: HashMap<LockKeyColor, GridCoords>,
}

#[derive(Default, Resource, Debug)]
struct Locks {
    color_location_pairing: HashMap<LockKeyColor, GridCoords>,
}

#[derive(Default, Resource, Debug)]
pub struct CarriedKeys {
    pub keys: HashSet<LockKeyColor>,
}

fn cache_locks_and_keys(
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

fn cache_collider_location(
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

fn cache_goal_location(
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

fn cache_entrance_location(
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

fn check_for_goals<Player: Component>(
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

fn move_player_to_entrance<Player: Component>(
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

fn pickup_key<Player: Component>(
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

fn is_near_lock<Player: Component>(
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


pub fn setup(
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