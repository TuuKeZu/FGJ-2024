use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    car::Car,
    parallax::{ParallaxImages, ParallaxSprite},
    trigger::Target,
};

#[derive(Component)]
pub struct Pointer {}

#[derive(Bundle)]
pub struct PointerBundle {
    pub sprite_bundle: ParallaxSprite,
    pub pointer: Pointer,
}

impl PointerBundle {
    pub fn new() -> Self {
        Self {
            pointer: Pointer {},
            sprite_bundle: ParallaxSprite {
                visibility: VisibilityBundle::default(),
                transform: TransformBundle::default(),
                images: ParallaxImages::new(
                    "arrow",
                    Sprite {
                        custom_size: Some(1. * Vec2::new(64., 64.)),
                        ..Default::default()
                    },
                ),
            },
        }
    }
}

pub fn handle_pointer(
    mut pointer_q: Query<(&mut Transform, &Parent), With<Pointer>>,
    car_q: Query<&GlobalTransform, (With<Car>, Without<Pointer>)>,
    target_q: Query<&Transform, (With<Target>, Without<Pointer>)>,
) {
    let (mut pointer_transform, parent) = pointer_q.get_single_mut().unwrap();
    let (_, parent_rotation, parent_translation) = car_q
        .get(parent.get())
        .unwrap()
        .to_scale_rotation_translation();

    // Make pointer point towards target
    if let Ok(target_transform) = target_q.get_single() {
        let delta = target_transform.translation - parent_translation;

        let angle = delta.y.atan2(delta.x) - parent_rotation.to_euler(EulerRot::XYZ).2;
        let target_rotation = Quat::from_euler(EulerRot::XYZ, 0., 0., angle - PI / 2.);
        pointer_transform.rotation = target_rotation;
    }
}
