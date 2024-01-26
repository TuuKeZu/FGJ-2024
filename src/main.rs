use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use bevy_rapier2d::prelude::*;

mod systems;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
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
        .add_systems(Update, ui::show_ball_position)
        .run();
}
