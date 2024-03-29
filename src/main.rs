use crate::appstate::*;
use crate::systems::*;
use crate::tilemap::setup_tilemap;
use crate::ui::*;
use atlas_loader::setup_atlases;
use bevy::window::WindowMode;
use bevy::window::WindowResolution;
use bevy::{
    asset::AssetMetaCheck, diagnostic::FrameTimeDiagnosticsPlugin,
    input::common_conditions::input_toggle_active, prelude::*,
};
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::{quick::ResourceInspectorPlugin, DefaultInspectorConfigPlugin};
use bevy_rapier2d::prelude::*;
use car::update_tire_forces;
use car::{car_control, tire_friction};
use constants::Constants;
use dialogues::{handle_dialogue_ui, setup_dialogues, DialogueList, DialogueState};
use missions::setup_missions;
use missions::MissionState;
use parallax::{ParallaxHeight, ParallaxPlugin};

mod appstate;
mod atlas_loader;
mod buildings;
mod car;
mod constants;
mod dialogues;
mod missions;
mod parallax;
mod piece;
mod pointer;
mod road;
mod systems;
mod tilemap;
mod trigger;
mod ui;
mod utility;

use pointer::handle_pointer;
use trigger::{handle_trigger_collisions, setup_trigger};

pub fn window_primary() -> Window {
    Window {
        canvas: Some("#tango-driver-canvas".into()),
        title: String::from("Tango Driver"),
        //present_mode: bevy::window::PresentMode::Immediate,
        fit_canvas_to_parent: true,
        resolution: WindowResolution::new(1920., 1080.).with_scale_factor_override(1.),
        mode: WindowMode::Windowed,
        ..default()
    }
}

fn main() {
    let plugins = (
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(window_primary()),
                ..Default::default()
            }),
        AppState::splash_screen(),
        FrameTimeDiagnosticsPlugin,
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0), //.in_fixed_schedule(),
        // RapierDebugRenderPlugin::default(),
        //RapierDebugRenderPlugin::default(),
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
        car_control,
        handle_pointer,
    );
    let startup = (
        setup_atlases,
        setup_graphics,
        setup_tilemap,
        setup_trigger,
        setup_physics,
        setup_buildings,
        setup_dialogues,
        setup_missions.after(setup_physics),
        setup_ui.after(setup_dialogues),
    );

    App::new()
        .add_state::<AppState>()
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(MissionState::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0., 0.),
            ..Default::default()
        })
        .init_resource::<Constants>()
        .register_type::<Constants>()
        .register_type::<ParallaxHeight>()
        .insert_resource(DialogueState::default())
        .add_plugins(plugins)
        .add_systems(Update, update)
        .add_systems(Startup, startup)
        .add_systems(FixedUpdate, tire_friction)
        .add_systems(FixedUpdate, update_tire_forces.after(tire_friction))
        .add_systems(PostUpdate, handle_pointer)
        .run();
}
