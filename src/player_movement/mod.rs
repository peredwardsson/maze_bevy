use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

/// Add this component to the player entity. This is used to
/// identify the player entity in the query and apply controls
/// to it.
#[derive(Component)]
pub struct PlayerControlled;

#[derive(Default)]
pub struct PlayerMovementPlugin;

#[derive(Component)]
/// Represents a collider used for collision detection. Set the
/// size to the size of the sprite. Must be rectangular. The
/// origin of the sprite is assumed to be in the center.
pub struct Collider {
    pub size: Vec2,
}

#[derive(Resource, Default, Debug, Clone)]
struct PlayerMovement {
    // wished movement of the player. This is not the actual movement
    // but will be used to check for collisions. If there is no collision
    // the player will be moved. we assume a 2d movement for now.
    player_movement: Vec2,
    has_moved: bool,
}

/// The speed of the player in some kind of arbitrary units. If you add
/// this resource to your app, it will override the default speed of 100.
#[derive(Resource)]
pub struct Speed(pub f32);

impl Default for Speed {
    fn default() -> Self {
        Self(100.)
    }

}

fn take_input(
    mut player_movement: ResMut<PlayerMovement>,
    speed: Res<Speed>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let left_button = KeyCode::A;
    let right_button = KeyCode::D;
    let up_button = KeyCode::W;
    let down_button = KeyCode::S;
    let x_sign = {
        if input.pressed(left_button) {
            -1.
        } else if input.pressed(right_button) {
            1.
        } else {
            0.
        }
    };
    let x_movement = x_sign * speed.0 * time.delta_seconds();
    let y_sign = {
        if input.pressed(up_button) {
            1.
        } else if input.pressed(down_button) {
            -1.
        } else {
            0.
        }
    };

    let y_movement = y_sign * speed.0 * time.delta_seconds();

    *player_movement = PlayerMovement {
        player_movement: Vec2::new(
            x_movement,
            y_movement,
        ),
        has_moved: false,
    };
}

fn check_for_collisions(
    player_controlled_query: Query<(&Transform, Option<&Collider>), With<PlayerControlled>>,
    colliders: Query<(&Transform, &Collider), Without<PlayerControlled>>,
    mut player_movement: ResMut<PlayerMovement>,
) {
    // let mut collided = vec![];
    let Ok(
        (&Transform{translation: player_translation,..}, maybe_player_collider)
    ) = player_controlled_query.get_single() else { return;};
    let Some(&Collider {size: player_size}) = maybe_player_collider else {return;};
    let next_position = player_translation + Vec3::new(player_movement.player_movement.x, player_movement.player_movement.y, 0.);
    colliders
        .iter()
        .map(|(tf, collider)| {
            collide(
                next_position,
                player_size,
                tf.translation,
                collider.size,
            )
        })
        .flatten()
        .for_each(|collision| deal_with_collision(collision, &mut player_movement));

}

fn deal_with_collision(
    collided: Collision,
    player_movement: &mut PlayerMovement,
) {
    match collided {
        Collision::Right => {
            if player_movement.player_movement.x < 0. {
                player_movement.player_movement.x = 0.;
            }
        },
        Collision::Left => {
            if player_movement.player_movement.x > 0. {
                player_movement.player_movement.x = 0.;

            }
        },
        Collision::Bottom => {
            if player_movement.player_movement.y > 0. {
                player_movement.player_movement.y = 0.;
            }
        },
        Collision::Top => {
            if player_movement.player_movement.y < 0. {
                player_movement.player_movement.y = 0.;
            }
        },
        _ => {}

    }
}

fn move_player(
    mut player_controlled_query: Query<&mut Transform, With<PlayerControlled>>,
    mut player_movement: ResMut<PlayerMovement>,
) {
    let Ok(mut player) = player_controlled_query.get_single_mut() else {return;};
    if !player_movement.has_moved {
        player.translation += Vec3::new(player_movement.player_movement.x, player_movement.player_movement.y, 0.);
        player_movement.has_moved = true;
    }
}

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PlayerMovement::default())
            .insert_resource(Speed::default())
            .add_systems(
                Update,
                (
                    take_input,
                    check_for_collisions,
                    move_player,
                ).chain()
            )
            ;
    }
}

pub mod prelude {
    pub use super::{PlayerControlled, PlayerMovementPlugin, Collider, Speed};
}