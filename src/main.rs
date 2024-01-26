use bevy::{input::common_conditions::input_toggle_active, prelude::*, window::PrimaryWindow};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::{bevy_inspector, DefaultInspectorConfigPlugin};
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(EguiPlugin)
        .add_plugins(DefaultInspectorConfigPlugin)
        .add_systems(
            Update,
            world_inspector.run_if(input_toggle_active(false, KeyCode::F1)),
        )
        .add_systems(
            Update,
            entity_inspector.run_if(input_toggle_active(false, KeyCode::F2)),
        )
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, show_ball_position)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(mut commands: Commands, assets: Res<AssetServer>) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(
            Transform::from_xyz(0.0, -100.0, 0.0).with_rotation(Quat::from_rotation_z(-0.1)),
        ));

    /* Create the bouncing ball. */
    let ball_radius = 50.;
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(ball_radius))
        .insert(Restitution::coefficient(0.7))
        .insert(SpriteBundle {
            texture: assets.load("ball.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(2. * ball_radius, 2. * ball_radius)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
}

fn show_ball_position(
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
fn world_inspector(world: &mut World) {
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

fn entity_inspector(world: &mut World) {
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
