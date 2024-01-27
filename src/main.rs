use crate::appstate::*;
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
use car::{car_control, tire_friction};
use constants::Constants;
use dialogues::{handle_dialogue_ui, setup_dialogues, DialogueList, DialogueState};
use parallax::{ParallaxHeight, ParallaxPlugin};

mod appstate;
mod car;
mod constants;
mod dialogues;
mod parallax;
mod road;
mod systems;
mod tilemap;
mod trigger;
mod ui;
mod utility;

use trigger::{handle_trigger_collisions, setup_trigger};

fn main() {
    let plugins = (
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    // provide the ID selector string here
                    canvas: Some("#tango-driver-canvas".into()),
                    fit_canvas_to_parent: true,
                    // ... any other window properties ...
                    ..default()
                }),
                ..Default::default()
            }),
        AppState::splash_screen(),
        FrameTimeDiagnosticsPlugin,
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        RapierDebugRenderPlugin::default(),
        EguiPlugin,
        DefaultInspectorConfigPlugin,
        ResourceInspectorPlugin::<Constants>::default(),
        JsonAssetPlugin::<DialogueList>::new(&["dialogues.json"]),
        ParallaxPlugin,
    );
    let update = (
        world_inspector.run_if(input_toggle_active(false, KeyCode::F1)),
        entity_inspector.run_if(input_toggle_active(false, KeyCode::F2)),
        show_fps,
        camera_follow,
        handle_dialogue_ui,
        handle_trigger_collisions,
        tire_friction,
        car_control,
    );
    let startup = (
        setup_graphics,
        setup_tilemap,
        setup_trigger,
        setup_physics,
        setup_dialogues,
        setup_ui.after(setup_dialogues),
    );

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
