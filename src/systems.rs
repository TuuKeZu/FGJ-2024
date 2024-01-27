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
    let (vec3d, rad) = transform.rotation.to_axis_angle();
    let rot_vec: Vec2 = Vec2 {
        x: -f32::sin(vec3d.z * rad),
        y: f32::cos(vec3d.z * rad),
    };
    // TODO calculate the force direction from the transform.rotation

    if keyboard_input.pressed(KeyCode::Up) {
        car_ef.force = rot_vec * constants.physics.engine_force;
        moving_y = true;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        car_ef.force = rot_vec * -constants.physics.engine_force;
        moving_y = true;
    }

    if !moving_y {
        car_ef.force = Vec2::ZERO;
    }

    if keyboard_input.pressed(KeyCode::Left) {
        // car_ef.force.x = -constants.physics.turn_force;
        car_ef.torque = constants.physics.steering_force;
        moving_angular = true;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        car_ef.torque = -constants.physics.steering_force;
        // car_ef.force.x = constants.physics.turn_force;
        moving_angular = true;
    }

    if !moving_y {
        car_ef.force = Vec2::ZERO;
    }

    if !moving_angular {
        car_ef.torque = 0.;
    }
}

pub fn camera_follow(car_q: Query<&Transform, With<Car>>, mut camera_q: Query<&mut Transform, (With<Camera2d>, Without<Car>)>) {
    let car_transform = car_q.get_single().unwrap();
    let mut camera_transform = camera_q.get_single_mut().unwrap();

    // TODO add easing
    camera_transform.translation.x = car_transform.translation.x;
    camera_transform.translation.y = car_transform.translation.y;
}
