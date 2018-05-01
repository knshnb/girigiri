use board::state::*;
use board::alpha_beta::*;
use board::move_encode::*;
use board::hash::*;
use board::eval::*;
use std::time::Instant;

pub struct AlphaBetaEngine {
    pub state: State,
    pub evaluator: Evaluator,
}

impl AlphaBetaEngine {
    pub fn new() -> AlphaBetaEngine {
        AlphaBetaEngine {
            state: State::new(),
            evaluator: Evaluator::new(),
        }
    }
    pub fn proceed_move(&mut self) -> Move {
        println!("{}", self.state);

        let mut mv = NULL_MOVE;
        for depth in 0..5 {
            let start = Instant::now();
            let eval = search(&mut self.state, depth as u8);
            unsafe {
                mv = HASH_TABLE[(self.state.hash_key & HASH_KEY_MASK) as usize].best_move;
            }
            let end = start.elapsed();
            println!("depth: {}, eval: {}, move: ", depth, eval);
            self.state.print_expectation(depth);
            println!(
                "time: {}.{:03} sec\n",
                end.as_secs(),
                end.subsec_nanos() / 1_000_000
            );
        }

        self.state.apply_move(&mv);
        println!("PP score: {}", self.evaluator.eval(&self.state));
        mv
    }

    pub fn proceed_move_learn(&mut self) -> bool {
        let mut mv = NULL_MOVE;
        let depth = 3;
        let eval = search(&mut self.state, depth as u8);
        if eval.abs() > 10000 {
            return false;
        }
        // println!("{}", self.state);
        unsafe {
            mv = HASH_TABLE[(self.state.hash_key & HASH_KEY_MASK) as usize].best_move;
        }
        // println!("depth: {}, eval: {}, move: ", depth, eval);
        // self.state.print_expectation(depth);

        let mvpos = mv.to & 0b01111111;
        self.state.apply_move(&mv);
        self.state.color = true;
        self.evaluator
            .update(&self.state, (mvpos / 9) as usize, (mvpos % 9) as usize);
        true
    }

    pub fn randomize_state(&mut self) {
        self.state.randomize();
    }
    pub fn is_lose(&self) -> bool {
        self.state.is_lose()
    }
}
