extern crate rand;

use self::rand::Rng;

use board::state::*;

pub struct RandomEngine {
    state: State,
}

impl RandomEngine {
    pub fn new() -> RandomEngine {
        RandomEngine {
            state: State::new(),
        }
    }
    pub fn proceed_move(&mut self) {
        println!("{}", self.state);

        let legal_moves = &self.state.legal_move();
        println!("legal moves: {}", legal_moves.len());
        let random_index = rand::thread_rng().gen_range(0, legal_moves.len());
        let mv = &legal_moves[random_index];
        self.state.print_move(&mv);
        self.state.apply_move(&mv);
    }
}
