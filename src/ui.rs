use bevy::{input::common_conditions::input_toggle_active, prelude::*, window::PrimaryWindow};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::{bevy_inspector, DefaultInspectorConfigPlugin};
use bevy_rapier2d::prelude::*;

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
