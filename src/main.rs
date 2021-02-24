use pokescript::map;

fn main() {
    env_logger::init();
    /* let mut world = map::World::initialize(); */
    /* let path = world.get_path(map::Coordinate{map_id: 22, x: 10, y: 0}, map::Coordinate{map_id: 12, x: 11, y: 32}); */
    let mut game = pokescript::Game::new();
    game.dumb_ai();
    }
