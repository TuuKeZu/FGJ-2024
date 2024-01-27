use crate::systems::*;
use crate::tilemap::setup_tilemap;
use crate::ui::*;
use bevy::{
    asset::AssetMetaCheck, diagnostic::FrameTimeDiagnosticsPlugin,
    input::common_conditions::input_toggle_active, prelude::*,
};
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::{quick::ResourceInspectorPlugin, DefaultInspectorConfigPlugin};
use bevy_rapier2d::prelude::*;
use constants::Constants;
use parallax::{ParallaxHeight, ParallaxPlugin};
use dialogues::{DialogueState, handle_dialogue_ui, setup_dialogues};

mod components;
mod constants;
mod parallax;
mod systems;
mod tilemap;
mod ui;
mod dialogues;

use dialogues::Dialogue;


#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Loading,
}


fn main() {
    let plugins = (
        DefaultPlugins,
        FrameTimeDiagnosticsPlugin,
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        RapierDebugRenderPlugin::default(),
        EguiPlugin,
        DefaultInspectorConfigPlugin,
        ResourceInspectorPlugin::<Constants>::default(),
        JsonAssetPlugin::<Dialogue>::new(&["dialogue.json"]),
        ParallaxPlugin,
    );
    let update = (
        world_inspector.run_if(input_toggle_active(false, KeyCode::F1)),
        entity_inspector.run_if(input_toggle_active(false, KeyCode::F2)),
        move_car,
        show_fps,
        camera_follow,
        handle_dialogue_ui
    );
    let startup = (setup_graphics, setup_tilemap, setup_physics, setup_dialogues, setup_ui.after(setup_dialogues));

    App::new()
        .add_state::<AppState>()
        .insert_resource(AssetMetaCheck::Never)
        .init_resource::<Constants>()
        .register_type::<Constants>()
        .register_type::<ParallaxHeight>()
        .insert_resource(DialogueState::default())
        .add_plugins(plugins)
        .add_systems(Update, update)
        .add_systems(Startup, startup)
        .run();
}
