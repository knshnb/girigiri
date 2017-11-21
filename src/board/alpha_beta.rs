use std::i32;
use std::cmp;

use board::move_encode::*;
use board::state::*;
use board::eval::*;

pub fn sub_search(ref mut state: &State, depth: u8, alpha: i32, beta:i32) -> (i32, Move) {
    if depth == 0 {
        (eval(&state), NULL_MOVE)
    } else {
        let moves = state.legal_move();
        let mut best_pair = (-(i32::max_value()), NULL_MOVE);
        for mv in moves {
            let mut new_state = state.clone();
            new_state.apply_move(&mv);
            new_state.change_color();
            let new_pair = sub_search(&new_state, depth - 1, -beta, -cmp::max(alpha, best_pair.0));
            let new_pair = (-new_pair.0, new_pair.1);
            if new_pair.0 > best_pair.0 {
                best_pair = (new_pair.0, mv);
            }
            if best_pair.0 >= beta {
                break;
            }
        }
        best_pair
    }
}
pub fn search(ref mut state: &State, depth: u8) -> (i32, Move) {
    sub_search(&state, depth, -(i32::max_value()), i32::max_value())
}
