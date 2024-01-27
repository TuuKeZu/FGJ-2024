use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum TileType {
    Grass,
    Pavement,
    Building,
}

#[derive(Component)]
pub struct Tile {
    //tp: TileType,
}

#[derive(Bundle)]
pub struct TileBundle {
    tile: Tile,
    sprite: SpriteBundle,
}

const TILE_SIZE: f32 = 64.;

impl TileBundle {
    pub fn new(tp: TileType, pos: Vec3) -> TileBundle {
        let color = match tp {
            TileType::Grass => Color::rgb(0., 1., 0.),
            TileType::Pavement => Color::rgb(0., 0., 0.),
            TileType::Building => Color::rgb(0.8, 0.5, 0.5),
        };
        TileBundle {
            tile: Tile {},
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
    let map = load_map();
    for (i, row) in map.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            let x = j as f32;
            let y = -(i as f32);
            let tp = match val {
                0 => continue,
                1 => TileType::Grass,
                2 => TileType::Pavement,
                3 => TileType::Building,
                _ => panic!("unsupported tile type"),
            };
            commands.spawn(TileBundle::new(tp, Vec3::new(x, y, -1.)));
        }
    }
}

const MAP_CSV_CONTENT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/tilemap/simplified/Level_0/Map.csv"
));

fn load_map() -> Vec<Vec<i32>> {
    MAP_CSV_CONTENT
        .lines()
        .map(|l| {
            l.split(',')
                .filter_map(|val| val.parse().ok())
                .collect::<Vec<i32>>()
        })
        .collect()
}
