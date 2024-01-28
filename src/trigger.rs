use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use bevy_rapier2d::{prelude::*, rapier::geometry::CollisionEventFlags};

use crate::{
    constants::Constants,
    dialogues::{DialogueHandle, DialogueList, DialogueState},
    missions::MissionState,
};

#[derive(Component)]
pub struct Trigger {}

#[derive(Component)]
pub struct Target {}

#[derive(Debug, Clone, Copy, Component, InspectorOptions)]
pub enum TriggerType {
    StartMission,
    StopMission,
}

#[derive(Bundle)]
pub struct TriggerBundle {
    trigger: Trigger,
    trigger_type: TriggerType,
    sprite: SpriteBundle,
    collider: Collider,
    sensor: Sensor,
    active_events: ActiveEvents,
}

impl TriggerBundle {
    pub fn new(trigger_type: TriggerType, constants: &Res<Constants>) -> Self {
        Self {
            trigger: Trigger {},
            trigger_type,
            sprite: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(-200., -200., 0.),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: constants.trigger.color,
                    custom_size: Some(Vec2::new(
                        2. * constants.trigger.size.x,
                        2. * constants.trigger.size.y,
                    )),
                    rect: Some(Rect::new(25., 12., 39., 50.)),
                    ..Default::default()
                },
                ..Default::default()
            },
            collider: Collider::cuboid(constants.trigger.size.x, constants.trigger.size.y),
            sensor: Sensor {},
            active_events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}

pub fn setup_trigger(commands: Commands, constants: Res<Constants>) {
    let _ = commands;
    let _ = constants;
    /*
    commands.spawn(TriggerBundle::new(&constants));
    commands
        .spawn(TriggerBundle::new(&constants))
        .insert(TriggerType::StopMission)
        .insert(Transform {
            translation: Vec3::new(200., 200., 0.),
            ..Default::default()
        });
    */
}

pub fn handle_trigger_collisions(
    mut commands: Commands,
    // mut car_q: Query<&mut CarState, (With<Car>, With<Player>)>,
    mut collision_events: EventReader<CollisionEvent>,
    mut mission_state: ResMut<MissionState>,
    spawn_q: Query<(Entity, &TriggerType), With<Trigger>>,
    constants: Res<Constants>,
    mut dialogue_state: ResMut<DialogueState>,
    mut dialogues: ResMut<Assets<DialogueList>>,
    dialogue: Res<DialogueHandle>,
) {
    // let mut car_state = car_q.get_single_mut().unwrap();

    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(c1, c2, flags) => {
                if flags != &CollisionEventFlags::SENSOR {
                    return;
                }

                let a = spawn_q
                    .iter()
                    .find(|(e, _)| commands.entity(*e).id() == commands.entity(*c1).id());
                let b = spawn_q
                    .iter()
                    .find(|(e, _)| commands.entity(*e).id() == commands.entity(*c2).id());

                let sensor = if a.is_some() {
                    a
                } else if b.is_some() {
                    b
                } else {
                    None
                };

                if let Some(trigger_type) = sensor.map(|(_, t)| t) {
                    match trigger_type {
                        TriggerType::StartMission => {
                            if mission_state.mission_active {
                                return;
                            }
                            commands.entity(sensor.unwrap().0).despawn();
                            let mission_idx = mission_state.next_target(&mut commands, &constants);

                            dialogue_state.load_dialogue(
                                &format!("p{}", mission_idx.unwrap()),
                                &mut dialogues,
                                &dialogue,
                            );
                        }
                        TriggerType::StopMission => {
                            if !mission_state.mission_active {
                                return;
                            }

                            commands.entity(sensor.unwrap().0).despawn();
                            mission_state.next_target(&mut commands, &constants);

                            dialogue_state.load_dialogue("p-end", &mut dialogues, &dialogue);
                        }
                    }
                }
            }
            CollisionEvent::Stopped(_, sensor, flags) => {
                if flags != &CollisionEventFlags::SENSOR {
                    return;
                }

                if let Some(trigger_type) = spawn_q
                    .iter()
                    .find(|(e, _)| commands.entity(*e).id() == commands.entity(*sensor).id())
                    .map(|(_e, t)| t)
                {
                    match trigger_type {
                        TriggerType::StartMission => {}
                        TriggerType::StopMission => {}
                    }
                }
            }
        }
    }
}
