use bevy::render::color::Color;
use bevy::{
    asset::AssetMetaCheck, diagnostic::FrameTimeDiagnosticsPlugin,
    input::common_conditions::input_toggle_active, prelude::*,
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::inspector_options::ReflectInspectorOptions;
use bevy_inspector_egui::{DefaultInspectorConfigPlugin, InspectorOptions};
use bevy_rapier2d::prelude::*;

#[derive(Resource, Reflect, Default, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct Constants {
    #[inspector(min = 10.0, max = 70.0)]
    pub font_size: f32,
    #[inspector(min = 10.0, max = 70.0)]
    pub fps_text_padding: f32,
    pub font_color: Color,
}

