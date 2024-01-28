//! Generic assets that may have colliders and anchors

use bevy::sprite::Anchor;
use bevy::{prelude::*, transform::commands};
use bevy_rapier2d::prelude::*;

use crate::{
    parallax::{ParallaxImages, ParallaxSprite},
    // constants::TILE_SIZE,
};

// /// An optional collider
// pub collider: Option<Collider>,
// /// The vector from the origin to the anchor of the texture
// pub anchor: Option<Vec2>,

#[derive(Component)]
pub struct Piece(String);

impl Piece {
    pub fn new(texture: impl Into<String>) -> Self {
        Self(texture.into())
    }
}

#[derive(Bundle)]
pub struct PieceBundle {
    piece: Piece,
    sprite: ParallaxSprite,
}

pub struct PieceMeta {
    pub collider: Option<Collider>,
    pub anchor: Option<Anchor>,
    pub transform: Transform,
}

impl PieceMeta {
    pub fn new(transform: Transform, collider: Option<Collider>, anchor: Option<Anchor>) -> Self {
        Self {
            collider,
            transform,
            anchor,
        }
    }
}

pub fn spawn_as_child(cb: &mut ChildBuilder, (piece, meta): (Piece, PieceMeta)) {
    let mut f = cb.spawn(PieceBundle::new(piece, meta.transform, meta.anchor));
    if let Some(collider) = meta.collider {
        f.insert(collider);
    }
}

pub fn spawn(commands: &mut Commands, (piece, meta): (Piece, PieceMeta)) {
    let mut f = commands.spawn(PieceBundle::new(piece, meta.transform, meta.anchor));
    if let Some(collider) = meta.collider {
        f.insert(collider);
    }
}

impl PieceBundle {
    pub fn new(piece: Piece, transform: Transform, anchor: Option<Anchor>) -> Self {
        let sprite = match anchor {
            Some(anchor) => Sprite {
                anchor,
                ..Default::default()
            },
            None => Default::default(),
        };
        Self {
            sprite: ParallaxSprite {
                transform: transform.into(),
                visibility: Default::default(),
                images: ParallaxImages::new(&piece.0, sprite),
            },
            piece,
        }
    }
}
