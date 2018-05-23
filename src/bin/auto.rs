#[macro_use]
extern crate lazy_static;
extern crate girigiri;

use std::time::Instant;

// use engine::first_engine::*;
// use engine::random_engine::*;
use girigiri::engine::alpha_beta_engine::*;

fn main() {
    let mut black_engine = AlphaBetaEngine::new(10, 10, true); // (depth, time_limit, use_pp)
    let mut white_engine = AlphaBetaEngine::new(10, 10, false);

    let start = Instant::now();
    loop {
        let mv = black_engine.proceed_move();
        if mv.is_null_move() {
            println!("Resign\nWhite won!");
            break;
        }
        white_engine.state.apply_move(&mv);

        let mv = white_engine.proceed_move();
        if mv.is_null_move() {
            println!("Resign\nBlack won!");
            break;
        }
        black_engine.state.apply_move(&mv);
    }
    let elapsed = start.elapsed();
    println!(
        "total time: {}.{:03} sec\n",
        elapsed.as_secs(),
        elapsed.subsec_nanos() / 1_000_000
    );
}
