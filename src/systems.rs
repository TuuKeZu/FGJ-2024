use bevy::prelude::*;

use crate::{
    car::*,
    constants::Constants,
    dialogues::{Dialogue, DialogueHandle, DialogueState},
};

pub fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn setup_physics(mut commands: Commands, constants: Res<Constants>) {
    commands.spawn(CarBundle::new(constants));
}

pub fn camera_follow(
    car_q: Query<&Transform, With<Car>>,
    mut camera_q: Query<&mut Transform, (With<Camera2d>, Without<Car>)>,
    mut dialogue_state: ResMut<DialogueState>,
    dialogues: ResMut<Assets<Dialogue>>,
    dialogue: Res<DialogueHandle>,
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
