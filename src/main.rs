use env_logger;
use log::{debug, info, warn};
use memory::*;
use mooneye_gb::config::{Bootrom, Cartridge, Model};
use mooneye_gb::GbKey;
use mooneye_gb_frontend;
use mooneye_gb_frontend::frontend::{InGameState, SdlFrontend};
use std::env;
use std::path::Path;

pub mod battle;
pub mod map;
pub mod memory;

struct Game {
    pub frontend: SdlFrontend,
    pub state: InGameState,
    // keep other ai stuff here
}

// use as input to Movement AI
// current map
// visited maps (vector)
// sprites iteracted with (vector)
//
// Then generate weights for each probability
//
// milestones ?
//
// use as input to battle AI
// Health of current pokemon
// Health of opponent pokemon

// Decisions become executed, can succeed or fail
// enum MovementDecision {
//     InteractWithPerson(sprite_number),
//     GoToWarp(warp),
//     UseItem(item),
//     // CatchPokemon,
//     // SwitchBoxPokemon,
// }

// enum BattleDecision {
//     UseItem(item),
//     Move(move_),
//     Run,
//     Switch(pokemon)
// }

impl Game {
    fn new() -> Self {
        let bootrom = Bootrom::lookup(&[]);
        let path = env::var("POKEMON_CARTRIDGE").unwrap();
        let cartridge = Cartridge::from_path(&Path::new(&path)).unwrap();
        let mut frontend = SdlFrontend::init().unwrap();
        let state = frontend.load_cartridge(bootrom, Some(cartridge)).unwrap();
        Game {
            frontend: frontend,
            state: state,
        }
    }

    fn run_frames(&mut self, num: u32, draw: bool) {
        // headless gives me about 2x speedup
        for _ in 0..num {
            if draw {
                self.frontend.next_frame(&mut self.state, true).unwrap();
            } else {
                self.frontend.next_frame(&mut self.state, false).unwrap();
            }
        }
    }

    fn read_addr(&self, address: u16) -> u8 {
        return self.state.machine.hardware.work_ram.read_lower(address);
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
        for _ in 0..50 {
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

    fn dumb_ai(&mut self) {
        match memory::IS_IN_BATTLE {
            1 => run_battle(), // wild
            2 => run_battle(), // trainer
            _ => main_ai()
        }
    }

    pub fn run_battle() {}

    pub fn main_ai() {}
}

fn main() {
    env_logger::init();
    let mut world = map::World::initialize();
    // let mut game = Game::new();
    // game.run();
}
