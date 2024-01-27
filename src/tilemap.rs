use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum TileType {
    Grass,
    Building,
}

#[derive(Component)]
pub struct Tile {
    tp: TileType,
}

#[derive(Bundle)]
pub struct TileBundle {
    tile: Tile,
    sprite: SpriteBundle,
}

const TILE_SIZE: f32 = 2.;

impl TileBundle {
    pub fn new(tp: TileType, pos: Vec3) -> TileBundle {
        let color = match tp {
            TileType::Grass => Color::rgb(0., 1., 0.),
            TileType::Building => Color::rgb(0.5, 0.5, 0.5),
        };
        TileBundle {
            tile: Tile { tp },
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    color,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(pos.x * TILE_SIZE, pos.y * TILE_SIZE, pos.z),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

pub fn setup_tilemap(mut commands: Commands) {
    for x in -10..=10 {
        for y in -10..=10 {
            let tp = if (x + y) % 2 == 0 {
                TileType::Grass
            } else {
                TileType::Building
            };
            commands.spawn(TileBundle::new(tp, Vec3::new(x as f32, y as f32, 0.)));
        }
    }
}
