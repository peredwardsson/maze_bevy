use bevy_ecs_ldtk::prelude::*;
use bevy::prelude::*;

use crate::constants::GRID_SIZE_IVEC;

pub (crate) fn to_grid_coords(
    tf: Transform
) -> GridCoords {
    bevy_ecs_ldtk::utils::translation_to_grid_coords(tf.translation.truncate(), GRID_SIZE_IVEC)
}

pub (crate) fn to_translation(
    gc: GridCoords,
    z: f32,
) -> Vec3 {
    bevy_ecs_ldtk::utils::grid_coords_to_translation(gc, GRID_SIZE_IVEC).extend(z)
}
