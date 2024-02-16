use crate::levels::components::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy::utils::{HashSet, HashMap};


#[derive(Default, Bundle, LdtkEntity)]
pub (crate) struct KeyBundle {
    #[sprite_sheet_bundle(no_grid)]
    sprite_sheet: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
    #[ldtk_entity]
    color: LockKeyColor,
    key: Key,
}

#[derive(Default, Bundle, LdtkEntity)]
pub (crate) struct LockBundle {
    #[sprite_sheet_bundle(no_grid)]
    sprite_sheet: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
    #[ldtk_entity]
    color: LockKeyColor,
    lock: Lock,
}

#[derive(Default, Resource, Debug)]
pub(crate) struct Keys {
    color_location_pairing: HashMap<LockKeyColor, GridCoords>,
}

#[derive(Default, Resource, Debug)]
pub(crate) struct Locks {
    color_location_pairing: HashMap<LockKeyColor, GridCoords>,
}

#[derive(Default, Resource, Debug)]
pub struct CarriedKeys {
    pub keys: HashSet<LockKeyColor>,
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
pub (crate) struct NextLevelBundle {
    #[grid_coords]
    grid_coords: GridCoords,
    #[ldtk_entity]
    next_level: NextLevel,
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

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle<PlayerComponent, AnimationTimer> where
PlayerComponent: Component + Default,
AnimationTimer: Resource + Component + Default, {
    player: PlayerComponent,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    timer: AnimationTimer,
}