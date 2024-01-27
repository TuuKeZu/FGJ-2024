use bevy::prelude::*;

use bevy_rapier2d::prelude::*;

use crate::{components::*, constants::Constants, dialogues::{DialogueState, Dialogue, DialogueHandle}};

pub fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn setup_physics(mut commands: Commands, constants: Res<Constants>) {
    commands.spawn(CarBundle::new(constants));
}

pub fn move_car(
    keyboard_input: Res<Input<KeyCode>>,
    constants: Res<Constants>,
    mut query: Query<(&mut ExternalForce, &mut Transform, &mut CarState), With<Car>>,
) {
    let (mut car_ef, mut transform, mut carstate) = query.single_mut();
    let mut moving_y = false;
    let mut moving_angular = false;

    let head_pointed_angle = carstate
        .head_pointed_at
        .angle_between(constants.car.head_pointed_start);
    transform.rotation = Quat::from_axis_angle(
        Vec3 {
            x: 0.,
            y: 0.,
            z: -f32::signum(head_pointed_angle),
        },
        head_pointed_angle.abs(),
    );
    if keyboard_input.pressed(KeyCode::Up) {
        car_ef.force = carstate.turn_direction() * constants.car.engine_force;
        carstate.head_pointed_at =
            Vec2::from_angle(carstate.steering_angle).rotate(carstate.head_pointed_at);
        moving_y = true;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        let mut turn_vec = carstate.turn_direction();
        turn_vec.x = -turn_vec.x;
        car_ef.force = turn_vec * -constants.car.engine_force;
        carstate.head_pointed_at =
            Vec2::from_angle(carstate.steering_angle).rotate(carstate.head_pointed_at);
        moving_y = true;
    }

    if keyboard_input.pressed(KeyCode::Left) {
        // car_ef.force.x = -constants.physics.turn_force;
        carstate.steering_angle += constants.car.steering_speed;
        carstate.steering_angle = carstate
            .steering_angle
            .clamp(-constants.car.max_steer, constants.car.max_steer);
        moving_angular = true;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        carstate.steering_angle -= constants.car.steering_speed;
        carstate.steering_angle = carstate
            .steering_angle
            .clamp(-constants.car.max_steer, constants.car.max_steer);
        // car_ef.force.x = constants.physics.turn_force;
        moving_angular = true;
    }

    if !moving_y {
        car_ef.force = Vec2::ZERO;
    }

    if !moving_angular {
        if (carstate.steering_angle - 0.0).abs() > constants.car.close_to_zero {
            carstate.steering_angle +=
                -f32::signum(carstate.steering_angle) * constants.car.steering_back;
        } else {
            carstate.steering_angle = 0.0;
        }
    }
}

pub fn camera_follow(car_q: Query<&Transform, With<Car>>, mut camera_q: Query<&mut Transform, (With<Camera2d>, Without<Car>)>,
    mut dialogue_state: ResMut<DialogueState>,
    dialogues: ResMut<Assets<Dialogue>>,
    dialogue: Res<DialogueHandle>
) {
    let car_transform = car_q.get_single().unwrap();
    let mut camera_transform = camera_q.get_single_mut().unwrap();

    // TODO add easing
    camera_transform.translation.x = car_transform.translation.x;
    camera_transform.translation.y = car_transform.translation.y;

    if !dialogue_state.active {
        dialogue_state.load_dialogue(dialogues, dialogue);
    }
}
