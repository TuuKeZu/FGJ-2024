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
    pub scale: f32,
    pub acceleration: f32,
    pub breaking_force: f32,
    pub max_speed: f32,
    pub max_backing_speed: f32,
    pub max_steer: f32,
}
#[derive(Clone, Copy, Resource, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct BuildingConstants {
    pub scale: f32,
}
#[derive(Clone, Copy, Resource, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct CameraConstants {
    #[inspector(min = 1.0, max = 700.0)]
    pub height: f32,
    #[inspector(min = 0.0, max = 3.0)]
    pub height_speed_factor: f32,
    #[inspector(min = 0.0, max = 3.0)]
    pub lookahead: f32,
    #[inspector(min = 0.1, max = 3.0)]
    pub easing_speed: f32,
}

impl Default for CameraConstants {
    fn default() -> Self {
        Self {
            height: 50.,
            height_speed_factor: 1.,
            lookahead: 1.,
            easing_speed: 2.,
        }
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

pub const CAR_COLLIDER_SIZE_PX: Vec2 = Vec2::new(14., 38.);

impl Default for CarConstants {
    fn default() -> Self {
        Self {
            scale: 3.0,
            max_speed: 5000.,
            max_backing_speed: 200.,
            acceleration: 70.,
            max_steer: std::f32::consts::PI / 6.0,
            breaking_force: 150.,
        }
    }
}

impl Default for BuildingConstants {
    fn default() -> Self {
        Self { scale: 2.0 }
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
