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
    pub passenger: bool,
}

#[derive(Bundle)]
pub struct CarBundle {
    car: Car,
    state: CarState,
    sprite: ParallaxSprite,
    active_events: ActiveEvents,
}

impl CarBundle {
    pub fn spawn(commands: &mut Commands, constants: Res<Constants>) {
        let sprite = Sprite {
            custom_size: Some(Vec2::new(
                2. * constants.car.size.x,
                2. * constants.car.size.y,
            )),
            ..Default::default()
        };

        // TODO helper for for this
        let indices_heights_sprites = (0..4)
            .map(|index| (3 - index, index as f32 + 0.5, sprite.clone()))
            .collect::<Vec<_>>();

        let car = CarBundle {
            car: Car {},
            state: CarState { passenger: false },
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
        };

        let car = commands
            .spawn(car)
            .insert(RigidBody::Dynamic)
            .insert(Collider::cuboid(constants.car.size.x, constants.car.size.y))
            .insert(ColliderMassProperties::Mass(0.1))
            .insert(ReadMassProperties::default())
            .insert(Velocity::default())
            .insert(ExternalForce::default())
            .insert(Damping {
                linear_damping: 0.3,
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
                .insert(ImpulseJoint::new(car, joint))
                .insert(Collider::round_cuboid(1., 10., 0.1))
                .insert(ColliderDebugColor(Color::rgb(1., 0., 1.)));

            let mut joint = FixedJointBuilder::new()
                .local_anchor1(Vec2::new(42., -80.))
                .build();
            joint.set_contacts_enabled(false);

            parent
                .spawn(TireBundle::default())
                .insert(RigidBody::Dynamic)
                .insert(ImpulseJoint::new(car, joint))
                .insert(Collider::round_cuboid(1., 10., 0.1))
                .insert(ColliderDebugColor(Color::rgb(1., 0., 1.)));

            // Spawn front tires
            let joint_builder = RevoluteJointBuilder::new()
                .limits([-0.5, 0.5])
                .motor_model(MotorModel::ForceBased)
                .motor(0., 0., 1., 0.3);
            let mut joint = joint_builder.local_anchor1(Vec2::new(-42., 80.)).build();
            joint.set_contacts_enabled(false);

            parent
                .spawn(TireBundle::default())
                .insert(Steering)
                .insert(RigidBody::Dynamic)
                .insert(ImpulseJoint::new(car, joint))
                .insert(Collider::round_cuboid(1., 10., 0.1))
                .insert(ColliderDebugColor(Color::rgb(1., 0., 1.)));

            let mut joint = joint_builder.local_anchor1(Vec2::new(42., 80.)).build();
            joint.set_contacts_enabled(false);

            parent
                .spawn(TireBundle::default())
                .insert(Steering)
                .insert(RigidBody::Dynamic)
                .insert(ImpulseJoint::new(car, joint))
                .insert(Collider::round_cuboid(1., 10., 0.1))
                .insert(ColliderDebugColor(Color::rgb(1., 0., 1.)));
        });
    }
}

#[derive(Bundle, Default)]
pub struct TireBundle {
    tire: Tire,
    force: ExternalForce,
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
    time: Res<Time>,
    mut tires: Query<
        (
            &mut Tire,
            &GlobalTransform,
            &Velocity,
            &Parent,
            &ReadMassProperties,
        ),
        Without<Car>,
    >,
    cars: Query<&ReadMassProperties, With<Car>>,
) {
    for (mut tire, transform, velocity, parent, mass) in tires.iter_mut() {
        let (_scale, rotation, _translation) = transform.to_scale_rotation_translation();
        if let Ok(car_mass) = cars.get(parent.get()) {
            let local_velocity = rotation
                .inverse()
                .mul_vec3(Vec3::new(velocity.linvel.x, velocity.linvel.y, 0.))
                .xy();
            // TODO: The constant 0.25 depends on the car having 4 tires. If more are needed, this needs to be changed to actually count the number of tires.
            tire.force.x +=
                -1. * time.delta_seconds() * local_velocity.x / (0.25 * car_mass.mass + mass.mass);
        }
    }
}

pub fn car_control(
    keyboard_input: Res<Input<KeyCode>>,
    constants: Res<Constants>,
    cars: Query<(&Velocity, &GlobalTransform, &Children), Without<Tire>>,
    mut tires: Query<(&mut Tire, &mut Transform, &mut ImpulseJoint), (With<Steering>)>,
) {
    for (velocity, car_transform, car_tires) in cars.iter() {
        // Compute car velocity
        let velocity = car_transform
            .to_scale_rotation_translation()
            .1
            .inverse()
            .mul_vec3(Vec3::new(velocity.linvel.x, velocity.linvel.y, 0.))
            .y;
        let mut acceleration_force = 0.;
        let mut steering = 0.;
        if keyboard_input.pressed(KeyCode::Up) {
            if velocity < constants.car.max_speed {
                acceleration_force += constants.car.acceleration;
            }
        }
        if keyboard_input.pressed(KeyCode::Down) {
            if velocity > 0. {
                // Breaking
                acceleration_force -= constants.car.breaking_force;
            } else if velocity > -constants.car.max_backing_speed {
                // Backing up
                // TODO: Less speed for backing up
                acceleration_force -= constants.car.acceleration;
            }
        }
        if keyboard_input.pressed(KeyCode::Left) {
            steering += constants.car.max_steer;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            steering += -constants.car.max_steer;
        }

        for &t in car_tires {
            if let Ok((mut tire, mut transform, mut joint)) = tires.get_mut(t) {
                tire.force.y += acceleration_force;

                transform.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), steering);
                joint.data.set_motor(JointAxis::AngX, steering, 0., 10., 1.);
            }
        }
    }
}

pub fn update_tire_forces(mut tires: Query<(&mut Tire, &GlobalTransform, &mut ExternalForce)>) {
    for (mut tire, transform, mut force) in tires.iter_mut() {
        let (_scale, rotation, _translation) = transform.to_scale_rotation_translation();

        force.force = rotation
            .mul_vec3(Vec3::new(tire.force.x, tire.force.y, 0.))
            .xy();
        tire.force = Vec2::new(0., 0.);
    }
}
