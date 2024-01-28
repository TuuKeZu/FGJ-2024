use crate::constants::*;
use crate::{dialogues::DialogueBundle, tilemap::Tile};
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::PrimaryWindow,
};
use bevy_egui::{egui, EguiContext};
use bevy_inspector_egui::bevy_inspector;

// Inspectors for debugging
pub fn world_inspector(world: &mut World) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();

    egui::Window::new("World").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::both().show(ui, |ui| {
            bevy_inspector::ui_for_world(world, ui);
            ui.allocate_space(ui.available_size());
        })
    });
}

pub fn entity_inspector(world: &mut World) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();

    egui::Window::new("Entities").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::both().show(ui, |ui| {
            bevy_inspector::ui_for_world_entities_filtered::<(Without<Parent>, Without<Tile>)>(
                world, ui, true,
            );
            ui.allocate_space(ui.available_size());
        })
    });
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>, constants: Res<Constants>) {
    let font = asset_server.load("fonts/ComicMono.ttf");
    commands.spawn(DialogueBundle::new(constants, font));
}

pub fn show_fps(
    mut egui_context: Query<&mut EguiContext, With<PrimaryWindow>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    let mut egui_context = egui_context.single_mut();
    egui::Window::new("FPS").show(egui_context.get_mut(), |ui| {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                egui::Grid::new("position").show(ui, |ui| {
                    ui.label("FPS: ");
                    ui.label(format!("{value:.2}"));
                });
            }
        }
    });
}
