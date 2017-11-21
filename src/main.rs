use std::io;
use std::time::Instant;

mod engine;
mod board;
// use engine::first_engine::*;
// use engine::random_engine::*;
use engine::alpha_beta_engine::*;

fn main() {
    let mut n = 1;
    let mut input = String::new();
    let mut engine = AlphaBetaEngine::new();
    let start = Instant::now();
    loop {
        if engine.is_lose() {
            println!("White won!");
            break;
        }
        println!("{}th move", n);
        engine.proceed_move();
        n += 1;

        if engine.is_lose() {
            println!("Black won!");
            break;
        }
        println!("{}th move", n);
        engine.proceed_move();
        n += 1;
    }
    let elapsed = start.elapsed();
    println!("total time: {}.{:03} sec\n", elapsed.as_secs(), elapsed.subsec_nanos() / 1_000_000);
}
