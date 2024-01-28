use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{car::Car, constants::Constants, trigger::Target};

#[derive(Component)]
pub struct Pointer {}

#[derive(Bundle)]
pub struct PointerBundle {
    pub sprite_bundle: SpriteBundle,
    pub pointer: Pointer,
}

impl PointerBundle {
    pub fn new(_constants: &Res<Constants>) -> Self {
        Self {
            pointer: Pointer {},
            sprite_bundle: SpriteBundle {
                transform: Transform::from_translation(Vec3::new(0., 0., 100.)),
                sprite: Sprite {
                    color: Color::rgb(255., 0., 0.),
                    custom_size: Some(Vec2::new(2. * 5., 2. * 20.)),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

pub fn setup_pointer(mut commands: Commands, constants: Res<Constants>) {
    commands.spawn(PointerBundle::new(&constants));
}

pub fn handle_pointer(
    mut pointer_q: Query<&mut Transform, With<Pointer>>,
    car_q: Query<(&Transform, &Car), Without<Pointer>>,
    target_q: Query<(&Transform, &Target), Without<Pointer>>,
) {
    let mut pointer_transform = pointer_q.get_single_mut().unwrap();
    let (car_transform, _) = car_q.get_single().unwrap();

    pointer_transform.translation.x = car_transform.translation.x;
    pointer_transform.translation.y = car_transform.translation.y;

    if let Ok((target_transform, _)) = target_q.get_single() {
        let delta = target_transform.translation - pointer_transform.translation;

        let angle = delta.y.atan2(delta.x);
        pointer_transform.rotation = Quat::from_euler(EulerRot::XYZ, 0., 0., angle - PI / 2.);
    }
}
