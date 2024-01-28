use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    constants::Constants,
    parallax::{ParallaxImages, ParallaxSprite},
};

#[derive(Component)]
pub enum Building {
    Building1,
}

impl Building {
    fn get_parallax_images(&self, constants: Constants) -> ParallaxImages {
        use Building::*;
        let sprite = Sprite {
            custom_size: Some(Vec2::new(
                192. * constants.building.scale, // TODO fix scaling
                16. * constants.building.scale,
            )),
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
                    .map(|(n, index)| (11 - index, n as f32 * 0.25 + 0.5, sprite.clone()))
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
    pub fn spawn(commands: &mut Commands, constants: Res<Constants>) {
        let building = BuildingBundle {
            building: Building::Building1,
            sprite: ParallaxSprite {
                images: Building::Building1.get_parallax_images(*constants),
                // ..Default::default()
                visibility: VisibilityBundle::default(),
                transform: TransformBundle::default(),
            },
        };

        commands
            .spawn(building)
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(
                192.0 / 2.0 * constants.building.scale,
                4.0 / 2.0 * constants.building.scale,
            ));
    }
}
