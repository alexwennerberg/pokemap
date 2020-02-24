use log::debug;
use ndarray::{Array, Array2, Axis};
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

// TODO -- figure out ledges

struct Square {
    walkable: bool,
    map_id: u8,
    x_coord: u8,
    y_coord: u8,
    grass: bool,
    sprite: Option<Sprite>,
    // TODO: warp
    // mutable information about sprites?
}

enum SpriteType {
    Item,
    Trainer,
    Person, // Not a battler
    Boulder,
}

struct Sprite {
    x_coord: u8,
    y_coord: u8,
    sprite_type: SpriteType,
}
// struct World {
//     squares: Vec<Vec<Square>>,
// }

struct Warp {}

impl Square {
    fn successors(&self) -> Vec<Square> {
        vec![]
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum TileProperties {
    // Grass,
    Walkable,
    NonWalkable,
    Warp(u8, u8, u8),
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

struct Map {
    squares: HashMap<(u16, u16), Square>,
}

impl Map {
    // this is a mess -- clean up later
    fn from_files(map_header_file: &str, map_data_file: &str) {
        // get tile/bockset
        // get height and width
        debug!("From {} {}", map_header_file, map_data_file);
        let header_data = read_to_string(map_header_file).unwrap();
        let re = Regex::new(r"db ([A-Z_0-9]*) ").unwrap();
        let caps = re.captures(&header_data).unwrap();
        let tileset = caps.get(1).map_or("", |m| m.as_str());

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
        let coord_data = read_to_string("maps/map_constants.asm").unwrap();
        let re = Regex::new(r"mapconst ([0-9A-Z_]*), *([0-9]*), *([0-9]*)").unwrap();
        let mut map_width: usize = 0;
        let mut map_height: usize = 0;
        for cap in re.captures_iter(&coord_data) {
            if &cap[1].to_lowercase().replace("_", "")
                == &map_header_file[13..map_header_file.len() - 4].to_lowercase()
            {
                map_height = cap[2].parse().unwrap();
                map_width = cap[3].parse().unwrap();
            }
        }
        debug!("width: {} height: {}", map_width, map_height);

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
                        values.push(TileProperties::Walkable);
                    } else if tileset == "OVERWORLD" {
                        // Ledge tiles
                        let t = match j {
                            55 => Some(TileProperties::Ledge(Direction::Down)),
                            54 => Some(TileProperties::Ledge(Direction::Down)),
                            39 => Some(TileProperties::Ledge(Direction::Left)),
                            13 => Some(TileProperties::Ledge(Direction::Right)),
                            29 => Some(TileProperties::Ledge(Direction::Right)),
                            _ => None,
                        };
                        if let Some(t) = t {
                            values.push(t);
                        } else {
                            values.push(TileProperties::NonWalkable);
                        }
                    } else {
                        values.push(TileProperties::NonWalkable);
                    }
                }
            }
        }

        // I think my indeces are messed up
        let squares: Array2<TileProperties> =
            Array2::from_shape_vec((map_height * 2, map_width * 2), values).unwrap();
        if map_header_file == "maps/headers/Route1.asm" {
            for (x, i) in squares.axis_iter(Axis(0)).enumerate() {
                for (y, j) in i.iter().enumerate() {
                    let vis = match j {
                        TileProperties::Walkable => "░",
                        TileProperties::NonWalkable => "█",
                        TileProperties::Ledge(_) => "═",
                        _ => "",
                    };
                    print!("{}", vis);
                }
                print!("\n");
            }
            panic!();
        }
        // Iterate over squares
        //
    }
}

pub fn initialize_maps() {
    for map_header in read_dir("maps/headers").unwrap() {
        let header_path = map_header.unwrap().path();
        let name = header_path.file_stem().unwrap().to_str().unwrap();
        if !name.contains("Copy") && !name.contains("UndergroundPathNorthSouth") {
            // TODO -- fix undegroundpathnorth south
            let data_file = &format!("maps/data/{}.blk", name);
            let map = Map::from_files(header_path.to_str().unwrap(), data_file);
        }
    }
}
