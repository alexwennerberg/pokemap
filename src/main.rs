use env_logger;
use log::{info, warn, debug};
use std::env;
use std::path::Path;
use mooneye_gb::config::{Bootrom, Cartridge, Model};
use mooneye_gb::GbKey;
use mooneye_gb_frontend;
use mooneye_gb_frontend::frontend::{SdlFrontend, InGameState};
use memory::*;

pub mod memory;
pub mod map;

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

    fn run_frames(&mut self, num: u32, draw: bool) {
        // headless gives me about 2x speedup
        for _ in 0..num {
            if draw {
                self.frontend.next_frame(&mut self.state, true).unwrap();
            }
            else {
                self.frontend.next_frame(&mut self.state, false).unwrap();
            }
        }
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
       for i in 0..800 { 
            // first iteration -- just smash A for a while
            self.key_down(GbKey::A);
            self.run_frames(1, false);
            self.key_up(GbKey::A);
            self.run_frames(1, false);
        }
       for _ in 0..50{
           self.key_down(GbKey::B);
           self.run_frames(1, false);
           self.key_up(GbKey::B);
           self.run_frames(1, false);
       }
    }
    fn walk_randomly(&mut self) {
        loop {
            let walk_length = 2;
            self.key_down(GbKey::Right);
            self.run_frames(walk_length, true);
            self.key_up(GbKey::Right);
            self.run_frames(walk_length, true);
            self.key_down(GbKey::Down);
            self.run_frames(walk_length, true);
            self.key_up(GbKey::Down);
            self.run_frames(walk_length, true);
            self.key_down(GbKey::Left);
            self.run_frames(walk_length, true);
            self.key_up(GbKey::Left);
            self.run_frames(walk_length, true);
            self.key_down(GbKey::Up);
            self.run_frames(walk_length, true);
            self.key_up(GbKey::Up);
            self.run_frames(walk_length, true);
        }
    }

    fn run(&mut self) {
        self.navigate_load_screen();
        self.walk_randomly();
    }
}

fn main() {
    env_logger::init();
    map::initialize_maps();
    // let mut game = Game::new();
    // game.run();
}
