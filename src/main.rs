use bevy::{input::common_conditions::input_toggle_active, prelude::*, diagnostic::FrameTimeDiagnosticsPlugin, asset::AssetMetaCheck};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use bevy_rapier2d::prelude::*;
use constants::Constants;

mod constants;
mod systems;
mod ui;
mod components;

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(Constants::default())
        .add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(EguiPlugin)
        .add_plugins(DefaultInspectorConfigPlugin)
        .add_systems(
            Update,
            ui::world_inspector.run_if(input_toggle_active(false, KeyCode::F1)),
        )
        .add_systems(
            Update,
            ui::entity_inspector.run_if(input_toggle_active(false, KeyCode::F2)),
        )
        .add_systems(Startup, systems::setup_graphics)
        .add_systems(Startup, systems::setup_physics)
        .add_systems(Startup, ui::setup_ui)
        .add_systems(Update, ui::show_ball_position)
        .add_systems(Update, ui::update_fps)
        .run();
}
