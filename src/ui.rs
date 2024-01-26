use bevy::{prelude::*, window::PrimaryWindow, diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}};
use bevy_egui::{egui, EguiContext};
use bevy_inspector_egui::bevy_inspector;
use bevy_rapier2d::prelude::*;
use crate::constants::*;

#[derive(Component)]
pub struct FpsText {}

#[derive(Bundle)]
pub struct FpsBundle {
    text_bundle: TextBundle,
    text: FpsText
}

impl FpsBundle {
    pub fn new(font: Handle<Font>) -> Self {
        Self {
            text_bundle: TextBundle::from_sections([
                TextSection::new(
                    "fps: ",
                    TextStyle {
                        font: font.clone(),
                        font_size: FONT_SIZE,
                        color: FONT_COLOR
                    }
                ),
                TextSection::from_style(TextStyle {
                    font,
                    font_size: FONT_SIZE,
                    color: FONT_COLOR,
                }),
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                right: Val::Px(FPS_TEXT_PADDING),
                top: Val::Px(FPS_TEXT_PADDING),
                ..default()
            }),
            text: FpsText { }
        }
    }
}

pub fn show_ball_position(
    mut egui_context: Query<&mut EguiContext, With<PrimaryWindow>>,
    positions: Query<&Transform, With<RigidBody>>,
) {
    let mut egui_context = egui_context.single_mut();
    egui::Window::new("Ball").show(egui_context.get_mut(), |ui| {
        for transform in positions.iter() {
            egui::Grid::new("position").show(ui, |ui| {
                ui.label("");
                ui.label("X");
                ui.label("Y");
                ui.end_row();
                ui.label("Position");
                ui.label(format!("{:4.1}", transform.translation.x));
                ui.label(format!("{:4.1}", transform.translation.y));
            });
        }
    });
}

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
            bevy_inspector::ui_for_world_entities(world, ui);
            ui.allocate_space(ui.available_size());
        })
    });
}


pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/ComicMono.ttf");

    commands.spawn(FpsBundle::new(font));
}

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
