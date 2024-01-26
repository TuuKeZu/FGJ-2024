use bevy::{prelude::*, diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin, DiagnosticsStore}};

use bevy_rapier2d::prelude::*;

use crate::ui::{FpsBundle, FpsText};

pub fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());
}

pub fn setup_physics(mut commands: Commands, assets: Res<AssetServer>) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(
            Transform::from_xyz(0.0, -100.0, 0.0).with_rotation(Quat::from_rotation_z(-0.1)),
        ));

    /* Create the bouncing ball. */
    let ball_radius = 50.;
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(ball_radius))
        .insert(Restitution::coefficient(0.7))
        .insert(SpriteBundle {
            texture: assets.load("ball.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(2. * ball_radius, 2. * ball_radius)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
}
