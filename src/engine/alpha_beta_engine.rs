use board::state::*;
use board::alpha_beta::*;
use board::move_encode::*;
use std::time::Instant;

pub struct AlphaBetaEngine {
    pub state: State,
}

impl AlphaBetaEngine {
    pub fn new() -> AlphaBetaEngine {
        AlphaBetaEngine {
            state: State::new()
        }
    }
    pub fn proceed_move(&mut self) {
        self.state.print();

        let mut pair = (0, NULL_MOVE);
        for depth in 0..5 {
            let start = Instant::now();
            pair = search(&mut self.state, depth as u8);
            let end = start.elapsed();
            println!("depth: {}, eval: {}, move: ", depth, pair.0);
            self.state.print_move(&pair.1);
            println!("time: {}.{:03} sec\n", end.as_secs(), end.subsec_nanos() / 1_000_000);
        }

        self.state.apply_move(&pair.1);
    }
    pub fn is_lose(&self) -> bool {
        self.state.is_lose()
    }
}
