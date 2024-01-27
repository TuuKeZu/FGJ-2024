use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::constants::Constants;

#[derive(Component)]
pub struct Car {}

#[derive(Component)]
pub struct CarState {
    pub head_pointed_at: Vec2,
    pub steering_angle: f32,
}

impl CarState {
    pub fn turn_direction(&self) -> Vec2 {
        Vec2 {
            x: -f32::sin(self.steering_angle),
            y: f32::cos(self.steering_angle),
        }
        .rotate(self.head_pointed_at)
    }
}

#[derive(Bundle)]
pub struct CarBundle {
    car: Car,
    state: CarState,
    sprite: SpriteBundle,
    rb: RigidBody,
    rs: Restitution,
    collider: Collider,
    damping: Damping,
    gravity: GravityScale,
    ef: ExternalForce,
}

impl CarBundle {
    pub fn new(constants: Res<Constants>) -> Self {
        CarBundle {
            car: Car {},
            state: CarState {
                head_pointed_at: constants.car.head_pointed_start,
                steering_angle: constants.car.steering_angle,
            },
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(constants.car.size.x, constants.car.size.y)),
                    color: Color::rgb(255., 255., 255.),
                    ..Default::default()
                },
                ..Default::default()
            },
            rb: RigidBody::Dynamic,
            rs: Restitution::coefficient(0.7),
            damping: Damping {
                linear_damping: constants.car.linear_damping,
                angular_damping: constants.car.angular_damping,
            },
            collider: Collider::cuboid(constants.car.size.x, constants.car.size.y),
            gravity: GravityScale(0.),
            ef: ExternalForce {
                force: Vec2::ZERO,
                torque: 0.,
            },
        }
    }
}
