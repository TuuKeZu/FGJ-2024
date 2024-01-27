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
                10. * constants.building.size.x,
                10. * constants.building.size.y,
            )),
            ..Default::default()
        };
        match *self {
            Building1 => {
                // TODO utility for this
                let indices_heights_sprites = (0..32)
                    .map(|index| ((256 - 7 - index) % 8, index as f32 + 0.5, sprite.clone())) // A building starts at height 0.5 with sprite 0
                    .collect::<Vec<_>>();
                ParallaxImages::new(
                    "building1/building1_atlas.png",
                    indices_heights_sprites,
                    Vec2::new(64.0, 8.0),
                    1,
                    8,
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
            .insert(GravityScale(0.))
            .insert(Collider::cuboid(
                constants.building.size.x,
                constants.building.size.y,
            ));
    }
}
