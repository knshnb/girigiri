#[macro_use]
extern crate lazy_static;

mod engine;
// use engine::first_engine::*;
// use engine::random_engine::*;
use engine::alpha_beta_engine::*;

const LOOP_MAX: i32 = 1000;
const NORMALIZE_TURNS: i32 = 100;

fn main() {
    let mut engine = AlphaBetaEngine::new();
    let start = Instant::now();
    for i in 0..LOOP_MAX {
        engine.randomize_state();
        engine.proceed_move();
        if i % NORMALIZE_TURNS + 1 == NORMALIZE_TURNS {
            engine.evaluator.normalize();
        }
    }
    engine.evaluator.save_pps();
    engine.evaluator.save_ppo();
}
