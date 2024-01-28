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
            Building1 => {
                // TODO utility for this

                let indices = [
                    // Increasing bottom to top
                    0, 1, 1, // Ground level
                    3, 3, 2, 2, 2, 2, 3, 3, 4, 4, 5, 5, // Windows
                    3, 3, 2, 2, 2, 2, 3, 3, 4, 4, 5, 5, // Windows
                    3, 3, 2, 2, 2, 2, 3, 3, 4, 4, 5, 5, // Windows
                    3, 3, 2, 2, 2, 2, 3, 3, 4, 4, 5, 5, // Windows
                    6, 6, 7, 7, 8, 9, 9, 9, 9, 10, 10, 9, 9, 11, 11, 11,
                ];
                let indices_heights_sprites = indices
                    .into_iter()
                    .enumerate()
                    .map(|(n, index)| {
                        (
                            11 - index,
                            n as f32 * 0.1 * BUILDING_SCALE + 0.5,
                            sprite.clone(),
                        )
                    })
                    .collect::<Vec<_>>();
                ParallaxImages::new(
                    "building2/building2.png",
                    indices_heights_sprites,
                    Vec2::new(192.0, 16.0),
                    1,
                    12,
                )
            }
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
