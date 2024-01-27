use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    constants::Constants,
    parallax::{ParallaxImages, ParallaxSprite},
};

#[derive(Component)]
pub struct Car {}

#[derive(Component)]
pub struct CarState {
    pub head_pointed_at: Vec2,
    pub steering_angle: f32,

    pub passenger: bool,
}

impl CarState {
    pub fn turn_direction(&self) -> Vec2 {
        Vec2 {
            x: -f32::sin(self.steering_angle),
            y: f32::cos(self.steering_angle),
        }
        .rotate(self.head_pointed_at)
    }
}

#[derive(Bundle)]
pub struct CarBundle {
    car: Car,
    state: CarState,
    sprite: ParallaxSprite,
    active_events: ActiveEvents,
    // rb: RigidBody,
    // rs: Restitution,
    // collider: Collider,
    // damping: Damping,
    // gravity: GravityScale,
    // ef: ExternalForce,
}

impl CarBundle {
    pub fn spawn(commands: &mut Commands, constants: Res<Constants>) {
        let car = CarBundle {
            car: Car {},
            state: CarState {
                head_pointed_at: constants.car.head_pointed_start,
                steering_angle: constants.car.steering_angle,
                passenger: false,
            },
            sprite: ParallaxSprite {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(
                        2. * constants.car.size.x,
                        2. * constants.car.size.y,
                    )),
                    rect: Some(Rect::new(25., 12., 39., 50.)),
                    ..Default::default()
                },
                images: ParallaxImages::new(vec![
                    ("car/car-0.png", 0.0),
                    ("car/car-1.png", 0.5),
                    ("car/car-2.png", 1.0),
                    ("car/car-3.png", 1.5),
                ]),
                ..Default::default()
            },
            active_events: ActiveEvents::COLLISION_EVENTS,
            // rb: RigidBody::Dynamic,
            // rs: Restitution::coefficient(0.7),
            // damping: Damping {
            //     linear_damping: constants.car.linear_damping,
            //     angular_damping: constants.car.angular_damping,
            // },
            // collider: Collider::cuboid(constants.car.size.x, constants.car.size.y),
            // gravity: GravityScale(0.),
            // ef: ExternalForce {
            //     force: Vec2::ZERO,
            //     torque: 0.,
            // },
        };

        let car = commands
            .spawn(car)
            .insert(RigidBody::Dynamic)
            .insert(GravityScale(0.))
            .insert(Collider::cuboid(constants.car.size.x, constants.car.size.y))
            .id();

        // Spawn back tires
        let mut joint = FixedJointBuilder::new()
            .local_anchor1(Vec2::new(-42., -80.))
            .build();
        joint.set_contacts_enabled(false);

        commands
            .spawn(TireBundle::default())
            .insert(RigidBody::Dynamic)
            .insert(GravityScale(0.))
            .insert(ImpulseJoint::new(car, joint))
            .insert(Collider::round_cuboid(1., 10., 0.1))
            .insert(ColliderDebugColor(Color::rgb(1., 0., 1.)));

        let mut joint = FixedJointBuilder::new()
            .local_anchor1(Vec2::new(42., -80.))
            .build();
        joint.set_contacts_enabled(false);

        commands
            .spawn(TireBundle::default())
            .insert(RigidBody::Dynamic)
            .insert(GravityScale(0.))
            .insert(ImpulseJoint::new(car, joint))
            .insert(Collider::round_cuboid(1., 10., 0.1))
            .insert(ColliderDebugColor(Color::rgb(1., 0., 1.)));

        // Spawn front tires
        let mut joint = FixedJointBuilder::new()
            .local_anchor1(Vec2::new(-42., 80.))
            .build();
        joint.set_contacts_enabled(false);

        commands
            .spawn(TireBundle::default())
            .insert(Steering)
            .insert(RigidBody::Dynamic)
            .insert(GravityScale(0.))
            .insert(ImpulseJoint::new(car, joint))
            .insert(Collider::round_cuboid(1., 10., 0.1))
            .insert(ColliderDebugColor(Color::rgb(1., 0., 1.)));

        let mut joint = FixedJointBuilder::new()
            .local_anchor1(Vec2::new(42., 80.))
            .build();
        joint.set_contacts_enabled(false);

        commands
            .spawn(TireBundle::default())
            .insert(Steering)
            .insert(RigidBody::Dynamic)
            .insert(GravityScale(0.))
            .insert(ImpulseJoint::new(car, joint))
            .insert(Collider::round_cuboid(1., 10., 0.1))
            .insert(ColliderDebugColor(Color::rgb(1., 0., 1.)));
    }
}

#[derive(Bundle, Default)]
pub struct TireBundle {
    tire: Tire,
    force: ExternalForce,
}

#[derive(Component, Default)]
pub struct Tire;

#[derive(Component)]
pub struct Steering;

pub fn tire_friction(_constants: Res<Constants>, _tires: Query<&mut ExternalForce, With<Tire>>) {}

pub fn car_control(
    keyboard_input: Res<Input<KeyCode>>,
    _constants: Res<Constants>,
    mut tires: Query<(&mut ExternalForce, &GlobalTransform), (With<Tire>, With<Steering>)>,
    // mut query: Query<(&mut ExternalForce, &mut Transform, &mut CarState), With<Car>>,
) {
    let mut acceleration_force = 0.;
    if keyboard_input.pressed(KeyCode::Up) {
        acceleration_force += 100.;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        acceleration_force += -100.;
    }

    for (_force, _transform) in tires.iter_mut() {

        // tire.force = transform.
    }
}

// pub fn move_car(
//     keyboard_input: Res<Input<KeyCode>>,
//     constants: Res<Constants>,
//     mut query: Query<(&mut ExternalForce, &mut Transform, &mut CarState), With<Car>>,
// ) {
//     let (mut car_ef, mut transform, mut carstate) = query.single_mut();
//     let mut moving_y = false;
//     let mut moving_angular = false;

//     let head_pointed_angle = carstate
//         .head_pointed_at
//         .angle_between(constants.car.head_pointed_start);
//     transform.rotation = Quat::from_axis_angle(
//         Vec3 {
//             x: 0.,
//             y: 0.,
//             z: -f32::signum(head_pointed_angle),
//         },
//         head_pointed_angle.abs(),
//     );
//     if keyboard_input.pressed(KeyCode::Up) {
//         car_ef.force = carstate.turn_direction() * constants.car.engine_force;
//         carstate.head_pointed_at =
//             Vec2::from_angle(carstate.steering_angle).rotate(carstate.head_pointed_at);
//         moving_y = true;
//     }

//     if keyboard_input.pressed(KeyCode::Down) {
//         let mut turn_vec = carstate.turn_direction();
//         turn_vec.x = -turn_vec.x;
//         car_ef.force = turn_vec * -constants.car.engine_force;
//         carstate.head_pointed_at =
//             Vec2::from_angle(carstate.steering_angle).rotate(carstate.head_pointed_at);
//         moving_y = true;
//     }

//     if keyboard_input.pressed(KeyCode::Left) {
//         // car_ef.force.x = -constants.physics.turn_force;
//         carstate.steering_angle += constants.car.steering_speed;
//         carstate.steering_angle = carstate
//             .steering_angle
//             .clamp(-constants.car.max_steer, constants.car.max_steer);
//         moving_angular = true;
//     }

//     if keyboard_input.pressed(KeyCode::Right) {
//         carstate.steering_angle -= constants.car.steering_speed;
//         carstate.steering_angle = carstate
//             .steering_angle
//             .clamp(-constants.car.max_steer, constants.car.max_steer);
//         // car_ef.force.x = constants.physics.turn_force;
//         moving_angular = true;
//     }

//     if !moving_y {
//         car_ef.force = Vec2::ZERO;
//     }

//     if !moving_angular {
//         if (carstate.steering_angle - 0.0).abs() > constants.car.close_to_zero {
//             carstate.steering_angle +=
//                 -f32::signum(carstate.steering_angle) * constants.car.steering_back;
//         } else {
//             carstate.steering_angle = 0.0;
//         }
//     }
// }
