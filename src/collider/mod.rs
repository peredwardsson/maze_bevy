use bevy::prelude::*;

enum ColliderType {
    Rectangle,
    // no other types yet
}

#[derive(Component)]
struct Collider {
    collider_type: ColliderType,
}

impl Default for Collider {
    fn default() -> Self {
        Self {
            collider_type: ColliderType::Rectangle,
        }
    }
}

#[derive(Component)]
struct ColliderSize {
    width: f32,
    height: f32,
}

impl Default for ColliderSize {
    fn default() -> Self {
        Self {
            width: 1.,
            height: 1.,
        }
    }
}

#[derive(Component)]
struct ColliderPosition {
    x: f32,
    y: f32,
}

impl Default for ColliderPosition {
    fn default() -> Self {
        Self { x: 0., y: 0. }
    }
}

struct ColliderBundle {
    collider: Collider,
    collider_size: ColliderSize,
    collider_position: ColliderPosition,
}

impl ColliderBundle {

    fn is_collide(&self, other: &ColliderBundle) -> bool {
        let self_x = self.collider_position.x;
        let self_y = self.collider_position.y;
        let self_width = self.collider_size.width;
        let self_height = self.collider_size.height;

        let other_x = other.collider_position.x;
        let other_y = other.collider_position.y;
        let other_width = other.collider_size.width;
        let other_height = other.collider_size.height;

        let self_left = self_x - self_width / 2.;
        let self_right = self_x + self_width / 2.;
        let self_top = self_y + self_height / 2.;
        let self_bottom = self_y - self_height / 2.;

        let other_left = other_x - other_width / 2.;
        let other_right = other_x + other_width / 2.;
        let other_top = other_y + other_height / 2.;
        let other_bottom = other_y - other_height / 2.;

        self_left < other_right
            && self_right > other_left
            && self_top > other_bottom
            && self_bottom < other_top
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_is_collide() {
        let collider1 = ColliderBundle {
            collider: Collider::default(),
            collider_size: ColliderSize {
                width: 1.,
                height: 1.,
            },
            collider_position: ColliderPosition { x: 0., y: 0. },
        };

        let collider2 = ColliderBundle {
            collider: Collider::default(),
            collider_size: ColliderSize {
                width: 1.,
                height: 1.,
            },
            collider_position: ColliderPosition { x: 0., y: 0. },
        };

        assert!(collider1.is_collide(&collider2));
    }
}