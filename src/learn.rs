#[macro_use]
extern crate lazy_static;

mod engine;
mod board;
// use engine::first_engine::*;
// use engine::random_engine::*;
use engine::alpha_beta_engine::*;
use board::state::*;
use board::hash::*;
use std::io::{self, Write};

const LOOP_MAX: i32 = 1000;
const NORMALIZE_TURNS: i32 = 100;

fn main() {
    let mut engine = AlphaBetaEngine::new();
    let mut counter = 0;
    while counter < LOOP_MAX {
        engine.state = State::new();
        unsafe {
            HASH_TABLE = [HASH_ENTRY_NONE; HASH_TABLE_SIZE];
        }
        engine.randomize_state();
        if engine.proceed_move_learn() {
            if counter % NORMALIZE_TURNS + 1 == NORMALIZE_TURNS {
                engine.evaluator.normalize(NORMALIZE_TURNS);
            }
            counter += 1;
            print!("\r{} / {} finished", counter + 1, LOOP_MAX);
            io::stdout().flush().unwrap();
        }
    }
    engine.evaluator.save_pps();
    engine.evaluator.save_ppo();
}
