use crate::systems::*;
use crate::tilemap::setup_tilemap;
use crate::ui::*;
use bevy::{
    asset::AssetMetaCheck, diagnostic::FrameTimeDiagnosticsPlugin,
    input::common_conditions::input_toggle_active, prelude::*,
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::{quick::ResourceInspectorPlugin, DefaultInspectorConfigPlugin};
use bevy_rapier2d::prelude::*;
use constants::Constants;
use parallax::{ParallaxHeight, ParallaxPlugin};

mod components;
mod constants;
mod parallax;
mod systems;
mod tilemap;
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
        ParallaxPlugin,
    );
    let update = (
        world_inspector.run_if(input_toggle_active(false, KeyCode::F1)),
        entity_inspector.run_if(input_toggle_active(false, KeyCode::F2)),
        move_car,
        show_fps,
        camera_follow,
    );
    let startup = (setup_graphics, setup_tilemap, setup_physics, setup_ui);

    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .init_resource::<Constants>()
        .register_type::<Constants>()
        .register_type::<ParallaxHeight>()
        .add_plugins(plugins)
        .add_systems(Update, update)
        .add_systems(Startup, startup)
        .run();
}
