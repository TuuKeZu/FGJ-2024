use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;

use crate::{
    constants::TILE_SIZE,
    piece::*,
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
            0 => Grass,
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
                    (tile.pos * TILE_SIZE).extend(-0.1)
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
                    spawn_as_child(
                        cb,
                        (
                            Piece::new("sidewalk"),
                            PieceMeta::new(tr(Transform::from_xyz(dx, dy, 0.5)), None, None),
                        ),
                    );

                    let ninety = std::f32::consts::PI / 2.;
                    // let quat = Quat::from_rotation_z(dx.signum() * ninety);
                    let mut t = Transform::from_xyz(dx, dy, 0.6);
                    t.rotate_z(-dx.signum() * ninety);
                    spawn_as_child(cb, (Piece::new("curb"), PieceMeta::new(tr(t), None, None)));
                }
            }
            let parity = ((tile.pos.x + tile.pos.y) as i32).rem_euclid(2);
            if parity == 0 {
                spawn_as_child(
                    cb,
                    make_lamppost(tr(Transform::from_xyz(TILE_SIZE / 3., 0., 0.))),
                );
            }
            if parity == 1 {
                let mut t = Transform::from_xyz(-TILE_SIZE / 3., 0., 0.);
                t.rotate_z(std::f32::consts::PI);
                spawn_as_child(cb, make_lamppost(tr(t)));
            }
            Ok(())
        }
        [false, true, true, false] => {
            spawn_as_child(
                cb,
                (
                    Piece::new("pavement_turn"),
                    PieceMeta::new(tr(Transform::default()), None, None),
                ),
            );
            Ok(())
        }
        _ => Err(()),
    }
}

fn spawn_pieces(cb: &mut ChildBuilder, tile: &Tile) {
    spawn_as_child(
        cb,
        (
            Piece::new("grass"),
            PieceMeta::new(Transform::from_xyz(0., 0., -1.), None, None),
        ),
    );
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
        TileType::Building => {
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
                spawn_as_child(
                    cb,
                    (
                        Piece::new("building2"),
                        PieceMeta::new(
                            tr(Transform::default()),
                            Some(Collider::cuboid(TILE_SIZE / 2., TILE_SIZE / 2.0)),
                            Some(bevy::sprite::Anchor::Custom(Vec2::new(0., 6.))),
                        ),
                    ),
                );
                spawn_as_child(
                    cb,
                    (
                        Piece::new("ceiling2"),
                        PieceMeta::new(tr(Transform::default()), None, None),
                    ),
                );
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
