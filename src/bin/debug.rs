extern crate lazy_static;
extern crate girigiri;

use std::io;

use girigiri::board::state::State;
use girigiri::board::move_encode::*;

fn main() {
    let mut state = State::new();
    let mut mv = NULL_MOVE;
    loop {
        println!("{:?}", state);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        let bytes = input.as_bytes();
        if bytes.len() == 2 {
            // undo
            state.print_move(&mv);
            state.undo_move(&mv);
        } else {
            mv = Move::from_usi(&input);
            state.print_move(&mv);
            state.apply_move(&mv);
        }
    }
}
