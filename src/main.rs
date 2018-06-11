extern crate cursive;

mod engine;

use cursive::traits::*;
use cursive::views::TextView;
use cursive::Cursive;
use cursive::XY;
use engine::Engine;

const WORLD_SIZE: (usize, usize) = (20, 10);

fn main() {
    let engine = Engine::new(String::from("world"), XY::new(WORLD_SIZE.0, WORLD_SIZE.1));

    engine.run();
}
