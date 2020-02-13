use std::fs::read;
use std::io::prelude::*;
use std::collections::HashSet;
use ndarray::{Array2, Array};

// See this document for more understanding about the terminology used here :
// https://bulbapedia.bulbagarden.net/wiki/User:Tiddlywinks/Map_header_data_structure_in_Generation_I

// Conversion between map in ROM and map as stored in memory & working coordinates
struct World {
}

// A single location, eg Pallet Town
struct Map {
    squares: Vec<Vec<Square>>,
    id: u8
}

// Not referenced in the above document - a 16x16 walkable location, the size of the player
struct Square {
    walkable: bool,
    x_coord: u8,
    y_coord: u8,
    grass: bool,
    // TODO: warp
    // mutable information about sprites?
}

fn main() {
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
    // println!("{:?}", pallet_town_array);
    let mut tile_array: Array2<u8> = Array2::zeros((9*4, 10*4));
    for (i, mut chunk) in tile_array.exact_chunks_mut((4, 4)).into_iter().enumerate() {
            chunk.assign(&Array2::from_shape_vec((4,4), blocks[pallet_town[i as usize] as usize].to_vec()).unwrap());
    }
    println!("{:?}", tile_array);
    // do a window function
    // let tiles: Vec<Vec<&[u8]>>  = pallet_town.into_iter()
    //     .map(|b| blocks[b as usize]) // Get 16 8x8 tiles for each block
        // .map(|x| x.chunks(2).collect()) // Break into groups of 4 for each  row
        // .collect();
    // read collision data
    // println!("{:?}", blocks);
    // println!("{:?}", tiles);

}
