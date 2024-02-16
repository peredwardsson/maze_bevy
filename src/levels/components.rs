use bevy::prelude::*;
use bevy::utils::{HashSet, HashMap};
use bevy_ecs_ldtk::prelude::*;


#[derive(Default, Resource)]
pub struct Colliders {
    pub (crate) collider_locations: HashSet<GridCoords>,
    level_width: i32,
    level_height: i32,
}

// NExt level structs
#[derive(Default, Component, Debug)]
pub (crate) struct NextLevel {
    pub (crate) next_level: String,
    pub (crate) entrance: String,
}

#[derive(Default, Resource)]
pub (crate) struct NextLevels {
    locations: HashSet<GridCoords>,
}
// Entrance structs
#[derive(Default, Component, Debug)]
pub (crate) struct Entrance {
    pub (crate) name: String,
}

#[derive(Default, Resource)]
pub (crate) struct Entrances {
    locations: HashMap<String, GridCoords>,
}

#[derive(Default, Bundle, LdtkEntity)]
pub (crate) struct EntranceBundle {
    #[grid_coords]
    grid_coords: GridCoords,
    #[ldtk_entity]
    entrance: Entrance,
}


#[derive(Default, Component)]
pub (crate) struct Unwalkable;

#[derive(Default, Component)]
pub (crate) struct Collider;

#[derive(Default, Bundle, LdtkIntCell)]
pub (crate) struct UnwalkablesBundle {
    unwalkable: Unwalkable,
}

#[derive(Resource, Default, Debug)]
pub (crate) struct CameFrom {
    pub (crate) from: String,
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


#[derive(Default, Component)]
pub (crate)struct Key;

#[derive(Default, Component)]
pub (crate)struct Lock {
    is_locked: bool,
}
