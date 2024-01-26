use bevy::prelude::*;

use bevy_rapier2d::prelude::*;

use crate::{components::*, constants::Constants};

pub fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());
}

pub fn setup_physics(mut commands: Commands, constants: Res<Constants>) {
    commands.spawn(CarBundle::new(constants));
}

pub fn move_car(
    keyboard_input: Res<Input<KeyCode>>,
    constants: Res<Constants>,
    mut query: Query<(&mut ExternalForce, &Transform), With<Car>>,
) {
    let (mut car_ef, transform) = query.single_mut();
    let mut moving_y = false;
    let mut moving_angular = false;

    // TODO calculate the force direction from the transform.rotation

    if keyboard_input.pressed(KeyCode::Up) {
        car_ef.force.y = constants.physics.engine_force;
        moving_y = true;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        car_ef.force.y = -constants.physics.engine_force;
        moving_y = true;
    }

    if !moving_y {
        car_ef.force.y = 0.;
    }

    if keyboard_input.pressed(KeyCode::Left) {
        car_ef.torque = -constants.physics.steering_force;
        moving_angular = true;
    }

    if keyboard_input.pressed(KeyCode::Left) {
        car_ef.torque = constants.physics.steering_force;
        moving_angular = true;
    }

    if !moving_y {
        car_ef.force.y = 0.;
    }

    if !moving_angular {
        car_ef.torque = 0.;
    }
}
