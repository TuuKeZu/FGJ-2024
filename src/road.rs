//! Road elements such as pavement
//!
//! Can render any static elements with the same image dimensions as pavement.

use bevy::prelude::*;

use crate::{
    parallax::{ParallaxImages, ParallaxSprite},
    // constants::TILE_SIZE,
};

#[derive(Debug, Clone, Copy)]
pub enum RoadTexture {
    Pavement,
    PavementTurn,
    PavementT,
    PavementCross,
}

impl RoadTexture {
    fn get_parallax_images(&self) -> ParallaxImages {
        use RoadTexture::*;
        let name = match self {
            Pavement => "pavement",
            PavementTurn => "pavement_turn",
            PavementT => "pavement_t",
            PavementCross => "pavement_cross",
        };
        let sprite = Sprite {
            custom_size: Some(Vec2::new(
                64., 64., // TODO road scale
            )),
            ..Default::default()
        };

        let indices_heights_sprites = vec![(0, 0.0, sprite)];

        // let imgs = if layers == 1 {
        //     vec![(format!("{name}/{name}.png"), 0.)]
        // } else {
        //     (0..layers)
        //         .map(|layer| (format!("{name}/{name}-{layer}.png"), (layer as f32) / 2.))
        //         .collect()
        // };
        ParallaxImages::new(
            format!("{name}/{name}.png"),
            indices_heights_sprites,
            Vec2::new(64.0, 64.0),
            1,
            1,
        )
    }
}

#[derive(Component)]
pub struct Road {
    pub texture: RoadTexture,
}

#[derive(Bundle)]
pub struct RoadBundle {
    piece: Road,
    sprite: ParallaxSprite,
}

impl RoadBundle {
    pub fn new(piece: Road, transform: Transform) -> Self {
        let images = piece.texture.get_parallax_images();
        Self {
            sprite: ParallaxSprite {
                transform: transform.into(),
                visibility: Default::default(),
                // sprite: Sprite {
                //     custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                //     rect: Some(Rect::new(32., 32., 224., 224.)),
                //     ..Default::default()
                // },
                images,
            },
            piece,
        }
    }
}
