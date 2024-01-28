//! Road elements such as pavement
//!
//! Can render any static elements with the same image dimensions as pavement.

use bevy::prelude::*;

use crate::{
    parallax::{ParallaxImages, ParallaxSprite},
    // constants::TILE_SIZE,
};

#[derive(Component)]
pub struct Road {
    pub texture: String,
}

impl Road {
    pub fn new(texture: impl Into<String>) -> Self {
        Self {
            texture: texture.into(),
        }
    }
}

#[derive(Bundle)]
pub struct RoadBundle {
    piece: Road,
    sprite: ParallaxSprite,
}

impl RoadBundle {
    pub fn new(piece: Road, transform: Transform) -> Self {
        Self {
            sprite: ParallaxSprite {
                transform: transform.into(),
                visibility: Default::default(),
                images: ParallaxImages::new_default(&piece.texture),
            },
            piece,
        }
    }
}
