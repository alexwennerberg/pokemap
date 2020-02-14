use std::fs::read;
use std::fs::read_dir;
use std::io::prelude::*;
use std::collections::{HashSet, HashMap};
use ndarray::{Axis, Array2, Array};

// See this document for more understanding about the terminology used here :
// https://bulbapedia.bulbagarden.net/wiki/User:Tiddlywinks/Map_header_data_structure_in_Generation_I

// Conversion between map in ROM and map as stored in memory & working coordinates
struct World {
    squares: Vec<Vec<Square>>,
}

struct Warp {
}

struct Sprite {
}
// Not referenced in the above document - a 16x16 walkable location, the size of the player
struct Square {
    walkable: bool,
    map_id: u8,
    x_coord: u8,
    y_coord: u8,
    grass: bool,
    sprite: Option<Sprite>,
    warp: Option<Warp>
    // TODO: warp
    // mutable information about sprites?
}

impl Square {
    fn successors(&self) {// -> Vec<Square> {
    if let Some(w) = &self.warp {
    }
    else {
    }
    }

}

fn get_squares(world_file: &str) {
    // get tile/bockset
    // get height and width
    // width = 9
    // height = 10
    let block_file = read("overworld.bst").unwrap();
    let collision_file = read("overworld.tilecoll").unwrap();
    let walkable_tiles: HashSet<&u8> = collision_file.iter().collect();
    let blocks: Vec<&[u8]> = block_file.chunks(16).collect();

    // get WALKABLE, GRASS, ?
    //
    // Still new to this library -- this is probably sloppy
    let mut pallet_town = read("PalletTown.blk").unwrap();
    println!("{:?}", pallet_town_array);
    let mut tile_array: Array2<u8> = Array2::zeros((9*4, 10*4));
    for (i, mut chunk) in tile_array.exact_chunks_mut((4, 4)).into_iter().enumerate() {
            chunk.assign(&Array2::from_shape_vec((4,4), blocks[pallet_town[i as usize] as usize].to_vec()).unwrap());
    }

    for (x, i) in tile_array.axis_iter(Axis(1)).enumerate() {
        for (y, j) in i.iter().enumerate() {
            if x % 2 == 0 && y % 2 == 1 {
                println!("{} {}", x/2,y/2)
                // Bottom left corner of each tile
            }
        }
    }
    println!("{:?}", tile_array);
}

fn main() {
    for map_file in read_dir("/home/alex/dev/pokered/data/mapHeaders").unwrap() {
        println!("{:?}", map_file);
    }
}
