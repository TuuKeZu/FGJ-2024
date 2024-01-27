//! Road elements such as pavement
//!
//! Can render any static elements with the same image dimensions as pavement.

use bevy::prelude::*;

use crate::{
    parallax::{ParallaxImages, ParallaxSprite},
    tilemap::TILE_SIZE,
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
        let (name, layers) = match self {
            Pavement => ("pavement", 1),
            PavementTurn => ("pavement_turn", 1),
            PavementT => ("pavement_t", 1),
            PavementCross => ("pavement_cross", 1),
        };
        let imgs = if layers == 1 {
            vec![(format!("{name}/{name}.png"), 0.)]
        } else {
            (0..layers)
                .map(|layer| (format!("{name}/{name}-{layer}.png"), (layer as f32) / 2.))
                .collect()
        };
        ParallaxImages::new(imgs)
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
                sprite: Sprite {
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    rect: Some(Rect::new(32., 32., 224., 224.)),
                    ..Default::default()
                },
                images,
                ..Default::default()
            },
            piece,
        }
    }
}
