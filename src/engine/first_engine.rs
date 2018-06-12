use shogi::state::*;

pub struct FirstEngine {
    state: State,
}

impl FirstEngine {
    pub fn new() -> FirstEngine {
        FirstEngine {
            state: State::new(),
        }
    }
    pub fn proceed_move(&mut self) {
        println!("{}", self.state);

        let legal_moves = &self.state.legal_move();
        println!("legal moves: {}", legal_moves.len());
        let mv = &legal_moves[0]; // pick FIRST move!

        self.state.print_move(&mv);
        self.state.apply_move(&mv);
    }
}
