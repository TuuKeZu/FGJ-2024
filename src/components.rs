use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::constants::Constants;

#[derive(Component)]
pub struct Car {}

#[derive(Component)]
pub struct CarState {}

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
        let car = CarBundle {
            car: Car {},
            state: CarState {},
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(
                        constants.physics.size.x,
                        constants.physics.size.y,
                    )),
                    color: Color::rgb(255., 255., 255.),
                    ..Default::default()
                },
                ..Default::default()
            },
            rb: RigidBody::Dynamic,
            rs: Restitution::coefficient(0.7),
            damping: Damping {
                linear_damping: constants.physics.linear_damping,
                angular_damping: constants.physics.angular_damping,
            },
            collider: Collider::cuboid(constants.physics.size.x, constants.physics.size.y),
            gravity: GravityScale(0.),
            ef: ExternalForce {
                force: Vec2::ZERO,
                torque: 0.,
            },
        };

        car
    }
}
