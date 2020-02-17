use env_logger;
use log::{info, warn, debug};
use std::env;
use std::path::Path;
use mooneye_gb::config::{Bootrom, Cartridge, Model};
use mooneye_gb::GbKey;
use mooneye_gb_frontend;
use mooneye_gb_frontend::frontend::{SdlFrontend, InGameState};
use std::fs::read;
use std::collections::{HashSet, HashMap};
use ndarray::{Axis, Array2, Array};
use memory::*;

pub mod memory;

// See this document for more understanding about the terminology used here :
// https://bulbapedia.bulbagarden.net/wiki/User:Tiddlywinks/Map_header_data_structure_in_Generation_I

// TODO -- figure out ledges 

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
// TODO -- every map ID
enum Map {
    A,
    B,
}

// impl Map {
// fn value(&self) -> i32 {
//     match *self {
//         MyEnum::A => 123,
//         MyEnum::B => 456,
//     }
// }
// }

struct World {
    squares: Vec<Vec<Square>>,
}

struct Warp {
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
// Not referenced in the above document - a 16x16 walkable location, the size of the player
//
//
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

// impl Square {
//     fn successors(&self) {// -> Vec<Square> {
//     if let Some(w) = &self.warp {
//     }
//     else {
//     }
//     }

// }

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

struct Game {
    pub frontend: SdlFrontend,
    pub state: InGameState
        // keep other ai stuff here
}

impl Game {
    fn new() -> Self {
        let bootrom = Bootrom::lookup(&[]);
        let path = env::var("POKEMON_CARTRIDGE").unwrap();
        let cartridge = Cartridge::from_path(&Path::new(&path)).unwrap();
        let mut frontend = SdlFrontend::init().unwrap();
        let state = frontend.load_cartridge(bootrom, Some(cartridge)).unwrap();
        Game {
            frontend: frontend,
            state: state
        }

    }

    fn next_frame(&mut self) {
        self.frontend.next_frame(&mut self.state).unwrap();
    }

    fn read_addr(&self, address: u16) -> u8 {
        return self.state.machine.hardware.work_ram.read_lower(address)
    }
    fn key_down(&mut self, key: GbKey) {
        self.state.machine.key_down(key);
    }
    fn key_up(&mut self, key: GbKey) {
        self.state.machine.key_up(key);
    }

    /// Navigate beginning of game -- naming characters, etc
    fn navigate_load_screen(&mut self) {
        debug!("Going through startup dialogue with Prof Oak");
       for _ in 0..150 { 
            // first iteration -- just smash A for a while
            self.key_down(GbKey::A);
            self.next_frame();
            self.key_up(GbKey::A);
            self.next_frame();
        }
       for _ in 0..100{
           self.key_down(GbKey::B);
           self.next_frame();
           self.key_up(GbKey::B);
           self.next_frame();
       }
    }

    fn run(&mut self) {
        self.navigate_load_screen();
    }
}

fn main() {
    env_logger::init();
    let mut game = Game::new();
    game.run();
}
