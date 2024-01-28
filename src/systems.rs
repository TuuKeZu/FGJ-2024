use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    buildings::*,
    car::*,
    constants::Constants,
    dialogues::{DialogueHandle, DialogueList, DialogueState},
};

pub fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn setup_physics(mut commands: Commands, constants: Res<Constants>) {
    CarBundle::spawn(&mut commands, &constants)
        .with_player()
        .at(Vec2::new(200., 5.));

    CarBundle::spawn(&mut commands, &constants).at(Vec2::new(800., 5.));
}

pub fn setup_buildings(mut commands: Commands, constants: Res<Constants>) {
    BuildingBundle::spawn(&mut commands, constants);
}

pub fn camera_follow(
    time: Res<Time>,
    constants: Res<Constants>,
    car_q: Query<(&Transform, &Velocity), (With<Car>, With<Player>)>,
    mut camera_q: Query<
        (&mut Transform, &mut OrthographicProjection),
        (With<Camera2d>, Without<Car>),
    >,
) {
    let (car_pos, car_vel) = car_q.get_single().unwrap();
    let car_pos = car_pos.translation.xy();
    let car_vel = car_vel.linvel;

    // The camera target is `lookahead` seconds in front of car
    let camera_target_pos = car_pos + constants.camera.lookahead * car_vel;
    let camera_target_scale =
        1. + constants.camera.height_speed_factor * car_vel.length() / constants.car.max_speed;
    // TODO: Should the height scale with speed? This creates weird "zoom out" effect
    let camera_target_height = constants.camera.height * camera_target_scale;

    // Ease in camera using exponential decay
    let easing_factor = (-time.delta_seconds() * constants.camera.easing_speed).exp();

    let (mut camera_transform, mut camera_projection) = camera_q.get_single_mut().unwrap();
    let camera_pos = easing_factor * camera_transform.translation.xy()
        + (1. - easing_factor) * camera_target_pos;
    let camera_height = easing_factor * camera_transform.translation.z
        + (1. - easing_factor) * camera_target_height;
    let camera_scale =
        easing_factor * camera_projection.scale + (1. - easing_factor) * camera_target_scale;

    camera_transform.translation = Vec3::new(camera_pos.x, camera_pos.y, camera_height);
    camera_projection.scale = camera_scale;
}

pub fn update_dialogue(
    mut dialogue_state: ResMut<DialogueState>,
    dialogues: ResMut<Assets<DialogueList>>,
    dialogue: Res<DialogueHandle>,
) {
    if !dialogue_state.active {
        dialogue_state.load_dialogue("p1", dialogues, dialogue);
    }
}
