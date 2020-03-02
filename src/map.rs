use log::debug;
use ndarray::{Array, Array2, Axis};
use pathfinding::prelude::dfs;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::{read, read_dir, read_to_string};

// Conversion between map in ROM and map as stored in memory & working coordinates
// LedgeTiles:
// ; (player direction) (tile player standing on) (ledge tile) (input required)
//     db SPRITE_FACING_DOWN, $2C,$37,D_DOWN
//     db SPRITE_FACING_DOWN, $39,$36,D_DOWN
//     db SPRITE_FACING_DOWN, $39,$37,D_DOWN
//     db SPRITE_FACING_LEFT, $2C,$27,D_LEFT
//     db SPRITE_FACING_LEFT, $39,$27,D_LEFT
//     db SPRITE_FACING_RIGHT,$2C,$0D,D_RIGHT
//     db SPRITE_FACING_RIGHT,$2C,$1D,D_RIGHT
//     db SPRITE_FACING_RIGHT,$39,$0D,D_RIGHT
//     db $FF
//
// See this document for more understanding about the terminology used here :
// https://bulbapedia.bulbagarden.net/wiki/User:Tiddlywinks/Map_header_data_structure_in_Generation_I

// TODO -- cleanup this module based on needs
//
// TODO -- figure out ledges
//

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Coordinate {
    map_id: u8,
    x: u8,
    y: u8,
}

struct World {
    data: HashMap<Coordinate, Square>,
}

impl World {
    fn successors(&self, coordinate: &Coordinate) -> Vec<Coordinate> {
        let square = self.data.get(coordinate).unwrap(); // error handling
        let mut successors = vec![];
        for direction in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        {
            let new_x_y: (u8, u8) = match direction {
                Direction::Up => (coordinate.x, coordinate.y - 1),
                Direction::Down => (coordinate.x, coordinate.y + 1),
                Direction::Left => (coordinate.x - 1, coordinate.y),
                Direction::Right => (coordinate.x + 1, coordinate.y),
            };
            let coord_to_check = Coordinate {
                map_id: coordinate.map_id,
                x: new_x_y.0,
                y: new_x_y.1,
            };
            // if it exists
            if let Some(s) = self.data.get(&coord_to_check) {
                // if it is walkable
                match s.property {
                    TileProperty::Walkable => {
                        if square.sprite.is_none() {
                            successors.push(s.coordinate);
                        }
                    }
                    TileProperty::Ledge(d) => (),
                    TileProperty::NonWalkable => (),
                    // a mess obvi
                    TileProperty::Warp(coord) => successors.push(self.data.get(&coord).unwrap().coordinate),
                }
                //figure out warps
            }
        }
        successors
    }
}

#[derive(Debug)]
struct Square {
    // grass: bool,
    // sprite -- mutable properties
    // other_barrier: Option<Barrier>,
    coordinate: Coordinate,
    sprite: Option<Sprite>,
    // properties -- fixed properties
    property: TileProperty,
    // which one is tree?
}

#[derive(Debug)]
enum SpriteType {
    Item,
    Trainer,
    Person, // Not a battler
    Boulder,
}

#[derive(Debug)]
struct Sprite {
    x_coord: u8,
    y_coord: u8,
    sprite_type: SpriteType,
}

struct Warp {}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
enum TileProperty {
    // Grass,
    Walkable,
    NonWalkable,
    Warp(Coordinate),
    Ledge(Direction),
    // Water,
    // Tree,
    // Spinner(Direction),
}
// waterfall?
// elevators / movements that arent warp based
// Spinner tiles -- viridian gym and rocket hideout
// TODO -- populate sprites from memory
//
fn squares_from_files(map_header_file: &str, map_data_file: &str) -> Vec<(Coordinate, Square)> {
    // get tile/bockset
    // get MAP ID
    // get height and width
    debug!("From {} {}", map_header_file, map_data_file);
    let header_data = read_to_string(map_header_file).unwrap();
    let re = Regex::new(r"db ([A-Z_0-9]*) ").unwrap();
    let caps = re.captures(&header_data).unwrap();
    let tileset = caps.get(1).map_or("", |m| m.as_str());

    // We need to manually rename a few entries -- unclear why
    let tileset = match tileset {
        "DOJO" => "gym",
        "MART" => "pokecenter",
        "FOREST_GATE" => "gate",
        "MUSEUM" => "gate",
        "REDS_HOUSE_2" => "reds_house",
        "REDS_HOUSE_1" => "reds_house",
        _ => tileset,
    };

    // get height and width. inefficient but who cares
    // get map id also
    let coord_data = read_to_string("maps/map_constants.asm").unwrap();
    let re =
        Regex::new(r"mapconst ([0-9A-Z_]*), *([0-9]*), *([0-9]*) ; \$([A-Z0-9][A-Z0-9])").unwrap();
    let mut map_width: usize = 0;
    let mut map_height: usize = 0;
    let mut map_id: u8 = 0;
    for cap in re.captures_iter(&coord_data) {
        if &cap[1].to_lowercase().replace("_", "")
            == &map_header_file[13..map_header_file.len() - 4].to_lowercase()
        {
            map_height = cap[2].parse().unwrap();
            map_width = cap[3].parse().unwrap();
            map_id = u8::from_str_radix(&cap[4], 16).unwrap();
        }
    }
    debug!(
        "map id: {} width: {} height: {}",
        map_id, map_width, map_height
    );

    let block_file_name = format!("maps/blocksets/{}.bst", tileset.to_lowercase());
    let block_file = read(block_file_name).unwrap();
    let collision_file_name = format!("maps/tilecolls/{}.tilecoll", tileset.to_lowercase());
    let collision_file = read(collision_file_name).unwrap();
    let walkable_tiles: HashSet<&u8> = collision_file.iter().collect();
    let blocks: Vec<&[u8]> = block_file.chunks(16).collect();
    // get WALKABLE, GRASS, ?
    //
    // Still new to this library -- this is probably sloppy
    let mut map_data = read(map_data_file).unwrap();
    let mut tile_array: Array2<u8> = Array2::zeros((map_height * 4, map_width * 4));
    for (i, mut chunk) in tile_array.exact_chunks_mut((4, 4)).into_iter().enumerate() {
        chunk.assign(
            &Array2::from_shape_vec((4, 4), blocks[map_data[i as usize] as usize].to_vec())
                .unwrap(),
        );
    }
    // create tile properties
    // create
    let mut values = vec![];
    for (x, i) in tile_array.axis_iter(Axis(0)).enumerate() {
        for (y, j) in i.iter().enumerate() {
            if x % 2 == 1 && y % 2 == 0 {
                // Bottom left corner of each tile checks collision data
                let walkable = walkable_tiles.contains(j);
                if walkable {
                    values.push(TileProperty::Walkable);
                } else if tileset == "OVERWORLD" {
                    // Ledge tiles
                    let t = match j {
                        55 => Some(TileProperty::Ledge(Direction::Down)),
                        54 => Some(TileProperty::Ledge(Direction::Down)),
                        39 => Some(TileProperty::Ledge(Direction::Left)),
                        13 => Some(TileProperty::Ledge(Direction::Right)),
                        29 => Some(TileProperty::Ledge(Direction::Right)),
                        _ => None,
                    };
                    if let Some(t) = t {
                        values.push(t);
                    } else {
                        values.push(TileProperty::NonWalkable);
                    }
                } else {
                    values.push(TileProperty::NonWalkable);
                }
            }
        }
    }

    // I think my indeces are messed up
    let squares: Array2<TileProperty> =
        Array2::from_shape_vec((map_height * 2, map_width * 2), values).unwrap();
    print_map(&squares);
    let mut result = vec![];
    for (y, i) in squares.axis_iter(Axis(0)).enumerate() {
        for (x, tile_prop) in i.iter().enumerate() {
            let coord = Coordinate {
                map_id: map_id,
                x: x as u8,
                y: y as u8,
            };
            result.push((
                coord,
                Square {
                    coordinate: coord,
                    sprite: None,
                    property: *tile_prop,
                },
            ));
        }
    }
    result
}

fn print_map(map: &Array2<TileProperty>) {
    for (y, i) in map.axis_iter(Axis(0)).enumerate() {
        for (x, j) in i.iter().enumerate() {
            let vis = match j {
                TileProperty::Walkable => "░",
                TileProperty::NonWalkable => "█",
                TileProperty::Ledge(_) => "═",
                _ => "",
            };
            print!("{}", vis);
        }
        print!("\n");
    }
}

pub fn initialize_maps() {
    let mut world = World {
        data: HashMap::new(),
    };
    for map_header in read_dir("maps/headers").unwrap() {
        let header_path = map_header.unwrap().path();
        let name = header_path.file_stem().unwrap().to_str().unwrap();
        if !name.contains("Copy") && !name.contains("UndergroundPathNorthSouth") {
            // TODO -- fix undegroundpathnorth south
            let data_file = &format!("maps/data/{}.blk", name);
            for (coord, square) in squares_from_files(header_path.to_str().unwrap(), data_file) {
                world.data.insert(coord, square);
            }
        }
    }
    // TODO work this out https://docs.rs/pathfinding/2.0.4/pathfinding/directed/dfs/fn.dfs.html
    println!("path"); // takes 5 seconds...
    let path = dfs(
        Coordinate {
            map_id: 12,
            x: 10,
            y: 34,
        },
        |x| {
            world.successors(&x)
                .into_iter()
        },
        |c| {
            *c == Coordinate {
                map_id: 12,
                x: 10,
                y: 0,
            }
        },
    );
    println!("{:?}", path);
    //
}
