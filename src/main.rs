use crate::systems::*;
use crate::ui::*;
use bevy::{
    asset::AssetMetaCheck, diagnostic::FrameTimeDiagnosticsPlugin,
    input::common_conditions::input_toggle_active, prelude::*,
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::{quick::ResourceInspectorPlugin, DefaultInspectorConfigPlugin};
use bevy_rapier2d::prelude::*;
use constants::Constants;

mod components;
mod constants;
mod systems;
mod ui;

fn main() {
    let plugins = (
        DefaultPlugins,
        FrameTimeDiagnosticsPlugin,
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        RapierDebugRenderPlugin::default(),
        EguiPlugin,
        DefaultInspectorConfigPlugin,
        ResourceInspectorPlugin::<Constants>::default(),
    );
    let update = (
        world_inspector.run_if(input_toggle_active(false, KeyCode::F1)),
        entity_inspector.run_if(input_toggle_active(false, KeyCode::F2)),
        show_ball_position,
        move_car,
        show_fps
    );
    let startup = (setup_graphics, setup_physics, setup_ui);

    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .init_resource::<Constants>()
        .register_type::<Constants>()
        .add_plugins(plugins)
        .add_systems(Update, update)
        .add_systems(Startup, startup)
        .run();
}
