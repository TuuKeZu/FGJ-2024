//! Generic assets that may have colliders and anchors

use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_rapier2d::prelude::*;

use crate::{
    parallax::{ParallaxImages, ParallaxSprite},
    // constants::TILE_SIZE,
};

#[derive(Component)]
pub struct Piece {
    /// The name of the instance in TOML
    pub texture: String,
    /// An optional collider
    pub collider: Option<Collider>,
    /// The vector from the origin to the anchor of the texture
    pub anchor: Option<Vec2>,
}

impl Piece {
    pub fn new(
        texture: impl Into<String>,
        collider: Option<Collider>,
        anchor: Option<Vec2>,
    ) -> Self {
        Self {
            texture: texture.into(),
            collider,
            anchor,
        }
    }
}

#[derive(Bundle)]
pub struct PieceBundle {
    piece: Piece,
    sprite: ParallaxSprite,
}

impl PieceBundle {
    pub fn new(piece: Piece, transform: Transform) -> Self {
        let sprite = match piece.anchor {
            Some(anchor) => Sprite {
                anchor: Anchor::Custom(anchor),
                ..Default::default()
            },
            None => Default::default(),
        };
        Self {
            sprite: ParallaxSprite {
                transform: transform.into(),
                visibility: Default::default(),
                images: ParallaxImages::new(&piece.texture, sprite),
            },
            piece,
        }
    }
}
