use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::geometry::CollisionEventFlags};

use crate::constants::Constants;

#[derive(Component)]
pub struct Trigger {}

#[derive(Debug, Clone, Copy, Component)]
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
    pub fn new(_trigger_type: TriggerType, constants: &Res<Constants>) -> Self {
        Self {
            trigger: Trigger {},
            trigger_type: TriggerType::StartMission,
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
    spawn_q: Query<(Entity, &TriggerType), With<Trigger>>,
) {
    // let mut car_state = car_q.get_single_mut().unwrap();

    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(sensor, _car, flags) => {
                if flags != &CollisionEventFlags::SENSOR {
                    return;
                }

                if let Some(trigger_type) = spawn_q
                    .iter()
                    .find(|(e, _)| commands.entity(*e).id() == commands.entity(*sensor).id())
                    .map(|(_e, t)| t)
                {
                    dbg!(trigger_type);
                    match trigger_type {
                        TriggerType::StartMission => {}
                        TriggerType::StopMission => {}
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
