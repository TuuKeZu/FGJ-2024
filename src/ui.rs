use crate::{constants::*, tilemap::Tile};
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::PrimaryWindow,
};
use bevy_egui::{egui, EguiContext};
use bevy_inspector_egui::bevy_inspector;
use bevy_rapier2d::prelude::*;

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

pub fn setup_ui(_commands: Commands, _asset_server: Res<AssetServer>, _constants: Res<Constants>) {
    // let font = asset_server.load("fonts/ComicMono.ttf");
    // commands.spawn(FpsBundle::new(font));
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

// Legacy FPS counter: do not use this; left for reference on how to use TextBundles
#[derive(Component)]
pub struct FpsText {}

#[derive(Bundle)]
pub struct FpsBundle {
    text_bundle: TextBundle,
    text: FpsText,
}

impl FpsBundle {
    pub fn new(constants: Res<Constants>, font: Handle<Font>) -> Self {
        Self {
            text_bundle: TextBundle::from_sections([
                TextSection::new(
                    "fps: ",
                    TextStyle {
                        font: font.clone(),
                        font_size: constants.ui.font_size,
                        color: constants.ui.font_color,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font,
                    font_size: constants.ui.font_size,
                    color: constants.ui.font_color,
                }),
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                right: Val::Px(constants.ui.fps_text_padding),
                top: Val::Px(constants.ui.fps_text_padding),
                ..default()
            }),
            text: FpsText {},
        }
    }
}

#[allow(unused)]
pub fn update_fps(diagnostics: Res<DiagnosticsStore>, mut fps_q: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut fps_q {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}
