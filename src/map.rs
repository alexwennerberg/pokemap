use ndarray::{Axis, Array2, Array};
use std::collections::{HashSet, HashMap};
use std::fs::{read, read_to_string, read_dir};
use regex::Regex;

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
    Boulder
}

struct Sprite {
    x_coord: u8,
    y_coord: u8,
    sprite_type: SpriteType
}
// struct World {
//     squares: Vec<Vec<Square>>,
// }


struct Warp {
}

impl Square {
    fn successors(&self) -> Vec<Square> {
        vec![]
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
enum TileProperties {
    Grass,
    Warp(u8,u8,u8),
    Ledge(Direction), 
    Water,
    Tree,
    Spinner(Direction),
}
// waterfall?
// elevators / movements that arent warp based
// Spinner tiles -- viridian gym and rocket hideout
// TODO -- populate sprites from memory 
//

struct Map {
    square: Vec<Square>
}

impl Map {
    fn from_files(map_header_file: &str, map_data_file: &str) {
        // get tile/bockset
        // get height and width
        let header_data = read_to_string(map_header_file).unwrap();
        let re = Regex::new(r"db ([A-Z]*) ").unwrap();
        let caps = re.captures(&header_data).unwrap();
        let map_width = 10; // NEXT
        let map_height = 10;
        let tileset =  caps.get(1).map_or("", |m| m.as_str());

        let tileset = match tileset {
            "DOJO" => "gym",
            "MART" => "pokecenter",
            "FOREST_GATE" => "gate",
            _ => tileset
        };
        let block_file_name = format!("maps/blocksets/{}.bst", tileset.to_lowercase());
        let block_file = read(block_file_name).unwrap();
        let collision_file_name = format!("maps/tilecolls/{}.tilecoll", tileset.to_lowercase());
        let collision_file = read(collision_file_name).unwrap();
        let walkable_tiles: HashSet<&u8> = collision_file.iter().collect();
        let blocks: Vec<&[u8]> = block_file.chunks(16).collect();
        // get WALKABLE, GRASS, ?
        //
        // Still new to this library -- this is probably sloppy
        let mut pallet_town = read(map_data_file).unwrap();
        let mut tile_array: Array2<u8> = Array2::zeros((map_width*4, map_height*4));
        println!("{}", map_header_file);
        for (i, mut chunk) in tile_array.exact_chunks_mut((4, 4)).into_iter().enumerate() {
                chunk.assign(&Array2::from_shape_vec((4,4), blocks[pallet_town[i as usize] as usize].to_vec()).unwrap());
        }
        for (x, i) in tile_array.axis_iter(Axis(1)).enumerate() {
            for (y, j) in i.iter().enumerate() {
                if x % 2 == 0 && y % 2 == 1 {
                    // Bottom left corner of each tile
                }
            }
        }
        println!("{:?}", tile_array);
    }
}

pub fn initialize_maps() {
    for map_header in read_dir("maps/headers").unwrap() {
        let header_path = map_header.unwrap().path();
        let name = header_path.file_stem().unwrap().to_str().unwrap();
        let data_file = &format!("maps/data/{}.blk", name);
        let map = Map::from_files(header_path.to_str().unwrap(), data_file);
    }
}
