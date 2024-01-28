//! Sidewalk elements

use bevy::prelude::*;

use crate::{
    parallax::{ParallaxImages, ParallaxSprite},
    // constants::TILE_SIZE,
};

#[derive(Component)]
pub struct Sidewalk {
    pub texture: String,
}

impl Sidewalk {
    pub fn new(texture: impl Into<String>) -> Self {
        Self {
            texture: texture.into(),
        }
    }
}

#[derive(Bundle)]
pub struct SidewalkBundle {
    piece: Sidewalk,
    sprite: ParallaxSprite,
}

impl SidewalkBundle {
    pub fn new(piece: Sidewalk, transform: Transform) -> Self {
        Self {
            sprite: ParallaxSprite {
                transform: transform.into(),
                visibility: Default::default(),
                images: ParallaxImages::new(&piece.texture, Default::default()),
            },
            piece,
        }
    }
}

// TODO sidewalk border
