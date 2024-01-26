use bevy::render::color::Color;
use bevy::{
    asset::AssetMetaCheck, diagnostic::FrameTimeDiagnosticsPlugin,
    input::common_conditions::input_toggle_active, prelude::*,
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use bevy_rapier2d::prelude::*;

#[derive(Resource)]
pub struct Constants {
    #[inspector(min = 10.0, max = 70.0)]
    pub font_size: f32,
    #[inspector(min = 10.0, max = 70.0)]
    pub fps_text_padding: f32,
    pub font_color: Color,
}

impl Default for Constants {
    fn default() -> Self {
        Self {
            font_size: 20.0,
            fps_text_padding: 10.0,
            font_color: Color::rgb(255., 255., 255.),
        }
    }
}
