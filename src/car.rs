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
        let sprite = Sprite {
            custom_size: Some(Vec2::new(
                2. * constants.car.size.x,
                2. * constants.car.size.y,
            )),
            // rect: Some(Rect::new(25., 12., 39., 50.)), // TODO not used???
            ..Default::default()
        };

        // TODO helper for for this
        let indices_heights_sprites = (0..4)
            .map(|index| (3 - index, index as f32 + 0.5, sprite.clone()))
            .collect::<Vec<_>>();

        let car = CarBundle {
            car: Car {},
            state: CarState {
                head_pointed_at: constants.car.head_pointed_start,
                steering_angle: constants.car.steering_angle,
                passenger: false,
            },
            sprite: ParallaxSprite {
                images: ParallaxImages::new(
                    "car/car.png",
                    indices_heights_sprites,
                    Vec2::new(64., 64.),
                    1,
                    4,
                ),
                visibility: VisibilityBundle::default(),
                transform: TransformBundle::default(),
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
            .insert(ColliderMassProperties::Mass(0.1))
            .insert(ReadMassProperties::default())
            .insert(Velocity::default())
            .insert(ExternalForce::default())
            .insert(Damping {
                linear_damping: 1.,
                angular_damping: 1.,
            })
            .id();

        commands.entity(car).with_children(|parent| {
            // Spawn back tires
            let mut joint = FixedJointBuilder::new()
                .local_anchor1(Vec2::new(-42., -80.))
                .build();
            joint.set_contacts_enabled(false);

            parent
                .spawn(TireBundle::default())
                .insert(RigidBody::Dynamic)
                .insert(GravityScale(0.))
                .insert(ImpulseJoint::new(car, joint))
                .insert(Collider::round_cuboid(1., 10., 0.1))
                // .insert(LockedAxes::TRANSLATION_LOCKED_Y)
                .insert(ColliderDebugColor(Color::rgb(1., 0., 1.)));

            let mut joint = FixedJointBuilder::new()
                .local_anchor1(Vec2::new(42., -80.))
                .build();
            joint.set_contacts_enabled(false);

            parent
                .spawn(TireBundle::default())
                .insert(RigidBody::Dynamic)
                .insert(GravityScale(0.))
                .insert(ImpulseJoint::new(car, joint))
                .insert(Collider::round_cuboid(1., 10., 0.1))
                // .insert(LockedAxes::TRANSLATION_LOCKED_Y)
                .insert(ColliderDebugColor(Color::rgb(1., 0., 1.)));

            // Spawn front tires
            let joint_builder = RevoluteJointBuilder::new()
                .limits([-0.5, 0.5])
                .motor_model(MotorModel::ForceBased)
                .motor(0., 0., 1., 0.3);
            let mut joint = joint_builder.local_anchor1(Vec2::new(-62., 80.)).build();
            joint.set_contacts_enabled(false);

            parent
                .spawn(TireBundle::default())
                .insert(Steering)
                .insert(RigidBody::Dynamic)
                .insert(GravityScale(0.))
                .insert(ImpulseJoint::new(car, joint))
                .insert(Collider::round_cuboid(1., 10., 0.1))
                .insert(ColliderDebugColor(Color::rgb(1., 0., 1.)))
                // .insert(LockedAxes::TRANSLATION_LOCKED_Y)
                .insert(Velocity::zero());

            let mut joint = joint_builder.local_anchor1(Vec2::new(62., 80.)).build();
            joint.set_contacts_enabled(false);

            parent
                .spawn(TireBundle::default())
                .insert(Steering)
                .insert(RigidBody::Dynamic)
                .insert(GravityScale(0.))
                .insert(ImpulseJoint::new(car, joint))
                .insert(Collider::round_cuboid(1., 10., 0.1))
                .insert(ColliderDebugColor(Color::rgb(1., 0., 1.)))
                // .insert(LockedAxes::TRANSLATION_LOCKED_Y)
                .insert(Velocity::zero());
        });
    }
}

#[derive(Bundle, Default)]
pub struct TireBundle {
    tire: Tire,
    force: ExternalForce,
    impulse: ExternalImpulse,
    transform: TransformBundle,
    velocity: Velocity,
    rmp: ReadMassProperties,
}

#[derive(Component, Default)]
pub struct Tire {
    force: Vec2,
}

#[derive(Component)]
pub struct Steering;

pub fn tire_friction(
    _constants: Res<Constants>,
    time: Res<Time>,
    mut tires: Query<
        (
            &mut Tire,
            &GlobalTransform,
            &mut Velocity,
            &Parent,
            &ReadMassProperties,
        ),
        Without<Car>,
    >,
    cars: Query<&ReadMassProperties, With<Car>>,
) {
    let mut printed = false;
    for (mut tire, transform, mut velocity, parent, mass) in tires.iter_mut() {
        let (_scale, rotation, _translation) = transform.to_scale_rotation_translation();
        // let global_tire_axis = rotation.mul_vec3(Vec3::new(0., 1., 0.)).xy();
        // let normal = velocity.linvel.project_onto(global_tire_axis);
        // let ortho = velocity.linvel - normal;

        // if !printed {
        //     printed = true;
        //     // println!("{global_tire_axis:?}");
        //     // println!(
        //     //     "New vel: {:?} + {:?} ({}) = {:?}",
        //     //     normal,
        //     //     ortho,
        //     //     ortho.dot(global_tire_axis),
        //     //     velocity.linvel
        //     // );
        //     assert!(ortho.dot(global_tire_axis) < 1e-3);
        // }
        // // let fix_vel = velocity.linvel - velocity.linvel.project_onto(global_tire_axis);
        // // impulse.impulse = -0.001 * ortho;
        // // velocity.linvel -= 2.*ortho;
        // // velocity.linvel -= ortho;
        // impulse.impulse = -0.3 * time.delta_seconds() * ortho / 4.;
        if let Ok(car_mass) = cars.get(parent.get()) {
            let local_velocity = rotation
                .inverse()
                .mul_vec3(Vec3::new(velocity.linvel.x, velocity.linvel.y, 0.))
                .xy();
            tire.force.x +=
                -1. * time.delta_seconds() * local_velocity.x / (0.25 * car_mass.mass + mass.mass);
        }
    }
}

pub fn car_control(
    keyboard_input: Res<Input<KeyCode>>,
    _constants: Res<Constants>,
    mut tires: Query<
        (
            &mut Tire,
            &mut ExternalForce,
            &mut Transform,
            &mut ImpulseJoint,
            &GlobalTransform,
        ),
        (With<Steering>),
    >,
) {
    let mut acceleration_force = 0.;
    let mut steering = 0.;
    if keyboard_input.pressed(KeyCode::Up) {
        acceleration_force += 100.;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        acceleration_force += -100.;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        steering += 0.4;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        steering += -0.4;
    }

    for (mut tire, mut force, mut transform, mut joint, global_transform) in tires.iter_mut() {
        let (_scale, rotation, _translation) = global_transform.to_scale_rotation_translation();

        // force.force = rotation
        //     .mul_vec3(Vec3::new(0., acceleration_force, 0.))
        //     .xy();
        tire.force.y += acceleration_force;

        transform.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), steering);
        // joint.data.raw.motors[JointAxis::AngX].target_pos = steering;
        joint.data.set_motor(JointAxis::AngX, steering, 0., 10., 1.);
    }
}

pub fn update_car_velocity(
    mut car: Query<(&mut Velocity, &Children), Without<Tire>>,
    tires: Query<&Velocity, With<Tire>>,
) {
    for (mut car_vel, children) in car.iter_mut() {
        let mut new_vel = Vec2::new(0., 0.);
        let mut tire_count = 0;
        for &child in children {
            if let Ok(tire) = tires.get(child) {
                new_vel += tire.linvel;
                tire_count += 1;
            }
        }
        new_vel *= 1. / tire_count as f32;
        // car_vel.linvel = new_vel;
        // println!("Setting car velocity to {new_vel:?}");
    }
}

pub fn reset_tires(mut tires: Query<&mut Tire>) {
    for mut tire in tires.iter_mut() {
        // tire.force = Vec2::new(0., 0.);
    }
}

pub fn update_tire_forces(
    mut cars: Query<(&mut ExternalForce), Without<Tire>>,
    mut tires: Query<(&mut Tire, &GlobalTransform, &Parent, &mut ExternalForce)>,
) {
    for mut car in cars.iter_mut() {
        car.force = Vec2::new(0., 0.);
    }

    for (mut tire, transform, parent, mut force) in tires.iter_mut() {
        let (_scale, rotation, translation) = transform.to_scale_rotation_translation();

        // println!("Applying force: {:?}", tire.force);
        let f = rotation
            .mul_vec3(Vec3::new(tire.force.x, tire.force.y, 0.))
            .xy();

        // if let Ok(mut car) = cars.get_mut(parent.get()) {
        //     *car += ExternalForce::at_point(f, translation.xy(), Vec2::new(0., 0.));
        // }

        force.force = f;
        tire.force = Vec2::new(0., 0.);
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
