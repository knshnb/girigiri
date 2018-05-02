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
    let mut black_engine = AlphaBetaEngine::new(10, 10, true);  // (depth, time_limit, use_pp)
    let mut white_engine = AlphaBetaEngine::new(10, 10, false);

    let start = Instant::now();
    loop {
        if black_engine.is_lose() {
            if black_engine.state.color {
                println!("White won!");
            } else {
                println!("Black won!");
            }
            break;
        }

        if black_engine.state.color {
            white_engine.state.apply_move(&black_engine.proceed_move());
        } else {
            black_engine.state.apply_move(&white_engine.proceed_move());
        }
    }
    let elapsed = start.elapsed();
    println!(
        "total time: {}.{:03} sec\n",
        elapsed.as_secs(),
        elapsed.subsec_nanos() / 1_000_000
    );
}
