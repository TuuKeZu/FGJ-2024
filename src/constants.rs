use bevy::prelude::*;
use bevy::render::color::Color;

use bevy_inspector_egui::inspector_options::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

#[derive(Resource, Reflect, Default, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct Constants {
    pub ui: UiConstants,
    pub physics: PhysicsConstants,
}

#[derive(Resource, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct PhysicsConstants {
    pub size: Vec2,
    pub linear_damping: f32,
    pub angular_damping: f32,
    pub engine_force: f32,
    pub turn_force: f32,
    pub steering_force: f32,
    #[inspector(min = 1.0, max = 700.0)]
    pub camera_height: f32,
}

impl Default for PhysicsConstants {
    fn default() -> Self {
        Self {
            size: Vec2::new(42., 114.),
            linear_damping: 3.,
            angular_damping: 1.,
            engine_force: 250.,
            turn_force: 150.,
            steering_force: 10.,
            camera_height: 20.,
        }
    }
}

#[derive(Resource, Reflect, Default, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct UiConstants {
    #[inspector(min = 10.0, max = 70.0)]
    pub font_size: f32,
    #[inspector(min = 10.0, max = 70.0)]
    pub fps_text_padding: f32,
    pub font_color: Color,
}
