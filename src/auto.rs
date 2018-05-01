#[macro_use]
extern crate lazy_static;

use std::time::Instant;

mod csa;
mod engine;
mod board;
// use engine::first_engine::*;
// use engine::random_engine::*;
use engine::alpha_beta_engine::*;

fn main() {
    let mut engine = AlphaBetaEngine::new();
    let start = Instant::now();
    loop {
        if engine.is_lose() {
            if engine.state.color { println!("White won!"); }
            else { println!("Black won!"); }
            break;
        }
        engine.proceed_move();
    }
    let elapsed = start.elapsed();
    println!("total time: {}.{:03} sec\n", elapsed.as_secs(), elapsed.subsec_nanos() / 1_000_000);
}
