use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    constants::Constants,
    parallax::{ParallaxImages, ParallaxSprite},
};

// TODO correct scaling
const BUILDING_SCALE: f32 = 6.0;

#[derive(Component)]
pub enum Building {
    Building1,
}

impl Building {
    fn get_parallax_images(&self) -> ParallaxImages {
        use Building::*;
        let sprite = Sprite {
            custom_size: Some(Vec2::new(192. * BUILDING_SCALE, 16. * BUILDING_SCALE)),
            ..Default::default()
        };
        match *self {
            Building1 => ParallaxImages::new("building2", sprite.clone()),
        }
    }
}

#[derive(Bundle)]
pub struct BuildingBundle {
    building: Building,
    sprite: ParallaxSprite,
}

impl BuildingBundle {
    pub fn spawn(commands: &mut Commands) {
        let building = BuildingBundle {
            building: Building::Building1,
            sprite: ParallaxSprite {
                images: Building::Building1.get_parallax_images(),
                // ..Default::default()
                visibility: VisibilityBundle::default(),
                transform: TransformBundle::default(),
            },
        };

        commands
            .spawn(building)
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(
                192.0 / 2.0 * BUILDING_SCALE,
                4.0 / 2.0 * BUILDING_SCALE,
            ));
    }
}
