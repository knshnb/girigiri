use board::state::*;
use board::alpha_beta::*;
use board::move_encode::*;
use board::hash::*;
use std::time::{Duration, Instant};

pub struct AlphaBetaEngine {
    pub state: State,
    // because of pp problem
    // pub evaluator: Evaluator,
    pub depth: u8,
    pub time_limit: Duration,
    pub instant: Instant,
    pub use_pp: bool,
}

impl AlphaBetaEngine {
    pub fn new(depth: u8, time_limit: u64, use_pp: bool) -> AlphaBetaEngine {
        AlphaBetaEngine {
            state: State::new(),
            // because of pp problem
            // evaluator: Evaluator::new(),
            depth: depth,
            time_limit: Duration::new(time_limit, 0),
            instant: Instant::now(),
            use_pp: use_pp,
        }
    }

    pub fn search(&mut self, depth: u8) -> Option<i32> {
        search(self, depth)
    }

    pub fn proceed_move(&mut self) -> Move {
        self.instant = Instant::now();
        println!("{}", self.state);

        let mut mv = NULL_MOVE;
        for depth in 0..self.depth {
            let start = Instant::now();
            let eval = self.search(depth as u8);
            if eval.is_none() {
                break;
            }
            let eval = eval.unwrap();
            unsafe {
                mv = HASH_TABLE[(self.state.hash_key & *HASH_KEY_MASK) as usize].best_move;
            }
            let end = start.elapsed();
            println!("depth: {}, eval: {}, move: ", depth, eval);
            self.state.print_expectation(depth);
            println!(
                "time: {}.{:03} sec\n",
                end.as_secs(),
                end.subsec_nanos() / 1_000_000
            );
            if depth > 0 && mv.is_null_move() { break; }
        }

        self.state.apply_move(&mv);
        // because of pp problem
        // println!("PP score: {}\n", self.evaluator.eval(&self.state));
        mv
    }

    pub fn proceed_move_without_print(&mut self) -> Move {
        let mut mv = NULL_MOVE;
        for depth in 0..self.depth {
            let eval = self.search(depth as u8);
            if eval.is_none() {
                break;
            }
            unsafe {
                mv = HASH_TABLE[(self.state.hash_key & *HASH_KEY_MASK) as usize].best_move;
            }
            if depth > 0 && mv.is_null_move() { break; }
        }

        self.state.apply_move(&mv);
        mv
    }

    pub fn proceed_move_learn(&mut self) -> bool {
        let mut mv = NULL_MOVE;
        let depth = self.depth;
        let eval = self.search(depth).unwrap();
        if eval.abs() > 10000 {
            return false;
        }
        // println!("{}", self.state);
        unsafe {
            mv = HASH_TABLE[(self.state.hash_key & *HASH_KEY_MASK) as usize].best_move;
        }
        // println!("depth: {}, eval: {}, move: ", depth, eval);
        // self.state.print_expectation(depth);

        // let mvpos = mv.to & 0b01111111;
        self.state.apply_move(&mv);
        // because of pp problem
        // self.evaluator
        //    .update(&self.state, (mvpos / 9) as usize, (mvpos % 9) as usize);
        true
    }

    pub fn randomize_state(&mut self) {
        self.state.randomize();
    }
    pub fn is_lose(&self) -> bool {
        self.state.is_lose()
    }
}
