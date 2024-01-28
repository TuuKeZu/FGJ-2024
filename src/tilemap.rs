use bevy::prelude::*;

use crate::{
    constants::TILE_SIZE,
    piece::{Piece, PieceBundle},
    road::{Road, RoadBundle},
};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TileType {
    Empty,
    Grass,
    Road,
    Building,
}

impl From<i32> for TileType {
    fn from(value: i32) -> Self {
        use TileType::*;
        match value {
            0 => Empty,
            1 => Grass,
            2 => Road,
            3 => Building,
            _ => panic!("unsupported tile number {value}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Neighbors {
    north: bool,
    east: bool,
    south: bool,
    west: bool,
}

impl Neighbors {
    pub fn from_array(arr: [bool; 4]) -> Self {
        Neighbors {
            north: arr[0],
            east: arr[1],
            south: arr[2],
            west: arr[3],
        }
    }
    pub fn as_array(&self) -> [bool; 4] {
        [self.north, self.east, self.south, self.west]
    }
    pub fn rotate_ccw(&self, count: i32) -> Self {
        let c = count.rem_euclid(4);
        let mut x = self.as_array();
        x.rotate_left(c.try_into().unwrap());
        Self::from_array(x)
    }
}

#[derive(Component, Debug, Clone)]
pub struct Tile {
    tp: TileType,
    pos: Vec2,
    neighbors: Neighbors,
}

#[derive(Bundle)]
struct TileBundle {
    tile: Tile,
    spatial: SpatialBundle,
}

impl TileBundle {
    fn new(tile: Tile) -> Self {
        TileBundle {
            spatial: SpatialBundle {
                transform: dbg!(Transform::from_translation(
                    (tile.pos * TILE_SIZE).extend(-100.)
                )),
                ..Default::default()
            },
            tile,
        }
    }
}

fn try_spawn_pavement(
    cb: &mut ChildBuilder,
    tile: &Tile,
    tr: impl Fn(Transform) -> Transform,
) -> Result<(), ()> {
    match tile.neighbors.as_array() {
        [true, true, true, true] => {
            cb.spawn(RoadBundle::new(
                Road::new("pavement_cross"),
                tr(Transform::default()),
            ));
            Ok(())
        }
        [false, true, true, true] => {
            cb.spawn(RoadBundle::new(
                Road::new("pavement_t"),
                tr(Transform::default()),
            ));
            Ok(())
        }
        [_, false, true, false] => {
            for dy in [-TILE_SIZE / 3., 0., TILE_SIZE / 3.] {
                cb.spawn(RoadBundle::new(
                    Road::new("pavement"),
                    tr(Transform::from_xyz(0., dy, 0.)),
                ));
                for dx in [-TILE_SIZE / 3., TILE_SIZE / 3.] {
                    cb.spawn(PieceBundle::new(
                        Piece::new("sidewalk", None, None),
                        tr(Transform::from_xyz(dx, dy, 0.5)),
                    ));
                }
            }
            Ok(())
        }
        [false, true, true, false] => {
            cb.spawn(PieceBundle::new(
                Piece::new("pavement_turn", None, None),
                tr(Transform::default()),
            ));
            Ok(())
        }
        _ => Err(()),
    }
}

fn spawn_pieces(cb: &mut ChildBuilder, tile: &Tile) {
    match tile.tp {
        TileType::Road => {
            for rot_count in 0..4 {
                let rotated_tile = Tile {
                    neighbors: tile.neighbors.rotate_ccw(-rot_count),
                    ..tile.clone()
                };
                let angle = rot_count as f32 * std::f32::consts::PI / 2.;
                let quat = Quat::from_rotation_z(angle);
                let tr = |mut transform: Transform| {
                    transform.rotate_around(Vec3::ZERO, quat);
                    transform
                };
                if let Ok(_) = try_spawn_pavement(cb, &rotated_tile, tr) {
                    break;
                }
            }
        }
        _ => {}
    }
}

pub fn setup_tilemap(mut commands: Commands) {
    let raw_map = load_map();
    let mut tiles = Vec::new();
    for (i, row) in raw_map.iter().enumerate() {
        for (j, &tp) in row.iter().enumerate() {
            tiles.push(Tile {
                tp,
                pos: Vec2::new(j as f32, i as f32),
                neighbors: Neighbors {
                    north: i + 1 < raw_map.len() && tp == raw_map[i + 1][j],
                    east: j + 1 < raw_map[i].len() && tp == raw_map[i][j + 1],
                    south: i > 0 && tp == raw_map[i - 1][j],
                    west: j > 0 && tp == raw_map[i][j - 1],
                },
            });
        }
    }
    for tile in &tiles {
        let mut entity = commands.spawn(TileBundle::new(tile.clone()));
        entity.with_children(|cb| {
            spawn_pieces(cb, tile);
        });
    }
}

const MAP_CSV_CONTENT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/tilemap/simplified/Level_0/Map.csv"
));

fn load_map() -> Vec<Vec<TileType>> {
    MAP_CSV_CONTENT
        .lines()
        .rev()
        .map(|l| {
            l.split(',')
                .filter_map(|val| val.parse::<i32>().ok().map(From::from))
                .collect::<Vec<TileType>>()
        })
        .collect()
}
