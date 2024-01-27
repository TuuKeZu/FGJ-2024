use bevy::prelude::*;
use bevy::render::color::Color;

use bevy_inspector_egui::inspector_options::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

pub const TILE_SIZE: f32 = 400.;

#[derive(Resource, Clone, Copy, Reflect, Default, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct Constants {
    pub ui: UiConstants,
    pub car: CarConstants,
    pub building: BuildingConstants,
    pub camera: CameraConstants,
    pub trigger: TriggerConstants,
}

#[derive(Clone, Copy, Resource, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct CarConstants {
    pub size: Vec2,
    pub linear_damping: f32,
    pub angular_damping: f32,
    pub engine_force: f32,
    pub turn_force: f32,
    pub steering_angle: f32,
    pub steering_speed: f32,
    pub steering_back: f32,
    pub max_steer: f32,
    pub head_pointed_start: Vec2,
    pub close_to_zero: f32,
}
#[derive(Clone, Copy, Resource, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct BuildingConstants {
    pub size: Vec2,
}
#[derive(Clone, Copy, Resource, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct CameraConstants {
    #[inspector(min = 1.0, max = 700.0)]
    pub camera_height: f32,
}

impl Default for CameraConstants {
    fn default() -> Self {
        Self { camera_height: 50. }
    }
}

#[derive(Clone, Copy, Resource, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct TriggerConstants {
    pub size: Vec2,
    pub color: Color,
}

impl Default for TriggerConstants {
    fn default() -> Self {
        Self {
            size: Vec2::new(100., 100.),
            color: Color::rgb(255., 0., 0.),
        }
    }
}

impl Default for CarConstants {
    fn default() -> Self {
        Self {
            size: Vec2::new(42., 114.),
            linear_damping: 3.,
            angular_damping: 1.,
            engine_force: 250.,
            turn_force: 150.,
            steering_angle: 0.,
            steering_speed: 0.001,
            steering_back: 0.1,
            max_steer: std::f32::consts::PI / 4.0,
            head_pointed_start: Vec2::X,
            close_to_zero: 0.1,
        }
    }
}

impl Default for BuildingConstants {
    fn default() -> Self {
        Self {
            size: Vec2::new(64., 8.),
        }
    }
}

#[derive(Clone, Copy, Resource, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct UiConstants {
    pub font_size: f32,
    pub fps_text_padding: f32,
    pub font_color: Color,
}

impl Default for UiConstants {
    fn default() -> Self {
        Self {
            font_size: 20.,
            fps_text_padding: 10.,
            font_color: Color::rgb(255., 255., 255.),
        }
    }
}
