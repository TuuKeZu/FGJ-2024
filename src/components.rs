use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    constants::Constants,
    parallax::{ParallaxImages, ParallaxSprite},
};

#[derive(Component)]
pub struct Car {}

#[derive(Component)]
pub struct CarState {}

#[derive(Bundle)]
pub struct CarBundle {
    car: Car,
    state: CarState,
    sprite: ParallaxSprite,
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
            sprite: ParallaxSprite {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(
                        2. * constants.physics.size.x,
                        2. * constants.physics.size.y,
                    )),
                    rect: Some(Rect::new(25., 12., 39., 50.)),
                    ..Default::default()
                },
                images: ParallaxImages::new(vec![
                    ("car/car-0.png", 0.0),
                    ("car/car-1.png", 0.5),
                    ("car/car-2.png", 1.0),
                    ("car/car-3.png", 1.5),
                ]),
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
